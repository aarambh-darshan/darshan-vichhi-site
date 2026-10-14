//! Everything that sits on top of the scenes: header, karaoke captions, controls and scrubber.
use crate::video::ctx::{Action, Ctx, PART_NAMES};
use crate::video::fx::{E, ez, prog, window};
use crate::video::g2d::{Align, Font};
use crate::video::theme::*;

pub fn draw(c: &Ctx) {
    header(c);
    if c.captions {
        captions(c);
    }
    controls(c);
}

fn fmt_time(s: f32) -> String {
    let s = s.max(0.0) as u32;
    format!("{}:{:02}", s / 60, s % 60)
}

fn header(c: &Ctx) {
    let g = c.g;
    let a = ez(E::EaseOutCubic, c.t, -0.2, 0.8);
    if a <= 0.0 {
        return;
    }
    let u = g.u;
    let pad = c.pad();
    g.save();
    g.alpha(a);
    g.text(
        "Darshan Vichhi",
        pad,
        46.0 * u.min(1.2),
        22.0 * u.clamp(0.8, 1.2),
        Font::Serif,
        &rgba(TEXT, 1.0),
        Align::Left,
    );
    if !g.compact {
        g.label(
            "Rust Systems & AI Engineer",
            pad,
            68.0 * u.min(1.2),
            11.0,
            2.6,
            &rgba(MUTED, 1.0),
            Align::Left,
        );
    }
    // close
    let (bx, by, br) = (g.w - pad - 20.0, 40.0 * u.min(1.2), 20.0);
    let hov = c.hovering(Action::Close);
    g.stroke_circle(
        bx,
        by,
        br,
        &rgba(if hov { ACCENT } else { BORDER }, 1.0),
        1.0,
    );
    let d = 6.5;
    let col = rgba(if hov { TEXT } else { TEXT2 }, 1.0);
    g.line(bx - d, by - d, bx + d, by + d, &col, 1.6);
    g.line(bx + d, by - d, bx - d, by + d, &col, 1.6);
    c.hit(bx - 24.0, by - 24.0, 48.0, 48.0, Action::Close);
    // part counter
    let part = c.words.part_at(c.t.max(0.0));
    let label = if g.compact {
        format!("{:02} / {:02}", part + 1, PART_NAMES.len())
    } else {
        format!(
            "{:02} / {:02}   {}",
            part + 1,
            PART_NAMES.len(),
            PART_NAMES[part.min(PART_NAMES.len() - 1)]
        )
    };
    g.label(
        &label,
        bx - br - 18.0,
        by + 4.0,
        11.0,
        2.4,
        &rgba(TEXT2, 1.0),
        Align::Right,
    );
    g.restore();
}

fn captions(c: &Ctx) {
    let g = c.g;
    let t = c.t;
    let Some(li) = c.words.line_at(t) else { return };
    let line = &c.words.lines[li];
    let next_s = c.words.lines.get(li + 1).map(|l| l.s).unwrap_or(f32::MAX);
    // fade out just before the next sentence takes over (or after the last one ends)
    let a_in = ez(E::EaseOutCubic, t, line.s - 0.3, 0.3);
    let a_out = if next_s == f32::MAX {
        1.0 - prog(t, line.e + 1.2, 0.6)
    } else {
        1.0 - prog(t, next_s - 0.42, 0.12)
    };
    let a = (a_in * a_out).clamp(0.0, 1.0);
    if a <= 0.01 {
        return;
    }
    let size = (28.0 * g.u).clamp(17.0, 34.0);
    let max_w = (g.w - 2.0 * c.pad()).min(1000.0 * g.u.max(0.8));
    let space = g.text_w(" ", size, Font::Serif);
    // greedy wrap by word widths
    let mut rows: Vec<Vec<(usize, f32)>> = vec![vec![]];
    let mut row_w = 0.0;
    for (i, w) in line.words.iter().enumerate() {
        let ww = g.text_w(&w.text, size, Font::Serif);
        if row_w + ww > max_w && !rows.last().unwrap().is_empty() {
            rows.push(vec![]);
            row_w = 0.0;
        }
        rows.last_mut().unwrap().push((i, ww));
        row_w += ww + space;
    }
    let lh = size * 1.36;
    let base_y =
        g.h - if g.compact {
            128.0
        } else {
            118.0 * g.u.clamp(0.8, 1.2)
        } - (rows.len() as f32 - 1.0) * lh;
    g.save();
    g.alpha(a);
    for (ri, row) in rows.iter().enumerate() {
        let total: f32 =
            row.iter().map(|(_, w)| *w).sum::<f32>() + space * (row.len() as f32 - 1.0);
        let mut x = c.cx() - total / 2.0;
        let y = base_y + ri as f32 * lh + (1.0 - a) * 8.0;
        for (i, w) in row {
            let word = &line.words[*i];
            let k = prog(t, word.s, (word.e - word.s).max(0.12));
            let (color, font) = if t >= word.e {
                (rgba(TEXT, 1.0), Font::Serif)
            } else if t >= word.s {
                (rgba(ACCENT, 1.0), Font::Serif)
            } else {
                (rgba(MUTED, 0.9), Font::Serif)
            };
            let lift = if t >= word.s && t < word.e + 0.15 {
                -2.5 * (1.0 - k)
            } else {
                0.0
            };
            g.text(&word.text, x, y + lift, size, font, &color, Align::Left);
            x += w + space;
        }
    }
    g.restore();
}

