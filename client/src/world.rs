//! Tile types, procedural map generation and per-tile rendering.

use crate::ffi::{circle, rect, rgb, set_color, tri};
use crate::math::sin;
use crate::state::{now, set_tile, tile, CAM_X, CAM_Y, F_KNOWS_RELIC, MAP_H, MAP_W, TILE};

// ---- Tile type constants ----
pub const GRASS: u8 = 0;
pub const GRASS_DARK: u8 = 1;
pub const FLOWER_Y: u8 = 2;
pub const FLOWER_P: u8 = 3;
pub const PATH: u8 = 4;
pub const COBBLE: u8 = 5;
pub const WATER: u8 = 6;
pub const TREE: u8 = 7;
pub const ROCK: u8 = 8;
pub const WOOD_FLOOR: u8 = 9;
pub const WALL: u8 = 10;
pub const ROOF: u8 = 11;
pub const DOOR: u8 = 12;
pub const STONE_FLOOR: u8 = 13;
pub const STONE_WALL: u8 = 14;
pub const LAVA: u8 = 15;
pub const PORTAL: u8 = 16;
pub const SCORCH: u8 = 17;
pub const PEDESTAL: u8 = 18;
pub const TABLE: u8 = 19;
pub const PEW: u8 = 20;
pub const ALTAR: u8 = 21;
pub const CARPET: u8 = 22;
pub const BAR_COUNTER: u8 = 23;
pub const BED: u8 = 24;
pub const FIREPLACE: u8 = 25;
pub const SHELF: u8 = 26;

pub fn is_solid(t: u8) -> bool {
    matches!(
        t,
        TREE | ROCK | WALL | ROOF | STONE_WALL | WATER | LAVA | PORTAL | PEDESTAL
            | TABLE | PEW | ALTAR | BAR_COUNTER | BED | FIREPLACE | SHELF
    )
}

fn hash2(x: i32, y: i32) -> u32 {
    let h = (x as u32).wrapping_mul(374761393) ^ (y as u32).wrapping_mul(668265263);
    h ^ (h >> 13)
}

fn fill_rect_tiles(x0: i32, y0: i32, w: i32, h: i32, t: u8) {
    let mut y = y0;
    while y < y0 + h {
        let mut x = x0;
        while x < x0 + w {
            set_tile(x, y, t);
            x += 1;
        }
        y += 1;
    }
}

fn stamp_house(x0: i32, y0: i32, w: i32, h: i32, stone: bool) {
    let wall = if stone { STONE_WALL } else { WALL };
    let roof = if stone { STONE_FLOOR } else { ROOF };
    fill_rect_tiles(x0, y0, w, h - 1, roof);
    fill_rect_tiles(x0, y0 + h - 1, w, 1, wall);
    let door_x = x0 + w / 2;
    set_tile(door_x, y0 + h - 1, DOOR);
}

