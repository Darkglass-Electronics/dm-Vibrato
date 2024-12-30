#[path = "../src/utils.rs"]
mod utils;
use criterion::{criterion_group, criterion_main, Criterion};
use utils::generate_signal_stream;
use vibrato::{Params, Vibrato};

fn vibrato_bench(c: &mut Criterion) {
  let mut vibrato = Vibrato::new(44100.);
  let mut params = Params::new(44100.);
  params.set(4., 0.1, 0, 1.);
  let signal_stream = generate_signal_stream(44100);

  c.bench_function("vibrato", |b| {
    b.iter(|| {
      for signal in &signal_stream {
        vibrato.process(*signal, &mut params);
      }
    })
  });
}

criterion_group!(benches, vibrato_bench);
criterion_main!(benches);
