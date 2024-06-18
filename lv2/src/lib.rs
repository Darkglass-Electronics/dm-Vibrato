extern crate lv2;
extern crate vibrato;
use lv2::prelude::*;
use vibrato::{LfoShape, Vibrato, MAX_DEPTH};

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<Control>,
  depth: InputPort<Control>,
  shape: InputPort<Control>,
  offset: InputPort<Control>,
  curve: InputPort<Control>,
  chance: InputPort<Control>,
  input_left: InputPort<Audio>,
  input_right: InputPort<Audio>,
  output_left: OutputPort<Audio>,
  output_right: OutputPort<Audio>,
}

#[uri("https://github.com/davemollen/dm-LFO")]
struct DmVibrato {
  vibrato: Vibrato,
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

  fn get_parameters(&self, ports: &mut Ports) -> (f32, f32, LfoShape, f32, f32, f32) {
    let depth = *ports.depth * 0.01;

    (
      *ports.freq,
      depth * depth * MAX_DEPTH,
      Self::map_shape(*ports.shape),
      *ports.offset * 0.01,
      *ports.chance * 0.01,
      2_f32.powf(*ports.curve * 0.02),
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
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    let (freq, depth, shape, offset, chance, curve) = self.get_parameters(ports);

    let input_channels = ports.input_left.iter().zip(ports.input_right.iter());
    let output_channels = ports
      .output_left
      .iter_mut()
      .zip(ports.output_right.iter_mut());

    for ((input_l, input_r), (output_l, output_r)) in input_channels.zip(output_channels) {
      let output = self.vibrato.process(
        (*input_l, *input_r),
        freq,
        depth,
        shape,
        offset,
        chance,
        curve,
      );
      *output_l = output.0;
      *output_r = output.1;
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmVibrato);
