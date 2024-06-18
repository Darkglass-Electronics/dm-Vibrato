mod lfo;
mod ramp_smooth;
mod stereo_delay_line;
mod shared {
  pub mod float_ext;
}

pub use lfo::LfoShape;
use {
  lfo::Lfo,
  ramp_smooth::RampSmooth,
  stereo_delay_line::{Interpolation, StereoDelayLine},
};

const MIN_LFO_FREQ: f32 = 0.1;
pub const MAX_DEPTH: f32 = 100.;

pub struct Vibrato {
  delay_line: StereoDelayLine,
  smooth_time: RampSmooth,
  lfo: Lfo,
}

impl Vibrato {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      delay_line: StereoDelayLine::new(
        (MIN_LFO_FREQ.recip() * sample_rate * MAX_DEPTH / 1000.) as usize,
        sample_rate,
      ),
      smooth_time: RampSmooth::new(sample_rate, 20.),
      lfo: Lfo::new(sample_rate),
    }
  }

  pub fn process(
    &mut self,
    input: (f32, f32),
    freq: f32,
    depth: f32,
    shape: LfoShape,
    offset: f32,
    chance: f32,
    curve: f32,
  ) -> (f32, f32) {
    let lfo = (self.lfo.process(freq, shape, chance) + offset)
      .clamp(0., 1.)
      .powf(curve);
    let time = Self::get_time(freq, lfo, depth);
    let time = self.smooth_time.process(time);
    let output = self.delay_line.read(time, Interpolation::Cubic);

    self.delay_line.write(input);

    output
  }

  fn get_time(lfo: f32, freq: f32, depth: f32) -> f32 {
    let depth_correction = freq.recip();
    2_f32.powf(lfo) * depth * depth_correction - depth_correction
  }
}
