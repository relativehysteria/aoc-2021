use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines();

    let mut coords: HashMap<(i32, i32), u32> = HashMap::new();
    let mut low_point_sum = 0;

    // Turn the data into `HashMap<coords, value>`.
    // This is inefficient but I'm lazy x)
    for (x, line) in data.into_iter().enumerate() {
        for (y, val) in line.chars().enumerate() {
            coords.insert((x as i32, y as i32), val.to_digit(10).unwrap());
        }
    }

    // Neded for the second part
    let mut basin_sizes = vec![];

    for ((x,y), value) in coords.iter() {
        let x = *x;
        let y = *y;

        // This is needed for the second part
        let mut wet: HashSet<(i32, i32)> = HashSet::new();
        let mut to_fill: Vec<(i32, i32)> = vec![(x, y)];

        // Check for low points
        if coords.get(&(x - 1, y)).unwrap_or(&10) > &value
        && coords.get(&(x + 1, y)).unwrap_or(&10) > &value
        && coords.get(&(x, y - 1)).unwrap_or(&10) > &value
        && coords.get(&(x, y + 1)).unwrap_or(&10) > &value {
            // FIRST PART
            low_point_sum += value + 1;

            // https://en.wikipedia.org/wiki/Flood_fill
            // SECOND PART
            while to_fill.len() != 0 {
                let (x, y) = to_fill.pop().unwrap();
                wet.insert((x, y));

                for adj in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                    if !wet.contains(&adj)
                    && coords.get(&adj).unwrap_or(&9) != &9 {
                        to_fill.push(adj);
                    }
                }
            }
            basin_sizes.push(wet.len());
        }
    }

    println!("First part: {}", low_point_sum);

    // Find the product of the three largest basins
    basin_sizes.sort();
    let product = basin_sizes.iter().rev().take(3).product::<usize>();
    println!("Second part: {}", product);
}
