use std::{fs, env, collections::{HashMap, VecDeque}};

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn setup<'a>(content: &'a String, all_modules: &mut HashMap<&'a str, Module<'a>>, part2_module: &mut &'a str) {
    let re_module = Regex::new(r"^([b%&])(\w+) -> ((?:\w(?:, )?)+)$").unwrap();
    let re_dest = Regex::new(r"\w+").unwrap();
    let mut conjonction_modules = vec!();
    for line in content.lines() {
        let module = Module::parse(line, &re_module, &re_dest);
        if module.dest.contains(&"rx") { *part2_module = module.name; }
        if let ModuleType::Conjonction(_) = module.mod_type {
            conjonction_modules.push(module.name);
        }
        all_modules.insert(module.name, module);
    }
    for name in conjonction_modules {
        let mut pulses = vec!();
        all_modules.values().filter(|module| module.dest.contains(&name)).for_each(|module| {
            pulses.push((module.name, Pulse::Low));
        });
        let Some(module) = all_modules.get_mut(name) else {continue;};
         module.mod_type = ModuleType::Conjonction(pulses);
    }
}

fn compute(content: &String) {
    let mut all_modules = HashMap::new();
    let mut part2_module = "";
    setup(content, &mut all_modules, &mut part2_module);

    let mut pulses_counter = (1000,0); // Start with the 1000 button push low pulse
    let mut pulses: VecDeque<(&str, &str, Pulse)> = VecDeque::new();

    let mut loops = HashMap::new(); // Part 2 loop setup
    if let Some(kz) = all_modules.get(part2_module) {
        if let ModuleType::Conjonction(source) = &kz.mod_type {
            source.iter().for_each(|(name,_)| {loops.insert(*name, -1);});
        }
    }
    let mut i = 0;
    loop {
        i+=1;
        pulses.push_front(("button", "roadcaster", Pulse::Low));
        while let Some((origin, target, pulse)) = pulses.pop_front() {
            let Some(module) = all_modules.get_mut(target) else {continue};
            if target == part2_module && pulse == Pulse::High { // ==== Part 2 loop detection
                loops.insert(origin, i);
                if !loops.values().any(|v| *v < 0) {
                    println!("The fewest number of button press to send a low pulse to rx is: {}", loops.values().fold(1u64, |acc,v| acc*(*v as u64)));
                    return;
                }
            }
            let Some(exit_pulse) = module.receive(pulse, origin) else {continue};
            if i <= 1000 { // ==== Part 1 count
                match exit_pulse {
                    Pulse::Low  => pulses_counter.0 += module.dest.len() as i32,
                    Pulse::High => pulses_counter.1 += module.dest.len() as i32,
                }
            }
            module.dest.iter().for_each(|dest| {
                pulses.push_back((target, dest, exit_pulse));
            });
        }
        if i == 1000 {
            println!("The total number of pulses is: {:?}", pulses_counter);
            println!("The product of pulse count is: {}", pulses_counter.0*pulses_counter.1);
        }
    }
}

#[derive(Debug)]
struct Module<'a>{
    name: &'a str,
    mod_type: ModuleType<'a>,
    dest: Vec<&'a str>,
}

impl <'a> Module<'a> {
    fn parse(s: &'a str, re: &Regex, re_dest: &Regex) -> Module<'a> {
        let [mod_type, name, dest] = re.captures(s).expect("Regex should match").extract().1;
        Module {
            name,
            mod_type: ModuleType::parse(mod_type),
            dest: re_dest.find_iter(dest).map(|m| m.as_str()).collect::<Vec<&str>>(),
        }

    }
    fn receive(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        match &mut self.mod_type {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlidFlop(status) =>
                match pulse  {
                    Pulse::High => None,
                    Pulse::Low => {
                        *status = !*status;
                        Some(if *status {Pulse::High} else {Pulse::Low})
                    },
                },
            ModuleType::Conjonction(source) => {
                let mut all_high = true;
                for (name, memory) in source.iter_mut() {
                    if *name == from {
                        *memory = pulse;
                    }
                    if *memory == Pulse::Low {
                        all_high = false;
                    }
                }
                Some(if all_high {Pulse::Low} else {Pulse::High})
            },
        }
    }
}

#[derive(Debug)]
enum ModuleType<'a> {
    Broadcaster,
    FlidFlop(bool),
    Conjonction(Vec<(&'a str, Pulse)>),
}

impl <'a> ModuleType<'a> {
    fn parse(s: &str) -> ModuleType {
        match s {
            "b" => ModuleType::Broadcaster,
            "%" => ModuleType::FlidFlop(false),
            "&" => ModuleType::Conjonction(vec!()),
            _ => panic!("Unknown ModuleType {s}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    High, Low
}