/// Build the village of Bruma: grass, forest border, pond, plaza, houses,
/// the Oblivion gate to the east and the Ayleid relic pedestal to the west.
pub fn init_map() {
    use crate::state::{INTERS, FACING};

    // base grass with variation + flowers
    let mut y = 0;
    while y < MAP_H {
        let mut x = 0;
        while x < MAP_W {
            let h = hash2(x, y);
            let t = if h % 7 == 0 {
                GRASS_DARK
            } else if h % 53 == 0 {
                FLOWER_Y
            } else if h % 97 == 0 {
                FLOWER_P
            } else {
                GRASS
            };
            set_tile(x, y, t);
            x += 1;
        }
        y += 1;
    }

    // forest border
    let mut i = 0;
    while i < MAP_W {
        if hash2(i, -1) % 3 != 0 {
            set_tile(i, 0, TREE);
            set_tile(i, 1, TREE);
        }
        set_tile(i, MAP_H - 1, TREE);
        i += 1;
    }
    let mut j = 2;
    while j < MAP_H - 1 {
        set_tile(0, j, TREE);
        set_tile(MAP_W - 1, j, TREE);
        j += 1;
    }
    let mut k = 0;
    while k < 60 {
        let x = (hash2(k, 11) % (MAP_W as u32 - 4)) as i32 + 2;
        let y = (hash2(k, 22) % 4) as i32 + 2;
        if tile(x, y) == GRASS || tile(x, y) == GRASS_DARK {
            set_tile(x, y, TREE);
        }
        k += 1;
    }

    // pond SE
    fill_rect_tiles(34, 26, 8, 6, WATER);

    // central plaza (cobblestone) with a little well
    fill_rect_tiles(20, 16, 8, 8, COBBLE);
    set_tile(24, 20, ROCK);
    set_tile(23, 20, COBBLE);
    set_tile(25, 20, COBBLE);

    // dirt paths radiating from the plaza
    fill_rect_tiles(24, 26, 1, 8, PATH);
    set_tile(24, 34, PATH);
    set_tile(23, 30, PATH);
    set_tile(25, 30, PATH);
    let mut x = 13;
    while x < 20 {
        set_tile(x, 19, PATH);
        x += 1;
    }
    let mut x = 28;
    while x < 37 {
        set_tile(x, 19, PATH);
        x += 1;
    }
    fill_rect_tiles(36, 13, 1, 7, PATH);
    let mut x = 28;
    while x < 43 {
        set_tile(x, 20, PATH);
        x += 1;
    }
    let mut x = 2;
    while x < 20 {
        set_tile(x, 20, PATH);
        x += 1;
    }

    // buildings
    stamp_house(9, 14, 6, 5, false); // tavern
    stamp_house(34, 6, 8, 6, true);  // chapel
    set_tile(38, 5, STONE_WALL);
    set_tile(38, 4, STONE_WALL);
    stamp_house(16, 28, 5, 4, false);
    stamp_house(30, 28, 5, 4, false);

    // Oblivion gate (east)
    fill_rect_tiles(43, 17, 4, 7, SCORCH);
    set_tile(44, 19, LAVA);
    set_tile(45, 19, LAVA);
    set_tile(44, 20, LAVA);
    set_tile(45, 20, LAVA);
    set_tile(44, 18, PORTAL);
    set_tile(45, 18, PORTAL);
    set_tile(44, 21, PORTAL);
    set_tile(45, 21, PORTAL);
    set_tile(46, 19, LAVA);
    set_tile(46, 20, LAVA);
    set_tile(42, 18, SCORCH);
    set_tile(42, 21, SCORCH);

    // Ayleid relic pedestal (west)
    set_tile(2, 19, ROCK);
    set_tile(3, 19, ROCK);
    set_tile(2, 20, PEDESTAL);
    set_tile(3, 20, PEDESTAL);
    set_tile(2, 21, ROCK);
    set_tile(3, 21, ROCK);

    // rocks around pond
    set_tile(33, 27, ROCK);
    set_tile(42, 27, ROCK);
    set_tile(33, 31, ROCK);
    set_tile(42, 31, ROCK);

    // interactable positions (sprite 20x28; feet at tile center)
    let place = |kind: usize, tx: i32, ty: i32| {
        let wx = tx as f64 * TILE as f64 + (TILE as f64 - 20.0) / 2.0;
        let wy = ty as f64 * TILE as f64 + (TILE as f64 - 28.0);
        unsafe {
            INTERS[kind].x = wx;
            INTERS[kind].y = wy;
            INTERS[kind].w = 20.0;
            INTERS[kind].h = 24.0;
            INTERS[kind].kind = kind as i32;
        }
    };
    place(0, 11, 20); // Mathilde in front of tavern door
    place(1, 37, 12); // Anselm in front of chapel door
    place(2, 24, 31); // Roderick south of plaza
    place(3, 21, 19); // Lyra at plaza edge (appears later)
    unsafe {
        INTERS[4].x = 44.0 * TILE as f64;
        INTERS[4].y = 18.0 * TILE as f64;
        INTERS[4].w = 3.0 * TILE as f64;
        INTERS[4].h = 4.0 * TILE as f64;
        INTERS[4].kind = 4;
        INTERS[5].x = 1.0 * TILE as f64;
        INTERS[5].y = 19.0 * TILE as f64;
        INTERS[5].w = 3.0 * TILE as f64;
        INTERS[5].h = 2.0 * TILE as f64;
        INTERS[5].kind = 5;
    }

    // player start at south entrance
    unsafe {
        FACING = 0;
        CAM_X = 0.0;
        CAM_Y = 0.0;
    }
}

