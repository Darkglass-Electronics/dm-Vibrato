extern crate lv2;
extern crate vibrato;
use lv2::prelude::*;
use vibrato::{Params, Vibrato};

#[derive(PortCollection)]
struct Ports {
  freq: InputPort<InPlaceControl>,
  depth: InputPort<InPlaceControl>,
  shape: InputPort<InPlaceControl>,
  chance: InputPort<InPlaceControl>,
  input: InputPort<InPlaceAudio>,
  output: OutputPort<InPlaceAudio>,
}

#[uri("https://github.com/davemollen/dm-Vibrato")]
struct DmVibrato {
  vibrato: Vibrato,
  params: Params,
}

impl Plugin for DmVibrato {
  // Tell the framework which ports this plugin has.
  type Ports = Ports;

  // We don't need any special host features; We can leave them out.
  type InitFeatures = ();
  type AudioFeatures = ();

  // Create a new instance of the plugin; Trivial in this case.
  fn new(plugin_info: &PluginInfo, _features: &mut ()) -> Option<Self> {
    let sample_rate = plugin_info.sample_rate() as f32;

    Some(Self {
      vibrato: Vibrato::new(sample_rate),
      params: Params::new(sample_rate),
    })
  }

  // Process a chunk of audio. The audio ports are dereferenced to slices, which the plugin
  // iterates over.
  fn run(&mut self, ports: &mut Ports, _features: &mut (), _sample_count: u32) {
    self.params.set(
      ports.freq.get(),
      ports.depth.get() * 0.01,
      ports.shape.get() as i32 - 1,
      ports.chance.get() * 0.01,
    );

    for (input, output) in ports.input.iter().zip(ports.output.iter()) {
      let vibrato_output = self.vibrato.process(input.get(), &mut self.params);
      output.set(vibrato_output);
    }
  }
}

// Generate the plugin descriptor function which exports the plugin to the outside world.
lv2_descriptors!(DmVibrato);
