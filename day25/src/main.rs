use std::{fs, env, collections::{HashSet, HashMap, VecDeque}};
use std::hash::{Hash, Hasher};
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let content = fs::read_to_string(file_path)
        .expect("should have been able to read the file");

    compute(&content);
}

fn compute(content: &String) {
    let content = content.replace(':', "");
    let mut nodes_set: HashSet<&str> = HashSet::new();
    let mut links: HashSet<Cable> = HashSet::new();
    content.lines().for_each(|l| {
        let one = &l[0..3];
        nodes_set.insert(one);
        l[4..].split(' ').for_each(|two| {
            links.insert(Cable { one, two});
            nodes_set.insert(two);
        })
    });

    let mut nodes: Vec<&str> = Vec::new();
        for n in &nodes_set {
        nodes.push(n);
    }

    let mut neighbours: HashMap<&str, Vec<&str>> = HashMap::new();
    nodes.iter().for_each(|n| {neighbours.insert(n, vec!());});
    for c in &links {
        if let Some(v) = neighbours.get_mut(c.one) {
            v.push(c.two);
        }
        if let Some(v) = neighbours.get_mut(c.two) {
            v.push(c.one);
        }
    }

    for _ in 0..3 {
        let most = get_most_visited_edge(&neighbours, &nodes, &links);
        remove_cable(&mut neighbours, most);
        println!("Removed cable: {:?}", most);
    }

    if let Some(res) = is_split(&neighbours) {
        println!("The result is: {:?}", res);
    } else {
        println!("Couldn't find the 3 correct cables")
    }
}

fn get_most_visited_edge<'a>(neighbours: &HashMap<&'a str, Vec<&'a str>>, nodes: &Vec<&'a str>, links: &'a HashSet<Cable>) -> &'a Cable<'a> {
    let mut rng = rand::thread_rng();
    let mut marked: HashMap<&Cable, i32> = HashMap::new();
    links.iter().for_each(|c| {marked.insert(c,0);});
    let nb = nodes.len();
    for _ in 0..1_000 {
        let one = rng.gen_range(0..nb);
        let two = rng.gen_range(0..nb);
        if one == two { continue;}
        let p = find_path(&neighbours, nodes[one], nodes[two]);
        for (i,n) in p[1..].iter().enumerate() {
            let i = marked.get_mut(&Cable {one: p[i], two: n}).expect("No marked");
            *i+=1;
        }
    }

    let mut arr = Vec::new();
    for (k,v) in marked {
        arr.push((v,k));
    }
    arr.sort();
    arr.last().unwrap().1
}

fn remove_cable(neighbours: &mut HashMap<&str, Vec<&str>>, cable: &Cable) {
    if let Some(v) = neighbours.get_mut(cable.one) {
        v.retain(|n| n!=&cable.two);
    }
    if let Some(v) = neighbours.get_mut(cable.two) {
        v.retain(|n| n!=&cable.one);
    }
}

fn is_split(neighbours: &HashMap<&str, Vec<&str>>) -> Option<usize> {
    let s = *neighbours.keys().next().expect("No neighbours");
    let mut queue: VecDeque<&str> = VecDeque::from([s]);
    let mut visited: HashSet<&str> = HashSet::from([s]);
    while let Some(node) = queue.pop_front() {
        let Some(neigh) = neighbours.get(node) else {panic!("No neigh")};
        for n in neigh {
            if !visited.contains(n) {
                queue.push_back(n);
                visited.insert(n);
            }
        }
    }
    let total = neighbours.keys().len();
    let len = visited.len();
    if len == total {
        None
    } else {
        Some(len*(total-len))
    }
}

fn find_path<'a>(neighbours: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, end: &'a str) -> Vec<&'a str> {
    let mut queue: VecDeque<&str> = VecDeque::from([start]);
    let mut visited: HashSet<&str> = HashSet::from([start]);
    let mut previous: HashMap<&str, &str> = HashMap::new();
    while let Some(node) = queue.pop_front() {
        let Some(neigh) = neighbours.get(node) else {panic!("No neigh")};
        for n in neigh {
            if n == &end {
                let mut path: Vec<&str> = vec![end];
                let mut current = node;
                while current != start {
                    path.push(current);
                    let Some(temp) = previous.get(current) else {panic!("No previous")};
                    current = temp;
                }
                path.push(current);
                path.reverse();
                return path;
            }
            if !visited.contains(n) {
                queue.push_back(n);
                visited.insert(n);
                previous.insert(n,node);
            }
        }
    }
    panic!("No path found");
}

#[derive(Debug, Eq, PartialOrd, Ord)]
struct Cable<'a> {
    one: &'a str,
    two: &'a str
}

impl<'a> PartialEq for Cable<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.one == other.one && self.two == other.two ||
        self.one == other.two && self.two == other.one
    }
}

impl <'a> Hash for Cable<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (b,c) in self.one.bytes().zip(self.two.bytes()) {
            (b as u16 * c as u16).hash(state);
        }
    }
}
