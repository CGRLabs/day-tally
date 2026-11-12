# Day Tally 0.1.0 — registry submission sheet

Submit at: https://playhorizon.dev/mods/submit (sign in with GitHub as CGRLabs), choose **new
project** — `day_tally` is not in the registry yet.
**Verified in-game 2026-11-12**: the tally appears in the bottom-left corner, goes away again, and
comes back when the day turns.

## Form fields

| Field | Value |
| --- | --- |
| Kind | Native mod (.hnmod) |
| File | `D:\CGRLabs\Horizon-Ecosystem\prototypes\day-tally\target\day_tally-0.1.0.hnmod` |
| Id | `day_tally` |
| Version | `0.1.0` |
| For Minecraft | `26.3` |
| Name | Day Tally |
| Licence | MIT |
| Authors | CGRLabs |
| Source code | *(not published yet — see "Still to do")* |
| Categories | utility, decoration |
| Tags | hud, day, counter, roman numerals, medieval, corner |
| Icon | `D:\CGRLabs\Horizon-Ecosystem\prototypes\day-tally\icon.png` **(to be supplied)** |
| Hook points | none declared — the bundle is pinned to Horizon `36f4bc00` |
| Public key | `RWSjKGIJUKKXHvvhxzNN2VPPrXSEr47JXVgGpCwrOOHoneuQjTWWvk1g` (the Hub key, the same one Gyro and Villager Names use) |
| Signature | `D:\CGRLabs\Horizon-Ecosystem\prototypes\day-tally\target\day_tally-0.1.0.hnmod.minisig` |

## Release facts

- File size: **5 232 bytes**
- SHA-256: `ac10732ed0c91718d87f851b1fdf1b0e35e3f37939706c7b646bf8d7115371c9`
- Pinned to Horizon `36f4bc00d6fc644f1ad118ddd926a997a034cb48`, horizon-mixins
  `d9c83d7096b7ea5b83678da2efdfd6161f74997e`, mixin schema 6
- The mod inspector: **0 errors**, 6 files verified in the bundle
- Signature: minisign, verified against the Hub public key (`signature: VALID`)
- Unit tests: 6 of 6 pass (`cargo test -p horizon-client --lib day_tally::tests`)

## What it does

The day of the world, in **Roman numerals**, on a dark plaque with a gold rim, in the **bottom left
corner** of the screen.

> **XII**
> *DAY*

It **appears when the day turns** and shrinks away again over the next few seconds — so it tells you
that a night passed, and then it leaves the screen to the world.

- **Reads the world's own clock** — the same number `/time query day` gives. It keeps no count of its
  own, so it can never disagree with the game.
- **Roman numerals**, because the day is a number and the numerals look like a tally.
- **Nothing else.** No items, no blocks, no screens, no settings.

> The Romans had no zero, so the first day reads **N** — *nulla*.

It runs in single player and on a LAN host — the places where the client and the world are the same
machine.

## With the Day Counter content mod

[Day Counter](https://modrinth.com/datapack/daycounter) does the same job as a small line of text
above the hotbar, and it works on any server, including a dedicated one.

Install both and they do not fight: this mod tells the content mod that it is here (by setting
`#day_tally daycounter 1` on the scoreboard), and the line of text stands down. What you see is the
tally in the corner. A server without this mod still gets the line of text.

## In-game proof — VERIFIED 2026-11-12

`python tools/proof_day_tally.py` against the real client on a scratch world.

| Check | Result |
| --- | --- |
| The tally appears | yes — a gold-framed plaque with the numeral in the bottom-left corner |
| The world was on day | 0 — the numeral reads **N** (*nulla*) |
| It goes away again | yes — the corner is empty after the tally's time is up |
| A new day brings it back | yes — after `/time add 24000` the world is on day **1** and the numeral reads **I** |
| The content mod stands down | yes — no `DAY 1` line on the action bar while the tally is showing |

The proof's own pictures show it: `target/proof/day-tally-v01-shown.png` (the tally on day 0),
`-gone.png` (the empty corner), `-new-day.png` (the tally on day 1).

The same proof was also run against a client built with **barrel_roll and Day Tally together** (the
two bundles applied cleanly side by side: 9 injections, 2 modules) — the tally still shows, still
goes away, and still comes back on the new day.

Unit tests, 6 of 6 pass (`cargo test -p horizon-client --lib day_tally::tests`):

| Test | What it holds |
| --- | --- |
| `the_first_day_is_nulla` | day 0 reads "N" — the Romans had no zero |
| `the_small_numerals_are_right` | I, II, III, IV, V, IX, XIV |
| `the_bigger_numerals_are_right` | XL, XLIX, XC, CD, CM, MCMXC, MMXXVI |
| `a_day_is_the_clock_divided_by_the_length_of_a_day` | the day is the clock over 24 000 |
| `the_tally_shows_and_then_goes` | the tally shows at full size |
| `the_same_day_does_not_note_the_tally_again` | the timer does not restart on the same day |

## Description (paste into the Markdown box)

**How many days has it been?** The answer, in Roman numerals, on a dark plaque with a gold rim, in
the bottom left corner of the screen.

> **XII**
> *DAY*

**It shows itself when the day turns.** The moment the night passes, the tally appears — and a few
seconds later it shrinks away again, leaving the world to itself. It is there when you need it, and
out of the way when you do not.

**It reads the world's own clock,** so it is always the number `/time query day` gives. It never
counts on its own and it can never disagree with the game.

**Roman numerals,** because a day is a number and a tally is what you get when you count. The first
day reads **N** — *nulla* — because the Romans had no zero.

**Nothing else changes.** No items, no blocks, no screens, no settings. One plaque, in the corner,
when the day turns.

*Native mods are unsandboxed code running with your account's permissions; install only from
authors you trust.*

## Still to do (needs the user)

- **The icon** — a Roman numeral on a dark plaque with a gold rim, as the picture shows the mod. PNG,
  at most 256 KiB, dropped at `prototypes/day-tally/icon.png`. The registry square-crops it, so a
  square picture is best.
- **Publish the source** — the registry's pull-request template asks that the `source` link has the
  code the file was built from. The source should go to `https://github.com/CGRLabs/day-tally` (a
  public repo), like Flesh to Leather and Gyro.
- **Fill in the form** at https://playhorizon.dev/mods/submit with the table above and the
  description.
- **Wait for the review.** A project's first listing is always reviewed by a person; later versions by
  the same owner are merged automatically once the checks pass.
