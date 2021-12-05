use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // First part
    let mut counter   = 0;
    let mut data_iter = data.iter();
    let mut prev      = data_iter.next().unwrap();

    for i in data_iter {
        if i > prev {
            counter += 1;
        }
        prev = i;
    }
    println!("First part:  {}", counter);

    // Second part
    let mut counter   = 0;
    let mut data_iter = data.windows(3);
    let mut prev      = data_iter.next().unwrap().iter().sum::<i32>();

    for i in data_iter {
        let next: i32 = i.iter().sum();
        if next > prev {
            counter += 1;
        }
        prev = next;
    }
    println!("Second part: {}", counter);
}
