//! All screen rendering: tiles, characters, dialogue UI, HUD and endings.

use crate::ffi::{circle, measure, rect, rgb, set_color, text, tri};
use crate::math::sin;
use crate::state::{
    has, l, now, state, CANVAS_H, CANVAS_W, CAM_X, CAM_Y, ENDING_ID, FACING,
    F_KNOWS_RELIC, F_TOLD_LYRA, HOVER, INTERS, LANG, LANG_DE, MAP_MODE, MAP_WORLD,
    MOVING, MOUSE_X, MOUSE_Y, PX, PY, ST_ROAM, TILE, WALK_PHASE, WRAP_COUNT, WRAP_LINES,
};
use crate::story::{cur_node, node, nodes};
use crate::world::draw_tile;

// ---- screen-space conversion ----
fn sx(wx: f64) -> f64 {
    unsafe { wx - CAM_X }
}
fn sy(wy: f64) -> f64 {
    unsafe { wy - CAM_Y }
}

// ---- map ----
pub fn draw_map() {
    let x0 = (unsafe { CAM_X } / TILE as f64) as i32;
    let y0 = (unsafe { CAM_Y } / TILE as f64) as i32;
    let x1 = x0 + CANVAS_W / TILE + 2;
    let y1 = y0 + CANVAS_H / TILE + 2;
    let mut ty = y0;
    while ty <= y1 {
        let mut tx = x0;
        while tx <= x1 {
            draw_tile(tx, ty);
            tx += 1;
        }
        ty += 1;
    }
}

// ---- characters ----
pub fn draw_character(wx: f64, wy: f64, cloak: u32, head: u32, facing: i32, bob: f64, talk: bool) {
    let x = sx(wx);
    let y = sy(wy);
    let bob = bob * 1.5;
    set_color(0x000000, 70);
    circle(x + 10.0, y + 27.0, 9.0);
    set_color(cloak, 255);
    rect(x + 3.0, y + 10.0 + bob, 14.0, 16.0);
    tri(x + 3.0, y + 24.0 + bob, x + 17.0, y + 24.0 + bob, x + 10.0, y + 27.0 + bob);
    set_color(crate::ffi::darken(cloak, 40), 255);
    rect(x + 3.0, y + 18.0 + bob, 14.0, 3.0);
    set_color(head, 255);
    circle(x + 10.0, y + 6.0 + bob, 5.0);
    set_color(crate::ffi::darken(cloak, 25), 255);
    circle(x + 10.0, y + 4.0 + bob, 6.0);
    if facing != 3 {
        set_color(0x202020, 255);
        let (ex, ey) = match facing {
            0 => (x + 8.0, y + 7.0 + bob),
            1 => (x + 7.0, y + 6.5 + bob),
            2 => (x + 12.0, y + 6.5 + bob),
            _ => (x + 10.0, y + 6.0 + bob),
        };
        circle(ex, ey, 1.0);
    }
    if talk {
        set_color(0x301010, 255);
        circle(x + 10.0, y + 8.0 + bob, 1.0);
    }
}

fn draw_npc(it: &crate::state::Inter, cloak: u32, head: u32, facing: i32, active: bool) {
    let bob = if active { sin(now() / 250.0) } else { 0.0 };
    draw_character(it.x, it.y, cloak, head, facing, bob, active);
}

pub fn draw_interactables() {
    let cur = cur_node();
    let nodes_ref = nodes();
    let active_speaker = if cur >= 0 && (cur as usize) < nodes_ref.len() {
        nodes_ref[cur as usize].speaker
    } else {
        ""
    };

    let in_world = unsafe { MAP_MODE } == MAP_WORLD;

    if in_world {
        draw_npc(unsafe { &INTERS[0] }, rgb(180, 120, 70), rgb(220, 180, 140), 0, active_speaker == "Mathilde");
        draw_npc(unsafe { &INTERS[1] }, rgb(180, 180, 195), rgb(225, 200, 175), 0, active_speaker == "Bruder Anselm");
        draw_npc(unsafe { &INTERS[2] }, rgb(180, 60, 50), rgb(225, 195, 160), 0, active_speaker == "Wache Roderick");
        if crate::actor::lyra_present() {
            draw_npc(unsafe { &INTERS[3] }, rgb(60, 30, 70), rgb(220, 200, 190), 0, active_speaker == "Lyra");
        }
    } else {
        let it = unsafe { &INTERS[0] };
        if it.w > 0.0 {
            let kind = it.kind;
            let (cloak, head) = match kind {
                6 => (rgb(160, 120, 60), rgb(210, 180, 140)),  // Konrad
                7 => (rgb(200, 190, 160), rgb(220, 200, 175)),   // Mildred
                8 => (rgb(100, 130, 70), rgb(210, 180, 145)),    // Erik
                9 => (rgb(60, 160, 120), rgb(215, 190, 175)),   // Sora
                _ => (rgb(120, 120, 120), rgb(200, 180, 160)),
            };
            let active = match kind {
                6 => active_speaker == "Bauer Konrad",
                7 => active_speaker == "Schwester Mildred",
                8 => active_speaker == "Jäger Erik",
                9 => active_speaker == "Alchemistin Sora",
                _ => false,
            };
            draw_npc(it, cloak, head, 0, active);
        }
    }

    // interaction prompt marker over nearest interactable
    if state() == ST_ROAM {
        let (px, py) = unsafe { (PX, PY) };
        let cx = px + 10.0;
        let cy = py + 22.0;
        let mut best: i32 = -1;
        let mut best_d = 9999.0;
        let limit = if in_world { 6 } else { 1 };
        let mut k = 0;
        while k < limit {
            unsafe {
                let it = &INTERS[k];
                if it.w <= 0.0 || it.h <= 0.0 {
                    k += 1;
                    continue;
                }
                if k == 3 && !crate::actor::lyra_present() {
                    k += 1;
                    continue;
                }
                let d = crate::actor::dist_to_box_pub(cx, cy, it.x, it.y, it.w, it.h);
                if d < 48.0 && d < best_d {
                    best_d = d;
                    best = k as i32;
                }
            }
            k += 1;
        }
        if best >= 0 {
            let it = unsafe { &INTERS[best as usize] };
            let bob = sin(now() / 200.0) * 3.0;
            set_color(0xffe070, 230);
            circle(sx(it.x + it.w / 2.0), sy(it.y - 8.0) + bob, 4.0);
            set_color(0x000000, 200);
            text("E", sx(it.x + it.w / 2.0) - 4.0, sy(it.y - 5.0) + bob, 12.0);
        }
    }
}

