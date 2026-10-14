//! Part 2: who I am. Surat (a 3D Scenix diamond), the compiler fight, the scoreboard, frontend -> Rust, the crab.
use crate::video::ctx::{Action, Ctx};
use crate::video::fx::*;
use crate::video::g2d::{Align, Font};
use crate::video::kit::*;
use crate::video::theme::*;
use crate::video::wire::camera;
use scenix::Vec3;

pub fn draw(c: &Ctx) {
    let a = c.a;
    let p0 = c.words.parts[1].0;
    surat(c, p0 - 0.3, a.byday - 0.3);
    compiler(c, a.byday - 0.2, a.every - 0.3);
    every_single_time(c, a.every - 0.2, a.borrow - 0.25);
    scoreboard(c, a.borrow - 0.2, a.before_rust - 0.3);
    frontend(c, a.before_rust - 0.2, a.then_met - 0.3);
    rules_and_crab(c, a.then_met - 0.2, c.words.parts[2].0 - 0.15);
}

fn surat(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.4);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.07).clamp(36.0, 96.0 * u.max(1.0));
    let left = if g.compact {
        c.pad()
    } else {
        (g.w * 0.08).max(c.pad())
    };
    let ya = c.cy() - size * if g.compact { 1.3 } else { 0.55 };
    let yb = ya + size * 1.02;
    for (word, y, st) in [("Surat,", ya, a.surat), ("Gujarat.", yb, a.surat + 0.5)] {
        let p = ez(E::EaseOutCubic, t, st, 0.8);
        let ww = g.text_w(word, size, Font::Serif);
        g.save();
        g.alpha(w);
        g.c.begin_path();
        g.c.rect(
            (left - 8.0) as f64,
            (y - size * 0.95) as f64,
            (ww + 16.0) as f64,
            (size * 1.3) as f64,
        );
        g.c.clip();
        g.text(
            word,
            left,
            y + (1.0 - p) * size,
            size,
            Font::Serif,
            &rgba(TEXT, 1.0),
            Align::Left,
        );
        g.restore();
    }
    g.label(
        "21.17° N  ·  72.83° E",
        left,
        yb + 38.0 * u,
        12.5,
        3.0,
        &rgba(MUTED, w * fade(c, a.surat + 1.0)),
        Align::Left,
    );

    // tags
    let cs = 16.5 * u.clamp(0.85, 1.25);
    let mut x = left;
    let ty = yb + 92.0 * u;
    for (label, st) in [
        ("Diamonds", a.diamonds),
        ("Sarees", a.sarees),
        ("Food", a.food),
    ] {
        let pw = pill_w(g, label, cs);
        let p = pop(c, st).clamp(0.0, 1.1);
        g.save();
        g.c.translate((x + pw / 2.0) as f64, ty as f64).ok();
        g.c.scale(p as f64, p as f64).ok();
        pill(g, label, -pw / 2.0, 0.0, cs, w * prog(t, st, 0.12), false);
        g.restore();
        x += pw + 10.0;
    }

    // saree ribbons
    let rp = ez(E::EaseInOutCubic, t, a.sarees, 1.4);
    if rp > 0.0 {
        g.save();
        g.alpha(w);
        for i in 0..3 {
            let base = c.bottom() - if g.compact { 34.0 } else { 4.0 * u } + i as f32 * 12.0 * u;
            let amp = (10.0 + i as f32 * 4.0) * u;
            g.c.begin_path();
            let n = 90;
            for k in 0..=n {
                let fx = k as f32 / n as f32;
                if fx > rp {
                    break;
                }
                let x = fx * g.w;
                let y = base + (fx * 9.0 + c.wall * (0.9 + i as f32 * 0.2) + i as f32).sin() * amp;
                if k == 0 {
                    g.c.move_to(x as f64, y as f64)
                } else {
                    g.c.line_to(x as f64, y as f64)
                }
            }
            g.c.set_stroke_style_str(&rgba(ACCENT, 0.16 + 0.07 * (2 - i) as f32));
            g.c.set_line_width((2.0 + (2 - i) as f32) as f64);
            g.c.stroke();
        }
        g.restore();
    }

    // Scenix diamond (drag to spin)
    let dw = prog(t, a.diamonds - 0.1, 0.9);
    if dw > 0.0 {
        let (rx, ry, rw, rh) = if g.compact {
            (g.w * 0.5, c.top() - 6.0, g.w * 0.48, c.stage_h() * 0.42)
        } else {
            (g.w * 0.52, c.top() - 4.0, g.w * 0.42, c.stage_h() * 0.68)
        };
        let cam = camera(rw / rh, 4.8);
        let scale = 0.62 + 0.38 * ez(E::EaseOutCubic, t, a.diamonds, 1.1);
        let model = c
            .orbit
            .model(c.wall * 0.45, scale, Vec3::new(0.0, 0.0, 0.0));
        c.assets.diamond.draw(
            g,
            model,
            &cam,
            (rx, ry, rw, rh),
            ACCENT,
            w * dw,
            1.5 * u.clamp(0.8, 1.2),
        );
        g.glow(
            rx + rw / 2.0,
            ry + rh / 2.0,
            rw * 0.5,
            ACCENT,
            0.07 * w * dw,
        );
        c.hit(rx, ry, rw, rh, Action::Orbit);
        if t > a.diamonds + 1.5 {
            g.label(
                "drag to spin",
                rx + rw / 2.0,
                ry + rh - 4.0,
                10.5,
                2.4,
                &rgba(MUTED, w * 0.8 * prog(t, a.diamonds + 1.5, 0.6)),
                Align::Center,
            );
        }
    }

    // the diet's complaint form
    let cp = pop(c, a.complaint);
    if cp > 0.0 {
        let cw = (250.0 * u).clamp(210.0, 300.0);
        let ch = cw * 0.62;
        let (x, y) = if g.compact {
            (g.w - c.pad() - cw, c.bottom() - ch - 6.0)
        } else {
            (g.w - c.pad() - cw, c.bottom() - ch + 6.0)
        };
        g.save();
        g.c.translate((x + cw / 2.0) as f64, (y + ch / 2.0) as f64)
            .ok();
        g.c.rotate((0.035 + 0.04 * (1.0 - cp.min(1.0))) as f64).ok();
        g.c.scale(cp.min(1.1) as f64, cp.min(1.1) as f64).ok();
        card(g, -cw / 2.0, -ch / 2.0, cw, ch, w);
        g.label(
            "complaint  no. 0001",
            -cw / 2.0 + 16.0,
            -ch / 2.0 + 28.0,
            10.5,
            2.2,
            &rgba(MUTED, w),
            Align::Left,
        );
        g.text(
            "From: Diet",
            -cw / 2.0 + 16.0,
            -ch / 2.0 + 58.0,
            17.0 * u.clamp(0.9, 1.2),
            Font::Serif,
            &rgba(TEXT, w),
            Align::Left,
        );
        g.text(
            "Against: Food",
            -cw / 2.0 + 16.0,
            -ch / 2.0 + 84.0,
            17.0 * u.clamp(0.9, 1.2),
            Font::Serif,
            &rgba(TEXT, w),
            Align::Left,
        );
        g.text(
            "(too good)",
            -cw / 2.0 + 16.0,
            -ch / 2.0 + 106.0,
            14.0,
            Font::SerifItalic,
            &rgba(TEXT2, w),
            Align::Left,
        );
        let sp = ez(E::EaseOutCubic, t, a.complaint + 0.55, 0.22);
        if sp > 0.0 {
            stamp(
                g,
                "filed",
                cw * 0.2,
                ch * 0.08,
                17.0,
                -9.0,
                1.0 + 1.1 * (1.0 - sp),
                w * sp,
                WARM,
            );
        }
        g.restore();
    }
}

