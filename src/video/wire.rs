//! Scenix drives the 3D in this video: geometry generators, the perspective camera and the math types
//! (Vec3, Quat, Mat4). Everything is projected on the CPU and drawn as crisp wireframe lines on the
//! same canvas as the typography, so the whole video is one tiny WebAssembly module with no GPU needed.
use crate::video::g2d::G;
use crate::video::theme::{Rgb, rgba};
use scenix::{
    Geometry, Mat4, PerspectiveCamera, Quat, Vec3, Vec4, box_geometry, cone_geometry,
    icosphere_geometry, torus_knot_geometry,
};
use std::collections::{BTreeMap, HashMap};

pub struct Wire {
    pub verts: Vec<Vec3>,
    pub edges: Vec<(u32, u32)>,
}

impl Wire {
    /// Welds vertices by position and keeps every edge (`crease = None`, organic shapes) or only the
    /// edges whose neighbouring faces bend by more than `crease` degrees (hard-edged shapes).
    pub fn from_geometry(g: &Geometry, crease: Option<f32>) -> Wire {
        let mut map: HashMap<(i32, i32, i32), u32> = HashMap::new();
        let mut verts: Vec<Vec3> = Vec::new();
        let mut remap = vec![0u32; g.positions.len()];
        for (i, p) in g.positions.iter().enumerate() {
            let key = (
                (p.x * 1000.0).round() as i32,
                (p.y * 1000.0).round() as i32,
                (p.z * 1000.0).round() as i32,
            );
            let id = *map.entry(key).or_insert_with(|| {
                verts.push(*p);
                (verts.len() - 1) as u32
            });
            remap[i] = id;
        }
        let mut edge_faces: BTreeMap<(u32, u32), Vec<Vec3>> = BTreeMap::new();

        for tri in g.indices.as_chunks::<3>().0 {
            let ids = [
                remap[tri[0] as usize],
                remap[tri[1] as usize],
                remap[tri[2] as usize],
            ];
            let a = verts[ids[0] as usize];
            let b = verts[ids[1] as usize];
            let c = verts[ids[2] as usize];
            let ab = Vec3::new(b.x - a.x, b.y - a.y, b.z - a.z);
            let ac = Vec3::new(c.x - a.x, c.y - a.y, c.z - a.z);
            let n = Vec3::new(
                ab.y * ac.z - ab.z * ac.y,
                ab.z * ac.x - ab.x * ac.z,
                ab.x * ac.y - ab.y * ac.x,
            );
            let len = (n.x * n.x + n.y * n.y + n.z * n.z).sqrt();
            if len < 1e-8 {
                continue;
            }
            let n = Vec3::new(n.x / len, n.y / len, n.z / len);
            for (i, j) in [(0, 1), (1, 2), (2, 0)] {
                let (x, y) = (ids[i], ids[j]);
                if x != y {
                    edge_faces
                        .entry(if x < y { (x, y) } else { (y, x) })
                        .or_default()
                        .push(n);
                }
            }
        }
        let cos_limit = crease.map(|d| (d.to_radians()).cos());
        let edges = edge_faces
            .into_iter()
            .filter(|(_, faces)| match cos_limit {
                None => true,
                Some(limit) => {
                    faces.len() == 1
                        || faces
                            .windows(2)
                            .any(|f| f[0].x * f[1].x + f[0].y * f[1].y + f[0].z * f[1].z < limit)
                }
            })
            .map(|(e, _)| e)
            .collect();
        Wire { verts, edges }
    }

    /// A brilliant-ish diamond: two eight-sided cones base to base.
    pub fn diamond() -> Wire {
        let mut top = cone_geometry(1.0, 0.9, 8, 1);
        let mut bottom = cone_geometry(1.0, 1.3, 8, 1);
        // cone: apex +y. Flip the lower one so the apex points down, then offset both from y = 0.
        for p in top.positions.iter_mut() {
            p.y += 0.45;
        }
        for p in bottom.positions.iter_mut() {
            p.y = -p.y - 0.65;
        }
        let mut g = Geometry::new();
        let base = g.positions.len() as u32;
        g.positions.extend(top.positions.iter().copied());
        g.indices.extend(top.indices.iter().copied());
        let off = base + top.positions.len() as u32;
        g.positions.extend(bottom.positions.iter().copied());
        g.indices.extend(bottom.indices.iter().map(|i| i + off));
        Wire::from_geometry(&g, Some(12.0))
    }
    pub fn knot() -> Wire {
        Wire::from_geometry(&torus_knot_geometry(1.0, 0.28, 90, 7, 2, 3), None)
    }
    pub fn orb() -> Wire {
        Wire::from_geometry(&icosphere_geometry(1.0, 2), None)
    }
    pub fn cube() -> Wire {
        Wire::from_geometry(&box_geometry(1.6, 1.6, 1.6, 1, 1, 1), Some(20.0))
    }