pub fn draw_player() {
    let bob = if unsafe { MOVING } { sin(unsafe { WALK_PHASE }) } else { 0.0 };
    draw_character(
        unsafe { PX },
        unsafe { PY },
        rgb(58, 80, 130),
        rgb(225, 190, 150),
        unsafe { FACING },
        bob,
        false,
    );
}

// ---- HUD ----
fn quest_status() -> &'static str {
    if unsafe { ENDING_ID != 0 } {
        ""
    } else if has(F_TOLD_LYRA) && has(crate::state::F_ACCEPTED_LYRA) {
        l("Auftrag: Geht mit Lyra zum Tor", "Quest: Go to the gate with Lyra")
    } else if has(crate::state::F_HAS_RELIC) {
        l("Auftrag: Tragt das Siegel zum Höllentor", "Quest: Bring the seal to the gate")
    } else if has(F_KNOWS_RELIC) {
        l("Auftrag: Holt das Herz von Akatosh aus den Westruinen", "Quest: Retrieve the Heart of Akatosh from the western ruins")
    } else if has(crate::state::F_TALKED_MATHILDE) || has(crate::state::F_TALKED_ANSELM) {
        l("Auftrag: Findet einen Weg, das Tor zu schließen", "Quest: Find a way to close the gate")
    } else {
        l("Auftrag: Sprecht mit den Dorfbewohnern", "Quest: Talk to the villagers")
    }
}

pub fn draw_hud() {
    set_color(0x0b0b14, 200);
    rect(0.0, 0.0, CANVAS_W as f64, 28.0);
    set_color(0x9affb0, 255);
    text(l("Schatten über Bruma", "Shadow over Bruma"), 10.0, 19.0, 16.0);

    // language indicator
    let lang_label = if unsafe { LANG == LANG_DE } { "DE" } else { "EN" };
    let lang_w = measure(lang_label, 12.0);
    set_color(0x6b6b85, 255);
    text(lang_label, (CANVAS_W as f64) / 2.0 - lang_w / 2.0, 19.0, 12.0);

    let status = quest_status();
    let w = measure(status, 14.0);
    set_color(0xc8c8d8, 255);
    text(status, CANVAS_W as f64 - w - 10.0, 19.0, 14.0);

    if state() == ST_ROAM {
        set_color(0x0b0b14, 180);
        rect(0.0, CANVAS_H as f64 - 22.0, CANVAS_W as f64, 22.0);
        set_color(0x8a8aa0, 255);
        text(
            l("WASD/Pfeile: bewegen    E: sprechen    Türen: hindurchgehen    L: Sprache",
              "WASD/Arrows: move    E: talk    Doors: walk through    L: language"),
            10.0,
            CANVAS_H as f64 - 8.0,
            13.0,
        );
    }
}

// ---- dialogue ----
fn digit_str(n: i32) -> &'static str {
    match n {
        1 => "1", 2 => "2", 3 => "3", 4 => "4", 5 => "5",
        6 => "6", 7 => "7", 8 => "8", 9 => "9", _ => "?",
    }
}

