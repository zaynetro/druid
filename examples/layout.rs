// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This example shows how to construct a basic layout.

use druid::shell::{runloop, WindowBuilder};
use druid::widget::{ActionWrapper, Button, Column, DynLabel, Padding, Row, SizedBox};
use druid::{Data, LensWrap, UiMain, UiState, Widget};

#[derive(Debug, Clone)]
struct AppState {
    pressed: Option<u32>,
    text: String,
}

impl Data for AppState {
    fn same(&self, o: &Self) -> bool {
        self.pressed == o.pressed && self.text == o.text
    }
}

mod lenses {
    macro_rules! impl_lens {
        ($struct:ident, $field:ident, $t:ty) => {
            pub struct $struct;

            impl Lens<AppState, $t> for $struct {
                fn get<'a>(&self, data: &'a AppState) -> &'a $t {
                    &data.$field
                }

                fn with_mut<V, F: FnOnce(&mut $t) -> V>(&self, data: &mut AppState, f: F) -> V {
                    f(&mut data.$field)
                }
            }
        };
    }

    pub mod app_state {
        use super::super::AppState;
        use druid::Lens;

        impl_lens!(Text, text, String);
        impl_lens!(Pressed, pressed, Option<u32>);
    }
}

fn build_header() -> impl Widget<String> {
    // Construct a horizontal layout.
    SizedBox::new(
        Row::new()
            .child(
                SizedBox::new(DynLabel::new(|data, _env| format!("Text: {}", data))).width(60.0),
                0.0,
            )
            // Spacing element that will fill all available space in between label
            // and a button. Notice that weight is non-zero.
            .child(SizedBox::empty().expand(), 1.0)
            .child(Padding::uniform(20.0, Button::new("Two")), 0.0),
    )
    .height(100.0)
}

fn build_buttons() -> impl Widget<Option<u32>> {
    Column::new()
        .child(
            DynLabel::new(|data, _env| match data {
                Some(count) => format!("Pressed button #{}", count),
                None => "No button was pressed".to_string(),
            }),
            1.0,
        )
        .children(0..4, |i, col| {
            // Give a larger weight to one of the buttons for it to occupy more space.
            let weight = if i == 1 { 3.0 } else { 1.0 };
            col.add_child(
                ActionWrapper::new(Button::new(format!("Button #{}", i)), move |data, _env| {
                    *data = Some(i);
                }),
                weight,
            );
        })
}

fn build() -> impl Widget<AppState> {
    Column::new()
        .child(
            // Limit data scope of header widget to only text field
            LensWrap::new(build_header(), lenses::app_state::Text),
            0.0,
        )
        .child(
            // Limit data scope of buttons list to only pressed field
            LensWrap::new(build_buttons(), lenses::app_state::Pressed),
            // Notice that weight is non-zero. This way we tell widget to occupy
            // the whole space available.
            1.0,
        )
}

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();

    // Build app layout
    let root = build();
    // Set up initial app state
    let state = UiState::new(
        root,
        AppState {
            pressed: None,
            text: "Hello world text".to_string(),
        },
    );
    builder.set_title("Layout example");
    builder.set_handler(Box::new(UiMain::new(state)));

    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