fn compiler(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let cx = c.cx();
    let hs = (g.w * 0.045).clamp(26.0, 62.0 * u.max(1.0));
    g.label(
        "by day",
        cx,
        c.top() + 22.0,
        13.0,
        4.0,
        &rgba(ACCENT, w * fade(c, a.byday)),
        Align::Center,
    );
    g.text(
        "Rust systems engineer.",
        cx,
        c.top() + 26.0 + hs,
        hs,
        Font::Serif,
        &rgba(TEXT, w * fade(c, a.byday + 0.2)),
        Align::Center,
    );

    let cw = (g.w - 2.0 * c.pad()).min(780.0 * u.max(1.0));
    let fs = (17.0 * u).clamp(11.5, 20.0);
    let lh = fs * 1.55;
    let code = [
        "error[E0382]: borrow of moved value: `darshan`",
        "  --> life.rs:7:18",
        "   |",
        "5  |     let me = darshan;",
        "   |              ------- value moved here",
        "7  |     argue(&compiler, darshan);",
        "   |                      ^^^^^^^ value borrowed here after move",
    ];
    let full = code.join("\n");
    let shown = typed(&full, t, c.a.argue, 44.0);
    let ch = lh * code.len() as f32 + 36.0;
    let x = cx - cw / 2.0;
    let y = c.top() + 26.0 + hs + 34.0 * u.min(1.2);
    if t >= a.argue - 0.2 {
        card(g, x, y, cw, ch, w * fade(c, a.argue - 0.2));
        g.save();
        g.alpha(w);
        let mut yy = y + 28.0;
        for line in shown.split('\n') {
            let col =
                if line.contains("^^^") || line.contains("-------") || line.starts_with("error") {
                    rgba(WARM, 1.0)
                } else if line.contains("-->") || line.starts_with("   |") {
                    rgba(MUTED, 1.0)
                } else {
                    rgba(TEXT, 1.0)
                };
            g.text(line, x + 18.0, yy, fs, Font::Mono, &col, Align::Left);
            yy += lh;
        }
        // caret
        if (c.wall * 2.0) as i32 % 2 == 0 && shown.len() < full.len() {
            g.fill_rrect(
                x + 18.0 + g.text_w(shown.rsplit('\n').next().unwrap_or(""), fs, Font::Mono) + 2.0,
                yy - lh - fs * 0.78,
                fs * 0.5,
                fs * 1.1,
                1.0,
                &rgba(ACCENT, 1.0),
            );
        }
        g.restore();
        let sp = pop(c, a.lose);
        stamp(
            g,
            "lost",
            x + cw - 70.0,
            y + ch - 26.0,
            20.0 * u.clamp(0.8, 1.2),
            -8.0,
            sp.min(1.2),
            w * prog(t, a.lose, 0.1),
            WARM,
        );
    }
}

