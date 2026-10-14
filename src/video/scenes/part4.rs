//! Part 4: the project speed run. A project list on the left (like the portfolio's "Selected Work"),
//! and a live mini-demo of each project on the right. Click a project to open it on GitHub.
use crate::video::ctx::{Action, Ctx, PROJECTS};
use crate::video::fx::*;
use crate::video::g2d::{Align, Font};
use crate::video::kit::*;
use crate::video::theme::*;
use crate::video::wire::camera;
use scenix::Vec3;

const TAGLINES: [&str; 4] = [
    "Motion values for any renderer",
    "3D scene infrastructure in Rust",
    "An LLM, built from first principles",
    "Rust types, for every other language",
];

pub fn draw(c: &Ctx) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let p4 = c.words.parts[3].0;
    let end = c.words.parts[4].0 - 0.15;

    // ---- intro: seatbelts
    let wi = window(t, p4 - 0.3, a.animato - 0.15, 0.3, 0.3);
    if wi > 0.01 {
        let size = (g.w * 0.085).clamp(40.0, 112.0 * u.max(1.0));
        let y = c.cy() - size * 0.1;
        let p = ez(E::EaseOutCubic, t, p4 + 0.1, 0.9);
        for (i, (word, dy)) in [("Project", 0.0), ("speed run.", size * 1.0)]
            .iter()
            .enumerate()
        {
            let st = p4 + 0.1 + i as f32 * 0.2;
            let pp = ez(E::EaseOutCubic, t, st, 0.8);
            let ww = g.text_w(word, size, Font::SerifItalic);
            g.save();
            g.alpha(wi);
            g.c.begin_path();
            g.c.rect(
                (c.cx() - ww / 2.0 - 10.0) as f64,
                (y + dy - size * 0.95) as f64,
                (ww + 20.0) as f64,
                (size * 1.3) as f64,
            );
            g.c.clip();
            g.text(
                word,
                c.cx(),
                y + dy + (1.0 - pp) * size,
                size,
                Font::SerifItalic,
                &rgba(if i == 1 { ACCENT } else { TEXT }, 1.0),
                Align::Center,
            );
            g.restore();
        }
        let _ = p;
        let sp = pop(c, a.seatbelts);
        if sp > 0.0 {
            let cs = 17.0 * u.clamp(0.85, 1.25);
            let pw = pill_w(g, "seatbelts: fastened", cs);
            g.save();
            g.c.translate(c.cx() as f64, (y + size * 1.75) as f64).ok();
            g.c.scale(sp.min(1.1) as f64, sp.min(1.1) as f64).ok();
            pill(g, "seatbelts: fastened", -pw / 2.0, 0.0, cs, wi, true);
            g.restore();
        }
    }

    // ---- main: list + panel
    let w = window(t, a.animato - 0.3, end, 0.4, 0.35);
    if w <= 0.01 {
        return;
    }
    let starts = [a.animato, a.scenix, a.ai, a.typebridge];
    let ends = [a.scenix, a.ai, a.typebridge, end + 0.5];
    let mut idx = 0;
    for (i, s) in starts.iter().enumerate() {
        if t >= *s - 0.05 {
            idx = i;
        }
    }
    let pad = c.pad();
    let (lx, lw) = (
        pad,
        if g.compact {
            g.w - 2.0 * pad
        } else {
            (g.w * 0.22).clamp(190.0, 300.0)
        },
    );
    let top = c.top() + 4.0;
    let (px, py, pw, ph);
    if g.compact {
        px = pad;
        py = top + 48.0;
        pw = g.w - 2.0 * pad;
        ph = c.bottom() - py + 6.0;
    } else {
        px = lx + lw + 28.0 * u.min(1.2);
        py = top;
        pw = g.w - pad - px;
        ph = c.bottom() - top;
    }

    // list (tabs in compact)
    let rh = if g.compact {
        36.0
    } else {
        ((ph - 8.0) / 4.0).min(96.0)
    };
    for (i, (name, _)) in PROJECTS.iter().enumerate() {
        let st = starts[i] - 0.25;
        let pp = pop(c, st).clamp(0.0, 1.0);
        if pp <= 0.0 {
            continue;
        }
        let active = i == idx;
        let hov = c.hovering(Action::Project(i));
        let a_row = w
            * pp
            * if active {
                1.0
            } else if hov {
                0.9
            } else {
                0.5
            };
        if g.compact {
            let tw = (g.w - 2.0 * pad) / 4.0;
            let x = lx + i as f32 * tw;
            g.text(
                name,
                x + tw / 2.0,
                top + 22.0,
                13.5,
                Font::SansMedium,
                &rgba(if active { ACCENT } else { TEXT2 }, a_row),
                Align::Center,
            );
            if active {
                g.line(
                    x + 10.0,
                    top + 32.0,
                    x + tw - 10.0,
                    top + 32.0,
                    &rgba(ACCENT, w),
                    2.0,
                );
            }
            c.hit(x, top, tw, 40.0, Action::Project(i));
        } else {
            let y = top + i as f32 * rh;
            if active {
                g.fill_rrect(lx, y + 6.0, 3.0, rh - 12.0, 1.5, &rgba(ACCENT, w * pp));
            }
            g.text(
                &format!("{:02}", i + 1),
                lx + 20.0,
                y + rh / 2.0 - 4.0,
                14.0,
                Font::Mono,
                &rgba(if active { ACCENT } else { MUTED }, a_row),
                Align::Left,
            );
            g.text(
                name,
                lx + 20.0,
                y + rh / 2.0 + 28.0 * u.clamp(0.8, 1.1),
                30.0 * u.clamp(0.8, 1.15),
                Font::Serif,
                &rgba(if active { TEXT } else { TEXT2 }, a_row),
                Align::Left,
            );
            c.hit(lx, y, lw, rh, Action::Project(i));
        }
    }

    // panels
    for i in 0..4 {
        let wp = window(t, starts[i] - 0.05, ends[i], 0.4, 0.3) * w;
        if wp <= 0.01 {
            continue;
        }
        let off = (1.0 - ez(E::EaseOutCubic, t, starts[i] - 0.05, 0.5)) * 26.0;
        g.save();
        g.c.translate(off as f64, 0.0).ok();
        card(g, px, py, pw, ph, wp);
        let hov = c.hovering(Action::Project(i));
        g.text(
            PROJECTS[i].0,
            px + 24.0,
            py + 52.0 * u.clamp(0.8, 1.1),
            34.0 * u.clamp(0.75, 1.15),
            Font::Serif,
            &rgba(TEXT, wp),
            Align::Left,
        );
        g.text(
            TAGLINES[i],
            px + 24.0,
            py + 80.0 * u.clamp(0.8, 1.1),
            17.0 * u.clamp(0.85, 1.1),
            Font::SerifItalic,
            &rgba(TEXT2, wp),
            Align::Left,
        );
        g.label(
            if hov { "open on github  ->" } else { "github" },
            px + pw - 24.0,
            py + 36.0,
            10.5,
            2.4,
            &rgba(if hov { ACCENT } else { MUTED }, wp),
            Align::Right,
        );
        let inner = (
            px + 24.0,
            py + 104.0 * u.clamp(0.8, 1.05),
            pw - 48.0,
            ph - 124.0 * u.clamp(0.8, 1.05),
        );
        g.save();
        g.c.begin_path();
        g.c.rect(
            px as f64 + 1.0,
            (py + 96.0) as f64,
            pw as f64 - 2.0,
            (ph - 98.0) as f64,
        );
        g.c.clip();
        match i {
            0 => animato(c, inner, wp),
            1 => scenix(c, inner, wp),
            2 => ai(c, inner, wp),
            _ => typebridge(c, inner, wp),
        }
        g.restore();
        g.restore();
    }
}

