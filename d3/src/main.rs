use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let (gamma, epsilon) = get_gamma_epsilon(&data);
    let oxygen = get_oxygen_co2(&data, true);
    let co2    = get_oxygen_co2(&data, false);
    println!("{}", gamma * epsilon);
    println!("{}", oxygen * co2);
}

fn get_gamma_epsilon(data: &String) -> (i32, i32) {
    let mut gamma_0 = [0i32; 12];
    let mut gamma_1 = [0i32; 12];

    let data = data.lines();
    for line in data {
        for c in line.chars().enumerate() {
            if c.1 == '0' {
                gamma_0[c.0] += 1;
            } else {
                gamma_1[c.0] += 1;
            }
        }
    }

    let mut gamma   = String::with_capacity(12);
    for i in gamma_0.iter().zip(gamma_1.iter()) {
        if i.0 > i.1 {
            gamma.push('0');
        } else {
            gamma.push('1');
        }
    }

    let gamma   = i32::from_str_radix(&gamma, 2).unwrap();
    let epsilon = gamma ^ 0b111111111111;

    (gamma, epsilon)
}

fn get_oxygen_co2(data: &String, greater: bool) -> i32 {
    let mut data = data.lines().collect::<Vec<&str>>();
    let mut idx = 0;

    loop {
        let mut z = Vec::<&str>::with_capacity(12);
        let mut o = Vec::<&str>::with_capacity(12);

        for line in data.iter() {
            if line.chars().nth(idx).unwrap() == '0' {
                z.push(line);
            } else {
                o.push(line);
            }
        }

        if greater {
            data = if o.len() >= z.len() { o } else { z }
        } else {
            data = if z.len() <= o.len() { z } else { o }
        }

        if data.len() == 1 {
            break;
        }

        idx += 1;
    }
    i32::from_str_radix(data.get(0).unwrap(), 2).unwrap()
}
