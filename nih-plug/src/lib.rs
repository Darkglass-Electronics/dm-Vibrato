use nih_plug::prelude::*;
use std::sync::Arc;
use vibrato::{LfoShape, Vibrato, MAX_DEPTH};
mod vibrato_parameters;
use vibrato_parameters::{LfoShape as LfoShapeParam, VibratoParameters};
mod editor;

struct DmVibrato {
  params: Arc<VibratoParameters>,
  vibrato: Vibrato,
}

impl DmVibrato {
  fn map_params(&self) -> (f32, f32, LfoShape, f32) {
    let depth = self.params.depth.value();

    (
      self.params.freq.value(),
      depth * MAX_DEPTH,
      match self.params.shape.value() {
        LfoShapeParam::Sine => LfoShape::Sine,
        LfoShapeParam::Triangle => LfoShape::Triangle,
        LfoShapeParam::SawUp => LfoShape::SawUp,
        LfoShapeParam::SawDown => LfoShape::SawDown,
        LfoShapeParam::Rectangle => LfoShape::Rectangle,
        LfoShapeParam::SampleAndHold => LfoShape::SampleAndHold,
        LfoShapeParam::Random => LfoShape::Random,
        LfoShapeParam::CurvedRandom => LfoShape::CurvedRandom,
        LfoShapeParam::Noise => LfoShape::Noise,
      },
      self.params.chance.value(),
    )
  }
}

impl Default for DmVibrato {
  fn default() -> Self {
    let params = Arc::new(VibratoParameters::default());
    Self {
      params: params.clone(),
      vibrato: Vibrato::new(44100.),
    }
  }
}

impl Plugin for DmVibrato {
  const NAME: &'static str = "dm-Vibrato";
  const VENDOR: &'static str = "DM";
  const URL: &'static str = "https://github.com/davemollen/dm-Vibrato";
  const EMAIL: &'static str = "davemollen@gmail.com";
  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
    main_input_channels: NonZeroU32::new(1),
    main_output_channels: NonZeroU32::new(1),
    ..AudioIOLayout::const_default()
  }];
  const MIDI_INPUT: MidiConfig = MidiConfig::None;
  const SAMPLE_ACCURATE_AUTOMATION: bool = true;

  // More advanced plugins can use this to run expensive background tasks. See the field's
  // documentation for more information. `()` means that the plugin does not have any background
  // tasks.
  type BackgroundTask = ();
  type SysExMessage = ();

  fn params(&self) -> Arc<dyn Params> {
    self.params.clone()
  }

  fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
    editor::create(self.params.clone(), self.params.editor_state.clone())
  }

  fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
  ) -> bool {
    self.vibrato = Vibrato::new(buffer_config.sample_rate);
    self.vibrato.initialize(self.params.chance.value());

    true
  }

  fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
  ) -> ProcessStatus {
    let (freq, depth, shape, chance) = self.map_params();

    buffer.iter_samples().for_each(|mut channel_samples| {
      let sample = channel_samples.iter_mut().next().unwrap();
      *sample = self.vibrato.process(*sample, freq, depth, shape, chance);
    });
    ProcessStatus::Normal
  }

  // This can be used for cleaning up special resources like socket connections whenever the
  // plugin is deactivated. Most plugins won't need to do anything here.
  fn deactivate(&mut self) {}
}

impl ClapPlugin for DmVibrato {
  const CLAP_ID: &'static str = "dm-Vibrato";
  const CLAP_DESCRIPTION: Option<&'static str> = Some("A vibrato plugin");
  const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
  const CLAP_SUPPORT_URL: Option<&'static str> = None;
  const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::AudioEffect,
    ClapFeature::Mono,
    ClapFeature::PitchShifter,
  ];
}

impl Vst3Plugin for DmVibrato {
  const VST3_CLASS_ID: [u8; 16] = *b"dm-Vibrato......";
  const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
    Vst3SubCategory::Fx,
    Vst3SubCategory::Mono,
    Vst3SubCategory::PitchShift,
    Vst3SubCategory::Modulation,
  ];
}

nih_export_clap!(DmVibrato);
nih_export_vst3!(DmVibrato);