// ---- Interior: Tavern ----
// 24x18 tiles. Wooden floor, bar counter, tables, exit door at bottom.
pub fn init_tavern() {
    use crate::state::{INTERS, INT_W, INT_H, PX, PY, FACING};

    // fill with wood floor
    fill_rect_tiles(0, 0, INT_W, INT_H, WOOD_FLOOR);

    // border walls
    fill_rect_tiles(0, 0, INT_W, 1, WALL);
    fill_rect_tiles(0, INT_H - 1, INT_W, 1, WALL);
    fill_rect_tiles(0, 0, 1, INT_H, WALL);
    fill_rect_tiles(INT_W - 1, 0, 1, INT_H, WALL);

    // exit door at bottom center
    set_tile(INT_W / 2, INT_H - 1, DOOR);

    // bar counter (L-shape) in the upper-left
    fill_rect_tiles(2, 2, 8, 1, BAR_COUNTER);
    fill_rect_tiles(2, 2, 1, 4, BAR_COUNTER);

    // tables with surrounding floor space
    let tables = [(6, 8), (14, 8), (10, 12), (6, 14), (14, 14)];
    for &(tx, ty) in tables.iter() {
        set_tile(tx, ty, TABLE);
        set_tile(tx + 1, ty, TABLE);
    }

    // a warm carpet near the entrance
    fill_rect_tiles(10, 15, 4, 2, CARPET);

    // place the interior NPC (Bauer Konrad) near a table
    unsafe {
        INTERS[0].x = 7.0 * TILE as f64 + 6.0;
        INTERS[0].y = 7.0 * TILE as f64 + 4.0;
        INTERS[0].w = 20.0;
        INTERS[0].h = 24.0;
        INTERS[0].kind = 6;
        // deactivate other interactables
        for k in 1..6 {
            INTERS[k].w = 0.0;
            INTERS[k].h = 0.0;
        }
        // player at entrance (just above the door)
        PX = (INT_W as f64 / 2.0) * TILE as f64 + 6.0;
        PY = (INT_H as f64 - 2.0) * TILE as f64;
        FACING = 0;
        CAM_X = 0.0;
        CAM_Y = 0.0;
    }
}

// ---- Interior: Chapel ----
// 24x18 tiles. Stone floor, pews, altar, exit door at bottom.
pub fn init_chapel() {
    use crate::state::{INTERS, INT_W, INT_H, PX, PY, FACING};

    // fill with cobblestone
    fill_rect_tiles(0, 0, INT_W, INT_H, COBBLE);

    // border stone walls
    fill_rect_tiles(0, 0, INT_W, 1, STONE_WALL);
    fill_rect_tiles(0, INT_H - 1, INT_W, 1, STONE_WALL);
    fill_rect_tiles(0, 0, 1, INT_H, STONE_WALL);
    fill_rect_tiles(INT_W - 1, 0, 1, INT_H, STONE_WALL);

    // exit door at bottom center
    set_tile(INT_W / 2, INT_H - 1, DOOR);

    // altar at the top center
    fill_rect_tiles(10, 2, 4, 2, ALTAR);
    // candle glow behind altar
    set_tile(11, 1, CARPET);
    set_tile(12, 1, CARPET);

    // pews in rows
    let pew_rows = [5, 7, 9, 11, 13];
    for &py in pew_rows.iter() {
        fill_rect_tiles(3, py, 4, 1, PEW);
        fill_rect_tiles(9, py, 6, 1, PEW);
        fill_rect_tiles(17, py, 4, 1, PEW);
    }

    // place the interior NPC (Schwester Mildred) near the altar
    unsafe {
        INTERS[0].x = 10.0 * TILE as f64 + 6.0;
        INTERS[0].y = 4.0 * TILE as f64 + 4.0;
        INTERS[0].w = 20.0;
        INTERS[0].h = 24.0;
        INTERS[0].kind = 7;
        for k in 1..6 {
            INTERS[k].w = 0.0;
            INTERS[k].h = 0.0;
        }
        PX = (INT_W as f64 / 2.0) * TILE as f64 + 6.0;
        PY = (INT_H as f64 - 2.0) * TILE as f64;
        FACING = 0;
        CAM_X = 0.0;
        CAM_Y = 0.0;
    }
}

