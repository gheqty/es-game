#![no_std]
#![cfg_attr(target_arch = "wasm32", no_main)]

//! "Schatten über Bruma" - a short Elder-Scrolls-flavoured RPG written
//! entirely in `no_std` Rust, compiled to WebAssembly.
//!
//! Module layout:
//! - `math`  - f64 transcendentals (core has no `sin`/`sqrt`)
//! - `ffi`   - JS bindings + safe drawing wrappers
//! - `state` - shared statics, flag constants, tile/map helpers
//! - `world` - tile types, procedural village map, per-tile rendering
//! - `story` - dialogue tree, quest logic, endings
//! - `actor` - player/NPC movement, collision, camera, interaction
//! - `ui`    - characters, dialogue UI, HUD, endings

use core::panic::PanicInfo;

pub mod actor;
pub mod dialogue;
pub mod ffi;
pub mod math;
pub mod state;
pub mod story;
pub mod ui;
pub mod world;

use crate::ffi::{clear, request_frame};
use crate::state::{
    now, reset_run, state, toggle_lang, DIALOG_ANNIM, HELD_DIR, MOUSE_X, MOUSE_Y, ST_DIALOG,
    ST_ENDING, ST_ROAM, PX, PY, TILE,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn init() {
    world::init_map();
    reset_run();
    unsafe {
        PX = 24.0 * TILE as f64 + 6.0;
        PY = 33.0 * TILE as f64;
    }
    actor::clamp_camera();
    request_frame();
}

#[no_mangle]
pub extern "C" fn update() {
    unsafe {
        DIALOG_ANNIM += 1.0;
    }
    if state() == ST_ROAM {
        actor::tick_movement();
        actor::check_door();
    }
    actor::update_lyra_position();
    actor::clamp_camera();

    clear();
    ui::draw_map();
    ui::draw_interactables();
    if state() != ST_ENDING {
        ui::draw_player();
    }
    ui::draw_hud();
    if state() == ST_DIALOG {
        ui::draw_dialog();
    }
    if state() == ST_ENDING {
        ui::draw_ending();
    }
    request_frame();
    let _ = now();
}

#[no_mangle]
pub extern "C" fn keydown(code: i32) {
    // Language toggle works in any state
    if code == 7 {
        toggle_lang();
        if state() == ST_DIALOG {
            story::rewrap_current();
        }
        return;
    }
    match state() {
        ST_ROAM => match code {
            0 | 1 | 2 | 3 => unsafe { HELD_DIR = code },
            4 => {
                unsafe { HELD_DIR = -1 };
                actor::try_interact();
            }
            _ => {}
        },
        ST_DIALOG => match code {
            5 => story::close_dialog(),
            6 => story::select_choice(0),
            c if c >= 100 && c < 109 => story::select_choice(c - 100),
            _ => {}
        },
        ST_ENDING => init(),
        _ => {}
    }
}

#[no_mangle]
pub extern "C" fn keyup(code: i32) {
    if matches!(code, 0 | 1 | 2 | 3) && unsafe { HELD_DIR == code } {
        unsafe { HELD_DIR = -1 };
    }
}

#[no_mangle]
pub extern "C" fn mousemove(x: f64, y: f64) {
    unsafe {
        MOUSE_X = x;
        MOUSE_Y = y;
    }
}

#[no_mangle]
pub extern "C" fn click(_x: f64, _y: f64) {
    if state() == ST_DIALOG {
        let h = unsafe { crate::state::HOVER };
        if h >= 0 {
            story::select_choice(h);
        }
    }
}
