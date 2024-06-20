mod delay_line;
mod lfo;
mod ramp_smooth;
mod shared {
  pub mod float_ext;
}

pub use lfo::LfoShape;
use {
  delay_line::{DelayLine, Interpolation},
  lfo::Lfo,
  ramp_smooth::RampSmooth,
};

pub const MIN_LFO_FREQ: f32 = 0.1;
pub const MAX_DEPTH: f32 = 100.;
const DEPTH_OFFSET: f32 = 2.;

pub struct Vibrato {
  delay_line: DelayLine,
  smooth_time: RampSmooth,
  lfo: Lfo,
}

impl Vibrato {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: DelayLine::new(
        ((MIN_LFO_FREQ.recip() * MAX_DEPTH + DEPTH_OFFSET) / 1000. * sample_rate) as usize,
        sample_rate,
      ),
      smooth_time: RampSmooth::new(sample_rate, 20.),
      lfo: Lfo::new(sample_rate),
    }
  }

  pub fn initialize(&mut self, chance: f32) {
    self.lfo.initialize(chance);
  }

  pub fn process(
    &mut self,
    input: f32,
    freq: f32,
    depth: f32,
    shape: LfoShape,
    chance: f32,
  ) -> f32 {
    let lfo = self.lfo.process(freq, shape, chance);
    let time = self.smooth_time.process(Self::get_time(lfo, freq, depth));
    let output = self.delay_line.read(time, Interpolation::Cubic);

    self.delay_line.write(input);

    output
  }

  fn get_time(lfo: f32, freq: f32, depth: f32) -> f32 {
    lfo * freq.recip() * depth + DEPTH_OFFSET
  }
}
