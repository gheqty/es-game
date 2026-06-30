//! Player and NPC movement, collision, camera, interaction and door transitions.

use crate::math::sqrt;
use crate::state::{
    has, tile, Inter, INTERS, MAP_MODE, MAP_WORLD, TILE, FACING, HELD_DIR,
    MOVING, PX, PY, WALK_PHASE, CAM_X, CAM_Y, F_ACCEPTED_LYRA, F_KNOWS_RELIC,
    F_TALKED_LYRA, WORLD_PX, WORLD_PY,
};
use crate::story::{open_node, start_node_for};
use crate::world::{is_solid, DOOR, init_map, init_tavern, init_chapel, init_house1, init_house2};

/// Lyra only appears once the player has learned of the relic (or has spoken
/// to her before).
pub fn lyra_present() -> bool {
    has(F_KNOWS_RELIC) || has(F_TALKED_LYRA)
}

/// Lyra's plaza position (tile 21, 19).
const LYRA_PLAZA_X: f64 = 21.0 * 32.0 + (32.0 - 20.0) / 2.0;
const LYRA_PLAZA_Y: f64 = 19.0 * 32.0 + (32.0 - 28.0);

/// If the player accepted Lyra's help, she moves to the gate area.
/// Called every frame; only repositions once.
pub fn update_lyra_position() {
    if unsafe { MAP_MODE } != MAP_WORLD {
        return;
    }
    if !has(F_ACCEPTED_LYRA) {
        return;
    }
    unsafe {
        // Only move her if she's still at the plaza
        if (INTERS[3].x - LYRA_PLAZA_X).abs() < 1.0 && (INTERS[3].y - LYRA_PLAZA_Y).abs() < 1.0 {
            // Move to the path near the gate (tile 42, 20)
            INTERS[3].x = 42.0 * 32.0 + (32.0 - 20.0) / 2.0;
            INTERS[3].y = 20.0 * 32.0 + (32.0 - 28.0);
        }
    }
}

/// Is the world tile at (wx, wy) solid for the player's feet?
fn solid_at_world(wx: f64, wy: f64) -> bool {
    let tx = (wx / TILE as f64) as i32;
    let ty = (wy / TILE as f64) as i32;
    if tx < 0 || ty < 0 || tx >= crate::state::MAP_W || ty >= crate::state::MAP_H {
        return true;
    }
    is_solid(tile(tx, ty))
}

/// True if placing the player sprite's feet at (wx, wy) collides.
fn player_blocked(wx: f64, wy: f64) -> bool {
    // feet hitbox
    let x0 = wx + 3.0;
    let y0 = wy + 18.0;
    let w = 14.0;
    let h = 9.0;
    let corners = [
        (x0, y0),
        (x0 + w - 1.0, y0),
        (x0, y0 + h - 1.0),
        (x0 + w - 1.0, y0 + h - 1.0),
    ];
    for &(cx, cy) in corners.iter() {
        if solid_at_world(cx, cy) {
            return true;
        }
    }
    // NPC bodies — only check active interactables
    let max = if unsafe { MAP_MODE } != MAP_WORLD { 1 } else { 4 };
    let mut k = 0;
    while k < max {
        unsafe {
            let it: Inter = INTERS[k];
            if it.w <= 0.0 || it.h <= 0.0 {
                k += 1;
                continue;
            }
            if k == 3 && !lyra_present() {
                k += 1;
                continue;
            }
            if wx + 18.0 > it.x
                && wx + 3.0 < it.x + it.w
                && wy + 27.0 > it.y
                && wy + 18.0 < it.y + it.h
            {
                return true;
            }
        }
        k += 1;
    }
    false
}

pub fn update_movement(dir: i32) {
    let speed = 1.6;
    let (dx, dy) = match dir {
        0 => (0.0, speed),  // down
        1 => (-speed, 0.0), // left
        2 => (speed, 0.0),  // right
        3 => (0.0, -speed), // up
        _ => return,
    };
    unsafe {
        FACING = dir;
        let nx = PX + dx;
        if !player_blocked(nx, PY) {
            PX = nx;
        }
        let ny = PY + dy;
        if !player_blocked(PX, ny) {
            PY = ny;
        }
        MOVING = true;
    }
}

pub fn clamp_camera() {
    use crate::state::{CANVAS_H, CANVAS_W, WORLD_H, WORLD_W};
    // Interiors are exactly canvas-sized: no scrolling needed.
    if unsafe { MAP_MODE } != MAP_WORLD {
        unsafe {
            CAM_X = 0.0;
            CAM_Y = 0.0;
        }
        return;
    }
    let max_x = (WORLD_W - CANVAS_W) as f64;
    let max_y = (WORLD_H - CANVAS_H) as f64;
    unsafe {
        CAM_X = (PX + 10.0 - CANVAS_W as f64 / 2.0).max(0.0).min(max_x.max(0.0));
        CAM_Y = (PY + 14.0 - CANVAS_H as f64 / 2.0).max(0.0).min(max_y.max(0.0));
    }
}

