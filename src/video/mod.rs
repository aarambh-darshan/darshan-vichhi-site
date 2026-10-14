//! Interactive intro video for darshan-vichhi-site, drawn in Rust and compiled to WebAssembly.
//!
//! * Animato (tweens, easing, springs) provides every piece of motion.
//! * Scenix (geometry, camera, math) provides the 3D, projected onto the same canvas.
//! * The narration (ElevenLabs) is the clock: the picture is a pure function of `audio.currentTime`,
//!   so pausing, scrubbing and replaying are exact and the words land on the beat.
//!
//! JavaScript only loads this module and hands it the play button and an <audio> element.

mod anchors;
mod chrome;
mod ctx;
mod fx;
mod g2d;
mod kit;
mod scenes;
mod theme;
mod wire;
mod words;

use anchors::A;
use ctx::{Action, Assets, Ctx, Hit, LINKS, PROJECTS, Ptr};
use fx::{E, ez, lerp, prog, shake};
use g2d::{Align, Font, G};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use theme::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d, Document, HtmlAudioElement, HtmlCanvasElement, HtmlElement,
    KeyboardEvent, PointerEvent, Window,
};
use wire::Orbit;
use words::Words;

/// Opening: the play button presses and springs (OPEN_BOUNCE), then grows into the page; the narration starts at OPEN_SECS.
const OPEN_BOUNCE: f32 = 0.45;
const OPEN_SECS: f32 = 1.6;
const CLOSE_SECS: f32 = 0.85;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Opening,
    Running,
    Closing,
}

#[derive(Clone, Copy, PartialEq)]
enum Drag {
    None,
    Scrub,
    Orbit,
}

type Listener = (String, Closure<dyn FnMut(JsValue)>);

struct Player {
    win: Window,
    doc: Document,
    canvas: HtmlCanvasElement,
    c2d: CanvasRenderingContext2d,
    button: HtmlElement,
    audio: HtmlAudioElement,
    words: Words,
    anchors: A,
    assets: Assets,
    orbit: Orbit,
    hits: RefCell<Vec<Hit>>,

    phase: Phase,
    phase_t0: f64,
    open_t0: f64,
    audio_started: bool,
    user_paused: bool,
    captions: bool,
    reduced: bool,
    muted: bool,
    /// Test hook: when set the video clock ignores the audio element.
    override_t: Option<f32>,

    ptr: Ptr,
    hover: Option<Action>,
    drag: Drag,
    drag_moved: bool,
    ripples: Vec<(f32, f32, f64)>,
    last_ms: f64,
    frame_ms: VecDeque<f32>,
    fps: f32,
    size: (f32, f32, f32),
    /// The play button's rectangle: left, top, width, height.
    btn: (f32, f32, f32, f32),
    /// When the narration finished (the end card keeps animating after the audio stops).
    ended_at: Option<f64>,
    /// A link to open once the event handler has released its borrow of the player.
    pending_open: Option<String>,
    scroll_to_contact: bool,
    listeners: Vec<Listener>,
    raf: Option<Closure<dyn FnMut(f64)>>,
    raf_id: Option<i32>,
    closed: bool,
}

thread_local! {
    static ACTIVE: RefCell<Option<Rc<RefCell<Player>>>> = const { RefCell::new(None) };
}

fn now_ms(win: &Window) -> f64 {
    win.performance().map(|p| p.now()).unwrap_or(0.0)
}

impl Player {
    fn video_time(&self) -> f32 {
        if let Some(t) = self.override_t {
            return t;
        }
        let wall = ((now_ms(&self.win) - self.open_t0) / 1000.0) as f32;
        if !self.audio_started {
            return wall - OPEN_SECS; // negative: the page is still opening
        }
        (self.audio.current_time() as f32).clamp(0.0, self.words.duration)
    }