pub fn draw_dialog() {
    let node_id = cur_node();
    let nodes_ref = nodes();
    if node_id < 0 || (node_id as usize) >= nodes_ref.len() {
        return;
    }
    let n = node(node_id);

    // ---- Dynamic panel sizing ----
    // Compute the space needed: speaker header + body lines + choices.
    let line_h = 20.0;
    let speaker_h = if n.speaker.is_empty() { 8.0 } else { 28.0 };
    let choice_h = 26.0;
    let choices_len = n.choices.len() as f64;
    let wrap_count = unsafe { WRAP_COUNT } as f64;

    // Body text area: cap at the maximum that fits without overlapping choices.
    // Total panel = padding(14) + speaker + body + gap(8) + choices + padding(14)
    // We want the panel to grow with content but never exceed 70% of canvas height.
    let max_panel = (CANVAS_H as f64) * 0.72;
    let needed = 14.0 + speaker_h + wrap_count * line_h + 8.0 + choices_len * choice_h + 14.0;
    let panel_h = needed.min(max_panel);

    // How many body lines actually fit?
    let body_area_h = panel_h - 14.0 - speaker_h - 8.0 - choices_len * choice_h - 14.0;
    let max_lines = ((body_area_h / line_h) as i32).max(1) as usize;
    let lines_to_draw = (unsafe { WRAP_COUNT } as usize).min(max_lines);

    let py = CANVAS_H as f64 - panel_h - 10.0;

    set_color(0x000000, 90);
    rect(0.0, 0.0, CANVAS_W as f64, CANVAS_H as f64);
    set_color(0x14141f, 245);
    rect(20.0, py, CANVAS_W as f64 - 40.0, panel_h);
    set_color(n.color, 255);
    rect(20.0, py, 4.0, panel_h);

    let mut ty = py + 14.0;
    if !n.speaker.is_empty() {
        set_color(n.color, 255);
        text(n.speaker, 36.0, ty + 14.0, 18.0);
        ty += speaker_h;
    }

    let body_text = l(n.text_de, n.text_en);
    set_color(0xe6e6f0, 255);
    let mut i = 0;
    while i < lines_to_draw {
        let (s, e) = unsafe { WRAP_LINES[i] };
        let line = &body_text[s..e];
        text(line, 36.0, ty + 16.0, 16.0);
        ty += line_h;
        i += 1;
    }

    // If we truncated lines, show an ellipsis
    if lines_to_draw < unsafe { WRAP_COUNT } {
        set_color(0x8a8aa0, 255);
        text("...", 36.0, ty + 16.0, 16.0);
    }

    // Choices are anchored to the bottom of the panel
    let choices = n.choices;
    let mut cy = py + panel_h - choices_len * choice_h - 14.0;
    let mouse = unsafe { (MOUSE_X, MOUSE_Y) };
    unsafe { HOVER = -1 };
    for (idx, ch) in choices.iter().enumerate() {
        let bx = 36.0;
        let bw = CANVAS_W as f64 - 72.0;
        let bh = 22.0;
        let hovered = mouse.0 >= bx && mouse.0 <= bx + bw && mouse.1 >= cy && mouse.1 <= cy + bh;
        if hovered {
            unsafe { HOVER = idx as i32 };
            set_color(n.color, 40);
            rect(bx, cy, bw, bh);
        }
        set_color(0x2a2a3a, 255);
        rect(bx, cy, 3.0, bh);
        set_color(n.color, 255);
        text(digit_str(idx as i32 + 1), bx + 10.0, cy + 16.0, 14.0);
        set_color(if hovered { 0xffffff } else { 0xc8c8d8 }, 255);
        text(l(ch.text_de, ch.text_en), bx + 30.0, cy + 16.0, 14.0);
        cy += choice_h;
    }
}

// ---- ending ----
pub fn draw_ending() {
    let eid = unsafe { ENDING_ID };
    let (title, color, sub) = match eid {
        1 => (
            l("HELD VON BRUMA", "HERO OF BRUMA"),
            0xffe070,
            l("Das Tor ist geschlossen. Bruma lebt.", "The gate is closed. Bruma lives."),
        ),
        2 => (
            l("VERRATEN", "BETRAYED"),
            0x9a6bd0,
            l("Die Mythische Morgenröte hat gesiegt. Bruma brennt.", "The Mythic Dawn has triumphed. Bruma burns."),
        ),
        3 => (
            l("DAS OPFER", "THE SACRIFICE"),
            0xb83c34,
            l("Ihr habt Euer Leben für Bruma gegeben.", "You gave your life for Bruma."),
        ),
        _ => (l("ENDE", "END"), 0xffffff, ""),
    };
    set_color(0x000000, 200);
    rect(0.0, 0.0, CANVAS_W as f64, CANVAS_H as f64);
    let tw = measure(title, 40.0);
    set_color(color, 255);
    text(title, (CANVAS_W as f64 - tw) / 2.0, CANVAS_H as f64 / 2.0 - 10.0, 40.0);
    let sw = measure(sub, 16.0);
    set_color(0xc8c8d8, 255);
    text(sub, (CANVAS_W as f64 - sw) / 2.0, CANVAS_H as f64 / 2.0 + 30.0, 16.0);
    let hint = l("Beliebige Taste für ein neues Spiel", "Press any key for a new game");
    let hw = measure(hint, 14.0);
    set_color(0x6b6b85, 255);
    text(hint, (CANVAS_W as f64 - hw) / 2.0, CANVAS_H as f64 - 30.0, 14.0);
}
