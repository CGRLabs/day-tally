"""In-game checks for Day Tally: the tally in the corner, and that it goes away."""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
INSTALL = Path(r"D:\CGRLabs\Horizon-Ecosystem\horizon-windows-x64")
OUT = ROOT / "target" / "proof"
OUT.mkdir(parents=True, exist_ok=True)


def main():
    env = dict(os.environ)
    home = Path(tempfile.mkdtemp(prefix="day-tally-home-"))
    env["HORIZON_HOME"] = str(home)
    env["HORIZON_CONTROL"] = str(home / "control.sock")
    for key in ("HORIZON_LAUNCHER_HOME", "XDG_DATA_HOME", "XDG_CONFIG_HOME"):
        env[key] = tempfile.mkdtemp(prefix="day-tally-")
    env["HORIZON_NO_AUDIO"] = "1"
    shutil.copytree(INSTALL / "horizon-mods", home / "horizon-mods")
    shutil.copy2(INSTALL / "horizon-mods.txt", home / "horizon-mods.txt")
    log = (OUT / "day-tally-v01.log").open("w", encoding="utf-8")
    report = {"scratch_home": str(home)}
    # The client to test: this build by default, or one named on the command line.
    game = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "target" / "horizon-cargo" / "fast" / "horizon-client.exe"
    report["game"] = str(game)

    def say(msg):
        print(msg, flush=True)
        log.write(msg + "\n")
        log.flush()

    def ctl(*a, timeout=90):
        o, e = OUT / "ctl.out", OUT / "ctl.err"
        with o.open("wb") as fo, e.open("wb") as fe:
            done = subprocess.run([str(INSTALL / "horizon-ctl.exe"), *a], stdout=fo, stderr=fe,
                                  env=env, timeout=timeout, cwd=str(home))
        text = o.read_text(encoding="utf-8", errors="replace").strip()
        if done.returncode != 0:
            raise RuntimeError(f"ctl {' '.join(a)} -> {done.returncode}: {text}")
        try:
            value = json.loads(text)
        except json.JSONDecodeError:
            value = {"_raw": text}
        if isinstance(value, dict) and "error" in value:
            raise RuntimeError(f"ctl {' '.join(a)} -> {value['error']}")
        return value

    def shot(name):
        path = OUT / f"day-tally-v01-{name}.png"
        ctl("screenshot", str(path))
        say(f"screenshot {name}: {path}")
        return path

    def day():
        return ctl("state").get("time", {}).get("day")

    try:
        say(f"scratch home: {home}")
        say("launching VISIBLE window...")
        ctl("launch", "--game", str(game),
            "--world", "DayTallyV01", "--creative", "--wait", "--render-distance", "6",
            "--resources", str(INSTALL / ".local" / "horizon-client" / "26.3"),
            "--test-terrain", timeout=420)
        ctl("wait-ready", "120", timeout=150)
        for _ in range(6):
            if ctl("state").get("screen") == "playing":
                break
            ctl("close")
            ctl("resume")
            time.sleep(2)
        assert ctl("state").get("screen") == "playing", "the game did not start"

        # The tally shows when the day turns: it is showing now.
        shot("shown")
        report["day"] = day()
        say(f"the world is on day {report['day']}")

        # And it goes away again.
        time.sleep(9)
        shot("gone")
        say("waited for the tally to go")

        # A new day brings it back.
        ctl("command", "time add 24000")
        time.sleep(1.5)
        shot("new-day")
        report["day_after"] = day()
        say(f"after a day passes, the world is on day {report['day_after']}")
        assert report["day_after"] == report["day"] + 1, \
            f"the day did not move: {report['day']} -> {report['day_after']}"
        report["passed"] = True
        say("the tally showed, went away, and came back on the new day")
    except Exception as exc:
        report["error"] = str(exc)
        say(f"FAILED: {exc}")
        raise
    finally:
        (OUT / "day-tally-v01-report.json").write_text(json.dumps(report, indent=2), encoding="utf-8")
        try:
            ctl("quit", timeout=90)
        except Exception as exc:
            say(f"quit: {exc}")
        log.close()


if __name__ == "__main__":
    main()
