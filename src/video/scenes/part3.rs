//! Part 3: Aarambh Dev Hub. Channel cards, the "forced to use Rust" checklist, 37 chapters, 1,000,000,000 rows.
use crate::video::ctx::{Action, Ctx, LINKS};
use crate::video::fx::*;
use crate::video::g2d::{Align, Font, G};
use crate::video::kit::*;
use crate::video::theme::*;

pub fn draw(c: &Ctx) {
    let a = c.a;
    let p0 = c.words.parts[2].0;
    hub(c, p0 - 0.3, a.forced - 0.25);
    forced(c, a.forced - 0.2, a.teach - 0.3);
    teach(c, a.teach - 0.25, a.thirty - 0.3);
    chapters(c, a.thirty - 0.25, a.sqlx - 0.3);
    rows(c, a.sqlx - 0.25, c.words.parts[3].0 - 0.15);
}

fn icon(g: &G, kind: usize, x: f32, y: f32, s: f32, col: &str) {
    match kind {
        0 => {
            // YouTube: rounded screen + play
            g.stroke_rrect(
                x - 22.0 * s,
                y - 15.0 * s,
                44.0 * s,
                30.0 * s,
                8.0 * s,
                col,
                2.0,
            );
            g.play_glyph(x, y, 8.0 * s, col);
        }
        1 => {
            // GitHub: a branch
            g.line(
                x - 10.0 * s,
                y - 14.0 * s,
                x - 10.0 * s,
                y + 14.0 * s,
                col,
                2.0,
            );
            g.c.begin_path();
            g.c.move_to((x - 10.0 * s) as f64, (y + 4.0 * s) as f64);
            g.c.quadratic_curve_to(
                (x - 10.0 * s) as f64,
                (y - 8.0 * s) as f64,
                (x + 10.0 * s) as f64,
                (y - 8.0 * s) as f64,
            );
            g.c.set_stroke_style_str(col);
            g.c.set_line_width(2.0);
            g.c.stroke();
            for (px, py) in [(-10.0, -14.0), (-10.0, 14.0), (10.0, -9.0)] {
                g.fill_circle(x + px * s, y + py * s, 4.2 * s, &rgba(BG, 1.0));
                g.stroke_circle(x + px * s, y + py * s, 4.2 * s, col, 2.0);
            }
        }
        _ => {
            // Discord: speech bubble
            g.stroke_rrect(
                x - 21.0 * s,
                y - 15.0 * s,
                42.0 * s,
                28.0 * s,
                10.0 * s,
                col,
                2.0,
            );
            g.line(
                x - 8.0 * s,
                y + 13.0 * s,
                x - 13.0 * s,
                y + 21.0 * s,
                col,
                2.0,
            );
            g.fill_circle(x - 7.0 * s, y - 1.0 * s, 3.0 * s, col);
            g.fill_circle(x + 7.0 * s, y - 1.0 * s, 3.0 * s, col);
        }
    }
}