// ---- Interior: House 1 (Hunter Erik's home) ----
pub fn init_house1() {
    use crate::state::{INTERS, INT_W, INT_H, PX, PY, FACING};

    fill_rect_tiles(0, 0, INT_W, INT_H, WOOD_FLOOR);
    fill_rect_tiles(0, 0, INT_W, 1, WALL);
    fill_rect_tiles(0, INT_H - 1, INT_W, 1, WALL);
    fill_rect_tiles(0, 0, 1, INT_H, WALL);
    fill_rect_tiles(INT_W - 1, 0, 1, INT_H, WALL);
    set_tile(INT_W / 2, INT_H - 1, DOOR);

    // bed in upper-right corner
    fill_rect_tiles(18, 2, 4, 3, BED);
    // fireplace on the left wall
    set_tile(2, 4, FIREPLACE);
    fill_rect_tiles(1, 5, 2, 1, WALL);
    fill_rect_tiles(1, 3, 2, 1, WALL);
    // shelves
    set_tile(6, 2, SHELF);
    set_tile(7, 2, SHELF);
    set_tile(8, 2, SHELF);
    // a table
    set_tile(10, 10, TABLE);
    set_tile(11, 10, TABLE);
    // rug
    fill_rect_tiles(10, 14, 5, 3, CARPET);

    unsafe {
        INTERS[0].x = 6.0 * TILE as f64 + 6.0;
        INTERS[0].y = 6.0 * TILE as f64 + 4.0;
        INTERS[0].w = 20.0;
        INTERS[0].h = 24.0;
        INTERS[0].kind = 8;
        for k in 1..6 {
            INTERS[k].w = 0.0;
            INTERS[k].h = 0.0;
        }
        PX = (INT_W as f64 / 2.0) * TILE as f64 + 6.0;
        PY = (INT_H as f64 - 2.0) * TILE as f64;
        FACING = 0;
        CAM_X = 0.0;
        CAM_Y = 0.0;
    }
}

// ---- Interior: House 2 (Alchemist Sora's home) ----
pub fn init_house2() {
    use crate::state::{INTERS, INT_W, INT_H, PX, PY, FACING};

    fill_rect_tiles(0, 0, INT_W, INT_H, WOOD_FLOOR);
    fill_rect_tiles(0, 0, INT_W, 1, WALL);
    fill_rect_tiles(0, INT_H - 1, INT_W, 1, WALL);
    fill_rect_tiles(0, 0, 1, INT_H, WALL);
    fill_rect_tiles(INT_W - 1, 0, 1, INT_H, WALL);
    set_tile(INT_W / 2, INT_H - 1, DOOR);

    // shelves full of flasks along the left wall
    fill_rect_tiles(2, 2, 1, 8, SHELF);
    fill_rect_tiles(2, 11, 1, 5, SHELF);
    // table with alchemy equipment
    set_tile(8, 3, TABLE);
    set_tile(9, 3, TABLE);
    set_tile(10, 3, TABLE);
    // bed in the upper-right
    fill_rect_tiles(18, 2, 4, 3, BED);
    // fireplace
    set_tile(18, 10, FIREPLACE);
    fill_rect_tiles(17, 11, 2, 1, WALL);
    fill_rect_tiles(17, 9, 2, 1, WALL);
    // carpet
    fill_rect_tiles(9, 10, 6, 4, CARPET);

    unsafe {
        INTERS[0].x = 8.0 * TILE as f64 + 6.0;
        INTERS[0].y = 7.0 * TILE as f64 + 4.0;
        INTERS[0].w = 20.0;
        INTERS[0].h = 24.0;
        INTERS[0].kind = 9;
        for k in 1..6 {
            INTERS[k].w = 0.0;
            INTERS[k].h = 0.0;
        }
        PX = (INT_W as f64 / 2.0) * TILE as f64 + 6.0;
        PY = (INT_H as f64 - 2.0) * TILE as f64;
        FACING = 0;
        CAM_X = 0.0;
        CAM_Y = 0.0;
    }
}

