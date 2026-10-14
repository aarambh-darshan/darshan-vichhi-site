// The static-site generator (content, models, render) runs natively. The intro video (src/video) is
// the same crate built for wasm32, so the site generator is left out of that build.
#[cfg(not(target_arch = "wasm32"))]
pub mod content;
#[cfg(not(target_arch = "wasm32"))]
pub mod models;
#[cfg(not(target_arch = "wasm32"))]
pub mod render;

pub mod video;