    fn start_audio(&mut self) {
        if self.audio_started {
            return;
        }
        self.audio_started = true;
        let _ = self.audio.set_attribute("data-started", "1");
        self.audio.set_current_time(0.0);
        self.audio.set_muted(self.muted);
        if let Ok(p) = self.audio.play() {
            // Autoplay can be refused: show the paused state instead of failing silently.
            let audio = self.audio.clone();
            let cb = Closure::<dyn FnMut(JsValue)>::new(move |_e: JsValue| {
                let _ = audio.pause();
            });
            let _ = p.catch(&cb);
            cb.forget();
        }
    }

    fn is_paused(&self) -> bool {
        self.user_paused || (self.audio_started && self.audio.paused() && !self.audio.ended())
    }

    fn toggle_pause(&mut self) {
        if !self.audio_started {
            return;
        }
        if self.audio.ended() || self.video_time() >= self.words.duration - 0.05 {
            self.replay();
            return;
        }
        if self.audio.paused() {
            self.user_paused = false;
            let _ = self.audio.play();
        } else {
            self.user_paused = true;
            let _ = self.audio.pause();
        }
    }

    fn seek(&mut self, secs: f32) {
        self.ended_at = None;
        let t = secs.clamp(0.0, (self.words.duration - 0.05).max(0.0));
        if !self.audio_started {
            self.start_audio();
        }
        self.audio.set_current_time(t as f64);
        if self.override_t.is_some() {
            self.override_t = Some(t);
        }
        if !self.user_paused && self.audio.paused() {
            let _ = self.audio.play();
        }
    }

    fn replay(&mut self) {
        self.ended_at = None;
        self.user_paused = false;
        self.override_t = None;
        self.audio_started = true;
        self.audio.set_current_time(0.0);
        let _ = self.audio.play();
    }

    fn begin_close(&mut self, contact: bool) {
        if self.phase == Phase::Closing {
            return;
        }
        self.phase = Phase::Closing;
        self.phase_t0 = now_ms(&self.win);
        self.scroll_to_contact = contact;
        let _ = self.audio.pause();
    }

    /// Links open from the event wrapper after this borrow is released: the browser can fire events
    /// synchronously while a tab opens, and those handlers need the player too.
    fn open_url(&mut self, url: &str) {
        self.pending_open = Some(url.to_string());
        // pause the narration so the visitor can come back and carry on
        if self.audio_started && !self.audio.paused() {
            self.user_paused = true;
            let _ = self.audio.pause();
        }
    }

    fn do_action(&mut self, a: Action, x: f32) {
        match a {
            Action::Close => self.begin_close(false),
            Action::Contact => self.begin_close(true),
            Action::PlayPause => self.toggle_pause(),
            Action::Mute => {
                self.muted = !self.muted;
                self.audio.set_muted(self.muted);
            }
            Action::Captions => self.captions = !self.captions,
            Action::Replay => self.replay(),
            Action::Link(i) => self.open_url(LINKS[i].2),
            Action::Project(i) => self.open_url(PROJECTS[i].1),
            Action::Part(_) | Action::Scrub => self.scrub_to(x),
            Action::Orbit => {}
        }
    }

    fn scrub_to(&mut self, x: f32) {
        let pad = (self.size.0 * 0.05).clamp(16.0, 56.0);
        let frac = ((x - pad) / (self.size.0 - 2.0 * pad)).clamp(0.0, 1.0);
        self.seek(frac * self.words.duration);
    }

    fn hit_test(&self, x: f32, y: f32) -> Option<Action> {
        self.hits
            .borrow()
            .iter()
            .rev()
            .find(|h| x >= h.x && x <= h.x + h.w && y >= h.y && y <= h.y + h.h)
            .map(|h| h.action)
    }

    fn resize(&mut self) {
        let w = self
            .win
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(1280.0) as f32;
        let h = self
            .win
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(720.0) as f32;
        let dpr = (self.win.device_pixel_ratio() as f32).clamp(1.0, 2.0);
        if (w, h, dpr) != self.size {
            self.size = (w, h, dpr);
            self.canvas.set_width((w * dpr) as u32);
            self.canvas.set_height((h * dpr) as u32);
        }
    }

