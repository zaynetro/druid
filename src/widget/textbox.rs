// Copyright 2018 The xi-editor Authors.
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

//! A textbox widget.

use crate::{
    Action, BaseState, BoxConstraints, Env, Event, EventCtx, KeyCode, LayoutCtx, PaintCtx,
    UpdateCtx, Widget,
};

use crate::kurbo::{Line, Point, RoundedRect, Size, Vec2};
use crate::piet::{
    Color, FillRule, FontBuilder, Piet, RenderContext, Text, TextLayout, TextLayoutBuilder,
};

use std::marker::PhantomData;

const BACKGROUND_GREY_LIGHT: Color = Color::rgba32(0x3a_3a_3a_ff);
const BORDER_GREY: Color = Color::rgba32(0x5a_5a_5a_ff);
const PRIMARY_LIGHT: Color = Color::rgba32(0x5c_c4_ff_ff);

const TEXT_COLOR: Color = Color::rgb24(0xf0_f0_ea);
const PLACEHOLDER_COLOR: Color = Color::rgba32(0xf0_f0_ea_88);
const CURSOR_COLOR: Color = Color::WHITE;

const BOX_HEIGHT: f64 = 24.;
const FONT_SIZE: f64 = 14.0;
const BORDER_WIDTH: f64 = 1.;
const PADDING_TOP: f64 = 5.;
const PADDING_LEFT: f64 = 4.;

#[derive(Debug, Clone)]
pub struct TextBox<V, C, P, T> {
    width: f64,
    value: Option<V>,
    placeholder: Option<P>,
    on_change: Option<C>,
    phantom: PhantomData<T>,
}

impl <V, C, P, T> TextBox<V, C, P, T> where
    V: Fn(&T) -> String,
    C: Fn(String, &mut T),
    P: Fn() -> String
{
    pub fn new(width: f64) -> Self {
        Self {
            width,
            value: None,
            placeholder: None,
            on_change: None,
            phantom: Default::default(),
        }
    }

    fn get_layout(
        &mut self,
        text: &mut <Piet as RenderContext>::Text,
        font_size: f64,
        data: &String,
    ) -> <Piet as RenderContext>::TextLayout {
        // TODO: caching of both the format and the layout
        let font = text
            .new_font_by_name("Roboto", font_size)
            .unwrap()
            .build()
            .unwrap();
        text.new_text_layout(&font, data).unwrap().build().unwrap()
    }

    pub fn value(mut self, v: V) -> Self {
        self.value = Some(v);
        self
    }

    pub fn placeholder(mut self, p: P) -> Self {
        self.placeholder = Some(p);
        self
    }

    pub fn on_change(mut self, o: C) -> Self {
        self.on_change = Some(o);
        self
    }

    fn get_text(&self, data: &T) -> String {
        match self.value {
            Some(ref v) => v(data),
            None => "no value".to_string(),
        }
    }

    fn change_text(&self, data: &mut T, text: String) {
        match self.on_change {
            Some(ref o) => {
                o(text, data);
            },
            None => {},
        }
    }
}

impl <V, C, P, T> Widget<T> for TextBox<V, C, P, T> where
    V: Fn(&T) -> String,
    C: Fn(String, &mut T),
    P: Fn() -> String
{
    fn paint(
        &mut self,
        paint_ctx: &mut PaintCtx,
        base_state: &BaseState,
        data: &T,
        _env: &Env,
    ) {
        let is_active = base_state.is_active();

        let border_color = if is_active {
            PRIMARY_LIGHT
        } else {
            BORDER_GREY
        };

        // Paint the border / background
        let background_brush = paint_ctx.render_ctx.solid_brush(BACKGROUND_GREY_LIGHT);
        let border_brush = paint_ctx.render_ctx.solid_brush(border_color);

        let clip_rect = RoundedRect::from_origin_size(
            Point::ORIGIN,
            Size::new(
                base_state.size().width - BORDER_WIDTH,
                base_state.size().height,
            )
            .to_vec2(),
            2.,
        );

        paint_ctx
            .render_ctx
            .fill(clip_rect, &background_brush, FillRule::NonZero);

        paint_ctx
            .render_ctx
            .stroke(clip_rect, &border_brush, BORDER_WIDTH, None);

        // Paint the text
        let mut text_value = self.get_text(data);
        let text_color = match self.placeholder {
            Some(ref p) if text_value.is_empty() => {
                text_value = p();
                PLACEHOLDER_COLOR
            },
            _ => TEXT_COLOR,
        };

        let text = paint_ctx.render_ctx.text();
        let text_layout = self.get_layout(text, FONT_SIZE, &text_value);
        let brush = paint_ctx.render_ctx.solid_brush(text_color);

        let text_height = FONT_SIZE * 0.8;
        let text_pos = Point::new(0.0 + PADDING_LEFT, text_height + PADDING_TOP);

        // Render text and cursor inside a clip
        paint_ctx
            .render_ctx
            .with_save(|rc| {
                rc.clip(clip_rect, FillRule::NonZero);
                rc.draw_text(&text_layout, text_pos, &brush);

                // Paint the cursor if focused
                if is_active {
                    let brush = rc.solid_brush(CURSOR_COLOR);

                    let xy = text_pos + Vec2::new(text_layout.width() + 2., 2. - FONT_SIZE);
                    let x2y2 = xy + Vec2::new(0., FONT_SIZE + 2.);
                    let line = Line::new(xy, x2y2);

                    rc.stroke(line, &brush, 1., None);
                }
                Ok(())
            })
            .unwrap();
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> Size {
        Size::new(self.width, BOX_HEIGHT)
    }

    fn event(
        &mut self,
        event: &Event,
        ctx: &mut EventCtx,
        data: &mut T,
        _env: &Env,
    ) -> Option<Action> {
        match event {
            Event::MouseDown(_) => {
                if ctx.is_hot() {
                    ctx.set_active(true);
                } else {
                    ctx.set_active(false);
                }
                ctx.invalidate();
            }
            Event::KeyDown(key_event) => {
                match key_event {
                    event if event.key_code == KeyCode::Backspace => {
                        let mut text = self.get_text(data);
                        text.pop();
                        self.change_text(data, text);
                    }
                    event if event.key_code.is_printable() => {
                        let mut text = self.get_text(data);
                        text.push_str(event.text().unwrap_or(""));
                        self.change_text(data, text);
                    }
                    _ => {}
                }
                ctx.invalidate();
            }
            _ => (),
        }
        None
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: Option<&T>,
        _data: &T,
        _env: &Env,
    ) {
        ctx.invalidate();
    }
}
