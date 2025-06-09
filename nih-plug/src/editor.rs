#[path = "./editor/components/param_knob.rs"]
mod param_knob;
use nih_plug::{params::Param, prelude::Enum};
use param_knob::{ParamKnob, ParamKnobSize};
mod ui_data;
use crate::vibrato_parameters::{LfoShape, VibratoParameters};
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::{
  binding::LensExt,
  context::EmitContext,
  model::Model,
  modifiers::{ActionModifiers, LayoutModifiers, StyleModifiers, TextModifiers},
  prelude::Units::{Pixels, Stretch},
  style::FontWeightKeyword,
  views::{Dropdown, HStack, Label, PopupEvent, VStack},
};
use nih_plug_vizia::{create_vizia_editor, vizia_assets, ViziaState, ViziaTheming};
use std::sync::Arc;
use ui_data::{ParamChangeEvent, UiData};

const STYLE: &str = include_str!("./editor/style.css");

// Makes sense to also define this here, makes it a bit easier to keep track of
pub(crate) fn default_state() -> Arc<ViziaState> {
  ViziaState::new(|| (256, 256))
}

pub(crate) fn create(
  params: Arc<VibratoParameters>,
  editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
  create_vizia_editor(
    editor_state,
    ViziaTheming::Custom,
    move |cx, gui_context| {
      vizia_assets::register_roboto(cx);
      vizia_assets::register_roboto_bold(cx);
      cx.add_stylesheet(STYLE).ok();

      UiData {
        params: params.clone(),
        gui_context: gui_context.clone(),
      }
      .build(cx);

      VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
          HStack::new(cx, |cx| {
            ParamKnob::new(
              cx,
              params.freq.name(),
              UiData::params,
              params.freq.as_ptr(),
              |params| &params.freq,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );

            ParamKnob::new(
              cx,
              params.depth.name(),
              UiData::params,
              params.depth.as_ptr(),
              |params| &params.depth,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );

            ParamKnob::new(
              cx,
              params.chance.name(),
              UiData::params,
              params.chance.as_ptr(),
              |params| &params.chance,
              |param_ptr, val| ParamChangeEvent::SetParam(param_ptr, val),
              ParamKnobSize::Regular,
            );
          })
          .child_left(Stretch(1.0))
          .child_right(Stretch(1.0));

          VStack::new(cx, |cx| {
            Label::new(cx, params.shape.name())
              .font_size(13.0)
              .font_weight(FontWeightKeyword::Bold)
              .text_wrap(false)
              .child_space(Stretch(1.0));

            Dropdown::new(
              cx,
              |cx| {
                Label::new(
                  cx,
                  UiData::params.map(|params| params.shape.value().to_string()),
                )
              },
              |cx| {
                let lens = UiData::params;

                for text in LfoShape::variants() {
                  Label::new(cx, *text)
                    .on_press(move |cx| {
                      cx.emit(ParamChangeEvent::SetParam(
                        lens.map(move |params| params.shape.as_ptr()).get(cx),
                        lens
                          .map(move |params| params.shape.string_to_normalized_value(*text))
                          .get(cx)
                          .unwrap_or_default(),
                      ));
                      cx.emit(PopupEvent::Close);
                    })
                    .width(Stretch(1.0))
                    .class("dropdown-item");
                }
              },
            )
            .top(Pixels(6.0));
          })
          .width(Pixels(144.))
          .left(Stretch(1.0))
          .right(Stretch(1.0))
          .top(Pixels(8.0));
        });

        Label::new(cx, "Vibrato")
          .font_size(22.0)
          .font_weight(FontWeightKeyword::Bold)
          .border_radius(Pixels(16.0))
          .color("#e1d9d1")
          .background_color("#de0a26")
          .child_space(Stretch(1.0))
          .child_top(Pixels(1.0))
          .child_bottom(Pixels(5.0))
          .width(Pixels(96.0))
          .left(Stretch(1.0));
      })
      .child_space(Pixels(16.0))
      .background_color("#505050");
    },
  )
}