type Rect = (f32, f32, f32, f32);

fn animato(c: &Ctx, (x, y, w, h): Rect, wp: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let gw = if g.compact {
        w
    } else {
        (w * 0.62).min(620.0 * u.max(1.0))
    };
    let gh = (h - 20.0).min(gw * 0.62).max(80.0);
    let (gx, gy) = (x, y + 8.0);
    g.stroke_rrect(gx, gy, gw, gh, 6.0, &rgba(BORDER, wp), 1.0);
    // linear reference
    g.dash(4.0, 6.0);
    g.line(gx, gy + gh, gx + gw, gy, &rgba(MUTED, wp * 0.7), 1.0);
    g.solid();
    // the curve, drawn in
    let dp = ez(E::EaseInOutCubic, t, a.animato + 0.3, 1.4);
    g.c.begin_path();
    let n = 80;
    for k in 0..=n {
        let fx = k as f32 / n as f32;
        if fx > dp {
            break;
        }
        let (px, py) = (gx + fx * gw, gy + gh - E::EaseOutCubic.apply(fx) * gh);
        if k == 0 {
            g.c.move_to(px as f64, py as f64)
        } else {
            g.c.line_to(px as f64, py as f64)
        }
    }
    g.c.set_stroke_style_str(&rgba(ACCENT, wp));
    g.c.set_line_width(3.0);
    g.c.set_line_cap("round");
    g.c.stroke();
    // the smooth dot (Animato easing, looping)
    if t > a.animato + 1.0 {
        let ph = (c.wall % 2.4) / 2.4;
        let (dx, dy) = (gx + ph * gw, gy + gh - E::EaseOutCubic.apply(ph) * gh);
        g.glow(dx, dy, 26.0, ACCENT, 0.5 * wp);
        g.fill_circle(dx, dy, 6.0, &rgba(TEXT, wp));
        g.label(
            "smooth",
            gx + gw - 10.0,
            gy + gh - 12.0,
            10.5,
            2.4,
            &rgba(ACCENT, wp * fade(c, a.smoothly)),
            Align::Right,
        );
    }
    // "my life" stutters
    if t > a.laggy - 0.4 {
        let la = fade(c, a.laggy - 0.4) * wp;
        let ph = (c.wall % 2.4) / 2.4;
        let steps = 6.0;
        let q = (ph * steps).floor() / steps;
        let hold = if (c.wall * 3.0) as i32 % 5 == 0 {
            0.0
        } else {
            1.0
        };
        let sp = if hold > 0.0 {
            q
        } else {
            (q - 1.0 / steps).max(0.0)
        };
        let (dx, dy) = (gx + sp * gw, gy + gh - sp * gh * 0.8);
        g.stroke_circle(dx, dy, 7.0, &rgba(WARM, la), 2.0);
        g.label(
            "my life",
            dx + 12.0,
            dy - 12.0,
            10.5,
            2.0,
            &rgba(WARM, la),
            Align::Left,
        );
    }
    // chips
    if !g.compact {
        let cx = gx + gw + 28.0;
        let cs = 16.0 * u.clamp(0.85, 1.2);
        for (i, label) in ["tweens", "springs", "timelines", "motion paths"]
            .iter()
            .enumerate()
        {
            let st = a.animato + 0.6 + i as f32 * 0.35;
            let pp = pop(c, st).clamp(0.0, 1.05);
            let pw = pill_w(g, label, cs);
            g.save();
            g.c.translate(
                (cx + pw / 2.0) as f64,
                (y + 30.0 + i as f32 * cs * 3.0) as f64,
            )
            .ok();
            g.c.scale(pp as f64, pp as f64).ok();
            pill(g, label, -pw / 2.0, 0.0, cs, wp * prog(t, st, 0.12), false);
            g.restore();
        }
    }
}