    fn frame(&mut self, now: f64) {
        self.resize();
        let (w, h, dpr) = self.size;
        let dt_ms = if self.last_ms > 0.0 {
            (now - self.last_ms) as f32
        } else {
            16.7
        };
        self.last_ms = now;
        if dt_ms > 0.0 && dt_ms < 500.0 {
            self.frame_ms.push_back(dt_ms);
            if self.frame_ms.len() > 120 {
                self.frame_ms.pop_front();
            }
            self.fps = lerp(self.fps, 1000.0 / dt_ms, 0.08);
        }
        let dt = (dt_ms / 1000.0).clamp(0.0, 0.1);
        self.orbit.step(dt);

        // phase bookkeeping
        let since_phase = ((now - self.phase_t0) / 1000.0) as f32;
        if self.phase == Phase::Opening {
            let wall = ((now - self.open_t0) / 1000.0) as f32;
            if wall >= OPEN_SECS && self.override_t.is_none() {
                self.start_audio();
            }
            if wall >= OPEN_SECS {
                self.phase = Phase::Running;
                let _ = self.canvas.style().set_property("clip-path", "none");
            }
        }
        if self.phase == Phase::Closing && since_phase >= CLOSE_SECS {
            self.finish_close();
            return;
        }

        let mut t = self.video_time();
        if self.override_t.is_none() && self.audio_started {
            let at = self.audio.current_time() as f32;
            if self.audio.ended() || at >= self.words.duration - 0.03 {
                // keep the clock running past the last word so the end card can finish animating
                let since = *self.ended_at.get_or_insert(now);
                t = self.words.duration + ((now - since) / 1000.0) as f32;
            } else {
                self.ended_at = None;
            }
        }
        let wall_s = (now / 1000.0) as f32;
        let u = (w / 1280.0).min(h / 720.0).clamp(0.55, 1.6);
        let g = G {
            c: self.c2d.clone(),
            w,
            h,
            u,
            compact: w < 720.0,
            reduced: self.reduced,
        };
        g.c.set_transform(dpr as f64, 0.0, 0.0, dpr as f64, 0.0, 0.0)
            .ok();
        g.c.set_global_alpha(1.0);
        self.hits.borrow_mut().clear();

        // page -> video: the play button (a pill) presses, springs, then grows into the whole page.
        // On close it shrinks back into the button.
        let (bl, bt, bw, bh) = self.btn;
        let wall_open = ((now - self.open_t0) / 1000.0) as f32;
        let (s, reveal) = match self.phase {
            Phase::Opening => (
                press(prog(wall_open, 0.0, OPEN_BOUNCE)),
                ez(
                    E::EaseInOutCubic,
                    wall_open,
                    OPEN_BOUNCE,
                    OPEN_SECS - OPEN_BOUNCE,
                ),
            ),
            Phase::Running => (1.0, 1.0),
            Phase::Closing => (
                1.0,
                1.0 - ez(E::EaseInOutCubic, since_phase, 0.0, CLOSE_SECS),
            ),
        };
        let (bcx, bcy) = (bl + bw / 2.0, bt + bh / 2.0);
        if self.phase != Phase::Running {
            let (sw, sh) = (bw * s, bh * s);
            let (x0, y0) = (bcx - sw / 2.0, bcy - sh / 2.0);
            let left = lerp(x0, 0.0, reveal);
            let top = lerp(y0, 0.0, reveal);
            let right = lerp(w - (x0 + sw), 0.0, reveal);
            let bottom = lerp(h - (y0 + sh), 0.0, reveal);
            let radius = lerp(sh / 2.0, 0.0, reveal);
            let _ = self.canvas.style().set_property(
                "clip-path",
                &format!(
                    "inset({top:.1}px {right:.1}px {bottom:.1}px {left:.1}px round {radius:.1}px)"
                ),
            );
        }

        // background: the portfolio's flat dark, a whisper of accent light, no grid
        let face = reveal.min(1.0);
        let bg = (
            lerp(SURFACE.0 as f32, BG.0 as f32, face) as u8,
            lerp(SURFACE.1 as f32, BG.1 as f32, face) as u8,
            lerp(SURFACE.2 as f32, BG.2 as f32, face) as u8,
        );
        g.fill(&rgba(bg, 1.0));
        g.c.fill_rect(0.0, 0.0, w as f64, h as f64);
        let par = if self.reduced { 0.0 } else { 0.04 };
        let (gx, gy) = (
            w * 0.5 + (self.ptr.x - w / 2.0) * par,
            h * 0.42 + (self.ptr.y - h / 2.0) * par,
        );
        g.glow(gx, gy, w.max(h) * 0.62, ACCENT, 0.045 * face);
        g.glow(
            w * 0.08 + (wall_s * 0.2).sin() * 30.0,
            h * 0.95,
            w.max(h) * 0.4,
            ACCENT,
            0.025 * face,
        );

        // a replica of the page's play button, so the swap from page to canvas is invisible
        if self.phase != Phase::Running {
            let k = (1.0 - reveal * 2.6).clamp(0.0, 1.0);
            if k > 0.0 {
                g.save();
                g.alpha(k);
                g.c.translate(bcx as f64, bcy as f64).ok();
                g.c.scale(s as f64, s as f64).ok();
                let (hw, hh) = (bw / 2.0, bh / 2.0);
                g.stroke_rrect(
                    -hw + 0.5,
                    -hh + 0.5,
                    bw - 1.0,
                    bh - 1.0,
                    hh,
                    &rgba(BORDER, 1.0),
                    1.0,
                );
                let ic = -hw + hh;
                g.stroke_circle(ic, 0.0, hh - 8.0, &rgba(ACCENT, 1.0), 1.0);
                g.play_glyph(ic + 1.8, 0.0, 7.5, &rgba(ACCENT, 1.0));
                let tx = ic + hh - 8.0 + 11.0;
                g.text(
                    "Watch the intro",
                    tx,
                    5.0,
                    14.0,
                    Font::Sans,
                    &rgba(TEXT, 1.0),
                    Align::Left,
                );
                if bw > 190.0 {
                    let lw = g.text_w("Watch the intro", 14.0, Font::Sans);
                    g.text(
                        "2:51",
                        tx + lw + 8.0,
                        5.0,
                        12.5,
                        Font::Sans,
                        &rgba(MUTED, 1.0),
                        Align::Left,
                    );
                }
                g.restore();
            }
        }

        // screen shake on the big beats
        let a = &self.anchors;
        let (mut sx, mut sy) = (0.0, 0.0);
        if !self.reduced {
            sx = shake(t, a.works, 0.45, 9.0) * 7.0
                + shake(t, a.every, 0.3, 11.0) * 6.0
                + shake(t, a.single, 0.3, 11.0) * 6.0
                + shake(t, a.time, 0.34, 11.0) * 8.0
                + shake(t, a.zero, 0.4, 10.0) * 5.0;
            sy = shake(t, a.works, 0.45, 13.0) * 5.0
                + shake(t, a.time, 0.34, 13.0) * 5.0
                + shake(t, a.please_stop, 0.4, 12.0) * 3.0;
        }

        let paused = self.is_paused();
        let ended = self.audio_started && (self.audio.ended() || t >= self.words.duration - 0.03);
        let loading = self.audio_started && self.audio.ready_state() < 3 && !paused && !ended;
        self.frame_ms.make_contiguous();
        let (frames, _) = self.frame_ms.as_slices();
        let hits = &self.hits;
        let ctx = Ctx {
            g: &g,
            t,
            a: &self.anchors,
            words: &self.words,
            assets: &self.assets,
            ptr: self.ptr,
            hover: self.hover,
            orbit: &self.orbit,
            hits,
            fps: self.fps,
            frame_ms: frames,
            wall: wall_s,
            captions: self.captions,
            paused,
            muted: self.muted,
            ended,
            loading,
        };
        let content_a = if self.phase == Phase::Closing {
            1.0 - ez(E::EaseOutCubic, since_phase, 0.0, 0.35)
        } else {
            1.0
        };
        g.save();
        g.alpha(content_a);
        g.c.translate(sx as f64, sy as f64).ok();
        scenes::draw(&ctx);
        g.restore();
        g.save();
        g.alpha(content_a);
        chrome::draw(&ctx);
        if ctx.loading {
            chrome::loading_ring(&ctx, w / 2.0, h - 108.0);
        }
        g.restore();

        // ripples (pointer feedback) and a soft cursor halo
        let wall_ms = now;
        self.ripples.retain(|r| wall_ms - r.2 < 700.0);
        if !self.reduced {
            for r in &self.ripples {
                let p = ((wall_ms - r.2) / 700.0) as f32;
                g.stroke_circle(
                    r.0,
                    r.1,
                    8.0 + 46.0 * p,
                    &rgba(ACCENT, 0.5 * (1.0 - p)),
                    1.5,
                );
            }
        }
        g.vignette();

        let cursor = match self.hover {
            Some(Action::Orbit) => {
                if self.drag == Drag::Orbit {
                    "grabbing"
                } else {
                    "grab"
                }
            }
            Some(_) => "pointer",
            None => "default",
        };
        let _ = self.canvas.style().set_property("cursor", cursor);
        let _ = prog;
    }

