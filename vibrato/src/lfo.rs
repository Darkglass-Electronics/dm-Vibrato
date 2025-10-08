mod phasor;
use {
  crate::{params::LfoShape, shared::float_ext::FloatExt},
  phasor::Phasor,
  std::f32::consts::{PI, TAU},
};

pub struct Lfo {
  phasor: Phasor,
  origin: f32,
  target: f32,
}

impl Lfo {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      phasor: Phasor::new(sample_rate),
      origin: 0.,
      target: 0.,
    }
  }

  pub fn reset(&mut self) {
    self.phasor.reset();
    self.origin = 0.;
    self.target = 0.;
  }

  pub fn process(&mut self, freq: f32, shape: LfoShape) -> f32 {
    let phase = self.phasor.process(freq);

    match shape {
      LfoShape::Sine => {
        ((phase + 0.75) * TAU).fast_sin() * 0.5 + 0.5
      }
      LfoShape::Triangle => {
        let phase = Self::wrap(phase + 0.25);
        if phase > 0.5 {
          (phase - 0.5) * -2. + 1.
        } else {
          phase * 2.
        }
      }
      LfoShape::SawUp => {
        1. - phase
      }
      LfoShape::SawDown => {
        phase
      }
      LfoShape::Square => {
        if phase > 0.5 {
          1.
        } else {
          0.
        }
      }
    }
  }

  fn linear_interp(&self, mix: f32) -> f32 {
    self.origin + (self.target - self.origin) * mix
  }

  fn cosine_interp(&self, mix: f32) -> f32 {
    let cosine_mix = (1. - (mix * PI).fast_cos()) * 0.5;
    self.origin + (self.target - self.origin) * cosine_mix
  }

  fn wrap(x: f32) -> f32 {
    if x >= 1. {
      x - 1.
    } else {
      x
    }
  }
}
