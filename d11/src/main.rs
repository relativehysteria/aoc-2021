use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let num_octopuses = data.replace("\n", "").len() as u32;

    let mut data = data.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // // Part 1
    // let mut total_flashes = 0;
    // for _ in 0..100 {
    //     total_flashes += emulate_step(&mut data);
    // }
    // println!("{}", total_flashes);

    // Part 2
    let mut num_steps = 1;
    while emulate_step(&mut data) != num_octopuses {
        num_steps += 1;
    }
    println!("{}", num_steps);
}

/// Emulates a step in the octopus system.
/// Returns the amount of flashes that have happened during the round
fn emulate_step(octopuses: &mut Vec<Vec<u32>>) -> u32 {
    let mut num_flashes = 0;

    // Increment everything by 1
    for x in 0..octopuses.len() {
        for y in 0..octopuses[x].len() {
            octopuses[x][y] += 1;
        }
    }

    // Create the initial version of the `to_flash` Vec
    let mut to_flash: Vec<(usize, usize)> = vec![];
    for x in 0..octopuses.len() {
        for y in 0..octopuses[x].len() {
            if octopuses[x][y] > 9 {
                to_flash.push((x, y));
            }
        }
    }

    // And then flash everything that needs be
    while let Some((x, y)) = to_flash.pop() {
        num_flashes += 1;

        let mut flash = |x: Option<usize>, y: Option<usize>| {
            if x.is_none() || y.is_none() { return; }
            let x = x.unwrap();
            let y = y.unwrap();
            if x >= octopuses.len() || y >= octopuses[x].len() { return; }

            let octo = &mut octopuses[x][y];
            *octo += 1;
            if *octo == 10 {
                to_flash.push((x, y));
            }
        };

        flash(x.checked_sub(1), y.checked_sub(1));
        flash(x.checked_sub(1), Some(y));
        flash(x.checked_sub(1), y.checked_add(1));
        flash(Some(x), y.checked_sub(1));
        flash(Some(x), y.checked_add(1));
        flash(x.checked_add(1), y.checked_sub(1));
        flash(x.checked_add(1), Some(y));
        flash(x.checked_add(1), y.checked_add(1));
    }

    // Set the energy of flashed octopuses to 0
    for x in 0..octopuses.len() {
        for y in 0..octopuses[x].len() {
            let oct = &mut octopuses[x][y];
            if *oct > 9 {
                *oct = 0;
            }
        }
    }

    num_flashes
}
