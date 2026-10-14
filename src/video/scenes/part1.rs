//! Part 1: the entrance. "Hiii!", the play button joke, the name, the "works on my machine" stamp.
use crate::video::ctx::Ctx;
use crate::video::fx::*;
use crate::video::g2d::{Align, Font};
use crate::video::kit::*;
use crate::video::theme::*;

pub fn draw(c: &Ctx) {
    let g = c.g;
    let t = c.t;
    let a = c.a;
    let u = g.u;
    let (cx, cy) = (c.cx(), c.cy());
    let end_b1 = a.most - 0.25;
    let end_b2 = a.name - 0.3;
    let end_b3 = c.words.parts[1].0 - 0.15;

    // ------------------------------------------------------------ B1: "Hi."
    let w1 = window(t, -0.3, end_b1, 0.12, 0.35);
    if w1 > 0.01 {
        let big = (c.stage_h() * 0.62)
            .min(g.w * 0.40)
            .clamp(80.0, 300.0 * u.max(1.0));
        let letters = ["H", "i", "."];
        let widths: Vec<f32> = letters
            .iter()
            .map(|l| g.text_w(l, big, Font::Serif))
            .collect();
        let total: f32 = widths.iter().sum();
        let mut x = cx - total / 2.0;
        let base = cy + big * 0.22;
        for (i, l) in letters.iter().enumerate() {
            let st = a.hi + i as f32 * 0.09;
            let p = c.assets.pop.at(t - st);
            let vis = prog(t, st, 0.18);
            g.save();
            g.alpha(w1 * vis);
            let col = if i == 2 {
                rgba(ACCENT, 1.0)
            } else {
                rgba(TEXT, 1.0)
            };
            g.text(
                l,
                x,
                base + (1.0 - p) * big * 0.28,
                big,
                Font::Serif,
                &col,
                Align::Left,
            );
            g.restore();
            x += widths[i];
        }
        // "+1 respect" floats up like a score
        let rp = prog(t, a.respect, 1.4);
        if rp > 0.0 && rp < 1.0 {
            let ra = (1.0 - rp).powf(1.4) * w1;
            g.label(
                "+1 respect",
                cx,
                base + 44.0 * u - 34.0 * ez(E::EaseOutCubic, t, a.respect, 1.4),
                14.0,
                3.2,
                &rgba(ACCENT, ra),
                Align::Center,
            );
        }
    }

    // ------------------------------------------------------------ B2: the play pill (and the trap)
    let w2 = window(t, end_b1 + 0.05, end_b2, 0.25, 0.35);
    if w2 > 0.01 {
        let k = (g.w * 0.72 / 440.0).clamp(0.55, 1.35 * u.max(1.0));
        let p = c.assets.pop.at(t - (end_b1 + 0.15)).max(0.0);
        let by = cy - 6.0 * u;
        g.save();
        g.alpha(w2);
        g.c.translate(cx as f64, by as f64).ok();
        g.c.scale((k * p.min(1.12)) as f64, (k * p.min(1.12)) as f64)
            .ok();
        // the pill (same look as the button on the page)
        g.fill_rrect(-215.0, -48.0, 430.0, 96.0, 48.0, &rgba(SURFACE, 0.98));
        g.stroke_rrect(
            -215.0,
            -48.0,
            430.0,
            96.0,
            48.0,
            &rgba(if t > a.fun { ACCENT } else { BORDER }, 1.0),
            1.5,
        );
        g.stroke_circle(-160.0, 0.0, 30.0, &rgba(ACCENT, 1.0), 1.6);
        g.play_glyph(-157.0, 0.0, 11.0, &rgba(ACCENT, 1.0));
        g.text(
            "Watch the intro",
            -112.0,
            9.0,
            30.0,
            Font::Sans,
            &rgba(TEXT, 1.0),
            Align::Left,
        );
        g.text(
            "2:51",
            178.0,
            8.0,
            17.0,
            Font::Mono,
            &rgba(MUTED, 1.0),
            Align::Right,
        );
        g.restore();
        // expanding ring on the click ("clicked")
        let rp = prog(t, end_b1 + 0.25, 1.1);
        if rp < 1.0 {
            g.save();
            g.alpha(w2 * (1.0 - rp));
            g.stroke_circle(
                cx - 160.0 * k,
                by,
                (30.0 + 90.0 * ez(E::EaseOutCubic, t, end_b1 + 0.25, 1.1)) * k,
                &rgba(ACCENT, 1.0),
                1.4,
            );
            g.restore();
        }
        // "it's a trap" -> "a fun one"
        let trap_p = pop(c, a.trap);
        let trap_a = (1.0 - prog(t, a.fun, 0.2)) * w2;
        stamp(
            g,
            "it's a trap",
            cx + 150.0 * k,
            by - 86.0 * k,
            22.0 * u.clamp(0.8, 1.3),
            -6.0,
            trap_p,
            trap_a,
            WARM,
        );
        let fun_p = pop(c, a.fun);
        stamp(
            g,
            "a fun one",
            cx + 150.0 * k,
            by - 86.0 * k,
            22.0 * u.clamp(0.8, 1.3),
            4.0,
            fun_p,
            w2,
            ACCENT,
        );
        // smile
        let sp = ez(E::EaseInOutCubic, t, a.fun + 0.1, 0.9);
        if sp > 0.0 {
            g.save();
            g.alpha(w2);
            g.c.begin_path();
            let r = 120.0 * k;
            let _ = g.c.arc(
                cx as f64,
                (by + 20.0 * k) as f64,
                r as f64,
                (0.25 * std::f32::consts::PI) as f64,
                (0.25 * std::f32::consts::PI + 0.5 * std::f32::consts::PI * sp) as f64,
            );
            g.c.set_stroke_style_str(&rgba(ACCENT, 1.0));
            g.c.set_line_width(3.0);
            g.c.set_line_cap("round");
            g.c.stroke();
            g.restore();
        }
    }

    // ------------------------------------------------------------ B3: the name
    let w3 = window(t, end_b2, end_b3, 0.25, 0.4);
    if w3 > 0.01 {
        let size = (g.w * 0.085).clamp(34.0, 118.0 * u.max(1.0));
        let stacked = g.compact;
        let (n1, n2) = ("Darshan", "Vichhi");
        let (w_a, w_b) = (
            g.text_w(n1, size, Font::Serif),
            g.text_w(n2, size, Font::Serif),
        );
        let sp = g.text_w(" ", size, Font::Serif);
        let base = cy - size * 0.32 - if stacked { size * 0.5 } else { 0.0 };
        let (xa, ya, xb, yb) = if stacked {
            (cx - w_a / 2.0, base, cx - w_b / 2.0, base + size * 1.05)
        } else {
            let total = w_a + sp + w_b;
            (cx - total / 2.0, base, cx - total / 2.0 + w_a + sp, base)
        };
        for (word, x, y, st) in [(n1, xa, ya, a.name), (n2, xb, yb, a.name + 0.5)] {
            let p = ez(E::EaseOutCubic, t, st, 0.8);
            g.save();
            g.alpha(w3);
            g.c.begin_path();
            g.c.rect(
                (x - 10.0) as f64,
                (y - size * 0.95) as f64,
                (g.text_w(word, size, Font::Serif) + 20.0) as f64,
                (size * 1.3) as f64,
            );
            g.c.clip();
            g.text(
                word,
                x,
                y + (1.0 - p) * size,
                size,
                Font::Serif,
                &rgba(TEXT, 1.0),
                Align::Left,
            );
            g.restore();
        }
        // underline sweep
        let ul = ez(E::EaseInOutCubic, t, a.name + 0.4, 0.9);
        let uy = (if stacked { yb } else { ya }) + size * 0.2;
        let (ux0, ux1) = if stacked {
            (xa.min(xb), xa.max(xb) + w_a.max(w_b))
        } else {
            (xa, xb + w_b)
        };
        g.line(ux0, uy, ux0 + (ux1 - ux0) * ul, uy, &rgba(ACCENT, w3), 2.0);

        // role chips
        let chips = [
            ("Rust developer", a.rust_dev),
            ("Open-source builder", a.oss),
            ("YouTuber", a.youtuber),
        ];
        let cs = 17.0 * u.clamp(0.85, 1.3);
        let widths: Vec<f32> = chips.iter().map(|(s, _)| pill_w(g, s, cs)).collect();
        let gap = 12.0;
        let total: f32 = widths.iter().sum::<f32>() + gap * 2.0;
        let cy2 = (if stacked { yb } else { ya }) + size * 0.75;
        let mut x = if total < g.w - 32.0 {
            cx - total / 2.0
        } else {
            16.0
        };
        let mut y = cy2;
        for ((s, st), w) in chips.iter().zip(&widths) {
            if x + w > g.w - 16.0 {
                x = 16.0;
                y += cs * 3.0;
            }
            let p = pop(c, *st).clamp(0.0, 1.1);
            g.save();
            g.c.translate((x + w / 2.0) as f64, y as f64).ok();
            g.c.scale(p as f64, p as f64).ok();
            pill(g, s, -w / 2.0, 0.0, cs, w3 * prog(t, *st, 0.15), false);
            g.restore();
            x += w + gap;
        }

        // certified seal
        let sx = (cx + (xb + w_b - xa) / 2.0 + 24.0 * u).min(g.w - 62.0 * u);
        let sx = if stacked {
            g.w - 56.0 * u
        } else {
            sx.max(cx + 40.0)
        };
        let sy = ya - size * 0.95;
        let sp_ = pop(c, a.certified);
        if sp_ > 0.0 {
            g.save();
            g.alpha(w3 * prog(t, a.certified, 0.15));
            g.c.translate(sx as f64, sy as f64).ok();
            g.c.rotate((-0.2 + 0.1 * (1.0 - sp_)) as f64).ok();
            g.c.scale(sp_.min(1.15) as f64, sp_.min(1.15) as f64).ok();
            let r = 44.0 * u.clamp(0.8, 1.2);
            g.stroke_circle(0.0, 0.0, r, &rgba(ACCENT, 1.0), 2.0);
            g.stroke_circle(0.0, 0.0, r - 6.0, &rgba(ACCENT, 0.5), 1.0);
            g.label(
                "certified",
                0.0,
                -5.0,
                10.5,
                2.0,
                &rgba(ACCENT, 1.0),
                Align::Center,
            );
            g.text(
                "PRO",
                0.0,
                17.0,
                22.0,
                Font::SerifBold,
                &rgba(ACCENT, 1.0),
                Align::Center,
            );
            g.restore();
        }

        // WORKS ON MY MACHINE: the slam
        let wp = prog(t, a.works, 0.28);
        if t >= a.works - 0.02 {
            let slam = 1.0 + 1.3 * (1.0 - ez(E::EaseOutCubic, t, a.works, 0.28));
            let sa = w3 * prog(t, a.works, 0.08);
            let stamp_y = (cy2 + size * 1.45).min(c.bottom() - 34.0 * u);
            stamp(
                g,
                "works on my machine",
                cx,
                stamp_y,
                30.0 * u.clamp(0.7, 1.3),
                -4.0,
                slam,
                sa,
                WARM,
            );
            if wp < 1.0 {
                g.save();
                g.alpha((1.0 - wp) * 0.5 * w3);
                g.stroke_circle(cx, stamp_y, 40.0 + 260.0 * wp, &rgba(WARM, 1.0), 2.0);
                g.restore();
            }
        }
    }
}
