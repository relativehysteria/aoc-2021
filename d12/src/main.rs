use std::fs::read_to_string;
use std::collections::{ HashSet, HashMap };

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines();

    let mut neighbors: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in data.into_iter() {
        let line = line.split('-').collect::<Vec<&str>>();

        let entry = neighbors.entry(line[0]).or_insert(vec![]);
        entry.push(line[1]);

        let entry = neighbors.entry(line[1]).or_insert(vec![]);
        entry.push(line[0]);
    }

    println!("{}", search(&neighbors, &mut HashSet::new(), "start", true));
}

fn search(
    neighbors: &HashMap<&str, Vec<&str>>,
    seen: &mut HashSet<&str>,
    cave: &str,
    can_visit_twice: bool,
) -> i32 {
    let mut can_visit_twice = can_visit_twice;

    if cave == "end" {
        return 1;
    }

    if seen.contains(cave) {
        if cave == "start" {
            return 0;
        }

        if cave == cave.to_ascii_lowercase() {
            if !can_visit_twice {
                return 0;
            }
            can_visit_twice = false;
        }
    }

    let mut sum = 0;
    for neighbor in neighbors[cave].iter() {
        // IT'S THREE IN THE MORNING AND I HAVEN'T CLONED YET AAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // I MUST CLONE UUAAAAAAAAUUAUGH~AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        let mut seen = seen.clone();
        seen.insert(cave);
        sum += search(neighbors, &mut seen, neighbor, can_visit_twice);
    }

    return sum;
}
