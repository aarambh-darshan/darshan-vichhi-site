//! Part 5: "a tiny secret". The page is Rust + Leptos; the animation is Animato + Scenix. Live frame-rate meter.
use crate::video::ctx::Ctx;
use crate::video::fx::*;
use crate::video::g2d::{Align, Font};
use crate::video::kit::*;
use crate::video::theme::*;

pub fn draw(c: &Ctx) {
    let a = c.a;
    let p0 = c.words.parts[4].0;
    secret(c, p0 - 0.3, a.page - 0.2);
    stack(c, a.page - 0.15, a.smooth - 0.3);
    meter(c, a.smooth - 0.25, c.words.parts[5].0 - 0.15);
}

fn secret(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.085).clamp(40.0, 112.0 * u.max(1.0));
    let words = ["A tiny", "secret."];
    for (i, word) in words.iter().enumerate() {
        let st = a.secret - 0.5 + i as f32 * 0.35;
        let p = ez(E::EaseOutCubic, t, st, 0.8);
        let y = c.cy() + (i as f32 - 0.3) * size * 1.05;
        let ww = g.text_w(word, size, Font::SerifItalic);
        g.save();
        g.alpha(w);
        g.c.begin_path();
        g.c.rect(
            (c.cx() - ww / 2.0 - 10.0) as f64,
            (y - size * 0.95) as f64,
            (ww + 20.0) as f64,
            (size * 1.3) as f64,
        );
        g.c.clip();
        g.text(
            word,
            c.cx(),
            y + (1.0 - p) * size,
            size,
            Font::SerifItalic,
            &rgba(if i == 1 { ACCENT } else { TEXT }, 1.0),
            Align::Center,
        );
        g.restore();
    }
}

fn stack(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let cs = 26.0 * u.clamp(0.7, 1.2);
    let rows = [
        (
            "this page",
            a.page,
            vec![("Rust", a.page + 0.2), ("Leptos", a.leptos)],
        ),
        (
            "this animation",
            a.watching,
            vec![("Animato", a.powered + 0.55), ("Scenix", a.powered + 1.3)],
        ),
    ];
    let row_h = (c.stage_h() * 0.42).min(170.0 * u.max(1.0));
    let y0 = c.cy() - row_h * 1.05;
    for (ri, (label, st, chips)) in rows.iter().enumerate() {
        let y = y0 + ri as f32 * row_h + row_h * 0.4;
        let total: f32 = chips.iter().map(|(l, _)| pill_w(g, l, cs)).sum::<f32>()
            + 16.0 * (chips.len() as f32 - 1.0);
        let x0 = c.cx() - total / 2.0;
        g.label(
            label,
            c.cx(),
            y - cs * 2.0,
            13.0,
            4.0,
            &rgba(MUTED, w * fade(c, *st)),
            Align::Center,
        );
        let mut x = x0;
        for (name, cst) in chips {
            let pw = pill_w(g, name, cs);
            let pp = pop(c, *cst).clamp(0.0, 1.06);
            g.save();
            g.c.translate((x + pw / 2.0) as f64, y as f64).ok();
            g.c.scale(pp as f64, pp as f64).ok();
            pill(
                g,
                name,
                -pw / 2.0,
                0.0,
                cs,
                w * prog(t, *cst, 0.12),
                ri == 1,
            );
            g.restore();
            x += pw + 16.0;
        }
    }
    let sp = ez(E::EaseOutCubic, t, a.mine, 0.24);
    if sp > 0.0 {
        stamp(
            g,
            "both mine",
            c.cx() + 170.0 * u.clamp(0.8, 1.3),
            y0 + row_h * 1.45 + 30.0 * u,
            20.0 * u.clamp(0.8, 1.3),
            -5.0,
            1.0 + 1.2 * (1.0 - sp),
            w * sp,
            ACCENT,
        );
    }
}

fn meter(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    if w <= 0.01 {
        return;
    }
    let cw = (g.w - 2.0 * c.pad()).min(700.0 * u.max(1.0));
    let ch = (c.stage_h() * 0.9).min(330.0 * u.max(1.0));
    let x = c.cx() - cw / 2.0;
    let y = c.cy() - ch / 2.0;
    card(g, x, y, cw, ch, w);
    // "smooth?" -> "genius."
    let (q, qa) = if t < a.genius {
        ("if it's smooth\u{2026}", fade(c, a.smooth))
    } else {
        ("genius.", fade(c, a.genius))
    };
    g.text(
        q,
        x + 28.0,
        y + 56.0 * u.clamp(0.8, 1.1),
        34.0 * u.clamp(0.75, 1.15),
        Font::SerifItalic,
        &rgba(if t < a.genius { TEXT } else { ACCENT }, w * qa),
        Align::Left,
    );
    // live fps, measured in your browser right now
    let fps = c.fps.round().max(0.0) as u32;
    g.text(
        &format!("{}", fps),
        x + 28.0,
        y + ch * 0.55,
        ch * 0.30,
        Font::Serif,
        &rgba(TEXT, w),
        Align::Left,
    );
    g.label(
        "fps  measured live, in your browser",
        x + 28.0,
        y + ch * 0.55 + 28.0,
        10.5,
        2.2,
        &rgba(TEXT2, w),
        Align::Left,
    );
    // frame-time sparkline
    let (sx, sy, sw, sh) = (
        x + cw * 0.46,
        y + 84.0 * u.clamp(0.8, 1.1),
        cw * 0.5,
        ch * 0.30,
    );
    g.stroke_rrect(sx, sy, sw, sh, 6.0, &rgba(BORDER, w), 1.0);
    let n = c.frame_ms.len();
    if n > 2 {
        g.c.begin_path();
        for (i, ms) in c.frame_ms.iter().enumerate() {
            let fx = sx + sw * i as f32 / (n - 1) as f32;
            let fy = sy + sh - (ms / 40.0).clamp(0.0, 1.0) * sh;
            if i == 0 {
                g.c.move_to(fx as f64, fy as f64)
            } else {
                g.c.line_to(fx as f64, fy as f64)
            }
        }
        g.c.set_stroke_style_str(&rgba(ACCENT, w));
        g.c.set_line_width(1.6);
        g.c.stroke();
    }
    g.label(
        "frame time",
        sx + sw,
        sy + sh + 16.0,
        9.5,
        2.0,
        &rgba(MUTED, w),
        Align::Right,
    );
    // browser jealousy
    if t > a.lags - 0.4 {
        let ja = fade(c, a.lags - 0.4);
        let gy = y + ch - 52.0 * u.clamp(0.8, 1.1);
        let gw = cw - 56.0;
        let fill = ez(
            E::EaseInOutCubic,
            t,
            a.lags,
            (a.jealous - a.lags + 1.2).max(1.0),
        );
        g.label(
            "if it lags\u{2026} browser jealousy",
            x + 28.0,
            gy - 16.0,
            10.5,
            2.4,
            &rgba(TEXT2, w * ja),
            Align::Left,
        );
        g.fill_rrect(x + 28.0, gy, gw, 12.0, 6.0, &rgba(BORDER, w * ja));
        let sh = shake(t, a.jealous, 0.6, 16.0) * 3.0;
        g.fill_rrect(x + 28.0 + sh, gy, gw * fill, 12.0, 6.0, &rgba(WARM, w * ja));
        g.text(
            &format!("{}%", (fill * 100.0).round() as u32),
            x + cw - 28.0,
            gy - 12.0,
            20.0,
            Font::SansBold,
            &rgba(WARM, w * ja),
            Align::Right,
        );
    }
}