fn scenix(c: &Ctx, (x, y, w, h): Rect, wp: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let kw = if g.compact { w } else { w * 0.48 };
    let kh = if g.compact { h * 0.55 } else { h };
    let cam = camera(kw / kh, 5.2);
    let sc = 0.7 + 0.3 * ez(E::EaseOutCubic, t, a.scenix, 1.0);
    let model = c.orbit.model(c.wall * 0.35, sc, Vec3::new(0.0, 0.0, 0.0));
    c.assets.knot.draw(
        g,
        model,
        &cam,
        (x, y, kw, kh),
        ACCENT,
        wp * prog(t, a.scenix, 0.6),
        1.2 * u.clamp(0.8, 1.2),
    );
    c.hit(x, y, kw, kh, Action::Orbit);
    g.label(
        "drag to spin",
        x + kw / 2.0,
        y + kh - 4.0,
        10.0,
        2.4,
        &rgba(MUTED, wp * 0.8 * prog(t, a.scenix + 1.5, 0.6)),
        Align::Center,
    );
    let (rx, mut ry) = if g.compact {
        (x, y + kh + 10.0)
    } else {
        (x + kw + 28.0, y + 6.0)
    };
    let cs = 16.0 * u.clamp(0.85, 1.2);
    // wgpu chip
    let pp = pop(c, a.wgpu).clamp(0.0, 1.05);
    let pw = pill_w(g, "wgpu", cs);
    g.save();
    g.c.translate((rx + pw / 2.0) as f64, (ry + cs) as f64).ok();
    g.c.scale(pp as f64, pp as f64).ok();
    pill(
        g,
        "wgpu",
        -pw / 2.0,
        0.0,
        cs,
        wp * prog(t, a.wgpu, 0.12),
        true,
    );
    g.restore();
    ry += cs * 3.4;
    // Three.js + borrow checker = Scenix
    let fs = 28.0 * u.clamp(0.75, 1.15);
    for (i, (txt, col)) in [
        ("Three.js", TEXT),
        ("+ a borrow checker", TEXT2),
        ("= Scenix", ACCENT),
    ]
    .iter()
    .enumerate()
    {
        let st = a.threejs + i as f32 * 0.5;
        let pa = fade(c, st);
        g.text(
            txt,
            rx,
            ry + i as f32 * fs * 1.35 + (1.0 - pa) * 10.0,
            fs,
            Font::Serif,
            &rgba(*col, wp * pa),
            Align::Left,
        );
    }
    // "nobody asked"
    let np = pop(c, a.nobody).clamp(0.0, 1.05);
    if np > 0.0 {
        let label = "nobody asked";
        let pw = pill_w(g, label, cs);
        g.save();
        g.c.translate((rx + pw / 2.0) as f64, (ry + 3.0 * fs * 1.35 + 24.0) as f64)
            .ok();
        g.c.scale(np as f64, np as f64).ok();
        pill(
            g,
            label,
            -pw / 2.0,
            0.0,
            cs,
            wp * prog(t, a.nobody, 0.12),
            false,
        );
        g.restore();
    }
    // passion <-> poor impulse control
    if t > a.passion - 0.4 {
        let sw = (if g.compact { w } else { w - kw - 28.0 }).max(160.0);
        let sx = if g.compact { x } else { rx };
        let sy = y + h - 36.0;
        let sa = wp * fade(c, a.passion - 0.4);
        g.line(sx, sy, sx + sw, sy, &rgba(BORDER, sa), 2.0);
        let k = 0.1 + 0.8 * c.assets.soft.at(t - (a.impulse - 0.25));
        g.fill_circle(sx + sw * k.clamp(0.0, 1.0), sy, 8.0, &rgba(ACCENT, sa));
        g.label(
            "passion",
            sx,
            sy + 24.0,
            10.0,
            2.0,
            &rgba(if t < a.impulse { ACCENT } else { MUTED }, sa),
            Align::Left,
        );
        g.label(
            "poor impulse control",
            sx + sw,
            sy + 24.0,
            10.0,
            2.0,
            &rgba(if t >= a.impulse { WARM } else { MUTED }, sa),
            Align::Right,
        );
    }
}

