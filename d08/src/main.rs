use std::fs::read_to_string;
use std::collections::{ HashSet, HashMap };

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines();

    // Results of this day
    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for line in data {
        let mut entry = line.split(" | ");

        // Get the signal patterns and output values.
        let signals = entry.next().unwrap();
        let outputs = entry.next().unwrap();

        // Turn them into a `HashMap<usize, HashSet<char>>`,
        // where the key (`usize`) is the length of the value (`HashSet<char>`).
        let signals = signals.split(' ')
            .map(|x| {
                (x.len(), x.chars().collect::<HashSet<char>>())
            })
            .collect::<HashMap<usize, HashSet<char>>>();

        // The final number constructed from the outputs
        let mut number = String::with_capacity(outputs.len());

        // Iterate through the output digits and find the correlation between
        // known and unknown digits
        for output in outputs.split(' ') {
            // Turn the output into a `HashSet`, so that we can use
            // `intersection`
            let output = output.chars()
                .collect::<HashSet<char>>();

            // Only the digit 1 has 2 segments
            // Only the digit 4 has 4 segments
            let signal1 = signals.get(&2).unwrap();
            let signal4 = signals.get(&4).unwrap();

            // Find the segment intersection between those digits and this one
            let signal1 = output.intersection(signal1);
            let signal4 = output.intersection(signal4);

            // Now we can identify the digit based solely on the number of
            // intersecting/identical segments with digits 1 and 4.
            // In the case of `1, 7, 4, 8`, we only need the number of segments.
            //
            // For example, the digit `3` has 5 segments, 3 in common with `4`
            // and 2 in common with `1`.
            let digit = match (output.len(), signal4.count(), signal1.count()) {
                (2,_,_) => { part1_sum += 1; '1' },
                (3,_,_) => { part1_sum += 1; '7' },
                (4,_,_) => { part1_sum += 1; '4' },
                (7,_,_) => { part1_sum += 1; '8' },
                (5,2,_) => '2',
                (5,3,1) => '5',
                (5,3,2) => '3',
                (6,4,_) => '9',
                (6,3,1) => '6',
                (6,3,2) => '0',
                _______ => unreachable!(),
            };

            number.push(digit);
        }

        part2_sum += number.parse::<i32>().unwrap();
    }

    println!("Part 1: {}", part1_sum);
    println!("Part 2: {}", part2_sum);
}
