#!/usr/bin/env python3
"""Builds src/video/data/words.json from the narration script and the ElevenLabs/Deepgram word timings.

    python3 scripts/build_words.py --audio-json path/to/ElevenLabs_....json

* src/video/data/script.md : the narration, ### Part N blocks, [audio tags] allowed (they are stripped here).
* The word timings come from a Deepgram-style JSON (results.channels[0].alternatives[0].words).
* The on-screen words use the SCRIPT's spelling and punctuation; their times come from aligning the script
  to the transcript (heard "scenics" -> shown "Scenix"). Words the transcriber missed are interpolated.

Output (compact): {"duration", "parts":[{s,e}], "lines":[{p, s, e, w:[[word, start, end], ...]}]}
"""
import argparse, difflib, json, re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ap = argparse.ArgumentParser()
ap.add_argument("--audio-json", required=True)
ap.add_argument("--script", default=str(ROOT / "src/video/data/script.md"))
ap.add_argument("--out", default=str(ROOT / "src/video/data/words.json"))
args = ap.parse_args()

# ------------------------------------------------------------------ script
parts, cur = [], None
for raw in Path(args.script).read_text(encoding="utf-8").splitlines():
    s = raw.strip()
    if s.startswith("### Part"):
        cur = []; parts.append(cur)
    elif s and cur is not None and not s.startswith("#"):
        cur.append(s)
tokens = []  # (display, part)
for pi, lines in enumerate(parts):
    text = " ".join(lines)
    text = re.sub(r"\[[^\]]*\]", " ", text)           # [tags]
    text = text.replace("\u2026", "... ").replace("\u2014", " ")
    for d in text.split():
        d = d.strip()
        if re.fullmatch(r"[.\-]+", d):                 # a lone "..." from "word... word"
            continue
        tokens.append((d, pi))

# spoken forms: how the TTS read what is written (so alignment matches the transcript)
SPOKEN = {"s-q-l-x": "sql x", "w-g-p-u": "wgpu", "a-i": "ai", "l-l-m": "llm", "three": "3", "j": "j", "s": "s"}
def pieces(d):
    low = d.lower().strip(".,!?\"'\u201c\u201d:;()")
    low = re.sub(r"(.)\1{2,}", r"\1", low)               # hiii -> hi
    if low in SPOKEN and "-" in low: return SPOKEN[low].split()
    return re.findall(r"[a-z0-9]+", low)

# ------------------------------------------------------------------ transcript
data = json.loads(Path(args.audio_json).read_text(encoding="utf-8"))
tw = data["results"]["channels"][0]["alternatives"][0]["words"]
duration = float(data["metadata"]["duration"])
S, S_own = [], []
for i, (d, _) in enumerate(tokens):
    for p in pieces(d):
        S.append(p); S_own.append(i)
T, T_own = [], []
for j, w in enumerate(tw):
    for p in re.findall(r"[a-z0-9]+", w["word"].lower()):
        T.append(p); T_own.append(j)
sm = difflib.SequenceMatcher(None, S, T, autojunk=False)
N = len(tokens)
t0 = [None] * N; t1 = [None] * N; matched = 0
for a, b, n in sm.get_matching_blocks():
    for k in range(n):
        i, j = S_own[a + k], T_own[b + k]
        s, e = tw[j]["start"], tw[j]["end"]
        t0[i] = s if t0[i] is None else min(t0[i], s)
        t1[i] = e if t1[i] is None else max(t1[i], e)
        matched += 1
timed = sum(x is not None for x in t0)
i = 0
while i < N:
    if t0[i] is not None: i += 1; continue
    j = i
    while j < N and t0[j] is None: j += 1
    lo = t1[i - 1] if i > 0 else max(0.0, (t0[j] if j < N else 0.2) - 0.25 * (j - i))
    hi = t0[j] if j < N else lo + 0.25 * (j - i)
    hi = max(hi, lo + 0.04 * (j - i))
    L = [max(1, len(tokens[k][0])) for k in range(i, j)]; tot = float(sum(L)); acc = 0.0
    for k, ln in zip(range(i, j), L):
        t0[k] = lo + (hi - lo) * acc / tot; acc += ln; t1[k] = lo + (hi - lo) * acc / tot
    i = j
prev = 0.0
for k in range(N):
    s = max(t0[k], prev - 0.02); e = max(t1[k], s + 0.04); t0[k], t1[k] = s, e; prev = e

# ------------------------------------------------------------------ sentences
lines, cur_l = [], []
def close():
    global cur_l
    if cur_l:
        ws = [[tokens[k][0], round(t0[k], 3), round(t1[k], 3)] for k in cur_l]
        lines.append({"p": tokens[cur_l[0]][1], "s": ws[0][1], "e": ws[-1][2], "w": ws})
    cur_l = []
for k in range(N):
    if cur_l and tokens[k][1] != tokens[cur_l[-1]][1]: close()
    cur_l.append(k)
    d = tokens[k][0]
    if re.search(r"[.?!][\"')]*$", d) and not re.search(r"\d\.$", d): close()
    elif len(cur_l) >= 22 and d.endswith((",", ";", ":")): close()
close()
pj = []
for pi in range(len(parts)):
    ls = [l for l in lines if l["p"] == pi]
    pj.append({"s": ls[0]["s"], "e": ls[-1]["e"]})
Path(args.out).write_text(json.dumps({"duration": duration, "parts": pj, "lines": lines}, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
print(f"tokens {N}  transcript words {len(tw)}  matched pieces {matched}/{len(S)}  directly timed {timed}/{N} ({100*timed/N:.1f}%)  duration {duration:.2f}s")
for i, p in enumerate(pj): print(f"  part {i+1}: {p['s']:7.2f} - {p['e']:7.2f}  ({p['e']-p['s']:5.1f}s)")
if timed / N < 0.9: print("WARNING: <90% matched", file=sys.stderr)
