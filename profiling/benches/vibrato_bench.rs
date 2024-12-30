#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use utils::generate_signal_stream;
use vibrato::{LfoShape, Vibrato};

fn vibrato_bench(c: &mut Criterion) {
  let mut vibrato = Vibrato::new(44100.);
  let signal_stream = generate_signal_stream(44100);

  let freq = 4.;
  let depth = 0.1;
  let shape = LfoShape::Sine;
  let chance = 1.;
  vibrato.initialize(freq, chance);

  c.bench_function("vibrato", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        vibrato.process(*signal, freq, depth, shape, chance);
      }
    })
  });
}

criterion_group!(benches, vibrato_bench);
criterion_main!(benches);
