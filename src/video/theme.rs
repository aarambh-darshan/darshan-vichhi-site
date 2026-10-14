//! The portfolio's own palette and type (assets/styles.css): a calm editorial dark theme.
pub type Rgb = (u8, u8, u8);

pub const BG: Rgb = (0x1b, 0x1b, 0x1a);
pub const SURFACE: Rgb = (0x22, 0x22, 0x20);
pub const SURFACE_HI: Rgb = (0x2a, 0x2a, 0x27);
pub const BORDER: Rgb = (0x34, 0x34, 0x31);
pub const TEXT: Rgb = (0xec, 0xe8, 0xdf);
pub const TEXT2: Rgb = (0xaa, 0xa7, 0x9f);
pub const MUTED: Rgb = (0x77, 0x74, 0x6e);
pub const ACCENT: Rgb = (0xae, 0xbc, 0xf4);
/// One quiet warm tone for the jokes (compiler errors, "complaints"): never used for chrome.
pub const WARM: Rgb = (0xd9, 0xa3, 0x8f);

pub const SERIF: &str =
    "\"Iowan Old Style\", \"Palatino Linotype\", Palatino, \"Book Antiqua\", Georgia, serif";
pub const SANS: &str =
    "ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", Roboto, Helvetica, Arial, sans-serif";
pub const MONO: &str =
    "ui-monospace, SFMono-Regular, Menlo, Consolas, \"Liberation Mono\", monospace";

pub fn rgba(c: Rgb, a: f32) -> String {
    format!("rgba({},{},{},{:.3})", c.0, c.1, c.2, a.clamp(0.0, 1.0))
}
