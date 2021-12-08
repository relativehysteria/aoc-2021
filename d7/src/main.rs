use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let mut data = data.trim()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let med = median(&mut data);
    let mut sum = 0;
    for i in &data {
        sum += (med - i).abs();
    }
    println!("Part 1 (median: {}) >> {}", med, sum);

    let mean = mean(&data).floor() as i32;
    let mut sum = 0;
    for i in data {
        let steps = (mean - i).abs();
        sum += (1..steps+1).sum::<i32>();
    }
    println!("Part 2 (mean:   {}) >> {}", mean, sum);
}

fn mean(nums: &Vec<i32>) -> f32 {
    nums.iter().sum::<i32>() as f32 / nums.len() as f32
}

fn median(nums: &mut Vec<i32>) -> i32 {
    nums.sort();

    let mid = nums.len() / 2;
    if nums.len() % 2 == 0 {
        mean(&vec![nums[mid], nums[mid-1]]) as i32
    } else {
        nums[mid]
    }
}
