//! Reusable visual pieces for the scenes.
use crate::video::ctx::Ctx;
use crate::video::fx::{E, ez};
use crate::video::g2d::{Align, Font, G};
use crate::video::theme::*;

pub fn typed(s: &str, t: f32, t0: f32, cps: f32) -> String {
    if t < t0 {
        return String::new();
    }
    let n = ((t - t0) * cps) as usize;
    s.chars().take(n).collect()
}

pub fn thousands(n: u64) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

/// Pill with text; (x, y) is the left edge / vertical centre. Returns its width.
pub fn pill(g: &G, text: &str, x: f32, y: f32, size: f32, a: f32, accent: bool) -> f32 {
    let w = g.text_w(text, size, Font::SansMedium) + size * 1.7;
    let h = size * 2.25;
    g.save();
    g.alpha(a);
    g.fill_rrect(x, y - h / 2.0, w, h, h / 2.0, &rgba(SURFACE, 0.95));
    g.stroke_rrect(
        x,
        y - h / 2.0,
        w,
        h,
        h / 2.0,
        &rgba(if accent { ACCENT } else { BORDER }, 1.0),
        1.0,
    );
    g.text(
        text,
        x + w / 2.0,
        y + size * 0.36,
        size,
        Font::SansMedium,
        &rgba(if accent { ACCENT } else { TEXT }, 1.0),
        Align::Center,
    );
    g.restore();
    w
}
pub fn pill_w(g: &G, text: &str, size: f32) -> f32 {
    g.text_w(text, size, Font::SansMedium) + size * 1.7
}

pub fn card(g: &G, x: f32, y: f32, w: f32, h: f32, a: f32) {
    g.save();
    g.alpha(a);
    g.fill_rrect(x, y, w, h, 10.0, &rgba(SURFACE, 0.94));
    g.stroke_rrect(x, y, w, h, 10.0, &rgba(BORDER, 1.0), 1.0);
    g.restore();
}

/// A rubber stamp: bordered caps text, rotated, scaled (use `scale` for the slam).
#[allow(clippy::too_many_arguments)]
pub fn stamp(
    g: &G,
    text: &str,
    cx: f32,
    cy: f32,
    size: f32,
    rot_deg: f32,
    scale: f32,
    a: f32,
    color: Rgb,
) {
    if a <= 0.01 || scale <= 0.01 {
        return;
    }
    g.save();
    g.alpha(a);
    g.c.translate(cx as f64, cy as f64).ok();
    g.c.rotate(rot_deg.to_radians() as f64).ok();
    g.c.scale(scale as f64, scale as f64).ok();
    let w = g.text_w(&text.to_uppercase(), size, Font::SansBold)
        + size * 1.6
        + text.chars().count() as f32 * size * 0.12;
    let h = size * 2.3;
    g.stroke_rrect(-w / 2.0, -h / 2.0, w, h, 6.0, &rgba(color, 1.0), 2.4);
    g.stroke_rrect(
        -w / 2.0 + 5.0,
        -h / 2.0 + 5.0,
        w - 10.0,
        h - 10.0,
        3.0,
        &rgba(color, 0.55),
        1.0,
    );
    g.label(
        text,
        0.0,
        size * 0.36,
        size,
        size * 0.14,
        &rgba(color, 1.0),
        Align::Center,
    );
    g.restore();
}

/// A spring "pop": 0 -> 1 with overshoot, starting at `t0`.
pub fn pop(c: &Ctx, t0: f32) -> f32 {
    c.assets.pop.at(c.t - t0)
}

pub fn fade(c: &Ctx, t0: f32) -> f32 {
    ez(E::EaseOutCubic, c.t, t0, 0.5)
}

