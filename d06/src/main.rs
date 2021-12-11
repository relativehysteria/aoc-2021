use std::fs::read_to_string;

/// A struct which simulates a village of fish.
struct FishVillage {
    /// Index = timer.
    /// So if we have one fish with timer 7, and one fish with timer 4,
    /// the Vec would look like this:
    /// `[0, 0, 0, 0, 1, 0, 0, 1, 0]`
    ///
    /// After each cycle, the fish is moved down in the vector, i.e.:
    /// `[0, 0, 0, 1, 0, 0, 1, 0, 0]`
    fish: Vec<usize>,
}

impl FishVillage {
    /// Creates a new `FishVillage`
    fn new() -> Self {
        Self { fish: vec![0usize; 9] }
    }

    /// Spawns a new fish in the `FishVillage`
    fn spawn(&mut self, timer: usize) {
        self.fish[timer] += 1;
    }

    /// Cycles the `FishVillage` exactly once.
    fn cycle(&mut self) {
        let new_spawn = self.fish[0];
        let len = self.fish.len();

        for i in 0..len-1 {
            self.fish[i] = self.fish[i+1];
        }
        self.fish[len-1] = new_spawn;
        self.fish[6] += new_spawn;
    }

    /// Returns the size of the village.
    fn size(&self) -> usize {
        self.fish.iter().sum()
    }
}

fn main() {
    let data = read_to_string("input").expect("Couldn't read input");
    let data = data.trim()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    // Spawn a new village
    let mut village = FishVillage::new();

    // Fill it with fish
    for i in data {
        village.spawn(i);
    }

    // Simulate the village for 256 cycles
    for _ in 0..256{
        village.cycle();
    }

    println!("{}", village.size());
}
