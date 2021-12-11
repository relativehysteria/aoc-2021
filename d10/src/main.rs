use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines();

    let mut error_score  = 0;
    let mut compl_scores = vec![];
    for line in data {
        // Part one
        let score = get_part_one_score(line);
        if score != 0 {
            error_score += score;
            continue;
        }

        // Part two
        compl_scores.push(get_part_two_score(line));
    }

    compl_scores.sort_unstable();
    let compl_score = compl_scores.get(compl_scores.len() / 2).unwrap();
    println!("{}", error_score);
    println!("{}", compl_score);
}

fn get_part_one_score(line: &str) -> usize {
    let mut delims = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => delims.push(c),
            ')' => { if delims.pop() != Some('(') { return 3;     } }
            ']' => { if delims.pop() != Some('[') { return 57;    } }
            '}' => { if delims.pop() != Some('{') { return 1197;  } }
            '>' => { if delims.pop() != Some('<') { return 25137; } }
            ___ => unreachable!(),
        }
    }
    0
}

fn get_part_two_score(line: &str) -> usize {
    let mut delims = vec![];
    for c in line.chars() {
        match c {
            // Here we know that no line is corrupted,
            // so we can simply pop a delim without checking
            ')' | ']' | '}' | '>' => { delims.pop();   },
            '(' | '[' | '{' | '<' => { delims.push(c); },
            _____________________ => { unreachable!(); },
        }
    }

    // Get the score
    let mut score = 0usize;
    for c in delims.iter().rev() {
        score *= 5;
        match c {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            ___ => unreachable!(),
        }
    }
    score
}