    fn finish_close(&mut self) {
        if self.closed {
            return;
        }
        self.closed = true;
        let _ = self.audio.pause();
        self.canvas.remove();
        let _ = self.button.style().remove_property("visibility");
        let _ = self.button.class_list().add_1("is-returning");
        if let Some(body) = self.doc.body() {
            let _ = body.class_list().remove_1("intro-open");
        }
        for (name, cb) in self.listeners.drain(..) {
            let _ = self
                .win
                .remove_event_listener_with_callback(&name, cb.as_ref().unchecked_ref());
        }
        if let Some(id) = self.raf_id.take() {
            let _ = self.win.cancel_animation_frame(id);
        }
        self.raf = None;
        let _ = self.button.focus();
        if let Some(el) = self
            .doc
            .get_element_by_id("contact")
            .filter(|_| self.scroll_to_contact)
        {
            el.scroll_into_view();
        }
        ACTIVE.with(|a| *a.borrow_mut() = None);
    }
}

/// The play button's click: squeeze down, spring past full size, settle.
fn press(b: f32) -> f32 {
    if b < 0.3 {
        lerp(1.0, 0.88, E::EaseOutCubic.apply(b / 0.3))
    } else if b < 0.7 {
        lerp(0.88, 1.12, E::EaseOutCubic.apply((b - 0.3) / 0.4))
    } else {
        lerp(1.12, 1.0, E::EaseInOutCubic.apply((b - 0.7) / 0.3))
    }
}