fn icon_button(c: &Ctx, x: f32, y: f32, r: f32, action: Action, draw: impl Fn(f32, f32, &str)) {
    let g = c.g;
    let hov = c.hovering(action);
    g.fill_circle(x, y, r, &rgba(if hov { SURFACE_HI } else { SURFACE }, 0.9));
    g.stroke_circle(x, y, r, &rgba(if hov { ACCENT } else { BORDER }, 1.0), 1.0);
    draw(x, y, &rgba(if hov { TEXT } else { TEXT2 }, 1.0));
    c.hit(
        x - r - 4.0,
        y - r - 4.0,
        2.0 * r + 8.0,
        2.0 * r + 8.0,
        action,
    );
}

fn controls(c: &Ctx) {
    let g = c.g;
    let a = ez(E::EaseOutCubic, c.t, 0.2, 0.8);
    let pad = c.pad();
    let y = g.h - 64.0;
    let r = 17.0;
    g.save();
    g.alpha(a);
    // play / pause
    icon_button(c, pad + r, y, r, Action::PlayPause, |x, y, col| {
        if c.paused || c.ended {
            g.play_glyph(x, y, 6.5, col);
        } else {
            g.fill_rrect(x - 6.0, y - 6.5, 4.0, 13.0, 1.0, col);
            g.fill_rrect(x + 2.0, y - 6.5, 4.0, 13.0, 1.0, col);
        }
    });
    // mute
    icon_button(c, pad + r + 46.0, y, r, Action::Mute, |x, y, col| {
        g.c.begin_path();
        g.c.move_to((x - 7.0) as f64, (y - 3.0) as f64);
        g.c.line_to((x - 3.5) as f64, (y - 3.0) as f64);
        g.c.line_to((x + 1.0) as f64, (y - 7.0) as f64);
        g.c.line_to((x + 1.0) as f64, (y + 7.0) as f64);
        g.c.line_to((x - 3.5) as f64, (y + 3.0) as f64);
        g.c.line_to((x - 7.0) as f64, (y + 3.0) as f64);
        g.c.close_path();
        g.c.set_fill_style_str(col);
        g.c.fill();
        if c.muted {
            g.line(x + 4.0, y - 4.0, x + 9.0, y + 4.0, col, 1.5);
            g.line(x + 9.0, y - 4.0, x + 4.0, y + 4.0, col, 1.5);
        } else {
            g.c.begin_path();
            let _ = g.c.arc((x + 1.5) as f64, y as f64, 5.0, -0.9, 0.9);
            g.c.set_stroke_style_str(col);
            g.c.set_line_width(1.5);
            g.c.stroke();
        }
    });
    // captions
    icon_button(c, pad + r + 92.0, y, r, Action::Captions, |x, y, col| {
        let accent = rgba(ACCENT, 1.0);
        g.text(
            "CC",
            x,
            y + 4.0,
            11.5,
            Font::SansBold,
            if c.captions { &accent } else { col },
            Align::Center,
        );
    });
    // time
    if !g.compact {
        let cur = c.t.clamp(0.0, c.words.duration);
        let label = format!("{} / {}", fmt_time(cur), fmt_time(c.words.duration));
        g.text(
            &label,
            g.w - pad,
            y + 5.0,
            13.0,
            Font::Mono,
            &rgba(TEXT2, 1.0),
            Align::Right,
        );
    }
    // scrubber
    let sy = g.h - 30.0;
    let (x0, x1) = (pad, g.w - pad);
    let frac = (c.t / c.words.duration).clamp(0.0, 1.0);
    let hov = c.hovering(Action::Scrub) || c.hovering(Action::Part(0));
    let th = if hov { 5.0 } else { 3.0 };
    g.fill_rrect(x0, sy - th / 2.0, x1 - x0, th, th / 2.0, &rgba(BORDER, 1.0));
    g.fill_rrect(
        x0,
        sy - th / 2.0,
        (x1 - x0) * frac,
        th,
        th / 2.0,
        &rgba(ACCENT, 1.0),
    );
    for (i, (s, _)) in c.words.parts.iter().enumerate() {
        let px = x0 + (x1 - x0) * (s / c.words.duration);
        if i > 0 {
            g.fill_rrect(px - 1.0, sy - 5.0, 2.0, 10.0, 1.0, &rgba(BG, 1.0));
        }
        // chapter hit areas (click to jump)
        let nx = c
            .words
            .parts
            .get(i + 1)
            .map(|p| p.0)
            .unwrap_or(c.words.duration);
        let wx = x0 + (x1 - x0) * (nx / c.words.duration) - px;
        c.hit(px, sy - 16.0, wx, 32.0, Action::Part(i));
        let _ = wx;
    }
    g.fill_circle(
        x0 + (x1 - x0) * frac,
        sy,
        if hov { 7.0 } else { 5.0 },
        &rgba(TEXT, 1.0),
    );
    // chapter name on hover
    if let Some(Action::Part(i)) = c.hover {
        let px = x0 + (x1 - x0) * (c.words.parts[i].0 / c.words.duration);
        g.label(
            PART_NAMES[i],
            px.clamp(x0 + 40.0, x1 - 40.0),
            sy - 18.0,
            11.0,
            2.0,
            &rgba(TEXT, 1.0),
            Align::Center,
        );
    }
    g.restore();
}

/// Opening-credit: shown only while the page expands into the video.
pub fn loading_ring(c: &Ctx, x: f32, y: f32) {
    let g = c.g;
    g.save();
    let a = 0.5 + 0.5 * (c.wall * 4.0).sin();
    g.c.begin_path();
    let _ = g.c.arc(
        x as f64,
        y as f64,
        18.0,
        (c.wall * 5.0) as f64,
        (c.wall * 5.0 + 2.2) as f64,
    );
    g.c.set_stroke_style_str(&rgba(ACCENT, 0.4 + 0.5 * a));
    g.c.set_line_width(2.0);
    g.c.stroke();
    g.restore();
    let _ = window;
}
