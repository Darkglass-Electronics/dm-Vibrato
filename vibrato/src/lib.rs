mod delay_line;
mod lfo;
mod params;
mod shared {
  pub mod float_ext;
}
pub use params::Params;
use {
  delay_line::{DelayLine, Interpolation},
  lfo::Lfo,
  params::Smoother,
};

pub const MIN_LFO_FREQ: f32 = 0.1;
const MAX_DEPTH: f32 = 100.;
const DEPTH_OFFSET: f32 = 2.;

pub struct Vibrato {
  delay_line: DelayLine,
  lfo: Lfo,
}

impl Vibrato {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new(
        ((MIN_LFO_FREQ.recip() * MAX_DEPTH + DEPTH_OFFSET) / 1000. * sample_rate) as usize,
        sample_rate,
      ),
      lfo: Lfo::new(sample_rate),
    }
  }

  pub fn process(&mut self, input: f32, params: &mut Params) -> f32 {
    let Params {
      shape,
      depth,
      ..
    } = *params;
    let freq = params.freq.next();
    let wet = params.wet.next();

    let lfo = self.lfo.process(freq, shape);
    params
      .time
      .set_target(lfo * freq.recip() * depth + DEPTH_OFFSET);
    let time = params.time.next();
    let output = self.delay_line.read(time, Interpolation::Cubic);

    self.delay_line.write(input);

    output * wet + input * (1.0 - wet)
  }
}
