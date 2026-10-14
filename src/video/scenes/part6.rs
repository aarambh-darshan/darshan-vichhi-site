//! Part 6: the deal, the links (all clickable), the contact button and the end card.
use crate::video::ctx::{Action, Ctx, LINKS};
use crate::video::fx::*;
use crate::video::g2d::{Align, Font};
use crate::video::kit::*;
use crate::video::theme::*;

pub fn draw(c: &Ctx) {
    let a = c.a;
    let p0 = c.words.parts[5].0;
    deal(c, p0 - 0.3, a.scroll - 0.3);
    links(c, a.scroll - 0.25, a.thanks - 0.3);
    finale(c, a.thanks - 0.2);
}

fn deal(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.07).clamp(36.0, 96.0 * u.max(1.0));
    let p = ez(E::EaseOutCubic, t, a.deal - 0.3, 0.8);
    let y = c.top() + 16.0 + size;
    let word = "The deal.";
    let ww = g.text_w(word, size, Font::Serif);
    g.save();
    g.alpha(w);
    g.c.begin_path();
    g.c.rect(
        (c.cx() - ww / 2.0 - 10.0) as f64,
        (y - size) as f64,
        (ww + 20.0) as f64,
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
    let fs = (g.w * 0.03).clamp(20.0, 38.0 * u.max(1.0));
    let lines = [
        ("a Rust engineer", a.engineer),
        ("an LLM nerd", a.llm),
        ("someone who explains lifetimes at 2 a.m.", a.lifetimes2),
    ];
    for (i, (txt, st)) in lines.iter().enumerate() {
        let pa = fade(c, *st - 0.1);
        let ly = y + size * 0.6 + 44.0 * u + i as f32 * fs * 1.6;
        g.fill_circle(
            c.cx() - g.text_w(txt, fs, Font::Serif) / 2.0 - 22.0,
            ly - fs * 0.3,
            4.0,
            &rgba(ACCENT, w * pa),
        );
        g.text(
            txt,
            c.cx(),
            ly + (1.0 - pa) * 10.0,
            fs,
            Font::Serif,
            &rgba(TEXT, w * pa),
            Align::Center,
        );
    }
    let gp = pop(c, a.guy - 0.1);
    if gp > 0.0 {
        let gs = (g.w * 0.05).clamp(28.0, 64.0 * u.max(1.0));
        g.save();
        g.c.translate(
            c.cx() as f64,
            (y + size * 0.6 + 44.0 * u + 3.0 * fs * 1.6 + gs) as f64,
        )
        .ok();
        g.c.scale(gp.min(1.1) as f64, gp.min(1.1) as f64).ok();
        g.text(
            "I'm your guy.",
            0.0,
            0.0,
            gs,
            Font::SerifItalic,
            &rgba(ACCENT, w * prog(t, a.guy - 0.1, 0.1)),
            Align::Center,
        );
        g.restore();
    }
}

#[allow(clippy::too_many_arguments)]
fn link_card(c: &Ctx, i: usize, x: f32, y: f32, w: f32, h: f32, st: f32, alpha: f32) {
    let g = c.g;
    let u = g.u;
    let pp = pop(c, st).clamp(0.0, 1.06);
    if pp <= 0.0 {
        return;
    }
    let hov = c.hovering(Action::Link(i));
    let lift = if hov { -4.0 } else { 0.0 };
    g.save();
    g.c.translate((x + w / 2.0) as f64, (y + h / 2.0 + lift) as f64)
        .ok();
    g.c.scale(pp as f64, pp as f64).ok();
    g.alpha(alpha * prog(c.t, st, 0.12));
    g.fill_rrect(
        -w / 2.0,
        -h / 2.0,
        w,
        h,
        10.0,
        &rgba(if hov { SURFACE_HI } else { SURFACE }, 0.96),
    );
    g.stroke_rrect(
        -w / 2.0,
        -h / 2.0,
        w,
        h,
        10.0,
        &rgba(if hov { ACCENT } else { BORDER }, 1.0),
        1.0,
    );
    g.text(
        LINKS[i].0,
        -w / 2.0 + 20.0,
        -h * 0.02,
        25.0 * u.clamp(0.8, 1.15),
        Font::Serif,
        &rgba(TEXT, 1.0),
        Align::Left,
    );
    g.text(
        LINKS[i].1,
        -w / 2.0 + 20.0,
        h * 0.28,
        13.0,
        Font::Mono,
        &rgba(if hov { ACCENT } else { TEXT2 }, 1.0),
        Align::Left,
    );
    g.restore();
    c.hit(x, y + lift, w, h, Action::Link(i));
}