fn every_single_time(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.08, 0.25);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.12).clamp(52.0, 150.0 * u.max(1.0));
    let words = [
        ("Every.", a.every, TEXT),
        ("Single.", a.single, TEXT),
        ("Time.", a.time, ACCENT),
    ];
    for (i, (word, st, col)) in words.iter().enumerate() {
        if t < *st - 0.02 {
            continue;
        }
        let k = 1.0 + 0.5 * (1.0 - ez(E::EaseOutCubic, t, *st, 0.22));
        let y = c.cy() + (i as f32 - 1.0) * size * 1.02 + size * 0.3;
        g.save();
        g.alpha(w * prog(t, *st, 0.06));
        g.c.translate(c.cx() as f64, (y - size * 0.3) as f64).ok();
        g.c.scale(k as f64, k as f64).ok();
        g.text(
            word,
            0.0,
            size * 0.3,
            size,
            Font::SerifItalic,
            &rgba(*col, 1.0),
            Align::Center,
        );
        g.restore();
    }
}

fn scoreboard(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.25, 0.3);
    if w <= 0.01 {
        return;
    }
    let cw = (g.w * 0.42).min(380.0 * u.max(1.0));
    let ch = (c.stage_h() * 0.72).min(300.0 * u.max(1.0));
    let gap = 28.0 * u.min(1.2);
    let y = c.cy() - ch / 2.0;
    for (i, (name, num, st, col)) in [
        ("Borrow checker", "1", a.one, TEXT),
        ("Darshan", "0", a.zero, ACCENT),
    ]
    .into_iter()
    .enumerate()
    {
        let x = if i == 0 {
            c.cx() - gap / 2.0 - cw
        } else {
            c.cx() + gap / 2.0
        };
        card(g, x, y, cw, ch, w);
        g.label(
            name,
            x + cw / 2.0,
            y + 36.0,
            12.5,
            3.0,
            &rgba(TEXT2, w),
            Align::Center,
        );
        let p = pop(c, st);
        let size = ch * 0.62;
        g.save();
        g.alpha(w * prog(t, st, 0.1));
        g.c.translate((x + cw / 2.0) as f64, (y + ch * 0.74) as f64)
            .ok();
        g.c.scale(p.min(1.18) as f64, p.min(1.18) as f64).ok();
        g.text(
            num,
            0.0,
            0.0,
            size,
            Font::Serif,
            &rgba(col, 1.0),
            Align::Center,
        );
        g.restore();
    }
    g.text(
        "–",
        c.cx(),
        y + ch * 0.62,
        40.0 * u.clamp(0.8, 1.2),
        Font::Serif,
        &rgba(MUTED, w),
        Align::Center,
    );
}