fn sx(wx: f64) -> f64 {
    unsafe { wx - CAM_X }
}

fn sy(wy: f64) -> f64 {
    unsafe { wy - CAM_Y }
}

/// Draw a single tile at world coords -> screen coords.
pub fn draw_tile(tx: i32, ty: i32) {
    let t = tile(tx, ty);
    let wx = tx as f64 * TILE as f64;
    let wy = ty as f64 * TILE as f64;
    let x = sx(wx);
    let y = sy(wy);
    let s = TILE as f64;
    let time = now() / 1000.0;
    let h = hash2(tx, ty);

    match t {
        GRASS => {
            set_color(rgb(86, 134, 60), 255);
            rect(x, y, s, s);
            if h % 3 == 0 {
                set_color(rgb(72, 116, 48), 255);
                rect(x + 4.0, y + 6.0, 3.0, 3.0);
                rect(x + 18.0, y + 20.0, 3.0, 3.0);
            }
        }
        GRASS_DARK => {
            set_color(rgb(72, 116, 48), 255);
            rect(x, y, s, s);
            set_color(rgb(86, 134, 60), 255);
            rect(x + 8.0, y + 12.0, 3.0, 3.0);
        }
        FLOWER_Y => {
            set_color(rgb(86, 134, 60), 255);
            rect(x, y, s, s);
            set_color(rgb(235, 213, 110), 255);
            circle(x + 10.0, y + 10.0, 2.0);
            circle(x + 22.0, y + 20.0, 2.0);
            set_color(rgb(70, 50, 30), 255);
            circle(x + 10.0, y + 10.0, 0.7);
            circle(x + 22.0, y + 20.0, 0.7);
        }
        FLOWER_P => {
            set_color(rgb(86, 134, 60), 255);
            rect(x, y, s, s);
            set_color(rgb(220, 90, 120), 255);
            circle(x + 14.0, y + 18.0, 2.0);
        }
        PATH => {
            set_color(rgb(150, 122, 82), 255);
            rect(x, y, s, s);
            set_color(rgb(130, 104, 70), 255);
            rect(x + (h % 8) as f64, y + (h % 5) as f64 * 2.0, 2.0, 2.0);
            rect(x + 20.0, y + 10.0, 2.0, 2.0);
        }
        COBBLE => {
            set_color(rgb(120, 120, 128), 255);
            rect(x, y, s, s);
            set_color(rgb(140, 140, 148), 255);
            circle(x + 8.0, y + 8.0, 4.0);
            circle(x + 22.0, y + 22.0, 4.0);
            set_color(rgb(100, 100, 108), 255);
            circle(x + 8.0, y + 24.0, 3.0);
        }
        WATER => {
            let wob = sin(time * 1.5 + (h as f64) * 0.3) * 8.0;
            set_color(rgb(40 + wob as i32, 90, 150), 255);
            rect(x, y, s, s);
            set_color(rgb(90, 150, 200), 160);
            let off = sin(time * 2.0 + h as f64) * 3.0;
            rect(x + 4.0, y + 10.0 + off, 10.0, 2.0);
            rect(x + 16.0, y + 22.0 - off, 8.0, 2.0);
        }
        TREE => {
            set_color(rgb(72, 116, 48), 255);
            rect(x, y, s, s);
            set_color(rgb(90, 60, 40), 255);
            rect(x + 13.0, y + 20.0, 6.0, 10.0);
            let cv = (h % 3) as i32;
            let g = 92 - cv * 12;
            set_color(rgb(44, g, 42), 255);
            circle(x + 16.0, y + 14.0, 12.0);
            circle(x + 9.0, y + 18.0, 8.0);
            circle(x + 23.0, y + 18.0, 8.0);
            set_color(rgb(60, g + 20, 56), 255);
            circle(x + 13.0, y + 11.0, 4.0);
        }
        ROCK => {
            set_color(rgb(72, 116, 48), 255);
            rect(x, y, s, s);
            set_color(rgb(110, 110, 120), 255);
            circle(x + 16.0, y + 20.0, 11.0);
            set_color(rgb(140, 140, 150), 255);
            circle(x + 12.0, y + 17.0, 4.0);
        }
        WOOD_FLOOR => {
            set_color(rgb(120, 88, 56), 255);
            rect(x, y, s, s);
            set_color(rgb(100, 72, 44), 255);
            rect(x, y + 15.0, s, 2.0);
        }
        WALL => {
            set_color(rgb(120, 80, 50), 255);
            rect(x, y, s, s);
            set_color(rgb(96, 62, 38), 255);
            rect(x, y + 15.0, s, 2.0);
            set_color(rgb(140, 96, 60), 255);
            rect(x + 4.0, y + 4.0, 6.0, 6.0);
            rect(x + 20.0, y + 4.0, 6.0, 6.0);
        }
        ROOF => {
            set_color(rgb(150, 50, 45), 255);
            rect(x, y, s, s);
            set_color(rgb(180, 70, 60), 255);
            tri(x, y + s, x + s, y + s, x + s / 2.0, y);
            set_color(rgb(120, 38, 35), 255);
            rect(x, y + s - 3.0, s, 3.0);
        }
        DOOR => {
            set_color(rgb(70, 45, 30), 255);
            rect(x, y, s, s);
            set_color(rgb(40, 25, 18), 255);
            rect(x + 8.0, y + 4.0, 16.0, s - 4.0);
            set_color(rgb(220, 180, 70), 255);
            circle(x + 20.0, y + 16.0, 1.5);
        }
        STONE_FLOOR => {
            set_color(rgb(150, 50, 45), 255);
            rect(x, y, s, s);
            set_color(rgb(180, 70, 60), 255);
            tri(x, y + s, x + s, y + s, x + s / 2.0, y);
        }
        STONE_WALL => {
            set_color(rgb(130, 130, 140), 255);
            rect(x, y, s, s);
            set_color(rgb(100, 100, 110), 255);
            rect(x, y + 14.0, s, 4.0);
            set_color(rgb(150, 150, 160), 255);
            rect(x + 4.0, y + 4.0, 8.0, 6.0);
            rect(x + 18.0, y + 4.0, 8.0, 6.0);
        }
        LAVA => {
            let flick = sin(time * 4.0 + h as f64) * 20.0;
            set_color(rgb(220, 70 + flick as i32, 30), 255);
            rect(x, y, s, s);
            set_color(rgb(255, 180, 60), 200);
            circle(x + 8.0, y + 10.0, 2.5);
            circle(x + 22.0, y + 20.0, 2.5);
        }
        PORTAL => {
            let p = sin(time * 2.5 + h as f64 * 0.5) * 0.5 + 0.5;
            set_color(rgb(60, 20, 70), 255);
            rect(x, y, s, s);
            set_color(rgb(130, 50 + (p * 60.0) as i32, 180), 220);
            circle(x + 16.0, y + 16.0, 11.0);
            set_color(rgb(200, 90 + (p * 80.0) as i32, 220), 200);
            circle(x + 16.0, y + 16.0, 6.0);
            set_color(rgb(240, 200, 255), 220);
            circle(x + 16.0, y + 16.0, 2.0);
        }
        SCORCH => {
            set_color(rgb(50, 40, 35), 255);
            rect(x, y, s, s);
            set_color(rgb(30, 24, 22), 255);
            circle(x + 10.0, y + 12.0, 3.0);
            circle(x + 22.0, y + 20.0, 3.0);
        }
        PEDESTAL => {
            set_color(rgb(72, 116, 48), 255);
            rect(x, y, s, s);
            set_color(rgb(150, 150, 165), 255);
            rect(x + 6.0, y + 12.0, 20.0, 16.0);
            set_color(rgb(120, 120, 135), 255);
            rect(x + 4.0, y + 10.0, 24.0, 4.0);
            rect(x + 4.0, y + 26.0, 24.0, 4.0);
            let glow = sin(time * 2.0) * 0.5 + 0.5;
            if crate::state::has(F_KNOWS_RELIC) {
                set_color(rgb(120, 200, 255), 200);
                circle(x + 16.0, y + 20.0, 2.0 + glow * 2.0);
            } else {
                set_color(rgb(80, 80, 90), 200);
                circle(x + 16.0, y + 20.0, 2.0);
            }
        }
        TABLE => {
            set_color(rgb(100, 70, 40), 255);
            rect(x, y, s, s);
            set_color(rgb(130, 95, 55), 255);
            circle(x + (s / 2.0), y + (s / 2.0), 10.0);
            set_color(rgb(160, 120, 75), 255);
            circle(x + (s / 2.0), y + (s / 2.0), 6.0);
        }
        PEW => {
            set_color(rgb(110, 78, 48), 255);
            rect(x, y, s, s);
            set_color(rgb(90, 62, 38), 255);
            rect(x + 2.0, y + 4.0, s - 4.0, 6.0);
            rect(x + 2.0, y + 16.0, s - 4.0, 4.0);
            set_color(rgb(130, 95, 58), 255);
            rect(x + 2.0, y + 4.0, s - 4.0, 2.0);
        }
        ALTAR => {
            set_color(rgb(180, 175, 185), 255);
            rect(x, y, s, s);
            set_color(rgb(150, 145, 158), 255);
            rect(x + 2.0, y + 2.0, s - 4.0, s - 4.0);
            set_color(rgb(220, 200, 100), 200);
            circle(x + s / 2.0, y + s / 2.0, 3.0);
        }
        CARPET => {
            set_color(rgb(120, 40, 50), 255);
            rect(x, y, s, s);
            set_color(rgb(150, 60, 70), 255);
            rect(x + 4.0, y + 4.0, s - 8.0, s - 8.0);
            set_color(rgb(180, 100, 80), 255);
            circle(x + s / 2.0, y + s / 2.0, 3.0);
        }
        BAR_COUNTER => {
            set_color(rgb(90, 60, 35), 255);
            rect(x, y, s, s);
            set_color(rgb(70, 45, 25), 255);
            rect(x, y + s - 4.0, s, 4.0);
            set_color(rgb(110, 78, 48), 255);
            rect(x + 2.0, y + 4.0, s - 4.0, 3.0);
        }
        BED => {
            set_color(rgb(120, 80, 50), 255);
            rect(x, y, s, s);
            set_color(rgb(180, 160, 140), 255);
            rect(x + 2.0, y + 4.0, s - 4.0, 10.0);
            set_color(rgb(100, 70, 120), 255);
            rect(x + 2.0, y + 2.0, s - 4.0, 4.0);
            set_color(rgb(140, 110, 80), 255);
            rect(x, y + s - 6.0, s, 6.0);
        }
        FIREPLACE => {
            set_color(rgb(80, 50, 30), 255);
            rect(x, y, s, s);
            set_color(rgb(50, 30, 20), 255);
            rect(x + 6.0, y + 8.0, s - 12.0, s - 8.0);
            let flick = sin(time * 5.0 + h as f64) * 30.0;
            set_color(rgb(220, 100 + flick as i32, 30), 255);
            tri(x + 10.0, y + s - 4.0, x + 22.0, y + s - 4.0, x + 16.0, y + 10.0 + flick.abs() as f64);
            set_color(rgb(255, 200, 80), 200);
            circle(x + 16.0, y + s - 8.0, 3.0);
        }
        SHELF => {
            set_color(rgb(90, 60, 35), 255);
            rect(x, y, s, s);
            set_color(rgb(70, 45, 25), 255);
            rect(x, y + 4.0, s, 3.0);
            rect(x, y + 14.0, s, 3.0);
            rect(x, y + 24.0, s, 3.0);
            // bottles on shelves
            set_color(rgb(100, 180, 120), 255);
            circle(x + 6.0, y + 8.0, 2.0);
            circle(x + 22.0, y + 8.0, 1.5);
            set_color(rgb(180, 100, 80), 255);
            circle(x + 10.0, y + 18.0, 2.0);
            circle(x + 22.0, y + 18.0, 1.5);
            set_color(rgb(80, 140, 200), 255);
            circle(x + 8.0, y + 28.0, 2.0);
        }
        _ => {
            set_color(rgb(255, 0, 255), 255);
            rect(x, y, s, s);
        }
    }
}
