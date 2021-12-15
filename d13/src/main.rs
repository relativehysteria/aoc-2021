use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Dot(usize, usize);

enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn fold(&self, dots: &mut [Dot]) {
        dots.iter_mut().for_each(|dot| match *self {
            Fold::X(x) => dot.0 = if dot.0 < x { dot.0 } else { x * 2 - dot.0 },
            Fold::Y(y) => dot.1 = if dot.1 < y { dot.1 } else { y * 2 - dot.1 },
        });
    }
}

fn first(mut dots: Vec<Dot>, folds: &Vec<Fold>) {
    folds.first().unwrap().fold(&mut dots);
    println!("First part: {}", dots.into_iter().collect::<HashSet<_>>().len());
}

fn second(mut dots: Vec<Dot>, folds: &Vec<Fold>) {
    folds.iter().for_each(|fold| fold.fold(&mut dots));
    let (w, h) = dots.iter()
        .fold((0, 0), |(w, h), &dot| (w.max(dot.0), h.max(dot.1)));

    let mut dot_buffer = vec![vec![' '; w + 1]; h + 1];

    dots.into_iter().for_each(|Dot(x, y)| dot_buffer[y][x] = '#');

    for dots in dot_buffer {
        dots.iter().for_each(|c| print!("{}", c));
        println!();
    }
}

fn main() {
    let data = read_to_string("input").expect("Couldn't read input.");
    let (dots, folds) = data.trim().split_once("\n\n").unwrap();

    let mut dots = dots.lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Dot(x.parse().unwrap(), y.parse().unwrap())
        })
        .collect::<Vec<Dot>>();

    let mut folds = folds.lines()
        .map(|line| {
            let (desc, num) = line.split_once('=').unwrap();
            match desc.chars().last() {
                Some('x') => Fold::X(num.parse().unwrap()),
                Some('y') => Fold::Y(num.parse().unwrap()),
                _________ => unreachable!(),
            }
        })
        .collect::<Vec<Fold>>();

    first(dots.clone(), &folds);
    second(dots, &folds);
}
