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

use crate::{
    Action, BaseState, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, Point,
    Rect, Size, UpdateCtx, Vec2, Widget, WidgetPod,
};

use crate::widget::{Scroll, Column};

use crate::piet::RenderContext;

pub struct ListView<T: Data> {
    child: Scroll<T>,
}

impl<T: Data + 'static> ListView<T> {
    pub fn new(children: Vec<impl Widget<T> + 'static>) -> Self {
        let mut flex = Column::new();
        for child in children {
            flex.add_child(child, 0.0);
        }

        Self {
            child: Scroll::new(flex).vertical(),
        }
    }
}

impl<T: Data> Widget<T> for ListView<T> {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, base_state: &BaseState, data: &T, env: &Env) {
        self.child.paint(paint_ctx, base_state, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut T,
        env: &Env,
    ) -> Option<Action> {
        self.child.event(event, ctx, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.child.update(ctx, old_data, data, env);
    }
}

pub struct DynListView<C> {
    child: Scroll<Vec<String>>,
    builder: Box<dyn Fn(usize, &String) -> C>,
}

impl<C: Widget<Vec<String>> + 'static> DynListView<C> {
    pub fn new(builder: impl Fn(usize, &String) -> C + 'static) -> Self {
        let flex = Column::new();
        Self {
            child: Scroll::new(flex).vertical(),
            builder: Box::new(builder),
        }
    }

    fn rebuild(&mut self, data: &Vec<String>) {
        // TODO: whenever we rebuild the scroll and the column scroll offset is reset
        //       can we remove individual widgets manually?
        let mut flex = Column::new();
        for (i, item) in data.iter().enumerate() {
            flex.add_child((self.builder)(i, item), 0.0);
        }

        self.child = Scroll::new(flex).vertical();
        println!("Updating child");
        dbg!(data);
    }
}

impl<C: Widget<Vec<String>> + 'static> Widget<Vec<String>> for DynListView<C> {
    fn paint(&mut self, paint_ctx: &mut PaintCtx, base_state: &BaseState, data: &Vec<String>, env: &Env) {
        self.child.paint(paint_ctx, base_state, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Vec<String>, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut Vec<String>,
        env: &Env,
    ) -> Option<Action> {
        self.child.event(event, ctx, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&Vec<String>>, data: &Vec<String>, env: &Env) {
        match old_data {
            Some(old) if !old.same(data) => {
                self.rebuild(data);
                ctx.invalidate();
            },
            None => {
                self.rebuild(data);
                ctx.invalidate();
            }
            _ => {
                self.child.update(ctx, old_data, data, env);
            },
        };

    }
}