type FrameCallback = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

fn start_loop(p: &Rc<RefCell<Player>>) {
    let Some(win) = web_sys::window() else { return };
    let slot: FrameCallback = Rc::new(RefCell::new(None));
    let (slot_in, player, win_in) = (slot.clone(), p.clone(), win.clone());
    *slot.borrow_mut() = Some(Closure::new(move |now: f64| {
        // try_borrow: if something else holds the player at this instant, skip the frame instead of panicking
        let again = match player.try_borrow_mut() {
            Ok(mut pl) => {
                if pl.closed {
                    false
                } else {
                    pl.frame(now);
                    !pl.closed
                }
            }
            Err(_) => true,
        };
        if again {
            if let Some(cb) = slot_in.borrow().as_ref() {
                let _ = win_in.request_animation_frame(cb.as_ref().unchecked_ref());
            }
        } else {
            slot_in.borrow_mut().take(); // loop finished: release the closure
        }
    }));
    if let Some(cb) = slot.borrow().as_ref() {
        let _ = win.request_animation_frame(cb.as_ref().unchecked_ref());
    }
}

fn on_pointer(
    p: &Rc<RefCell<Player>>,
    canvas: &HtmlCanvasElement,
    name: &str,
    f: impl Fn(&mut Player, PointerEvent) + 'static,
) {
    let rc = p.clone();
    let cb = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
        if let Ok(ev) = e.dyn_into::<PointerEvent>() {
            let url = match rc.try_borrow_mut() {
                Ok(mut p) => {
                    f(&mut p, ev);
                    p.pending_open.take()
                }
                Err(_) => None,
            };
            // opened after the borrow is released (a new tab can fire events right away)
            if let (Some(url), Some(win)) = (url, web_sys::window()) {
                let _ = win.open_with_url_and_target_and_features(
                    &url,
                    "_blank",
                    "noopener,noreferrer",
                );
            }
        }
    });
    let _ = canvas.add_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
    cb.forget(); // lives as long as the canvas element
}

