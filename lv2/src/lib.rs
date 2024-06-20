extern crate lv2;
extern crate vibrato;
use lv2::prelude::*;
use vibrato::{LfoShape, Vibrato, MAX_DEPTH};

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<Control>,
  depth: InputPort<Control>,
  shape: InputPort<Control>,
  chance: InputPort<Control>,
  input: InputPort<Audio>,
  output: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-Vibrato")]
struct DmVibrato {
  vibrato: Vibrato,
  is_active: bool,
}

impl DmVibrato {
  fn map_shape(shape: f32) -> LfoShape {
    match shape {
      1. => LfoShape::Sine,
      2. => LfoShape::Triangle,
      3. => LfoShape::SawUp,
      4. => LfoShape::SawDown,
      5. => LfoShape::Rectangle,
      6. => LfoShape::SampleAndHold,
      7. => LfoShape::Random,
      8. => LfoShape::CurvedRandom,
      9. => LfoShape::Noise,
      _ => panic!("Shape is invalid."),
    }
  }

  fn get_parameters(&self, ports: &mut Ports) -> (f32, f32, LfoShape, f32) {
    let depth = *ports.depth * 0.01;

    (
      *ports.freq,
      depth * MAX_DEPTH,
      Self::map_shape(*ports.shape),
      *ports.chance * 0.01,
    )
  }
}

impl Plugin for DmVibrato {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(_plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    Some(Self {
      vibrato: Vibrato::new(_plugin_info.sample_rate() as f32),
      is_active: false,
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (freq, depth, shape, chance) = self.get_parameters(ports);

    if !self.is_active {
      self.vibrato.initialize(chance);
      self.is_active = true;
    }

    for (input, output) in ports.input.iter().zip(ports.output.iter_mut()) {
      *output = self.vibrato.process(*input, freq, depth, shape, chance);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmVibrato);
