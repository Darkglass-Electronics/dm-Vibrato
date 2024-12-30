mod utils;
use utils::generate_signal;
use vibrato::{LfoShape, Vibrato};

fn main() {
  let mut vibrato = Vibrato::new(44100.);

  let freq = 4.;
  let depth = 0.1;
  let shape = LfoShape::Sine;
  let chance = 1.;
  vibrato.initialize(freq, chance);

  loop {
    let input = generate_signal();
    vibrato.process(input, freq, depth, shape, chance);
  }
}