/// Called by the page when the play button is clicked: the button grows into the full-screen video.
#[wasm_bindgen]
pub fn open_intro(button: HtmlElement, audio: HtmlAudioElement) -> Result<(), JsValue> {
    if ACTIVE.with(|a| a.borrow().is_some()) {
        return Ok(());
    }
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&JsValue::from_str(&format!("intro video stopped: {info}")));
    }));
    let win = web_sys::window().ok_or("no window")?;
    let doc = win.document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = doc.create_element("canvas")?.dyn_into()?;
    let st = canvas.style();
    st.set_property("position", "fixed")?;
    st.set_property("inset", "0")?;
    st.set_property("width", "100%")?;
    st.set_property("height", "100%")?;
    st.set_property("z-index", "2147483000")?;
    st.set_property("touch-action", "none")?;
    st.set_property("outline", "none")?;
    st.set_property("background", "#222220")?;
    canvas.set_attribute("role", "dialog")?;
    canvas.set_attribute("aria-modal", "true")?;
    canvas.set_attribute(
        "aria-label",
        "Intro video: press Space to pause, Escape to close",
    )?;
    canvas.set_attribute("tabindex", "0")?;
    canvas.set_class_name("intro-overlay");
    let c2d: CanvasRenderingContext2d = canvas
        .get_context("2d")?
        .ok_or("no 2d context")?
        .dyn_into()?;

    let rect = button.get_bounding_client_rect();
    let btn = (
        rect.left() as f32,
        rect.top() as f32,
        rect.width() as f32,
        rect.height() as f32,
    );
    let (w0, h0) = (
        win.inner_width()?.as_f64().unwrap_or(1280.0) as f32,
        win.inner_height()?.as_f64().unwrap_or(720.0) as f32,
    );
    st.set_property(
        "clip-path",
        &format!(
            "inset({:.1}px {:.1}px {:.1}px {:.1}px round {:.1}px)",
            btn.1,
            w0 - btn.0 - btn.2,
            h0 - btn.1 - btn.3,
            btn.0,
            btn.3 / 2.0
        ),
    )?;

    let words = Words::load();
    let (anchors, errs) = A::build(&words);
    if !errs.is_empty() {
        web_sys::console::warn_1(&JsValue::from_str(&format!(
            "intro video: {}",
            errs.join("; ")
        )));
    }
    let reduced = win
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|m| m.matches())
        .unwrap_or(false);
    let now = now_ms(&win);

    let player = Rc::new(RefCell::new(Player {
        win: win.clone(),
        doc: doc.clone(),
        canvas: canvas.clone(),
        c2d,
        button: button.clone(),
        audio,
        words,
        anchors,
        assets: Assets::new(),
        orbit: Orbit::new(),
        hits: RefCell::new(Vec::new()),
        phase: Phase::Opening,
        phase_t0: now,
        open_t0: now,
        audio_started: false,
        user_paused: false,
        captions: true,
        reduced,
        muted: false,
        override_t: None,
        ptr: Ptr::default(),
        hover: None,
        drag: Drag::None,
        drag_moved: false,
        ripples: Vec::new(),
        last_ms: 0.0,
        frame_ms: VecDeque::new(),
        fps: 60.0,
        size: (0.0, 0.0, 0.0),
        btn,
        ended_at: None,
        pending_open: None,
        scroll_to_contact: false,
        listeners: Vec::new(),
        raf: None,
        raf_id: None,
        closed: false,
    }));
    let _ = (w0, h0);

    // ---------------------------------------------------------------- pointer
    on_pointer(&player, &canvas, "pointermove", |p, e| {
        let (x, y) = (e.client_x() as f32, e.client_y() as f32);
        let (dx, dy) = (x - p.ptr.x, y - p.ptr.y);
        p.ptr.x = x;
        p.ptr.y = y;
        p.ptr.inside = true;
        p.hover = p.hit_test(x, y);
        match p.drag {
            Drag::Orbit => {
                p.orbit.drag(dx, dy);
                p.drag_moved = true;
            }
            Drag::Scrub => p.scrub_to(x),
            Drag::None => {}
        }
    });
    on_pointer(&player, &canvas, "pointerdown", |p, e| {
        let (x, y) = (e.client_x() as f32, e.client_y() as f32);
        p.ptr = Ptr {
            x,
            y,
            down: true,
            inside: true,
        };
        p.drag_moved = false;
        let _ = p.canvas.set_pointer_capture(e.pointer_id());
        let _ = p.canvas.focus();
        let hit = p.hit_test(x, y);
        if !p.reduced {
            let now = now_ms(&p.win);
            p.ripples.push((x, y, now));
        }
        match hit {
            Some(Action::Orbit) => {
                p.drag = Drag::Orbit;
                p.orbit.dragging = true;
            }
            Some(Action::Part(_)) | Some(Action::Scrub) => {
                p.drag = Drag::Scrub;
                p.scrub_to(x);
            }
            Some(a) => p.do_action(a, x),
            None => {}
        }
    });
    on_pointer(&player, &canvas, "pointerup", |p, e| {
        let (x, y) = (e.client_x() as f32, e.client_y() as f32);
        let was = p.drag;
        p.drag = Drag::None;
        p.orbit.dragging = false;
        p.ptr.down = false;
        let _ = p.canvas.release_pointer_capture(e.pointer_id());
        // a plain click on empty space toggles play / pause, like a video player
        if was == Drag::None && p.hit_test(x, y).is_none() {
            p.toggle_pause();
        }
        let _ = p.drag_moved;
    });
    on_pointer(&player, &canvas, "pointerleave", |p, _| {
        p.ptr.inside = false;
        p.hover = None;
    });

    // ---------------------------------------------------------------- keyboard
    {
        let rc = player.clone();
        let cb = Closure::<dyn FnMut(JsValue)>::new(move |e: JsValue| {
            let Ok(ev) = e.dyn_into::<KeyboardEvent>() else {
                return;
            };
            let Ok(mut p) = rc.try_borrow_mut() else {
                return;
            };
            if p.phase == Phase::Closing {
                return;
            }
            let t = p.video_time();
            let handled = match ev.key().as_str() {
                "Escape" => {
                    p.begin_close(false);
                    true
                }
                " " | "k" | "K" => {
                    p.toggle_pause();
                    true
                }
                "ArrowRight" => {
                    p.seek(t + if ev.shift_key() { 15.0 } else { 5.0 });
                    true
                }
                "ArrowLeft" => {
                    p.seek(t - if ev.shift_key() { 15.0 } else { 5.0 });
                    true
                }
                "m" | "M" => {
                    p.muted = !p.muted;
                    let m = p.muted;
                    p.audio.set_muted(m);
                    true
                }
                "c" | "C" => {
                    p.captions = !p.captions;
                    true
                }
                k if k.len() == 1 && ("1"..="6").contains(&k) => {
                    let i = k.parse::<usize>().unwrap_or(1) - 1;
                    let s = p.words.parts[i].0;
                    p.seek(s - 0.1);
                    true
                }
                _ => false,
            };
            if handled {
                ev.prevent_default();
            }
        });
        win.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())?;
        player.borrow_mut().listeners.push(("keydown".into(), cb));
    }

    // ---------------------------------------------------------------- go
    doc.body().ok_or("no body")?.append_child(&canvas)?;
    if let Some(body) = doc.body() {
        let _ = body.class_list().add_1("intro-open");
    }
    button.style().set_property("visibility", "hidden")?;
    let _ = canvas.focus();
    ACTIVE.with(|a| *a.borrow_mut() = Some(player.clone()));
    start_loop(&player);
    Ok(())
}