/// Line-art crab (a nod to Ferris) that scuttles. (x, y) = body centre.
pub fn crab(g: &G, x: f32, y: f32, s: f32, wall: f32, pinch: f32, a: f32) {
    g.save();
    g.alpha(a);
    g.c.translate(x as f64, y as f64).ok();
    g.c.scale(s as f64, s as f64).ok();
    let col = rgba(WARM, 1.0);
    let fillc = rgba(SURFACE_HI, 1.0);
    let bob = (wall * 9.0).sin() * 2.0;
    g.c.translate(0.0, bob as f64).ok();
    // legs
    for side in [-1.0f32, 1.0] {
        for i in 0..3 {
            let ph = wall * 11.0 + i as f32 * 1.7 + if side > 0.0 { 1.4 } else { 0.0 };
            let ky = 6.0 + i as f32 * 9.0;
            let kx = side * (46.0 + (ph.sin()) * 4.0);
            g.c.begin_path();
            g.c.move_to((side * 34.0) as f64, ky as f64);
            g.c.line_to(kx as f64, (ky + 10.0 + ph.cos() * 3.0) as f64);
            g.c.line_to((side * (60.0 + i as f32 * 3.0)) as f64, (ky + 26.0) as f64);
            g.c.set_stroke_style_str(&col);
            g.c.set_line_width(3.0);
            g.c.set_line_cap("round");
            g.c.set_line_join("round");
            g.c.stroke();
        }
    }
    // arms and claws
    for side in [-1.0f32, 1.0] {
        let open = 0.35 + pinch * 0.5 * (0.5 + 0.5 * (wall * 7.0 + side).sin());
        let (ax, ay): (f32, f32) = (side * 66.0, -30.0);
        g.c.begin_path();
        g.c.move_to((side * 38.0) as f64, -12.0);
        g.c.line_to((side * 56.0) as f64, -26.0);
        g.c.line_to(ax as f64, ay as f64);
        g.c.set_stroke_style_str(&col);
        g.c.set_line_width(4.0);
        g.c.stroke();
        // pincers
        for k in [-1.0f32, 1.0] {
            g.c.begin_path();
            let _ = g.c.arc(
                (ax + side * 8.0) as f64,
                (ay - 10.0) as f64,
                16.0,
                (-1.2 + k * open) as f64,
                (1.2 + k * open) as f64,
            );
            g.c.set_stroke_style_str(&col);
            g.c.set_line_width(3.2);
            g.c.stroke();
        }
    }
    // body
    g.c.begin_path();
    let _ =
        g.c.ellipse(0.0, 0.0, 44.0, 28.0, 0.0, 0.0, std::f64::consts::TAU);
    g.c.set_fill_style_str(&fillc);
    g.c.fill();
    g.c.set_stroke_style_str(&col);
    g.c.set_line_width(3.2);
    g.c.stroke();
    // eyes on stalks
    for side in [-1.0f32, 1.0] {
        g.line(side * 14.0, -24.0, side * 16.0, -40.0, &col, 2.6);
        g.fill_circle(side * 16.0, -43.0, 6.0, &rgba(TEXT, 1.0));
        g.fill_circle(side * 16.0 + side * -0.5, -42.0, 2.6, &rgba(BG, 1.0));
    }
    // grumpy brow + mouth
    g.line(-12.0, -33.0, -4.0, -29.0, &col, 2.4);
    g.line(12.0, -33.0, 4.0, -29.0, &col, 2.4);
    g.c.begin_path();
    let _ = g.c.arc(0.0, 17.0, 11.0, 3.6, 5.8);
    g.c.set_stroke_style_str(&col);
    g.c.set_line_width(2.4);
    g.c.stroke();
    g.restore();
}

/// Side-view sports car silhouette. (x, y) = centre of the underbody.
pub fn car(g: &G, x: f32, y: f32, s: f32, wheel_rot: f32, a: f32) {
    g.save();
    g.alpha(a);
    g.c.translate(x as f64, y as f64).ok();
    g.c.scale(s as f64, s as f64).ok();
    let col = rgba(WARM, 1.0);
    g.c.begin_path();
    g.c.move_to(-86.0, 0.0);
    g.c.line_to(-86.0, -14.0);
    g.c.quadratic_curve_to(-80.0, -22.0, -56.0, -24.0);
    g.c.line_to(-30.0, -36.0);
    g.c.quadratic_curve_to(-8.0, -44.0, 24.0, -36.0);
    g.c.line_to(48.0, -22.0);
    g.c.quadratic_curve_to(80.0, -20.0, 88.0, -8.0);
    g.c.line_to(88.0, 0.0);
    g.c.close_path();
    g.c.set_fill_style_str(&rgba(SURFACE_HI, 1.0));
    g.c.fill();
    g.c.set_stroke_style_str(&col);
    g.c.set_line_width(3.0);
    g.c.set_line_join("round");
    g.c.stroke();
    g.line(-26.0, -34.0, 22.0, -34.0, &col, 2.0);
    for wx in [-52.0f32, 54.0] {
        g.fill_circle(wx, 2.0, 15.0, &rgba(BG, 1.0));
        g.stroke_circle(wx, 2.0, 15.0, &col, 3.0);
        for k in 0..4 {
            let an = wheel_rot + k as f32 * std::f32::consts::FRAC_PI_2;
            g.line(
                wx,
                2.0,
                wx + an.cos() * 11.0,
                2.0 + an.sin() * 11.0,
                &col,
                1.6,
            );
        }
    }
    g.restore();
}
