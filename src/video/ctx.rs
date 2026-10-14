//! Shared per-frame context handed to every scene.
use crate::video::anchors::A;
use crate::video::fx::SpringCurve;
use crate::video::g2d::G;
use crate::video::wire::{Orbit, Wire};
use crate::video::words::Words;
use animato::SpringConfig;
use std::cell::RefCell;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Action {
    Close,
    PlayPause,
    Mute,
    Captions,
    Replay,
    Contact,
    /// A link from LINKS.
    Link(usize),
    /// A project from PROJECTS.
    Project(usize),
    /// Jump to a part (0-based).
    Part(usize),
    /// Drag the scrubber.
    Scrub,
    /// Drag to rotate the 3D object.
    Orbit,
}

pub struct Hit {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub action: Action,
}

/// (label, subtitle, url)
pub const LINKS: [(&str, &str, &str); 6] = [
    (
        "GitHub",
        "aarambh-darshan",
        "https://github.com/aarambh-darshan",
    ),
    (
        "YouTube",
        "@AarambhDevHub",
        "https://www.youtube.com/@AarambhDevHub",
    ),
    (
        "Discord",
        "join the community",
        "https://discord.gg/HDth6PfCnp",
    ),
    (
        "LinkedIn",
        "Darshan Vichhi",
        "https://www.linkedin.com/in/darshan-vichhi-rust-developer",
    ),
    (
        "Medium",
        "@aarambhdevhub",
        "https://medium.com/@aarambhdevhub",
    ),
    (
        "AarambhDevHub",
        "GitHub org",
        "https://github.com/AarambhDevHub",
    ),
];
/// Projects named in the speech: (name, url)
pub const PROJECTS: [(&str, &str); 4] = [
    ("Animato", "https://github.com/AarambhDevHub/animato"),
    ("Scenix", "https://github.com/AarambhDevHub/scenix"),
    ("Aarambh AI", "https://github.com/AarambhDevHub/aarambh-ai"),
    (
        "TypeBridge",
        "https://github.com/aarambh-darshan/typewriter",
    ),
];
pub const PART_NAMES: [&str; 6] = [
    "Hello",
    "Who I am",
    "Aarambh Dev Hub",
    "Projects",
    "The secret",
    "The deal",
];

#[allow(dead_code)] // `orb`, `cube` and `snap` are ready for new scenes
pub struct Assets {
    pub diamond: Wire,
    pub knot: Wire,
    pub orb: Wire,
    pub cube: Wire,
    pub pop: SpringCurve,
    pub snap: SpringCurve,
    pub soft: SpringCurve,
}
impl Assets {
    pub fn new() -> Self {
        Assets {
            diamond: Wire::diamond(),
            knot: Wire::knot(),
            orb: Wire::orb(),
            cube: Wire::cube(),
            pop: SpringCurve::new(SpringConfig::wobbly(), 3.0),
            snap: SpringCurve::new(SpringConfig::snappy(), 2.0),
            soft: SpringCurve::new(SpringConfig::gentle(), 3.0),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub struct Ptr {
    pub x: f32,
    pub y: f32,
    pub down: bool,
    pub inside: bool,
}

#[allow(dead_code)]
pub struct Ctx<'a> {
    pub g: &'a G,
    /// Video clock in seconds (negative while the page is opening).
    pub t: f32,
    pub a: &'a A,
    pub words: &'a Words,
    pub assets: &'a Assets,
    pub ptr: Ptr,
    pub hover: Option<Action>,
    pub orbit: &'a Orbit,
    pub hits: &'a RefCell<Vec<Hit>>,
    pub fps: f32,
    pub frame_ms: &'a [f32],
    /// Wall-clock seconds (keeps ambient motion alive while paused).
    pub wall: f32,
    pub captions: bool,
    pub paused: bool,
    pub muted: bool,
    pub ended: bool,
    pub loading: bool,
}

#[allow(dead_code)]
impl<'a> Ctx<'a> {
    pub fn u(&self) -> f32 {
        self.g.u
    }
    pub fn cx(&self) -> f32 {
        self.g.w / 2.0
    }
    /// Margin from the viewport edge.
    pub fn pad(&self) -> f32 {
        (self.g.w * 0.05).clamp(16.0, 56.0)
    }
    /// Top of the stage (below the header).
    pub fn top(&self) -> f32 {
        if self.g.compact {
            84.0
        } else {
            100.0 * self.g.u.min(1.2)
        }
    }
    /// Bottom of the stage (above captions and controls).
    pub fn bottom(&self) -> f32 {
        let cap = if self.g.compact {
            190.0
        } else {
            215.0 * self.g.u.clamp(0.8, 1.2)
        };
        self.g.h - cap
    }
    pub fn cy(&self) -> f32 {
        (self.top() + self.bottom()) / 2.0
    }
    pub fn stage_h(&self) -> f32 {
        self.bottom() - self.top()
    }
    pub fn hit(&self, x: f32, y: f32, w: f32, h: f32, action: Action) {
        self.hits.borrow_mut().push(Hit { x, y, w, h, action });
    }
    pub fn hovering(&self, action: Action) -> bool {
        self.hover == Some(action)
    }
}
