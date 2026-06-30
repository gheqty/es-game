//! Shared global game state.
//!
//! Everything lives in `static mut` since the WASM module is single-threaded.
//! Centralising it here keeps the other modules focused on logic.

use crate::ffi::now_ms;

// ---- World geometry ----
pub const TILE: i32 = 32;
pub const MAP_W: i32 = 48;
pub const MAP_H: i32 = 36;
pub const CANVAS_W: i32 = 768;
pub const CANVAS_H: i32 = 576;
pub const WORLD_W: i32 = MAP_W * TILE;
pub const WORLD_H: i32 = MAP_H * TILE;

// ---- Game states ----
pub const ST_ROAM: i32 = 0;
pub const ST_DIALOG: i32 = 1;
pub const ST_ENDING: i32 = 2;

// ---- Dialogue effect opcodes ----
pub const EFF_NONE: i32 = 0;
pub const EFF_SET_PROGRESS: i32 = 1;
pub const EFF_SET_FLAG: i32 = 2;
pub const EFF_CLEAR_FLAG: i32 = 3;
pub const EFF_ENDING: i32 = 4;

// ---- Quest flags (bits in `FLAGS`) ----
pub const F_TALKED_MATHILDE: u32 = 1 << 0;
pub const F_TALKED_ANSELM: u32 = 1 << 1;
pub const F_TALKED_RODERICK: u32 = 1 << 2;
pub const F_TALKED_LYRA: u32 = 1 << 3;
pub const F_KNOWS_RELIC: u32 = 1 << 4;
pub const F_HAS_RELIC: u32 = 1 << 5;
pub const F_TOLD_LYRA: u32 = 1 << 6;
pub const F_ACCEPTED_LYRA: u32 = 1 << 7;
pub const F_REFUSED_LYRA: u32 = 1 << 8;
pub const F_RODERICK_LINE: u32 = 1 << 9;
pub const F_ASSAULT: u32 = 1 << 10;
pub const F_GATE_VISITED_EARLY: u32 = 1 << 11;
pub const F_TALKED_KONRAD: u32 = 1 << 12;
pub const F_TALKED_MILDRED: u32 = 1 << 13;
pub const F_TALKED_ERIK: u32 = 1 << 14;
pub const F_TALKED_SORA: u32 = 1 << 15;

// ---- Map modes ----
pub const MAP_WORLD: i32 = 0;
pub const MAP_TAVERN: i32 = 1;
pub const MAP_CHAPEL: i32 = 2;
pub const MAP_HOUSE1: i32 = 3;
pub const MAP_HOUSE2: i32 = 4;

// ---- Interior dimensions (fits exactly on canvas, no scrolling) ----
pub const INT_W: i32 = 24;
pub const INT_H: i32 = 18;

// ---- Language ----
pub const LANG_DE: i32 = 0;
pub const LANG_EN: i32 = 1;

/// Pick a string based on the current language.
pub fn l(de: &'static str, en: &'static str) -> &'static str {
    unsafe {
        if LANG == LANG_DE {
            de
        } else {
            en
        }
    }
}

pub fn toggle_lang() {
    unsafe {
        LANG = if LANG == LANG_DE { LANG_EN } else { LANG_DE };
    }
}

// ---- An interactable (NPC, gate, pedestal) ----
#[derive(Copy, Clone)]
pub struct Inter {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub kind: i32, // 0..3 NPC, 4 gate, 5 pedestal
}

pub static mut TILES: [u8; (MAP_W * MAP_H) as usize] = [0; (MAP_W * MAP_H) as usize];

pub fn tile(x: i32, y: i32) -> u8 {
    if x < 0 || y < 0 || x >= MAP_W || y >= MAP_H {
        // out of bounds is treated as solid forest
        return 7; // TREE
    }
    unsafe { TILES[(y as usize) * (MAP_W as usize) + (x as usize)] }
}

pub fn set_tile(x: i32, y: i32, t: u8) {
    if x < 0 || y < 0 || x >= MAP_W || y >= MAP_H {
        return;
    }
    unsafe { TILES[(y as usize) * (MAP_W as usize) + (x as usize)] = t }
}

