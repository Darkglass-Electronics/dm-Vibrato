mod utils;
use utils::generate_signal;
use vibrato::{Params, Vibrato};

fn main() {
  let mut vibrato = Vibrato::new(44100.);
  let mut params = Params::new(44100.);
  params.set(4., 0.1, 0, 1.);

  loop {
    let input = generate_signal();
    vibrato.process(input, &mut params);
  }
}
