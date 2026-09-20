"""Walk plan.md's VT criteria against the suite and the injection record.

Answers, per criterion, the two questions F-S4 turned on:
  1. every case the phase sheet names for it exists in the built suite;
  2. every injection letter it cites is defined in the same phase sheet.
"""
import re, subprocess, sys, pathlib

ROOT = pathlib.Path("/home/david/dev/goad")
S = ROOT / "docs/slices/009"
def built_suite():
  """The cases the suite actually carries, from the built binaries themselves."""
  names = set()
  for target in (["--workspace"], ["-p", "goad-semantics"]):
    out = subprocess.run(["cargo", "test", *target, "--", "--list"],
                         capture_output=True, text=True, cwd=ROOT).stdout
    names |= {l[:-len(": test")] for l in out.splitlines() if l.endswith(": test")}
  return names

suite = built_suite()
suite_bare = {n.rsplit("::", 1)[-1] for n in suite}

plan = (S / "plan.md").read_text().split("\n")
notes = (S / "notes.md").read_text().split("\n")

# plan.md: criteria per phase
crit, p = {}, None
for l in plan:
    m = re.match(r"^## (PHASE-\d+)", l)
    if m:
        p = m.group(1); crit[p] = []
    m = re.match(r"^- ((?:VT|VA)-\d+) —", l)
    if m and p:
        crit[p].append(m.group(1))

# notes.md: phase sheet bounds
bounds = [(m.group(1), i) for i, l in enumerate(notes)
          if (m := re.match(r"^### (PHASE-\d+)", l))]
bounds.append(("END", len(notes)))
sheet = {bounds[k][0]: (bounds[k][1], bounds[k + 1][1]) for k in range(len(bounds) - 1)}

def block(phase, c):
    """Lines of the phase sheet that speak about criterion c.

    A mention on one line is that line. A titled section — `**VT-1 — ...**`
    — carries its prose until the next bolded title or heading."""
    a, b = sheet[phase]
    pat = re.compile(r"(?<![A-Za-z0-9-])" + c + r"(?![0-9])")
    out, i = [], a
    while i < b:
        if pat.search(notes[i]):
            out.append(notes[i])
            if re.match(r"^\*\*" + c + r"\b", notes[i]):
                j = i + 1
                while j < b and not re.match(r"^(\*\*|#{2,4} |---)", notes[j]):
                    out.append(notes[j]); j += 1
                i = j; continue
        i += 1
    return out

def injections_defined(phase):
    """Injection letters the phase sheet defines: `- **A — ...` or `| **I-3** |`."""
    a, b = sheet[phase]
    d = set()
    for l in notes[a:b]:
        for pat in (r"^\|\s*\*{0,2}([A-Z]|I-\d+[a-z]?)\*{0,2}\s*\|",
                    r"^-\s*\*\*([A-Z]|I-\d+[a-z]?)\b"):
            for m in re.finditer(pat, l):
                d.add(m.group(1))
    return d

CASE = re.compile(r"`([A-Za-z0-9_./:-]*?)`")
NAME = re.compile(r"^[a-z][a-z0-9_]*$")
INJ = re.compile(r"injections?\s+((?:\*\*[A-Z]\*\*|\*\*I-\d+[a-z]?\*\*|[A-Z]\b|,|\s|and)+)", re.I)

rows = []
for phase in sorted(crit):
    defined = injections_defined(phase)
    for c in crit[phase]:
        blk = block(phase, c)
        text = "\n".join(blk)
        named, missing, loose = [], [], []
        for tick in CASE.findall(text):
            if "::" in tick:                       # a case, qualified by its file
                cand, qualified = tick.rsplit("::", 1)[-1].strip(), True
            elif "/" in tick or tick.endswith(".rs"):
                continue                           # a path, not a case
            else:
                cand, qualified = tick.strip(), False
            if not (NAME.match(cand) and cand.count("_") >= 2):
                continue
            named.append(cand)
            if cand not in suite_bare:
                (missing if qualified else loose).append(cand)
        cited = set()
        for m in INJ.finditer(text):
            cited |= set(re.findall(r"\b([A-Z]|I-\d+[a-z]?)\b", m.group(1)))
        cited.discard("VT"); cited.discard("VA")
        undef = sorted(cited - defined)
        rows.append((phase, c, len(blk), sorted(set(named)), sorted(set(missing)), sorted(cited), undef, sorted(set(loose))))

bad = [r for r in rows if r[4] or r[6]]
maybe = [r for r in rows if r[7] and not r[4]]
thin = [r for r in rows if not r[3] and not r[5]]
print(f"criteria: {len(rows)}   with a named case: {sum(1 for r in rows if r[3])}"
      f"   citing an injection: {sum(1 for r in rows if r[5])}")
print(f"cases named across all rows: {len({n for r in rows for n in r[3]})} distinct\n")
print("== named a case the suite does not have, or an injection the sheet does not define ==")
for r in bad:
    print(f"  {r[0]}/{r[1]}: missing_cases={r[4]} undefined_injections={r[6]}")
print(f"  ({len(bad)} of {len(rows)})\n")
print("== bare backticked name absent from the suite — may be an API symbol, hand-check ==")
for r in maybe:
    print(f"  {r[0]}/{r[1]}: {r[7]}")
print(f"  ({len(maybe)} of {len(rows)})\n")
print("== names no case and cites no injection (script cannot reach these) ==")
for r in thin:
    print(f"  {r[0]}/{r[1]}  ({r[2]} lines of sheet)")
print(f"  ({len(thin)} of {len(rows)})")