fn hub(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    // "on purpose" outlives the beat a little (it is part of the same sentence)
    let w_ext = window(t, s, a.forced - 0.15, 0.3, 0.3);
    if w_ext <= 0.01 {
        return;
    }
    let size = (g.w * 0.062).clamp(34.0, 88.0 * u.max(1.0));
    let y = c.top() + 20.0 + size;
    let word = "Aarambh Dev Hub";
    let ww = g.text_w(word, size, Font::Serif);
    let p = ez(E::EaseOutCubic, t, a.aarambh, 0.8);
    g.save();
    g.alpha(w_ext);
    g.c.begin_path();
    g.c.rect(
        (c.cx() - ww / 2.0 - 8.0) as f64,
        (y - size) as f64,
        (ww + 16.0) as f64,
        (size * 1.3) as f64,
    );
    g.c.clip();
    g.text(
        word,
        c.cx(),
        y + (1.0 - p) * size,
        size,
        Font::Serif,
        &rgba(TEXT, 1.0),
        Align::Center,
    );
    g.restore();
    let ul = ez(E::EaseInOutCubic, t, a.aarambh + 0.4, 0.9);
    g.line(
        c.cx() - ww / 2.0,
        y + size * 0.2,
        c.cx() - ww / 2.0 + ww * ul,
        y + size * 0.2,
        &rgba(ACCENT, w_ext),
        2.0,
    );
    g.label(
        "founder",
        c.cx(),
        c.top() + 14.0,
        12.0,
        4.0,
        &rgba(MUTED, w_ext * fade(c, a.aarambh)),
        Align::Center,
    );

    // three cards
    let gap = 16.0;
    let (cw, ch, cols) = if g.compact {
        (g.w - 2.0 * c.pad(), 96.0, 1)
    } else {
        (
            ((g.w - 2.0 * c.pad() - 2.0 * gap) / 3.0).min(360.0 * u.max(1.0)),
            170.0 * u.clamp(0.8, 1.2),
            3,
        )
    };
    let y0 = y + size * 0.55 + 22.0 * u.min(1.2);
    let total_w = if cols == 3 { cw * 3.0 + gap * 2.0 } else { cw };
    let x0 = c.cx() - total_w / 2.0;
    let items = [
        (0usize, "YouTube", "a channel for Rust", a.youtube, 1usize),
        (1, "GitHub", "an open-source organization", a.github, 5),
        (2, "Discord", "", a.discord, 2),
    ];
    for (i, (kind, name, sub, st, link)) in items.into_iter().enumerate() {
        let (x, y) = if cols == 3 {
            (x0 + i as f32 * (cw + gap), y0)
        } else {
            (x0, y0 + i as f32 * (ch + 10.0))
        };
        let pp = pop(c, st).clamp(0.0, 1.06);
        if pp <= 0.0 {
            continue;
        }
        let hov = c.hovering(Action::Link(link));
        let lift = if hov { -4.0 } else { 0.0 };
        g.save();
        g.c.translate((x + cw / 2.0) as f64, (y + ch / 2.0 + lift) as f64)
            .ok();
        g.c.scale(pp as f64, pp as f64).ok();
        g.alpha(w_ext * prog(t, st, 0.12));
        g.fill_rrect(
            -cw / 2.0,
            -ch / 2.0,
            cw,
            ch,
            10.0,
            &rgba(if hov { SURFACE_HI } else { SURFACE }, 0.96),
        );
        g.stroke_rrect(
            -cw / 2.0,
            -ch / 2.0,
            cw,
            ch,
            10.0,
            &rgba(if hov { ACCENT } else { BORDER }, 1.0),
            1.0,
        );
        let (ix, iy) = if g.compact {
            (-cw / 2.0 + 46.0, 0.0)
        } else {
            (0.0, -ch * 0.2)
        };
        icon(g, kind, ix, iy, 1.0, &rgba(ACCENT, 1.0));
        let (tx, ty, al) = if g.compact {
            (-cw / 2.0 + 96.0, -4.0, Align::Left)
        } else {
            (0.0, ch * 0.22, Align::Center)
        };
        g.text(
            name,
            tx,
            ty,
            26.0 * u.clamp(0.85, 1.2),
            Font::Serif,
            &rgba(TEXT, 1.0),
            al,
        );
        let subtxt = if kind == 2 {
            let n = tween(0.0, 500.0, E::EaseOutExpo, prog(t, a.five, 1.6)).round() as u32;
            if prog(t, a.five, 1.6) >= 1.0 {
                "500+ humans".to_string()
            } else {
                format!("{} humans", n)
            }
        } else {
            sub.to_string()
        };
        g.text(
            &subtxt,
            tx,
            ty + 26.0 * u.clamp(0.85, 1.1),
            15.0 * u.clamp(0.9, 1.1),
            if kind == 2 {
                Font::SansMedium
            } else {
                Font::SerifItalic
            },
            &rgba(if kind == 2 { ACCENT } else { TEXT2 }, 1.0),
            al,
        );
        g.restore();
        c.hit(x, y + lift, cw, ch, Action::Link(link));
    }
    let _ = LINKS;

    // "On purpose."
    let sp = ez(E::EaseOutCubic, t, a.purpose, 0.24);
    if sp > 0.0 {
        let yy = if cols == 3 {
            y0 + ch + 60.0 * u
        } else {
            y0 + 3.0 * (ch + 10.0) + 40.0
        };
        stamp(
            g,
            "on purpose",
            c.cx(),
            yy.min(c.bottom() - 10.0),
            22.0 * u.clamp(0.8, 1.3),
            -3.0,
            1.0 + 1.1 * (1.0 - sp),
            w_ext * sp,
            ACCENT,
        );
    }
    let _ = w;
}

