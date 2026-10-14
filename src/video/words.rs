//! Word-level timing of the narration (data/words.json, built by scripts/build_words.py).
use serde::Deserialize;

#[derive(Deserialize)]
struct RawLine {
    p: usize,
    s: f32,
    e: f32,
    w: Vec<(String, f32, f32)>,
}
#[derive(Deserialize)]
struct RawPart {
    s: f32,
    e: f32,
}
#[derive(Deserialize)]
struct Raw {
    duration: f32,
    parts: Vec<RawPart>,
    lines: Vec<RawLine>,
}

pub struct Word {
    pub text: String,
    pub s: f32,
    pub e: f32,
    norm: String,
}
pub struct Line {
    pub part: usize,
    pub s: f32,
    pub e: f32,
    pub words: Vec<Word>,
}
pub struct Words {
    pub duration: f32,
    /// (start, end) of each "### Part N" of the script, in seconds.
    pub parts: Vec<(f32, f32)>,
    pub lines: Vec<Line>,
}

/// Lower-case alphanumerics only: "Open-source," -> "opensource".
pub fn norm(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

impl Words {
    pub fn load() -> Words {
        Self::parse(include_str!("data/words.json"))
    }

    pub fn parse(json: &str) -> Words {
        let raw: Raw = serde_json::from_str(json).expect("data/words.json is invalid");
        Words {
            duration: raw.duration,
            parts: raw.parts.iter().map(|p| (p.s, p.e)).collect(),
            lines: raw
                .lines
                .into_iter()
                .map(|l| Line {
                    part: l.p,
                    s: l.s,
                    e: l.e,
                    words: l
                        .w
                        .into_iter()
                        .map(|(t, s, e)| Word {
                            norm: norm(&t),
                            text: t,
                            s,
                            e,
                        })
                        .collect(),
                })
                .collect(),
        }
    }

    /// Start time of the `nth` (1-based) place inside `part` where the words of `phrase` are spoken in a row.
    pub fn find(&self, part: usize, phrase: &str, nth: usize) -> Option<f32> {
        let toks: Vec<String> = phrase.split_whitespace().map(norm).collect();
        if toks.is_empty() {
            return None;
        }
        let ws: Vec<&Word> = self
            .lines
            .iter()
            .filter(|l| l.part == part)
            .flat_map(|l| l.words.iter())
            .collect();
        let mut seen = 0;
        for k in 0..ws.len() {
            if k + toks.len() <= ws.len()
                && toks.iter().enumerate().all(|(j, t)| ws[k + j].norm == *t)
            {
                seen += 1;
                if seen == nth {
                    return Some(ws[k].s);
                }
            }
        }
        None
    }

    /// Index of the part playing at time `t` (clamped).
    pub fn part_at(&self, t: f32) -> usize {
        let mut idx = 0;
        for (i, (s, _)) in self.parts.iter().enumerate() {
            if t >= *s - 0.2 {
                idx = i;
            }
        }
        idx
    }

    /// The sentence being spoken (or about to be) at `t`.
    pub fn line_at(&self, t: f32) -> Option<usize> {
        let mut idx = None;
        for (i, l) in self.lines.iter().enumerate() {
            if t >= l.s - 0.3 {
                idx = Some(i);
            }
        }
        idx
    }
}
