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

use druid::shell::{runloop, WindowBuilder};
use druid::widget::{Column, DynLabel, Padding, TextBox};
use druid::{UiMain, UiState, Data};

#[derive(Debug, Clone)]
struct AppData {
    text: String,
}

impl Data for AppData {
    fn same(&self, o: &Self) -> bool {
        self.text == o.text
    }
}

fn main() {
    druid::shell::init();

    let mut run_loop = runloop::RunLoop::new();
    let mut builder = WindowBuilder::new();
    let mut col = Column::new();

    let textbox = TextBox::new(200.)
        .placeholder(|| "Input text...".to_string())
        .value(|data: &AppData| data.text.clone())
        .on_change(|v: String, data: &mut AppData| {
            data.text = v.to_string();
        });
    let label = DynLabel::new(|data: &AppData, _env| format!("value: {}", data.text));

    col.add_child(Padding::uniform(5.0, textbox), 1.0);
    col.add_child(Padding::uniform(5.0, label), 1.0);

    let state = UiState::new(col, AppData{
        text: "typing is fun!".to_string(),
    });
    builder.set_title("TextBox example");
    builder.set_handler(Box::new(UiMain::new(state)));
    let window = builder.build().unwrap();
    window.show();
    run_loop.run();
}