fn forced(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let cw = (g.w - 2.0 * c.pad()).min(560.0 * u.max(1.0));
    let ch = 230.0 * u.clamp(0.8, 1.2);
    let p = pop(c, a.forced - 0.1);
    g.save();
    g.c.translate(c.cx() as f64, c.cy() as f64).ok();
    g.c.scale(p.min(1.06) as f64, p.min(1.06) as f64).ok();
    card(g, -cw / 2.0, -ch / 2.0, cw, ch, w);
    g.label(
        "community survey",
        -cw / 2.0 + 24.0,
        -ch / 2.0 + 36.0,
        11.5,
        3.0,
        &rgba(MUTED, w),
        Align::Left,
    );
    g.text(
        "Forced to use Rust?",
        -cw / 2.0 + 24.0,
        -ch / 2.0 + 84.0,
        30.0 * u.clamp(0.8, 1.2),
        Font::Serif,
        &rgba(TEXT, w),
        Align::Left,
    );
    let by = 14.0;
    for (i, label) in ["Yes", "No"].iter().enumerate() {
        let bx = -cw / 2.0 + 28.0 + i as f32 * 150.0 * u.clamp(0.8, 1.1);
        g.stroke_rrect(bx, by, 30.0, 30.0, 6.0, &rgba(BORDER, w), 1.6);
        g.text(
            label,
            bx + 44.0,
            by + 23.0,
            22.0 * u.clamp(0.8, 1.2),
            Font::Serif,
            &rgba(TEXT2, w),
            Align::Left,
        );
        if i == 1 {
            let cp = ez(E::EaseOutCubic, t, a.checked, 0.35);
            if cp > 0.0 {
                g.c.begin_path();
                g.c.move_to((bx + 6.0) as f64, (by + 16.0) as f64);
                let p1 = (cp * 2.0).min(1.0);
                g.c.line_to((bx + 6.0 + 7.0 * p1) as f64, (by + 16.0 + 8.0 * p1) as f64);
                if cp > 0.5 {
                    let p2 = (cp - 0.5) * 2.0;
                    g.c.line_to(
                        (bx + 13.0 + 14.0 * p2) as f64,
                        (by + 24.0 - 18.0 * p2) as f64,
                    );
                }
                g.c.set_stroke_style_str(&rgba(ACCENT, w));
                g.c.set_line_width(3.2);
                g.c.set_line_cap("round");
                g.c.set_line_join("round");
                g.c.stroke();
            }
        }
    }
    g.text(
        "We checked.",
        cw / 2.0 - 24.0,
        ch / 2.0 - 22.0,
        18.0,
        Font::SerifItalic,
        &rgba(TEXT2, w * fade(c, a.checked)),
        Align::Right,
    );
    g.restore();
}

