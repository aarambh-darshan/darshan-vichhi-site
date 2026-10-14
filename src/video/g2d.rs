//! A thin drawing toolkit over the browser's Canvas 2D context.
use crate::video::theme::{MONO, Rgb, SANS, SERIF, rgba};
use std::cell::RefCell;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

thread_local! {
    static SPRITES: RefCell<Vec<(Option<Rgb>, HtmlCanvasElement)>> = const { RefCell::new(Vec::new()) };
}

/// Pre-rendered radial sprites (a glow per colour, and the vignette): drawing a scaled image is far
/// cheaper than filling a large gradient every frame.
fn sprite(color: Option<Rgb>) -> Option<HtmlCanvasElement> {
    SPRITES.with(|sp| {
        if let Some((_, c)) = sp.borrow().iter().find(|(k, _)| *k == color) {
            return Some(c.clone());
        }
        let doc = web_sys::window()?.document()?;
        let cv: HtmlCanvasElement = doc.create_element("canvas").ok()?.dyn_into().ok()?;
        let n = if color.is_some() { 128 } else { 256 };
        cv.set_width(n);
        cv.set_height(n);
        let ctx: CanvasRenderingContext2d = cv.get_context("2d").ok()??.dyn_into().ok()?;
        let half = n as f64 / 2.0;
        let grad = match color {
            Some(c) => {
                let g = ctx
                    .create_radial_gradient(half, half, 0.0, half, half, half)
                    .ok()?;
                let _ = g.add_color_stop(0.0, &rgba(c, 1.0));
                let _ = g.add_color_stop(1.0, &rgba(c, 0.0));
                g
            }
            None => {
                let g = ctx
                    .create_radial_gradient(half, half, half * 0.5, half, half, half * 1.42)
                    .ok()?;
                let _ = g.add_color_stop(0.0, "rgba(0,0,0,0)");
                let _ = g.add_color_stop(1.0, "rgba(0,0,0,0.36)");
                g
            }
        };
        ctx.set_fill_style_canvas_gradient(&grad);
        ctx.fill_rect(0.0, 0.0, n as f64, n as f64);
        sp.borrow_mut().push((color, cv.clone()));
        Some(cv)
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum Font {
    Serif,
    SerifItalic,
    SerifBold,
    Sans,
    SansMedium,
    SansBold,
    Mono,
    MonoBold,
}
impl Font {
    pub fn css(self, size: f32) -> String {
        let (style, family) = match self {
            Font::Serif => ("400", SERIF),
            Font::SerifItalic => ("italic 400", SERIF),
            Font::SerifBold => ("700", SERIF),
            Font::Sans => ("400", SANS),
            Font::SansMedium => ("500", SANS),
            Font::SansBold => ("700", SANS),
            Font::Mono => ("400", MONO),
            Font::MonoBold => ("700", MONO),
        };
        format!("{} {:.1}px {}", style, size, family)
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

/// Per-frame drawing context: the canvas plus layout units.
#[allow(dead_code)]
pub struct G {
    pub c: CanvasRenderingContext2d,
    /// Logical (CSS pixel) size of the viewport.
    pub w: f32,
    pub h: f32,
    /// Base unit: scales type and spacing with the viewport (1.0 at 1280x720).
    pub u: f32,
    pub compact: bool,
    pub reduced: bool,
}

#[allow(dead_code)]
impl G {
    pub fn alpha(&self, a: f32) {
        self.c.set_global_alpha(a.clamp(0.0, 1.0) as f64);
    }
    pub fn fill(&self, color: &str) {
        self.c.set_fill_style_str(color);
    }
    pub fn stroke(&self, color: &str) {
        self.c.set_stroke_style_str(color);
    }
    pub fn save(&self) {
        self.c.save();
    }
    pub fn restore(&self) {
        self.c.restore();
    }

    // ---------------------------------------------------------------- text
    #[allow(clippy::too_many_arguments)]
    pub fn text(&self, s: &str, x: f32, y: f32, size: f32, font: Font, color: &str, align: Align) {
        self.c.set_font(&font.css(size));
        self.c.set_text_align(match align {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
        });
        self.c.set_text_baseline("alphabetic");
        self.c.set_fill_style_str(color);
        let _ = self.c.fill_text(s, x as f64, y as f64);
    }
    pub fn text_w(&self, s: &str, size: f32, font: Font) -> f32 {
        self.c.set_font(&font.css(size));
        self.c
            .measure_text(s)
            .map(|m| m.width() as f32)
            .unwrap_or(0.0)
    }
    /// Small caps style label with manual letter spacing (the canvas has no portable letter-spacing).
    #[allow(clippy::too_many_arguments)]
    pub fn label(
        &self,
        s: &str,
        x: f32,
        y: f32,
        size: f32,
        spacing: f32,
        color: &str,
        align: Align,
    ) {
        let font = Font::SansMedium;
        self.c.set_font(&font.css(size));
        let up = s.to_uppercase();
        let chars: Vec<char> = up.chars().collect();
        let widths: Vec<f32> = chars
            .iter()
            .map(|ch| {
                self.c
                    .measure_text(&ch.to_string())
                    .map(|m| m.width() as f32)
                    .unwrap_or(size * 0.6)
            })
            .collect();
        let total: f32 =
            widths.iter().sum::<f32>() + spacing * (chars.len().saturating_sub(1)) as f32;
        let mut cx = match align {
            Align::Left => x,
            Align::Center => x - total / 2.0,
            Align::Right => x - total,
        };
        self.c.set_text_align("left");
        self.c.set_text_baseline("alphabetic");
        self.c.set_fill_style_str(color);
        for (ch, w) in chars.iter().zip(widths) {
            let _ = self.c.fill_text(&ch.to_string(), cx as f64, y as f64);
            cx += w + spacing;
        }
    }
    pub fn wrap(&self, s: &str, size: f32, font: Font, max_w: f32) -> Vec<String> {
        self.c.set_font(&font.css(size));
        let mut out = Vec::new();
        let mut line = String::new();
        for w in s.split_whitespace() {
            let test = if line.is_empty() {
                w.to_string()
            } else {
                format!("{} {}", line, w)
            };
            let width = self
                .c
                .measure_text(&test)
                .map(|m| m.width() as f32)
                .unwrap_or(0.0);
            if !line.is_empty() && width > max_w {
                out.push(std::mem::take(&mut line));
                line = w.to_string();
            } else {
                line = test;
            }
        }
        if !line.is_empty() {
            out.push(line);
        }
        out
    }

    // ---------------------------------------------------------------- shapes
    pub fn rrect_path(&self, x: f32, y: f32, w: f32, h: f32, r: f32) {
        self.c.begin_path();
        let r = r.min(w / 2.0).min(h / 2.0).max(0.0);
        let _ = self
            .c
            .round_rect_with_f64(x as f64, y as f64, w as f64, h as f64, r as f64);
    }
    pub fn fill_rrect(&self, x: f32, y: f32, w: f32, h: f32, r: f32, color: &str) {
        self.rrect_path(x, y, w, h, r);
        self.c.set_fill_style_str(color);
        self.c.fill();
    }
    #[allow(clippy::too_many_arguments)]
    pub fn stroke_rrect(&self, x: f32, y: f32, w: f32, h: f32, r: f32, color: &str, lw: f32) {
        self.rrect_path(x, y, w, h, r);
        self.c.set_stroke_style_str(color);
        self.c.set_line_width(lw as f64);
        self.c.stroke();
    }
    pub fn line(&self, x0: f32, y0: f32, x1: f32, y1: f32, color: &str, lw: f32) {
        self.c.begin_path();
        self.c.move_to(x0 as f64, y0 as f64);
        self.c.line_to(x1 as f64, y1 as f64);
        self.c.set_stroke_style_str(color);
        self.c.set_line_width(lw as f64);
        self.c.set_line_cap("round");
        self.c.stroke();
    }
    pub fn circle(&self, x: f32, y: f32, r: f32) {
        self.c.begin_path();
        let _ = self.c.arc(
            x as f64,
            y as f64,
            r.max(0.0) as f64,
            0.0,
            std::f64::consts::TAU,
        );
    }
    pub fn fill_circle(&self, x: f32, y: f32, r: f32, color: &str) {
        self.circle(x, y, r);
        self.c.set_fill_style_str(color);
        self.c.fill();
    }
    pub fn stroke_circle(&self, x: f32, y: f32, r: f32, color: &str, lw: f32) {
        self.circle(x, y, r);
        self.c.set_stroke_style_str(color);
        self.c.set_line_width(lw as f64);
        self.c.stroke();
    }
    /// Soft radial glow.
    pub fn glow(&self, x: f32, y: f32, r: f32, color: Rgb, a: f32) {
        if a <= 0.001 {
            return;
        }
        if let Some(sp) = sprite(Some(color)) {
            self.c.save();
            self.c.set_global_alpha(a.clamp(0.0, 1.0) as f64);
            let _ = self.c.draw_image_with_html_canvas_element_and_dw_and_dh(
                &sp,
                (x - r) as f64,
                (y - r) as f64,
                (r * 2.0) as f64,
                (r * 2.0) as f64,
            );
            self.c.restore();
        }
    }
    /// Darkened corners.
    pub fn vignette(&self) {
        if let Some(sp) = sprite(None) {
            let _ = self.c.draw_image_with_html_canvas_element_and_dw_and_dh(
                &sp,
                0.0,
                0.0,
                self.w as f64,
                self.h as f64,
            );
        }
    }
    pub fn dash(&self, on: f32, off: f32) {
        let arr = js_sys::Array::of2(
            &JsValue::from_f64(on as f64),
            &JsValue::from_f64(off as f64),
        );
        let _ = self.c.set_line_dash(&arr);
    }
    pub fn solid(&self) {
        let _ = self.c.set_line_dash(&js_sys::Array::new());
    }
    /// Play triangle centred on (x, y) with "radius" r.
    pub fn play_glyph(&self, x: f32, y: f32, r: f32, color: &str) {
        self.c.begin_path();
        self.c.move_to((x - r * 0.62) as f64, (y - r) as f64);
        self.c.line_to((x + r * 0.95) as f64, y as f64);
        self.c.line_to((x - r * 0.62) as f64, (y + r) as f64);
        self.c.close_path();
        self.c.set_fill_style_str(color);
        self.c.fill();
    }
}
