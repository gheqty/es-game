# Schatten über Bruma

A short story-driven RPG set in the Elder Scrolls universe, written entirely
in Rust and shipped as a single self-contained binary. A village near Bruma
is threatened by an Oblivion gate; you talk to four NPCs, make choices, and
reach one of three endings.

All game logic (movement, collision, dialogue, quest state, rendering) is
`no_std` Rust compiled to WebAssembly. The HTML page contains only glue JS
to bootstrap the WASM and shim browser APIs — no game logic in JavaScript.

## Run

```sh
rustup target add wasm32-unknown-unknown   # one-time prerequisite
cargo run --release
# open http://127.0.0.1:12080
```

**Controls:** WASD / arrow keys to move, `E` (or space) to talk / interact,
`1`–`9` or mouse click for dialogue choices, `Esc` to close a dialogue,
`L` to switch language (English default, German available).

## Play online

The game is deployed on GitHub Pages — the WASM client runs entirely in
the browser, no server needed. The Rust binary is only for local play.

## Story

An Oblivion gate burns east of Bruma. Four characters shape your path:

- **Mathilde** — the tavern keeper, your starting point.
- **Bruder Anselm** — a priest who knows of an Ayleid sealing relic.
- **Wache Roderick** — the gate guard holding the south road.
- **Lyra** — a stranger who knows more about Oblivion than she lets on.

Find the *Herz von Akatosh* relic in the western ruins, then decide how to
close the gate. Three endings:

| Ending | Path |
|--------|------|
| **Held von Bruma** | Seal the gate alone with the relic. |
| **Verraten** | Trust Lyra with the relic — she serves the Mythic Dawn. |
| **Das Opfer** | Assault the gate with no relic; it closes, but consumes you. |

## Architecture

The server is one Rust binary (`src/main.rs`) — a std-only HTTP server that
embeds the HTML (`include_str!`) and the prebuilt WASM
(`include_bytes!`) at compile time. `build.rs` shells out to build the
`client/` crate to `wasm32-unknown-unknown`, then bakes the result in.

The client crate is split into modules:

```
client/src/
  lib.rs    crate root, #[no_mangle] entry points (init/update/keydown/...)
  math.rs   f64 transcendentals (core has no sin/cos/sqrt)
  ffi.rs    extern "C" JS bindings + safe drawing wrappers
  state.rs  shared statics, quest-flag constants, tile/map helpers
  world.rs  tile types, procedural village map, per-tile rendering
  story.rs  dialogue tree (Node/Choice), quest logic, 3 endings
  actor.rs  player/NPC movement, collision, camera, interaction
  ui.rs     characters, dialogue UI, HUD, ending screens
```

```
build.rs ─compiles─▶ client/ ─▶ client.wasm ─┐
                                             │ include_bytes!
src/main.rs ◀────────────────────────────────┘
   │ serves /          (HTML)
   │ serves /game.wasm (embedded WASM)
   └─ binds 127.0.0.1:12080
```

No runtime dependencies beyond libc; the binary is ~345 KB.