fn links(c: &Ctx, s: f32, e: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = window(t, s, e, 0.3, 0.35);
    if w <= 0.01 {
        return;
    }
    g.label(
        "say hi",
        c.cx(),
        c.top() + 14.0,
        12.5,
        4.0,
        &rgba(ACCENT, w * fade(c, a.scroll)),
        Align::Center,
    );
    let cols = if g.compact { 2 } else { 3 };
    let gap = 14.0;
    let cw =
        ((g.w - 2.0 * c.pad() - gap * (cols as f32 - 1.0)) / cols as f32).min(310.0 * u.max(1.0));
    let ch = (c.stage_h() * 0.24).clamp(64.0, 100.0 * u.max(1.0));
    let x0 = c.cx() - (cw * cols as f32 + gap * (cols as f32 - 1.0)) / 2.0;
    let y0 = c.top() + 36.0;
    let times = [
        a.links_github,
        a.links_youtube,
        a.links_discord,
        a.links_discord + 0.55,
        a.links_discord + 0.9,
        a.links_discord + 1.25,
    ];
    for (i, st) in times.iter().enumerate() {
        let (cx_, cy_) = (i % cols, i / cols);
        link_card(
            c,
            i,
            x0 + cx_ as f32 * (cw + gap),
            y0 + cy_ as f32 * (ch + gap),
            cw,
            ch,
            *st,
            w,
        );
    }
    // contact
    let rows = 6 / cols;
    let by = y0 + rows as f32 * (ch + gap) + 6.0;
    if t > a.contact - 0.6 {
        contact_button(
            c,
            c.cx(),
            by + 24.0,
            w * fade(c, a.contact - 0.6),
            t - a.contact,
        );
    }
    let fs = (g.w * 0.024).clamp(16.0, 28.0 * u.max(1.0));
    g.text(
        "I work fast.",
        c.cx(),
        by + 24.0 + 66.0 * u.min(1.2),
        fs,
        Font::SerifItalic,
        &rgba(TEXT2, w * fade(c, a.fast)),
        Align::Center,
    );
    g.text(
        "I only argue with the compiler.",
        c.cx(),
        by + 24.0 + 66.0 * u.min(1.2) + fs * 1.5,
        fs,
        Font::SerifItalic,
        &rgba(TEXT, w * fade(c, a.compiler)),
        Align::Center,
    );
}

fn contact_button(c: &Ctx, cx: f32, cy: f32, alpha: f32, since: f32) {
    let g = c.g;
    let u = g.u;
    let hov = c.hovering(Action::Contact);
    let size = 20.0 * u.clamp(0.85, 1.2);
    let w = g.text_w("Contact me", size, Font::SansMedium) + size * 3.4;
    let h = size * 2.5;
    // attention pulse right after "contact" is spoken
    if since > 0.0 && since < 1.6 {
        let p = since / 1.6;
        g.save();
        g.alpha(alpha * (1.0 - p));
        g.stroke_rrect(
            cx - w / 2.0 - 20.0 * p,
            cy - h / 2.0 - 20.0 * p,
            w + 40.0 * p,
            h + 40.0 * p,
            h / 2.0 + 20.0 * p,
            &rgba(ACCENT, 1.0),
            1.5,
        );
        g.restore();
    }
    g.save();
    g.alpha(alpha);
    g.fill_rrect(
        cx - w / 2.0,
        cy - h / 2.0,
        w,
        h,
        h / 2.0,
        &rgba(if hov { TEXT } else { ACCENT }, 1.0),
    );
    g.text(
        "Contact me",
        cx,
        cy + size * 0.36,
        size,
        Font::SansMedium,
        &rgba(BG, 1.0),
        Align::Center,
    );
    g.restore();
    c.hit(cx - w / 2.0, cy - h / 2.0, w, h, Action::Contact);
}

