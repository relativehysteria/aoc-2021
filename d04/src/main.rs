/// This could be done in a more abstracted and readable way by turning
/// the cards into a struct, but whatever.
///
/// When a number is drawn, it is turned into -1 on the card.
/// If there are only -1's in a column or a line, bingo is called.
/// Cards with bingo on them are then completely marked with -1's to invalidate
/// them (you can't delete them because Rust's borrow checker doesn't allow
/// that, so marking them is an alternative).
use std::fs::read_to_string;

fn main() {
    let     data = read_to_string("input").expect("Couldn't read input");
    let mut data = data.split("\n\n");

    // Get the numbers to draw
    let draw_numbers = data.next().unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // Turn the data into a Vec of `Vec<Vec<i32>>`,
    // where each `Vec<Vec<i32>>` is a bingo card.
    let mut cards = vec![];
    for card in data {
        let card = card.split('\n').collect::<Vec<&str>>();

        let card = card.into_iter()
            .map(|x| x.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let card = card.into_iter()
            .map(|x| {
                 x.into_iter()
                 .map(|y| y.parse::<i32>().unwrap())
                 .collect::<Vec<i32>>()
            })
            .filter(|x| !x.is_empty())
            .collect::<Vec<Vec<i32>>>();

        cards.push(card);
    }

    // Draw the numbers on each card
    'bingo: for number in draw_numbers {
        for card in cards.iter_mut() {
            draw_number(card, number);

            // Check whether bingo can be called
            if check_bingo(card) {
                let result = sum_unmarked(card) * number;

                // Mark all numbers on the card so we don't call it again
                mark_all(card);
                if result != 0 {
                    println!("BINGO: {}", result);
                }

                // // Uncomment this for the FIRST PART
                // break 'bingo;
            }
        }
    }
}

/// Marks all numbers on a card
fn mark_all(card: &mut Vec<Vec<i32>>) {
    for line in card.iter_mut() {
        for number in line.iter_mut() {
            *number = -1;
        }
    }
}

/// Calculate the sum of unmarked numbers
fn sum_unmarked(card: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in card {
        sum += line.iter()
            .filter(|x| **x != -1)
            .sum::<i32>();
    }
    sum
}

/// If a `drawn_number` is found on the `card`, the number on the card is turned
/// into -1.
fn draw_number(card: &mut Vec<Vec<i32>>, drawn_number: i32) {
    for line in card {
        for number in line.iter_mut() {
            if &drawn_number == number {
                *number = -1;
            }
        }
    }
}

/// Checks if a bingo can be called on a `card`.
/// If yes, `true` is returned.
fn check_bingo(card: &Vec<Vec<i32>>) -> bool {
    // Check lines for [-1, -1, -1, -1, ...].
    for line in card {
        if line.iter().filter(|x| **x != -1).count() == 0 {
            return true;
        }
    }

    // Check columns for [-1, -1, -1 -1, ...].
    let num_columns = card.get(0).unwrap().len();
    'columns: for idx in 0..num_columns {
        for line in card {
            if *line.get(idx).unwrap() != -1 {
                continue 'columns;
            }
        }
        return true;
    }

    false
}
