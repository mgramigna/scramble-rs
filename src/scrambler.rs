use rand::distributions::{Distribution, Uniform};

const SCRAMBLE_LENGTH: u8 = 25;

pub fn get_scramble() -> String {
    let mut rng = rand::thread_rng();

    let base_range: Uniform<u8> = Uniform::from(1..=3);
    let move_adjustment_range: Uniform<u8> = Uniform::from(1..=2);

    let mut scramble = String::new();
    let mut previous_move: u8 = 0;
    for _ in 0..SCRAMBLE_LENGTH {
        let mut current_move = base_range.sample(&mut rng);

        while current_move == previous_move {
            current_move = base_range.sample(&mut rng);
        }

        // Determines whether or not to add ' or 2 as a modifier
        let modifier = base_range.sample(&mut rng);

        // Determines whether to select from FRU or BLD
        let move_adjustment = move_adjustment_range.sample(&mut rng);

        match move_adjustment {
            1 => match current_move {
                1 => scramble += "F",
                2 => scramble += "R",
                _ => scramble += "U",
            },
            _ => match current_move {
                1 => scramble += "B",
                2 => scramble += "L",
                _ => scramble += "D",
            },
        }

        match modifier {
            1 => scramble += "'",
            2 => scramble += "2",
            _ => {}
        }

        scramble += " ";
        previous_move = current_move
    }
    scramble
}
