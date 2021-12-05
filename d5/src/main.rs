use std::fs::read_to_string;
use std::collections::HashMap;

/// A coordinate description of a line of hydrothermal vents.
#[derive(Debug)]
struct VentLine {
    first:  (i32, i32),
    second: (i32, i32),
}

impl VentLine {
    /// Parses an input like "808,720 -> 808,385" into
    /// VentLine {
    ///     first:  (808, 720),
    ///     second: (808, 385)
    /// }
    fn parse_from(input: &str) -> Self {
        let mut coords = input.split(" -> ");

        // Get the &str coordinates
        let first  = coords.next().unwrap().split(',');
        let second = coords.next().unwrap().split(',');

        // And parse them
        let first = first
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let second = second
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        VentLine {
            first:  (first[0], first[1]),
            second: (second[0], second[1]),
        }
    }

    /// Whether this VentLine is placed horizontally.
    fn is_diagonal(&self) -> bool {
        if self.first.0 != self.second.0 && self.first.1 != self.second.1 {
            true
        } else {
            false
        }
    }

    /// Returns all the coordinates that are covered by this `VentLine`
    /// "1,1 -> 1,3" returns `[(1,1), (1,2), (1,3)]`
    fn cover_coords(&self) -> Vec<(i32, i32)> {
        // Diagonal and non-diagonal lines have to be treated differently.
        if self.is_diagonal() {
            self._diagonal_cover_coords()
        } else {
            self._non_diagonal_cover_coords()
        }
    }

    fn _diagonal_cover_coords(&self) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = vec![];

        for i in 0..((self.first.0 - self.second.0).abs() + 1) {
            let x = if self.first.0 > self.second.0 {
                self.first.0 - i
            } else {
                self.first.0 + i
            };

            let y = if self.first.1 > self.second.1 {
                self.first.1 - i
            } else {
                self.first.1 + i
            };

            result.push((x, y))
        }

        result
    }

    fn _non_diagonal_cover_coords(&self) -> Vec<(i32, i32)> {
        let mut result: Vec<(i32, i32)> = vec![];

        // One of the coordinates doesn't change -- find out which
        let x_is_static = self.first.0 == self.second.0;

        // If the x coordinates are static, get the range between y's.
        // Otherwise get the range between x's.
        let mut coords = if !x_is_static {
            self.first.0..self.second.0
        } else {
            self.first.1..self.second.1
        };

        // Start should always be lower than end
        if coords.start > coords.end {
            let start    = coords.start;
            coords.start = coords.end;
            coords.end   = start;
        }

        // Inclusive range
        coords.end += 1;

        // Now get the coords
        for i in coords {
            if x_is_static {
                result.push((self.first.0, i))
            } else {
                result.push((i, self.first.1));
            }
        }

        result
    }
}

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines().collect::<Vec<&str>>();

    // Parse the data into VentLine
    let data = data.into_iter()
        .map(|x| VentLine::parse_from(x))
        //.filter(|x| !x.is_diagonal())  // First part
        .collect::<Vec<VentLine>>();

    // Get the vent positions and the amount of the times they overlap.
    let mut vent_positions: HashMap<(i32, i32), i32> = HashMap::new();
    for i in data {
        i.cover_coords();
        for coord in i.cover_coords() {
            let entry = vent_positions.entry(coord).or_insert(0);
            *entry += 1;
        }
    }

    // Count the amount of overlaps
    println!("{}", vent_positions.values().filter(|x| **x > 1).count());
}
