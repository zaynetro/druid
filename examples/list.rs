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

//! This example shows how to construct a list view.

use druid::piet::Color;
use druid::shell::{runloop, WindowBuilder};
use druid::widget::{
    ActionWrapper, Button, Column, DynListView, Label, ListView, Padding, Row, SizedBox,
};
use druid::{Data, UiMain, UiState, Widget};

fn build_app() -> impl Widget<Vec<String>> {
    let mut row = Row::new();

    row.add_child(
        ListView::new(vec![
            SizedBox::new(Label::new("Fixed list"))
                .expand()
                .height(40.0)
                .bg(Color::rgb(0.7, 0.7, 0.7)),
            SizedBox::new(Label::new("is"))
                .expand()
                .height(60.0)
                .bg(Color::rgb(0.4, 0.4, 0.4)),
            SizedBox::new(Label::new("here"))
                .expand()
                .height(40.0)
                .bg(Color::rgb(0.2, 0.2, 0.2)),
            SizedBox::new(ActionWrapper::new(
                Button::new("Add"),
                |data: &mut Vec<String>, _env| {
                    data.push(format!("Dynamically added: #{}", data.len()))
                },
            ))
            .expand()
            .height(40.0),
            SizedBox::new(ActionWrapper::new(
                Button::new("Remove"),
                |data: &mut Vec<String>, _env| {
                    data.pop();
                },
            ))
            .expand()
            .height(40.0),
        ]),
        1.0,
    );

    row.add_child(
        DynListView::new(|i, v: &String| {
            SizedBox::new(Label::new(format!("{}: {}", i, v)))
                .expand()
                .height(((i + 1) as f64) * 20.0)
                .bg(Color::rgb(0.4, 0.4, 0.4))
        }),
        1.0,
    );

    row
}

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();

    let mut data = Vec::new();
    for i in 0..3 {
        data.push(format!("Dynamic line #{}", i));
    }

    // Build app layout
    let root = build_app();
    // Set up initial app state
    let state = UiState::new(root, data);
    builder.set_title("List example");
    builder.set_handler(Box::new(UiMain::new(state)));

    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