// ---- Bounding-box distance: 0 if inside the box, otherwise the
// nearest-edge distance. This makes large objects (gate, pedestal)
// interactable from a natural standing position beside them. ----
fn dist_to_box(px: f64, py: f64, bx: f64, by: f64, bw: f64, bh: f64) -> f64 {
    if bw <= 0.0 || bh <= 0.0 {
        return 9999.0;
    }
    let dx = if px < bx { bx - px } else if px > bx + bw { px - bx - bw } else { 0.0 };
    let dy = if py < by { by - py } else if py > by + bh { py - by - bh } else { 0.0 };
    sqrt(dx * dx + dy * dy)
}

/// Public wrapper for ui.rs to use for the interaction prompt.
pub fn dist_to_box_pub(px: f64, py: f64, bx: f64, by: f64, bw: f64, bh: f64) -> f64 {
    dist_to_box(px, py, bx, by, bw, bh)
}

/// How many interactable slots to check: in interiors, only slot 0 (the
/// interior NPC). In the world, slots 0–5.
fn interact_limit() -> usize {
    if unsafe { MAP_MODE } != MAP_WORLD {
        1
    } else {
        6
    }
}

/// Interact with the nearest NPC/object within reach.
pub fn try_interact() {
    let (px, py) = unsafe { (PX, PY) };
    let cx = px + 10.0;
    let cy = py + 22.0;
    let mut best_kind: i32 = -1;
    let mut best_d = 9999.0;
    let limit = interact_limit();
    let mut k = 0;
    while k < limit {
        unsafe {
            let it: Inter = INTERS[k];
            if it.w <= 0.0 || it.h <= 0.0 {
                k += 1;
                continue;
            }
            if k == 3 && !lyra_present() {
                k += 1;
                continue;
            }
            let d = dist_to_box(cx, cy, it.x, it.y, it.w, it.h);
            if d < 48.0 && d < best_d {
                best_d = d;
                best_kind = it.kind;
            }
        }
        k += 1;
    }
    if best_kind >= 0 {
        let node = start_node_for(best_kind);
        if node >= 0 {
            open_node(node);
        }
    }
}

// ---- Door transitions ----

/// Door tile positions in the world map (tile coords).
const TAVERN_DOOR_TX: i32 = 12;
const TAVERN_DOOR_TY: i32 = 18;
const CHAPEL_DOOR_TX: i32 = 38;
const CHAPEL_DOOR_TY: i32 = 11;
const HOUSE1_DOOR_TX: i32 = 18;
const HOUSE1_DOOR_TY: i32 = 31;
const HOUSE2_DOOR_TX: i32 = 32;
const HOUSE2_DOOR_TY: i32 = 31;

/// Check whether the player's feet are on a DOOR tile and trigger a
/// map transition. Called every frame after movement.
pub fn check_door() {
    let (px, py) = unsafe { (PX, PY) };
    let cx = px + 10.0;
    let cy = py + 22.0;
    let tx = (cx / TILE as f64) as i32;
    let ty = (cy / TILE as f64) as i32;
    if tile(tx, ty) != DOOR {
        return;
    }

    let mode = unsafe { MAP_MODE };
    if mode == MAP_WORLD {
        // entering a building
        if tx == TAVERN_DOOR_TX && ty == TAVERN_DOOR_TY {
            enter_interior(1);
        } else if tx == CHAPEL_DOOR_TX && ty == CHAPEL_DOOR_TY {
            enter_interior(2);
        } else if tx == HOUSE1_DOOR_TX && ty == HOUSE1_DOOR_TY {
            enter_interior(3);
        } else if tx == HOUSE2_DOOR_TX && ty == HOUSE2_DOOR_TY {
            enter_interior(4);
        }
    } else {
        // exiting back to the world
        exit_interior();
    }
}

fn enter_interior(which: i32) {
    unsafe {
        WORLD_PX = PX;
        WORLD_PY = PY;
        MAP_MODE = which;
    }
    match which {
        1 => init_tavern(),
        2 => init_chapel(),
        3 => init_house1(),
        4 => init_house2(),
        _ => {}
    }
}

fn exit_interior() {
    let saved_mode = unsafe { MAP_MODE };
    unsafe {
        MAP_MODE = MAP_WORLD;
    }
    init_map();
    let (door_tx, door_ty) = match saved_mode {
        1 => (TAVERN_DOOR_TX, TAVERN_DOOR_TY + 1),
        2 => (CHAPEL_DOOR_TX, CHAPEL_DOOR_TY + 1),
        3 => (HOUSE1_DOOR_TX, HOUSE1_DOOR_TY + 1),
        4 => (HOUSE2_DOOR_TX, HOUSE2_DOOR_TY + 1),
        _ => (TAVERN_DOOR_TX, TAVERN_DOOR_TY + 1),
    };
    unsafe {
        PX = door_tx as f64 * TILE as f64 + 6.0;
        PY = door_ty as f64 * TILE as f64;
    }
}

/// Mark the player as not moving (called when no direction is held).
pub fn stop_moving() {
    unsafe {
        MOVING = false;
    }
}

pub fn step_walk_phase() {
    unsafe {
        WALK_PHASE += 0.25;
    }
}

/// Per-frame movement update driven by the held direction.
pub fn tick_movement() {
    let d = unsafe { HELD_DIR };
    if d >= 0 {
        update_movement(d);
        step_walk_phase();
    } else {
        stop_moving();
    }
}