pub static mut PX: f64 = 0.0;
pub static mut PY: f64 = 0.0;
pub static mut FACING: i32 = 0; // 0 down, 1 left, 2 right, 3 up
pub static mut MOVING: bool = false;
pub static mut WALK_PHASE: f64 = 0.0;
pub static mut HELD_DIR: i32 = -1;

pub static mut CAM_X: f64 = 0.0;
pub static mut CAM_Y: f64 = 0.0;

pub static mut MAP_MODE: i32 = 0;
pub static mut WORLD_PX: f64 = 0.0;
pub static mut WORLD_PY: f64 = 0.0;
pub static mut LANG: i32 = 1; // default: English

pub static mut STATE: i32 = ST_ROAM;
pub static mut FLAGS: u32 = 0;
pub static mut PROGRESS: i32 = 0;
pub static mut CUR_NODE: i32 = -1;
pub static mut ENDING_ID: i32 = 0;

pub static mut MOUSE_X: f64 = 0.0;
pub static mut MOUSE_Y: f64 = 0.0;
pub static mut HOVER: i32 = -1;

pub static mut DIALOG_ANNIM: f64 = 0.0;

pub static mut INTERS: [Inter; 6] = [
    Inter { x: 0.0, y: 0.0, w: 20.0, h: 24.0, kind: 0 },
    Inter { x: 0.0, y: 0.0, w: 20.0, h: 24.0, kind: 1 },
    Inter { x: 0.0, y: 0.0, w: 20.0, h: 24.0, kind: 2 },
    Inter { x: 0.0, y: 0.0, w: 20.0, h: 24.0, kind: 3 },
    Inter { x: 0.0, y: 0.0, w: 40.0, h: 40.0, kind: 4 },
    Inter { x: 0.0, y: 0.0, w: 40.0, h: 40.0, kind: 5 },
];

// Wrapped line ranges for the current node text (indices into the &str).
pub static mut WRAP_LINES: [(usize, usize); 12] = [(0, 0); 12];
pub static mut WRAP_COUNT: usize = 0;

/// Word-wrap `text` into up to 8 lines, storing byte ranges in `WRAP_LINES`.
/// Generic (no `Node` dependency) so it lives here next to the buffers.
pub fn wrap_text(text: &str, max_w: f64, size: f64) {
    use crate::ffi::measure;
    unsafe { WRAP_COUNT = 0 };
    let mut count = 0usize;
    let mut line_start: usize = 0;
    let mut line_end: usize = 0;
    let mut first = true;
    let base = text.as_ptr() as usize;

    for word in text.split_whitespace() {
        let ws = (word.as_ptr() as usize) - base;
        let we = ws + word.len();
        if first {
            line_start = ws;
            line_end = we;
            first = false;
            continue;
        }
        let candidate = measure(&text[line_start..we], size);
        if candidate > max_w {
            if count < 12 {
                unsafe { WRAP_LINES[count] = (line_start, line_end) };
                count += 1;
            }
            line_start = ws;
            line_end = we;
        } else {
            line_end = we;
        }
    }
    if !first && count < 12 {
        unsafe { WRAP_LINES[count] = (line_start, line_end) };
        count += 1;
    }
    unsafe { WRAP_COUNT = count };
}

// ---- Accessors ----

pub fn has(flag: u32) -> bool {
    unsafe { (FLAGS & flag) != 0 }
}

pub fn state() -> i32 {
    unsafe { STATE }
}

pub fn now() -> f64 {
    now_ms()
}

/// Reset all quest/player state for a fresh game.
pub fn reset_run() {
    unsafe {
        FLAGS = 0;
        PROGRESS = 0;
        CUR_NODE = -1;
        ENDING_ID = 0;
        HELD_DIR = -1;
        MOVING = false;
        WALK_PHASE = 0.0;
        STATE = ST_ROAM;
        MAP_MODE = MAP_WORLD;
    }
}
