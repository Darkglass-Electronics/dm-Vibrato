use crate::editor;
use nih_plug::{
  formatters::{s2v_f32_hz_then_khz, s2v_f32_percentage, v2s_f32_hz_then_khz, v2s_f32_percentage},
  params::EnumParam,
  prelude::{Enum, FloatParam, FloatRange, Params},
};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;
use vibrato::MIN_LFO_FREQ;

#[derive(Clone, Copy, Enum, PartialEq)]
pub enum LfoShape {
  #[id = "sine"]
  #[name = "Sine"]
  Sine,

  #[id = "triangle"]
  #[name = "Triangle"]
  Triangle,

  #[id = "saw_up"]
  #[name = "Saw Up"]
  SawUp,

  #[id = "saw_down"]
  #[name = "Saw Down"]
  SawDown,

  #[id = "rectangle"]
  #[name = "Rectangle"]
  Rectangle,

  #[id = "sample_and_hold"]
  #[name = "Sample And Hold"]
  SampleAndHold,

  #[id = "random"]
  #[name = "Random"]
  Random,

  #[id = "curved_random"]
  #[name = "Curved Random"]
  CurvedRandom,
}

impl LfoShape {
  pub fn to_string(self) -> String {
    match self {
      LfoShape::Sine => "Sine",
      LfoShape::Triangle => "Triangle",
      LfoShape::SawUp => "Saw Up",
      LfoShape::SawDown => "Saw Down",
      LfoShape::Rectangle => "Rectangle",
      LfoShape::SampleAndHold => "Sample And Hold",
      LfoShape::Random => "Random",
      LfoShape::CurvedRandom => "Curved Random",
    }
    .to_string()
  }
}

#[derive(Params)]
pub struct VibratoParameters {
  /// The editor state, saved together with the parameter state so the custom scaling can be
  /// restored.
  #[persist = "editor-state"]
  pub editor_state: Arc<ViziaState>,

  #[id = "freq"]
  pub freq: FloatParam,

  #[id = "depth"]
  pub depth: FloatParam,

  #[id = "shape"]
  pub shape: EnumParam<LfoShape>,

  #[id = "chance"]
  pub chance: FloatParam,
}

impl Default for VibratoParameters {
  fn default() -> Self {
    Self {
      editor_state: editor::default_state(),

      freq: FloatParam::new(
        "Freq",
        2.,
        FloatRange::Skewed {
          min: MIN_LFO_FREQ,
          max: 30.,
          factor: 0.3,
        },
      )
      .with_value_to_string(v2s_f32_hz_then_khz(2))
      .with_string_to_value(s2v_f32_hz_then_khz()),

      depth: FloatParam::new("Depth", 0.1, FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),

      shape: EnumParam::new("Shape", LfoShape::Sine),

      chance: FloatParam::new("Chance", 1., FloatRange::Linear { min: 0., max: 1. })
        .with_unit(" %")
        .with_value_to_string(v2s_f32_percentage(2))
        .with_string_to_value(s2v_f32_percentage()),
    }
  }
}