/// Stops and removes the video (used by the page if it needs to).
#[wasm_bindgen]
pub fn close_intro() {
    ACTIVE.with(|a| {
        if let Some(p) = a.borrow().as_ref() {
            p.borrow_mut().begin_close(false);
        }
    });
}

/// Test / tooling hook: freeze the video clock at `t` seconds (the narration is paused).
#[wasm_bindgen]
pub fn intro_debug_time(t: f32) {
    ACTIVE.with(|a| {
        if let Some(p) = a.borrow().as_ref() {
            let mut p = p.borrow_mut();
            p.override_t = Some(t);
            p.audio_started = true;
            p.phase = Phase::Running;
            let _ = p.canvas.style().set_property("clip-path", "none");
            let _ = p.audio.pause();
        }
    });
}

/// Test / tooling hook: put the pointer at (x, y) as if the mouse moved there.
#[wasm_bindgen]
pub fn intro_debug_pointer(x: f32, y: f32) {
    ACTIVE.with(|a| {
        if let Some(p) = a.borrow().as_ref() {
            let mut p = p.borrow_mut();
            p.ptr = Ptr {
                x,
                y,
                down: false,
                inside: true,
            };
            p.hover = p.hit_test(x, y);
        }
    });
}

/// Test / tooling hook: the clickable regions of the last frame as JSON ([{"a": "Contact", "x":..,"y":..,"w":..,"h":..}]).
#[wasm_bindgen]
pub fn intro_debug_hits() -> String {
    ACTIVE.with(|a| {
        a.borrow()
            .as_ref()
            .map(|p| {
                let p = p.borrow();
                let items: Vec<String> = p
                    .hits
                    .borrow()
                    .iter()
                    .map(|h| {
                        format!(
                            "{{\"a\":\"{:?}\",\"x\":{:.1},\"y\":{:.1},\"w\":{:.1},\"h\":{:.1}}}",
                            h.action, h.x, h.y, h.w, h.h
                        )
                    })
                    .collect();
                format!("[{}]", items.join(","))
            })
            .unwrap_or_else(|| "[]".into())
    })
}

/// Test / tooling hook: "video_time,audio_time,paused,started,ready_state".
#[wasm_bindgen]
pub fn intro_debug_clock() -> String {
    ACTIVE.with(|a| {
        a.borrow()
            .as_ref()
            .map(|p| {
                let p = p.borrow();
                format!(
                    "{:.2},{:.2},{},{},{}",
                    p.video_time(),
                    p.audio.current_time(),
                    p.audio.paused(),
                    p.audio_started,
                    p.audio.ready_state()
                )
            })
            .unwrap_or_default()
    })
}
