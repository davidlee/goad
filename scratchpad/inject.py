"""Apply one textual defect, run one test target, restore, verify the restore."""
import subprocess, sys, pathlib

def run(path, old, new, target):
    p = pathlib.Path(path); orig = p.read_text()
    assert orig.count(old) == 1, f"anchor matched {orig.count(old)} times"
    p.write_text(orig.replace(old, new))
    try:
        r = subprocess.run(["cargo", "test", "--test", target],
                           capture_output=True, text=True, cwd="/home/david/dev/goad")
        out = r.stdout + r.stderr
    finally:
        p.write_text(orig)
    diff = subprocess.run(["git", "diff", "--stat", "--", path],
                          capture_output=True, text=True, cwd="/home/david/dev/goad").stdout
    tail = [l for l in out.splitlines()
            if "test result" in l or l.startswith("test ") or "panicked" in l
            or "assertion" in l or "error[" in l or "left" in l or "right" in l]
    print("\n".join(tail[:30]))
    print("restore clean:", diff.strip() == "")

if __name__ == "__main__":
    run(*sys.argv[1:])