fn finale(c: &Ctx, s: f32) {
    let g = c.g;
    let (t, a, u) = (c.t, c.a, g.u);
    let w = ez(E::EaseOutCubic, t, s, 0.4);
    if w <= 0.01 {
        return;
    }
    let size = (g.w * 0.075).clamp(34.0, 104.0 * u.max(1.0));
    let y = c.top() + 24.0 + size;
    for (word, st, col, dy) in [
        ("Thanks for watching.", a.thanks, TEXT, 0.0),
        ("Happy coding!", a.happy - 0.4, ACCENT, size * 1.1),
    ] {
        let p = ez(E::EaseOutCubic, t, st, 0.8);
        let ww = g.text_w(word, size, Font::Serif);
        g.save();
        g.alpha(w);
        g.c.begin_path();
        g.c.rect(
            (c.cx() - ww / 2.0 - 10.0) as f64,
            (y + dy - size) as f64,
            (ww + 20.0) as f64,
            (size * 1.3) as f64,
        );
        g.c.clip();
        g.text(
            word,
            c.cx(),
            y + dy + (1.0 - p) * size,
            size,
            Font::Serif,
            &rgba(col, 1.0),
            Align::Center,
        );
        g.restore();
    }
    // the links stay clickable
    let t0 = a.happy + 0.6;
    let cols = if g.compact { 2 } else { 3 };
    let gap = 12.0;
    let cw =
        ((g.w - 2.0 * c.pad() - gap * (cols as f32 - 1.0)) / cols as f32).min(250.0 * u.max(1.0));
    let ch = (c.stage_h() * 0.17).clamp(54.0, 74.0 * u.max(1.0));
    let x0 = c.cx() - (cw * cols as f32 + gap * (cols as f32 - 1.0)) / 2.0;
    let y0 = y + size * 1.1 + 36.0 * u.min(1.2);
    for i in 0..6 {
        let (cx_, cy_) = (i % cols, i / cols);
        link_card(
            c,
            i,
            x0 + cx_ as f32 * (cw + gap),
            y0 + cy_ as f32 * (ch + gap),
            cw,
            ch,
            t0 + i as f32 * 0.12,
            w,
        );
    }
    // buttons
    let rows = 6 / cols;
    let by = y0 + rows as f32 * (ch + gap) + 22.0;
    let bt = t0 + 1.0;
    if t > bt {
        let ba = fade(c, bt) * w;
        let size_b = 18.0 * u.clamp(0.85, 1.2);
        let labels = [
            ("Contact me", Action::Contact),
            ("Replay", Action::Replay),
            ("Close", Action::Close),
        ];
        let widths: Vec<f32> = labels
            .iter()
            .map(|(l, _)| g.text_w(l, size_b, Font::SansMedium) + size_b * 2.8)
            .collect();
        let total: f32 = widths.iter().sum::<f32>() + 12.0 * 2.0;
        let mut x = c.cx() - total / 2.0;
        let h = size_b * 2.5;
        for ((label, act), bw) in labels.iter().zip(&widths) {
            let hov = c.hovering(*act);
            g.save();
            g.alpha(ba);
            if *act == Action::Contact {
                g.fill_rrect(
                    x,
                    by,
                    *bw,
                    h,
                    h / 2.0,
                    &rgba(if hov { TEXT } else { ACCENT }, 1.0),
                );
                g.text(
                    label,
                    x + bw / 2.0,
                    by + h / 2.0 + size_b * 0.36,
                    size_b,
                    Font::SansMedium,
                    &rgba(BG, 1.0),
                    Align::Center,
                );
            } else {
                g.fill_rrect(
                    x,
                    by,
                    *bw,
                    h,
                    h / 2.0,
                    &rgba(if hov { SURFACE_HI } else { SURFACE }, 1.0),
                );
                g.stroke_rrect(
                    x,
                    by,
                    *bw,
                    h,
                    h / 2.0,
                    &rgba(if hov { ACCENT } else { BORDER }, 1.0),
                    1.0,
                );
                g.text(
                    label,
                    x + bw / 2.0,
                    by + h / 2.0 + size_b * 0.36,
                    size_b,
                    Font::SansMedium,
                    &rgba(TEXT, 1.0),
                    Align::Center,
                );
            }
            g.restore();
            c.hit(x, by, *bw, h, *act);
            x += bw + 12.0;
        }
    }
}