fn frontend(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.3);
    if w <= 0.01 {
        return;
    }
    let cx = c.cx();
    g.label(
        "before rust",
        cx,
        c.cy() - 90.0 * u,
        13.0,
        4.0,
        &rgba(ACCENT, w * fade(c, a.before_rust)),
        Align::Center,
    );
    let cw = (g.w - 2.0 * c.pad()).min(520.0 * u.max(1.0));
    let ch = 120.0 * u.clamp(0.8, 1.2);
    let p = pop(c, a.frontend - 0.3);
    g.save();
    g.c.translate(cx as f64, c.cy() as f64).ok();
    g.c.scale(p.min(1.08) as f64, p.min(1.08) as f64).ok();
    card(g, -cw / 2.0, -ch / 2.0, cw, ch, w);
    g.text(
        "</>",
        -cw / 2.0 + 34.0,
        10.0,
        34.0 * u.clamp(0.8, 1.2),
        Font::MonoBold,
        &rgba(ACCENT, w),
        Align::Left,
    );
    g.text(
        "Frontend developer",
        -cw / 2.0 + 118.0 * u.clamp(0.8, 1.2),
        -4.0,
        26.0 * u.clamp(0.8, 1.2),
        Font::Serif,
        &rgba(TEXT, w),
        Align::Left,
    );
    g.text(
        "right here in Surat",
        -cw / 2.0 + 118.0 * u.clamp(0.8, 1.2),
        24.0,
        16.0,
        Font::SerifItalic,
        &rgba(TEXT2, w),
        Align::Left,
    );
    g.restore();
}

