use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines();

    let mut horizontal = 0;
    let mut depth      = 0;
    let mut aim        = 0;

    for line in data {
        let mut split = line.split(' ');
        let command = split.next().unwrap();
        let value   = split.next().unwrap().parse::<i32>().unwrap();

        // // Part one
        // match command {
        //     "forward" => horizontal += value,
        //     "down"    => depth += value,
        //     "up"      => depth -= value,
        //     _         => (),
        // }

        // Part two
        match command {
            "forward" => {
                horizontal += value;
                depth      += aim * value;
            },
            "down"    => aim += value,
            "up"      => aim -= value,
            _         => (),
        }
    }
    println!("H: {} | D: {} | {}", horizontal, depth, horizontal * depth);
}