fn teach(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let (x0, x1) = (c.pad() + 20.0, g.w - c.pad() - 20.0);
    let y = c.cy() - 30.0 * u;
    g.label(
        "teaching rust",
        c.cx(),
        c.top() + 18.0,
        12.5,
        4.0,
        &rgba(ACCENT, w * fade(c, a.teach)),
        Align::Center,
    );
    g.line(x0, y, x1, y, &rgba(BORDER, w), 2.0);
    let p = ez(E::EaseInOutCubic, t, a.teach, (a.future - a.teach).max(1.0));
    g.line(x0, y, lerp(x0, x1, p), y, &rgba(ACCENT, w), 2.5);
    g.fill_circle(lerp(x0, x1, p), y, 7.0, &rgba(TEXT, w));
    g.text(
        "fn main() {}",
        x0,
        y + 40.0 * u,
        20.0 * u.clamp(0.8, 1.2),
        Font::Mono,
        &rgba(TEXT2, w * fade(c, a.teach)),
        Align::Left,
    );
    g.label(
        "the very first line",
        x0,
        y + 66.0 * u,
        10.5,
        2.4,
        &rgba(MUTED, w * fade(c, a.teach)),
        Align::Left,
    );
    let qp = pop(c, a.future - 0.1);
    if qp > 0.0 {
        let q = "\u{201c}why is this future not Send?\u{201d}";
        let qs = (g.w * 0.034).clamp(20.0, 44.0 * u.max(1.0));
        g.save();
        g.c.translate(c.cx() as f64, (y - 70.0 * u) as f64).ok();
        g.c.scale(qp.min(1.06) as f64, qp.min(1.06) as f64).ok();
        g.text(
            q,
            0.0,
            0.0,
            qs,
            Font::SerifItalic,
            &rgba(TEXT, w),
            Align::Center,
        );
        g.restore();
        let ep = fade(c, a.future + 0.3);
        let fs = (15.0 * u).clamp(11.0, 18.0);
        let msg = "error: future cannot be sent between threads safely";
        let mw = g.text_w(msg, fs, Font::Mono) + 32.0;
        g.fill_rrect(
            c.cx() - mw / 2.0,
            y + 18.0 * u,
            mw,
            fs * 2.5,
            8.0,
            &rgba(SURFACE, 0.95 * w * ep),
        );
        g.stroke_rrect(
            c.cx() - mw / 2.0,
            y + 18.0 * u,
            mw,
            fs * 2.5,
            8.0,
            &rgba(WARM, 0.8 * w * ep),
            1.0,
        );
        g.text(
            msg,
            c.cx(),
            y + 18.0 * u + fs * 1.65,
            fs,
            Font::Mono,
            &rgba(WARM, w * ep),
            Align::Center,
        );
    }
}

fn chapters(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.16).clamp(70.0, 200.0 * u.max(1.0));
    let n = ez(E::EaseOutCubic, t, a.thirty, 1.2);
    let shown = (37.0 * n).round() as u32;
    g.text(
        &format!("{}", shown),
        c.cx(),
        c.cy() - size * 0.05,
        size,
        Font::Serif,
        &rgba(TEXT, w),
        Align::Center,
    );
    g.label(
        "chapters",
        c.cx(),
        c.cy() + 26.0 * u,
        14.0,
        5.0,
        &rgba(ACCENT, w * fade(c, a.thirty + 0.4)),
        Align::Center,
    );
    g.text(
        "how large language models work",
        c.cx(),
        c.cy() + 70.0 * u,
        24.0 * u.clamp(0.8, 1.2),
        Font::SerifItalic,
        &rgba(TEXT2, w * fade(c, a.thirty + 0.7)),
        Align::Center,
    );
    // 37 ticks fill left to right
    let tw = (g.w - 2.0 * c.pad()).min(900.0 * u.max(1.0));
    let tx0 = c.cx() - tw / 2.0;
    let step = tw / 37.0;
    let ty = c.cy() + 120.0 * u;
    for i in 0..37 {
        let st = a.thirty + 0.2 + i as f32 * 0.045;
        let p = ez(E::EaseOutBack, t, st, 0.3);
        let h = 22.0 * u.clamp(0.7, 1.2) * p.max(0.0);
        let col = if i < shown as usize {
            rgba(ACCENT, w)
        } else {
            rgba(BORDER, w)
        };
        g.fill_rrect(
            tx0 + i as f32 * step + 1.5,
            ty - h,
            (step - 3.0).max(2.0),
            h,
            2.0,
            &col,
        );
    }
}

