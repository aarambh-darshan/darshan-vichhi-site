//! Every moment of the video that is tied to a spoken word. `A::build` resolves each one against the
//! word timings; `cargo test` fails if a word is missing (for example after the script is edited).
use crate::video::words::Words;

macro_rules! anchors {
    ($($name:ident: $part:expr, $phrase:expr, $nth:expr;)*) => {
        #[derive(Clone, Debug)]
        pub struct A { $(pub $name: f32,)* }
        impl A {
            /// Returns the anchors and the list of problems (empty when everything resolves).
            pub fn build(w: &Words) -> (A, Vec<String>) {
                let mut errs: Vec<String> = Vec::new();
                $(
                    let $name = match w.find($part, $phrase, $nth) {
                        Some(t) => t,
                        None => { errs.push(format!("part {}: \"{}\" (#{}) is not spoken", $part + 1, $phrase, $nth)); w.parts[$part].0 }
                    };
                )*
                (A { $($name,)* }, errs)
            }
        }
    };
}

anchors! {
    // ---- part 1: the entrance
    hi: 0, "Hiii", 1;
    respect: 0, "Respect", 1;
    most: 0, "Most people", 1;
    trap: 0, "trap", 1;
    fun: 0, "fun one", 1;
    name: 0, "Darshan", 1;
    rust_dev: 0, "Rust developer", 1;
    oss: 0, "Open-source", 1;
    youtuber: 0, "YouTuber", 1;
    certified: 0, "certified", 1;
    works: 0, "works on my machine", 1;
    // ---- part 2: who I am
    surat: 1, "Surat", 1;
    before_rust: 1, "Before Rust", 1;
    then_met: 1, "Then I met", 1;
    diamonds: 1, "diamonds", 1;
    sarees: 1, "sarees", 1;
    food: 1, "food", 1;
    complaint: 1, "complaint", 1;
    byday: 1, "By day", 1;
    argue: 1, "argue", 1;
    lose: 1, "lose", 1;
    every: 1, "Every", 1;
    single: 1, "Single", 1;
    time: 1, "Time", 1;
    borrow: 1, "Borrow checker", 1;
    one: 1, "one", 1;
    zero: 1, "zero", 1;
    frontend: 1, "frontend", 1;
    ownership: 1, "ownership", 1;
    lifetimes: 1, "lifetimes", 1;
    rules: 1, "rules", 1;
    strict: 1, "Strict", 1;
    crab: 1, "crab", 1;
    // ---- part 3: Aarambh Dev Hub
    aarambh: 2, "Aarambh", 1;
    youtube: 2, "YouTube", 1;
    github: 2, "GitHub", 1;
    discord: 2, "Discord", 1;
    five: 2, "five", 1;
    purpose: 2, "purpose", 1;
    forced: 2, "forced", 1;
    checked: 2, "checked", 1;
    teach: 2, "teach", 1;
    future: 2, "future", 1;
    thirty: 2, "thirty-seven", 1;
    sqlx: 2, "S-Q-L-X", 1;
    crore: 2, "crore", 1;
    billion: 2, "billion", 1;
    please_stop: 2, "stop", 1;
    more_query: 2, "one more", 1;
    // ---- part 4: the speed run
    seatbelts: 3, "Seatbelts", 1;
    animato: 3, "Animato", 1;
    smoothly: 3, "smoothly", 1;
    laggy: 3, "laggy", 1;
    scenix: 3, "Scenix", 1;
    wgpu: 3, "W-G-P-U", 1;
    threejs: 3, "Three J S", 1;
    nobody: 3, "Nobody", 1;
    passion: 3, "passion", 1;
    impulse: 3, "impulse", 1;
    ai: 3, "Aarambh", 1;
    scratch: 3, "scratch", 1;
    no_python: 3, "No Python", 1;
    i3: 3, "Intel", 1;
    eight: 3, "eight", 1;
    no_gpu: 3, "graphics", 1;
    kaggle: 3, "Kaggle", 1;
    ferrari: 3, "Ferrari", 1;
    typebridge: 3, "typebridge", 1;
    typescript: 3, "TypeScript", 1;
    python: 3, "Python", 2;
    go_lang: 3, "Go", 1;
    swift: 3, "Swift", 1;
    kotlin: 3, "Kotlin", 1;
    messaging: 3, "messaging", 1;
    // ---- part 5: the tiny secret
    secret: 4, "secret", 1;
    page: 4, "page", 1;
    leptos: 4, "Leptos", 1;
    watching: 4, "watching", 1;
    powered: 4, "Powered", 1;
    mine: 4, "mine", 1;
    smooth: 4, "smooth", 1;
    genius: 4, "genius", 1;
    lags: 4, "lags", 1;
    jealous: 4, "jealous", 1;
    // ---- part 6: the deal
    deal: 5, "deal", 1;
    engineer: 5, "engineer", 1;
    llm: 5, "L-L-M", 1;
    lifetimes2: 5, "lifetimes", 1;
    guy: 5, "guy", 1;
    scroll: 5, "Scroll", 1;
    links_github: 5, "GitHub", 1;
    links_youtube: 5, "YouTube", 1;
    links_discord: 5, "Discord", 1;
    contact: 5, "contact", 1;
    fast: 5, "fast", 1;
    compiler: 5, "compiler", 1;
    thanks: 5, "Thanks", 1;
    happy: 5, "happy", 1;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_anchor_is_spoken() {
        let w = Words::load();
        let (_, errs) = A::build(&w);
        assert!(errs.is_empty(), "unresolved anchors:\n{}", errs.join("\n"));
    }
    #[test]
    fn six_parts_in_order() {
        let w = Words::load();
        assert_eq!(w.parts.len(), 6);
        assert!(w.parts.windows(2).all(|p| p[0].1 <= p[1].0 + 0.01));
        assert!(w.duration > 170.0);
    }
}