fn rules_and_crab(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    if w <= 0.01 {
        return;
    }
    let left = if g.compact {
        c.pad()
    } else {
        (g.w * 0.07).max(c.pad())
    };
    let cs = 17.0 * u.clamp(0.85, 1.25);
    // the two ideas
    let mut x = left;
    let y0 = c.top() + 26.0;
    for (label, st) in [("ownership", a.ownership), ("'a lifetimes", a.lifetimes)] {
        let pw = pill_w(g, label, cs);
        let p = pop(c, st).clamp(0.0, 1.1);
        g.save();
        g.c.translate((x + pw / 2.0) as f64, y0 as f64).ok();
        g.c.scale(p as f64, p as f64).ok();
        pill(g, label, -pw / 2.0, 0.0, cs, w * prog(t, st, 0.12), true);
        g.restore();
        x += pw + 12.0;
    }
    // the rulebook
    let cw = if g.compact {
        g.w - 2.0 * c.pad()
    } else {
        (g.w * 0.5).min(560.0 * u.max(1.0))
    };
    let rules = [
        "Every value has exactly one owner.",
        "Borrow it. Do not grab it.",
        "Nothing outlives its welcome.",
    ];
    let rh = 56.0 * u.clamp(0.8, 1.15);
    let ch = rh * 3.0 + 66.0;
    let y = y0 + 40.0 * u;
    let rp = pop(c, a.rules - 0.2);
    if rp > 0.0 {
        g.save();
        g.c.translate((left + cw / 2.0) as f64, (y + ch / 2.0) as f64)
            .ok();
        g.c.scale(rp.min(1.04) as f64, rp.min(1.04) as f64).ok();
        card(g, -cw / 2.0, -ch / 2.0, cw, ch, w);
        g.label(
            "rulebook",
            -cw / 2.0 + 20.0,
            -ch / 2.0 + 30.0,
            11.0,
            3.0,
            &rgba(MUTED, w),
            Align::Left,
        );
        for (i, r) in rules.iter().enumerate() {
            let st = a.rules + 0.25 + i as f32 * 0.55;
            let ra = w * fade(c, st);
            let yy = -ch / 2.0 + 62.0 + rh * i as f32 + rh * 0.4;
            g.text(
                &format!("{}", i + 1),
                -cw / 2.0 + 24.0,
                yy,
                22.0 * u.clamp(0.8, 1.2),
                Font::Mono,
                &rgba(ACCENT, ra),
                Align::Left,
            );
            g.text(
                r,
                -cw / 2.0 + 62.0,
                yy,
                21.0 * u.clamp(0.8, 1.2),
                Font::Serif,
                &rgba(TEXT, ra),
                Align::Left,
            );
        }
        g.restore();
        // STRICT
        let sp = ez(E::EaseOutCubic, t, a.strict, 0.24);
        if sp > 0.0 {
            stamp(
                g,
                "strict",
                left + cw - 56.0,
                y + ch - 20.0,
                20.0 * u.clamp(0.8, 1.2),
                -7.0,
                1.0 + 1.2 * (1.0 - sp),
                w * sp,
                ACCENT,
            );
        }
    }
    // the very rude crab scuttles in
    let walk = ez(E::EaseOutCubic, t, a.crab - 1.4, 1.6);
    if t > a.crab - 1.6 {
        let (tx, ty, sc) = if g.compact {
            (g.w * 0.74, c.bottom() - 40.0, 0.9)
        } else {
            (g.w * 0.78, c.cy() + 40.0 * u, 1.25 * u.clamp(0.8, 1.3))
        };
        let x = lerp(g.w + 120.0, tx, walk);
        let moving = if walk < 0.98 { 1.0 } else { 0.0 };
        crab(
            g,
            x,
            ty,
            sc,
            c.wall + moving * 0.0,
            if t > a.crab { 1.0 } else { 0.25 },
            w,
        );
        // speech bubble
        let bp = pop(c, a.crab + 0.35);
        if bp > 0.0 {
            let bw = 190.0 * u.clamp(0.8, 1.2);
            let bh = 62.0 * u.clamp(0.8, 1.2);
            g.save();
            g.c.translate((x - 10.0) as f64, (ty - 120.0 * sc) as f64)
                .ok();
            g.c.scale(bp.min(1.08) as f64, bp.min(1.08) as f64).ok();
            card(g, -bw / 2.0, -bh / 2.0, bw, bh, w);
            g.text(
                "cargo check.",
                0.0,
                7.0,
                21.0 * u.clamp(0.8, 1.2),
                Font::Mono,
                &rgba(WARM, w),
                Align::Center,
            );
            g.restore();
        }
    }
}
