//! Motion helpers. All time-based values are pure functions of the video clock, so scrubbing,
//! pausing and replaying are exact. Easing, tweening and springs come from Animato.
use animato::{Easing, Spring, SpringConfig, Tween, Update};

pub use animato::Easing as E;

#[inline]
pub fn clamp01(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
/// Linear progress 0..1 of `t` through [t0, t0 + dur].
#[inline]
pub fn prog(t: f32, t0: f32, dur: f32) -> f32 {
    ((t - t0) / dur.max(1e-4)).clamp(0.0, 1.0)
}
/// Eased progress (Animato easing) of `t` through [t0, t0 + dur].
#[inline]
pub fn ez(e: Easing, t: f32, t0: f32, dur: f32) -> f32 {
    e.apply(prog(t, t0, dur))
}
/// An Animato `Tween<f32>` sampled at normalised time `p` (so it can be scrubbed).
pub fn tween(from: f32, to: f32, e: Easing, p: f32) -> f32 {
    let mut tw = Tween::new(from, to).duration(1.0).easing(e).build();
    tw.seek(clamp01(p));
    tw.value()
}
/// Smooth fade window: 0 before `a`, rises to 1 over `fade_in`, falls after `b` over `fade_out`.
pub fn window(t: f32, a: f32, b: f32, fade_in: f32, fade_out: f32) -> f32 {
    let i = ez(Easing::EaseOutCubic, t, a, fade_in);
    let o = 1.0 - ez(Easing::EaseInCubic, t, b, fade_out);
    (i * o).clamp(0.0, 1.0)
}
#[allow(dead_code)]
/// A bell that peaks at `t0` and decays: used for flashes and thumps.
pub fn pulse(t: f32, t0: f32, dur: f32) -> f32 {
    if t < t0 {
        0.0
    } else {
        (1.0 - prog(t, t0, dur)).powi(2)
    }
}
/// Damped shake: returns -1..1.
pub fn shake(t: f32, t0: f32, dur: f32, freq: f32) -> f32 {
    if t < t0 || t > t0 + dur {
        return 0.0;
    }
    let p = (t - t0) / dur;
    (1.0 - p).powi(2) * (p * dur * freq * std::f32::consts::TAU).sin()
}
#[allow(dead_code)]
/// Cheap deterministic noise in 0..1.
pub fn hash(i: u32) -> f32 {
    let mut x = i.wrapping_mul(0x9E37_79B1) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}

/// A spring step response (0 -> 1) sampled once from Animato's `Spring`, then looked up by elapsed time.
/// Springs are stateful, so this keeps them scrubbable.
pub struct SpringCurve {
    v: Vec<f32>,
    dt: f32,
}
impl SpringCurve {
    pub fn new(cfg: SpringConfig, secs: f32) -> Self {
        let dt = 1.0 / 240.0;
        let mut s = Spring::new(cfg);
        s.set_target(1.0);
        let n = (secs / dt) as usize;
        let mut v = Vec::with_capacity(n + 1);
        v.push(0.0);
        for _ in 0..n {
            s.update(dt);
            v.push(s.position());
        }
        SpringCurve { v, dt }
    }
    pub fn at(&self, elapsed: f32) -> f32 {
        if elapsed <= 0.0 {
            return 0.0;
        }
        let f = elapsed / self.dt;
        let i = f as usize;
        if i + 1 >= self.v.len() {
            return *self.v.last().unwrap_or(&1.0);
        }
        lerp(self.v[i], self.v[i + 1], f - i as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spring_settles_near_one() {
        let c = SpringCurve::new(SpringConfig::wobbly(), 3.0);
        assert!((c.at(3.0) - 1.0).abs() < 0.05);
        assert_eq!(c.at(-1.0), 0.0);
    }
    #[test]
    fn tween_endpoints() {
        assert!((tween(0.0, 10.0, Easing::EaseOutCubic, 1.0) - 10.0).abs() < 1e-3);
        assert!(tween(0.0, 10.0, Easing::EaseOutCubic, 0.0).abs() < 1e-3);
    }
}