fn rows(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    if w <= 0.01 {
        return;
    }
    let cx = c.cx();
    g.label(
        "sqlx course",
        cx,
        c.top() + 18.0,
        12.5,
        4.0,
        &rgba(ACCENT, w * fade(c, a.sqlx)),
        Align::Center,
    );
    let p = prog(t, a.crore - 0.2, (a.billion - a.crore + 0.9).max(1.0));
    let val = tween(0.0, 1_000_000_000.0, E::EaseOutExpo, p).round() as u64;
    let txt = thousands(val);
    let size = ((g.w - 2.0 * c.pad()) / (13.0 * 0.62))
        .min(130.0 * u.max(1.0))
        .max(30.0);
    let y = c.cy() - 10.0 * u;
    g.text(
        &txt,
        cx,
        y,
        size,
        Font::Serif,
        &rgba(if p >= 1.0 { ACCENT } else { TEXT }, w),
        Align::Center,
    );
    g.label(
        "100 crore rows",
        cx,
        y + 40.0 * u,
        15.0,
        5.0,
        &rgba(TEXT2, w * fade(c, a.crore)),
        Align::Center,
    );
    g.text(
        "= one billion",
        cx,
        y + 80.0 * u,
        24.0 * u.clamp(0.8, 1.2),
        Font::SerifItalic,
        &rgba(TEXT2, w * fade(c, a.billion)),
        Align::Center,
    );

    // "please stop."
    let sp = pop(c, a.please_stop - 0.2);
    if sp > 0.0 {
        let cw = (300.0 * u).clamp(240.0, 360.0);
        let ch = 84.0 * u.clamp(0.8, 1.2);
        let x = if g.compact {
            cx - cw / 2.0
        } else {
            g.w - c.pad() - cw
        };
        let y2 = if g.compact {
            y + 110.0 * u
        } else {
            y + 118.0 * u
        };
        let sh = shake(t, a.please_stop, 0.5, 14.0) * 4.0;
        g.save();
        g.c.translate((x + cw / 2.0 + sh) as f64, (y2 + ch / 2.0) as f64)
            .ok();
        g.c.scale(sp.min(1.08) as f64, sp.min(1.08) as f64).ok();
        card(g, -cw / 2.0, -ch / 2.0, cw, ch, w);
        g.label(
            "postgres",
            -cw / 2.0 + 16.0,
            -ch / 2.0 + 24.0,
            10.5,
            2.6,
            &rgba(WARM, w),
            Align::Left,
        );
        g.text(
            "\u{201c}please stop.\u{201d}",
            -cw / 2.0 + 16.0,
            ch / 2.0 - 20.0,
            26.0 * u.clamp(0.8, 1.2),
            Font::SerifItalic,
            &rgba(TEXT, w),
            Align::Left,
        );
        g.restore();
    }
    // "one more query"
    if t > a.more_query - 0.1 {
        let line = typed("> one more query;", t, a.more_query, 16.0);
        let fs = 22.0 * u.clamp(0.8, 1.2);
        let ww = g.text_w("> one more query;_", fs, Font::Mono) + 40.0;
        let y3 = c.bottom() - 14.0;
        g.fill_rrect(
            cx - ww / 2.0,
            y3 - fs * 1.6,
            ww,
            fs * 2.3,
            8.0,
            &rgba(SURFACE, 0.95 * w),
        );
        g.stroke_rrect(
            cx - ww / 2.0,
            y3 - fs * 1.6,
            ww,
            fs * 2.3,
            8.0,
            &rgba(BORDER, w),
            1.0,
        );
        let blink = if (c.wall * 2.0) as i32 % 2 == 0 {
            "_"
        } else {
            " "
        };
        g.text(
            &format!("{}{}", line, blink),
            cx - ww / 2.0 + 20.0,
            y3 - fs * 0.35,
            fs,
            Font::Mono,
            &rgba(ACCENT, w),
            Align::Left,
        );
    }
}
