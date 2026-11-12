# Day Tally

The day of the world, big, in the bottom left corner.

> **12**

It appears the moment the day turns and goes away again a few seconds later — so it tells you that a
night passed, and then it leaves the screen to the world.

## What it does

- Shows the day of the world, **big**, in the **bottom left corner**.
- **Shows itself when the day changes**, and shrinks away over the last two seconds.
- Reads the **world's own clock** — the same number `/time query day` gives. It keeps no count of its
  own.
- Nothing else. No items, no blocks, no screens, no settings.

## Install

1. Install the mod and enable it in Options → Mods, then restart the game.
2. Play. Watch the corner when the night passes.

It runs in single player and on a LAN host — the places where the client and the world are the same
machine.

## With the Day Counter content mod

[Day Counter](https://modrinth.com/mod/day-counter) does the same job as a small line of text
above the hotbar, and it works on any server, including a dedicated one.

Install both and they will not fight: this mod tells the content mod that it is here, and the
line of text stands down. What you see is the tally in the corner, and a server without this mod
still gets the line of text.

## Build from source

```sh
HORIZON_MIXINS=/path/to/horizon-mixins \
  python tools/horizon.py build --horizon /path/to/horizon
```

## License

[MIT](LICENSE) — Copyright (c) 2026 CGRLabs.

Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft.
