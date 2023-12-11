use std::fs;
use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    first_part(&content);
    second_part(&content);
}

fn second_part(content: &String) {
    let txt = format!("{content}\n");
    let mut lines = txt.lines();

    let re = Regex::new(r"\d+").unwrap();
    let seeds: Vec<i64> = re.find_iter(lines.next().map_or("",|l| l)).map(|m| m.as_str().parse().unwrap_or(0)).collect();
    lines.next();
    lines.next();
    let mut intervals: Vec<MapInterval> = vec!();
    for idx in 0..seeds.len()/2 {
        let i = idx*2;
        intervals.push(MapInterval::new(seeds[i],seeds[i+1], seeds[i]));
    }
    let mut maps: Vec<MapInterval> = vec!();
    for line in lines.into_iter() {
        if line.len() == 0 {
            let mut mapped_intervals: Vec<MapInterval> = vec!();
            let mut i = 0;

            while i<intervals.len() {
                let interval = &intervals[i];
                let mut matched = false;
                for m in maps.iter() {
                    if let (Some(mapped), overflow, overflow2) = m.map_interval(interval) {
                        mapped_intervals.push(mapped);
                        if let Some(over) = overflow {
                            intervals.push(over);
                        }
                        if let Some(over2) = overflow2 {
                            intervals.push(over2);
                        }
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    mapped_intervals.push(intervals[i].clone())
                }
                i+=1;
            }
            intervals = mapped_intervals;
            maps.clear();
            continue;
        }
        if line.contains("map") {continue;}
        let raw_map: Vec<i64> = re.find_iter(line).map(|m| m.as_str().parse().unwrap_or(0)).collect();
        maps.push(MapInterval::new(raw_map[1], raw_map[2], raw_map[0]));
    }

    let total = intervals.iter().fold(intervals[0].mapping, |acc,val| if acc>val.mapping {val.mapping} else {acc});
    println!("Closest location using interval: {total}");
}
#[derive(Debug, Clone)]
struct MapInterval {
    start: i64,
    end: i64,
    lenght: i64,
    mapping: i64,
}

impl MapInterval {
    fn new(start: i64, lenght: i64, mapping: i64) -> MapInterval {
        MapInterval { start, end: start+lenght, lenght, mapping }
    }
    fn map_interval(self: &MapInterval, source: &MapInterval)-> (Option<MapInterval>,Option<MapInterval>,Option<MapInterval>) {
        let mut map = None;
        let mut overflow = None;
        let mut overflow2 = None;
        let source_mapping_end = source.mapping+source.lenght;
        if self.contains(source.mapping){ // map start before
            if source_mapping_end <= self.end { // no overflow at end
                map=Some(MapInterval::new(source.start, source.lenght, self.mapping+source.mapping-self.start));
            }
            else { // over flow at end
                map=Some(MapInterval::new(source.start, self.end - source.mapping, self.mapping+source.mapping-self.start));
                overflow=Some(MapInterval::new(source.start+self.end-source.mapping, source_mapping_end-self.end, self.end));
            }
        } else if source.mapping <= self.start && self.start < source_mapping_end {
            if self.end < source_mapping_end { // overflow at both end
                map=Some(MapInterval::new(source.start+self.start-source.mapping, self.lenght, self.mapping));
                overflow=Some(MapInterval::new(source.start+self.end-source.mapping, source_mapping_end-self.end, self.end));
            }
            else { // no overflow at end
                map=Some(MapInterval::new(source.start+self.start-source.mapping, source_mapping_end-self.start, self.mapping));
            }
            overflow2=Some(MapInterval::new(source.start, self.start-source.mapping, source.mapping));
        }
        return (map,overflow, overflow2);
    }
    fn map(self: &MapInterval, source: i64)->i64 {
        source-self.start+self.mapping
    }
    fn contains(self: &MapInterval, value: i64) -> bool {
        self.start <= value && value < self.end
    }
}


fn first_part(content: &String) {
    let txt = format!("{content}\n");
    let mut lines = txt.lines();

    let re = Regex::new(r"\d+").unwrap();
    let seeds_line = lines.next().map_or("",|l| l);
    let mut seeds: Vec<i64> = re.find_iter(seeds_line).map(|m| m.as_str().parse().unwrap_or(0)).collect();
    lines.next();
    lines.next();
    let mut maps: Vec<MapInterval> = vec!();
    for line in lines.into_iter() {
        if line.len() == 0 {
            let mut mapped_seeds: Vec<i64> = vec!();
            for seed in seeds.iter() {
                let mut matched = false;
                for m in maps.iter() {
                    if m.contains(*seed) {
                        mapped_seeds.push(m.map(*seed));
                        matched = true;
                        break;
                    }
                }
                if !matched {
                    mapped_seeds.push(*seed);
                }
            }
            maps.clear();
            seeds = mapped_seeds;
            continue;
        }
        if line.contains("map") {continue;}
        let raw_map: Vec<i64> = re.find_iter(line).map(|m| m.as_str().parse().unwrap_or(0)).collect();
        maps.push(MapInterval::new(raw_map[1], raw_map[2], raw_map[0]));
    }

    let total = seeds.iter().min().unwrap_or(&0);
    println!("Closest location using seed value: {total}");
}