fn ai(c: &Ctx, (x, y, w, h): Rect, wp: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let fs = (g.w * 0.03).clamp(20.0, 36.0 * u.max(1.0));
    let lines = [
        ("A language model,", a.ai),
        ("from scratch,", a.scratch),
        ("in Rust.", a.scratch + 0.6),
    ];
    let _ = &lines;
    for (i, (txt, st)) in lines.iter().enumerate() {
        let pa = fade(c, *st);
        g.text(
            txt,
            x,
            y + 22.0 + i as f32 * fs * 1.25 + (1.0 - pa) * 10.0,
            fs,
            Font::Serif,
            &rgba(TEXT, wp * pa),
            Align::Left,
        );
    }
    // No Python.
    let ny = y + 22.0 + 3.0 * fs * 1.25 + 8.0;
    let np = fade(c, a.no_python);
    g.text(
        "No",
        x,
        ny + fs,
        fs,
        Font::SerifItalic,
        &rgba(TEXT2, wp * np),
        Align::Left,
    );
    let nx = x + g.text_w("No ", fs, Font::SerifItalic);
    g.text(
        "Python.",
        nx,
        ny + fs,
        fs,
        Font::SerifItalic,
        &rgba(WARM, wp * np),
        Align::Left,
    );
    let sp = ez(E::EaseOutCubic, t, a.no_python + 0.35, 0.35);
    g.line(
        nx - 4.0,
        ny + fs * 0.62,
        nx - 4.0 + (g.text_w("Python.", fs, Font::SerifItalic) + 8.0) * sp,
        ny + fs * 0.62,
        &rgba(WARM, wp),
        2.4,
    );
    // hardware chips
    let cs = 16.0 * u.clamp(0.85, 1.2);
    let mut cxp = if g.compact { x } else { x + w * 0.5 };
    let mut cyp = if g.compact { ny + fs * 2.3 } else { y + 34.0 };
    for (label, st, warm) in [
        ("Intel i3", a.i3, false),
        ("8 GB RAM", a.eight, false),
        ("no GPU", a.no_gpu, true),
        ("free Kaggle GPU", a.kaggle, false),
    ] {
        let pw = pill_w(g, label, cs);
        if cxp + pw > x + w {
            cxp = if g.compact { x } else { x + w * 0.5 };
            cyp += cs * 3.0;
        }
        let pp = pop(c, st).clamp(0.0, 1.05);
        g.save();
        g.c.translate((cxp + pw / 2.0) as f64, cyp as f64).ok();
        g.c.scale(pp as f64, pp as f64).ok();
        pill(
            g,
            label,
            -pw / 2.0,
            0.0,
            cs,
            wp * prog(t, st, 0.12),
            label.contains("Kaggle"),
        );
        if warm {
            g.stroke_rrect(
                -pw / 2.0,
                -cs * 1.125,
                pw,
                cs * 2.25,
                cs * 1.125,
                &rgba(WARM, wp),
                1.0,
            );
        }
        g.restore();
        cxp += pw + 10.0;
    }
    // the rented Ferrari
    let cp = prog(t, a.ferrari - 0.3, 3.6);
    if cp > 0.0 && cp < 1.0 {
        let xx = lerp(x - 120.0, x + w + 120.0, cp);
        let yy = y + h - 34.0;
        car(g, xx, yy, 0.62 * u.clamp(0.8, 1.2), -xx * 0.05, wp);
        g.line(x, yy + 18.0, x + w, yy + 18.0, &rgba(BORDER, wp), 1.0);
        g.text(
            "a rented Ferrari",
            xx,
            yy - 58.0 * u.clamp(0.8, 1.2),
            16.0,
            Font::SerifItalic,
            &rgba(TEXT2, wp * (1.0 - (cp - 0.5).abs() * 2.0).clamp(0.25, 1.0)),
            Align::Center,
        );
    }
}