    /// Draws the wireframe into `rect` (x, y, w, h). Nearer edges are brighter.
    #[allow(clippy::too_many_arguments)]
    pub fn draw(
        &self,
        g: &G,
        model: Mat4,
        cam: &PerspectiveCamera,
        rect: (f32, f32, f32, f32),
        color: Rgb,
        alpha: f32,
        lw: f32,
    ) {
        if alpha <= 0.002 {
            return;
        }
        let mvp = cam.view_projection() * model;
        let (rx, ry, rw, rh) = rect;
        let mut pts: Vec<Option<(f32, f32, f32)>> = Vec::with_capacity(self.verts.len());
        let (mut dmin, mut dmax) = (f32::MAX, f32::MIN);
        for v in &self.verts {
            let c = mvp.mul_vec4(Vec4::new(v.x, v.y, v.z, 1.0));
            if c.w <= 1e-4 {
                pts.push(None);
                continue;
            }
            let (nx, ny, nz) = (c.x / c.w, c.y / c.w, c.z / c.w);
            dmin = dmin.min(nz);
            dmax = dmax.max(nz);
            pts.push(Some((
                rx + (nx * 0.5 + 0.5) * rw,
                ry + (1.0 - (ny * 0.5 + 0.5)) * rh,
                nz,
            )));
        }
        let span = (dmax - dmin).max(1e-5);
        const BUCKETS: usize = 4;
        let mut paths: Vec<Vec<(f32, f32, f32, f32)>> = vec![Vec::new(); BUCKETS];
        for &(a, b) in &self.edges {
            if let (Some(p), Some(q)) = (pts[a as usize], pts[b as usize]) {
                let d = ((p.2 + q.2) * 0.5 - dmin) / span; // 0 = near
                let bi = ((1.0 - d) * BUCKETS as f32)
                    .min(BUCKETS as f32 - 1.0)
                    .max(0.0) as usize;
                paths[bi].push((p.0, p.1, q.0, q.1));
            }
        }
        g.c.set_line_cap("round");
        g.c.set_line_width(lw as f64);
        for (bi, segs) in paths.iter().enumerate() {
            if segs.is_empty() {
                continue;
            }
            let near = (bi as f32 + 0.5) / BUCKETS as f32;
            g.c.set_stroke_style_str(&rgba(color, alpha * (0.22 + 0.78 * near)));
            g.c.begin_path();
            for s in segs {
                g.c.move_to(s.0 as f64, s.1 as f64);
                g.c.line_to(s.2 as f64, s.3 as f64);
            }
            g.c.stroke();
        }
        // vertices as tiny nodes on the near half
        g.c.set_fill_style_str(&rgba(color, alpha * 0.9));
        for p in pts.iter().flatten() {
            let d = (p.2 - dmin) / span;
            if d < 0.5 {
                g.c.begin_path();
                let _ = g.c.arc(
                    p.0 as f64,
                    p.1 as f64,
                    (lw * 0.9) as f64,
                    0.0,
                    std::f64::consts::TAU,
                );
                g.c.fill();
            }
        }
    }
}

/// Rotation state that can be dragged with the pointer and coasts to a stop.
pub struct Orbit {
    pub yaw: f32,
    pub pitch: f32,
    vyaw: f32,
    vpitch: f32,
    pub dragging: bool,
}
impl Orbit {
    pub fn new() -> Self {
        Orbit {
            yaw: 0.6,
            pitch: 0.35,
            vyaw: 0.0,
            vpitch: 0.0,
            dragging: false,
        }
    }
    pub fn drag(&mut self, dx: f32, dy: f32) {
        self.yaw += dx * 0.011;
        self.pitch = (self.pitch + dy * 0.011).clamp(-1.3, 1.3);
        self.vyaw = dx * 0.011 * 60.0;
        self.vpitch = dy * 0.011 * 60.0;
    }
    pub fn step(&mut self, dt: f32) {
        if !self.dragging {
            self.yaw += self.vyaw * dt;
            self.pitch = (self.pitch + self.vpitch * dt).clamp(-1.3, 1.3);
            let k = (-3.0 * dt).exp();
            self.vyaw *= k;
            self.vpitch *= k;
        }
    }
    /// Model matrix: user orbit, plus an automatic spin `auto` (radians) around Y.
    pub fn model(&self, auto: f32, scale: f32, pos: Vec3) -> Mat4 {
        let q = Quat::from_axis_angle(Vec3::X, self.pitch)
            .mul_quat(Quat::from_axis_angle(Vec3::Y, self.yaw + auto));
        Mat4::from_translation(pos)
            * Mat4::from_rotation(q)
            * Mat4::from_scale(Vec3::new(scale, scale, scale))
    }
}
impl Default for Orbit {
    fn default() -> Self {
        Self::new()
    }
}

pub fn camera(aspect: f32, distance: f32) -> PerspectiveCamera {
    PerspectiveCamera::new(34.0, aspect.max(0.2), 0.1, 100.0)
        .position(Vec3::new(0.0, 0.0, distance))
        .target(Vec3::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shapes_have_geometry() {
        for w in [Wire::diamond(), Wire::knot(), Wire::orb(), Wire::cube()] {
            assert!(w.verts.len() > 4 && w.edges.len() > 6);
        }
    }
    #[test]
    fn cube_keeps_only_hard_edges() {
        let c = Wire::cube();
        assert_eq!(c.verts.len(), 8);
        assert_eq!(c.edges.len(), 12);
    }
}
