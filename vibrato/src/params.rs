mod smooth;
pub use smooth::Smoother;
use {
  crate::MAX_DEPTH,
  smooth::{LinearSmooth, LogarithmicSmooth},
};

#[derive(Clone, Copy)]
pub enum LfoShape {
  Sine,
  Triangle,
  SawUp,
  SawDown,
  Square,
}

pub struct Params {
  pub freq: LogarithmicSmooth,
  pub depth: f32,
  pub shape: LfoShape,
  pub time: LinearSmooth,
  pub wet: LogarithmicSmooth,
  is_initialized: bool,
}

impl Params {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      freq: LogarithmicSmooth::new(sample_rate, 0.25),
      depth: 0.,
      shape: LfoShape::Sine,
      time: LinearSmooth::new(sample_rate, 30.),
      wet: LogarithmicSmooth::new(sample_rate, 0.03),
      is_initialized: false,
    }
  }

  pub fn reset(&mut self) {
    self.is_initialized = false;
  }

  pub fn set(&mut self, freq: f32, depth: f32, shape: i32, wet: f32) {
    self.depth = depth * depth * MAX_DEPTH;
    self.shape = match shape {
      0 => LfoShape::Sine,
      1 => LfoShape::Triangle,
      2 => LfoShape::SawUp,
      3 => LfoShape::SawDown,
      4 => LfoShape::Square,
      _ => panic!("Unknown lfo shape"),
    };

    if self.is_initialized {
      self.freq.set_target(freq);
      self.wet.set_target(wet);
    } else {
      self.freq.reset(freq);
      self.wet.reset(wet);
      self.is_initialized = true;
    }
  }
}