fn typebridge(c: &Ctx, (x, y, w, h): Rect, wp: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let fs = (18.0 * u).clamp(13.0, 22.0);
    let cw = if g.compact {
        w * 0.5
    } else {
        (w * 0.36).min(300.0 * u.max(1.0))
    };
    let code = ["struct User {", "    id: u64,", "    name: String,", "}"];
    let ch = fs * 1.6 * code.len() as f32 + 34.0;
    let cy0 = y + (h - ch) / 2.0 - if g.compact { 10.0 } else { 0.0 };
    let pa = fade(c, a.typebridge);
    card(g, x, cy0, cw, ch, wp * pa);
    for (i, line) in code.iter().enumerate() {
        let col = if i == 0 || i == 3 { ACCENT } else { TEXT };
        g.text(
            line,
            x + 18.0,
            cy0 + 28.0 + i as f32 * fs * 1.6,
            fs,
            Font::Mono,
            &rgba(col, wp * pa),
            Align::Left,
        );
    }
    g.label(
        "rust source",
        x + 18.0,
        cy0 - 10.0,
        10.0,
        2.4,
        &rgba(MUTED, wp * pa),
        Align::Left,
    );
    // language chips
    let cs = 16.0 * u.clamp(0.8, 1.2);
    let langs = [
        ("TypeScript", a.typescript),
        ("Python", a.python),
        ("Go", a.go_lang),
        ("Swift", a.swift),
        ("Kotlin", a.kotlin),
    ];
    let col_x = x + cw + (w - cw) * 0.42;
    let step = ((h - 20.0) / 5.0).min(cs * 3.0);
    let y_top = y + (h - step * 5.0) / 2.0 + step / 2.0;
    for (i, (label, st)) in langs.iter().enumerate() {
        let ly = y_top + i as f32 * step;
        let lp = ez(E::EaseOutCubic, t, *st, 0.45);
        if lp <= 0.0 {
            continue;
        }
        // arrow from the struct to the chip
        let (sx, sy) = (x + cw, cy0 + ch / 2.0);
        g.c.begin_path();
        g.c.move_to(sx as f64, sy as f64);
        g.c.bezier_curve_to(
            (sx + 40.0) as f64,
            sy as f64,
            (col_x - 50.0) as f64,
            ly as f64,
            (sx + (col_x - sx) * lp) as f64,
            (sy + (ly - sy) * lp) as f64,
        );
        g.c.set_stroke_style_str(&rgba(ACCENT, wp * 0.7));
        g.c.set_line_width(1.5);
        g.c.stroke();
        let pp = pop(c, *st).clamp(0.0, 1.05);
        let pw = pill_w(g, label, cs);
        g.save();
        g.c.translate((col_x + pw / 2.0) as f64, ly as f64).ok();
        g.c.scale(pp as f64, pp as f64).ok();
        pill(g, label, -pw / 2.0, 0.0, cs, wp * prog(t, *st, 0.12), false);
        g.restore();
    }
    // inbox: 99+ -> 0
    let ip = fade(c, a.kotlin + 0.2);
    if ip > 0.0 {
        let n = tween(99.0, 0.0, E::EaseOutCubic, prog(t, a.messaging, 1.5)).round() as u32;
        let label = if t < a.messaging {
            "99+".to_string()
        } else {
            format!("{}", n)
        };
        let bx = x + w - 6.0;
        let by = y + 16.0;
        let done = t >= a.messaging + 1.5;
        g.label(
            "messages from frontend people",
            bx - 74.0,
            by + 5.0,
            9.5,
            1.6,
            &rgba(TEXT2, wp * ip),
            Align::Right,
        );
        g.fill_rrect(
            bx - 62.0,
            by - 18.0,
            62.0,
            36.0,
            18.0,
            &rgba(SURFACE_HI, wp * ip),
        );
        g.stroke_rrect(
            bx - 62.0,
            by - 18.0,
            62.0,
            36.0,
            18.0,
            &rgba(if done { ACCENT } else { WARM }, wp * ip),
            1.4,
        );
        g.text(
            &label,
            bx - 31.0,
            by + 7.0,
            19.0,
            Font::SansBold,
            &rgba(if done { ACCENT } else { WARM }, wp * ip),
            Align::Center,
        );
    }
}
