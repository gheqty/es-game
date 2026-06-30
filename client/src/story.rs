//! Dialogue tree logic: opening nodes, applying effects, choosing options,
//! and deciding which node an interactable opens with.

use crate::dialogue::{Node, NODES};
use crate::state::{
    has, l, wrap_text, CUR_NODE, DIALOG_ANNIM, EFF_ENDING, F_ACCEPTED_LYRA, F_ASSAULT,
    F_GATE_VISITED_EARLY, F_HAS_RELIC, F_KNOWS_RELIC, F_REFUSED_LYRA, F_RODERICK_LINE,
    F_TALKED_ANSELM, F_TALKED_ERIK, F_TALKED_LYRA, F_TALKED_MATHILDE, F_TALKED_MILDRED,
    F_TALKED_RODERICK, F_TALKED_SORA, F_TOLD_LYRA, ST_DIALOG,
};

pub fn node_count() -> usize {
    NODES.len()
}

pub fn node(id: i32) -> &'static Node {
    &NODES[id as usize]
}

pub fn nodes() -> &'static [Node] {
    NODES
}

pub fn cur_node() -> i32 {
    unsafe { CUR_NODE }
}

pub fn open_node(id: i32) {
    unsafe {
        CUR_NODE = id;
        crate::state::STATE = ST_DIALOG;
        DIALOG_ANNIM = 0.0;
    }
    rewrap_current();
}

pub fn rewrap_current() {
    let id = cur_node();
    if id >= 0 && (id as usize) < NODES.len() {
        let max_w = (crate::state::CANVAS_W - 80) as f64;
        let text = l(NODES[id as usize].text_de, NODES[id as usize].text_en);
        wrap_text(text, max_w, 17.0);
    }
}

pub fn close_dialog() {
    unsafe {
        CUR_NODE = -1;
        crate::state::STATE = crate::state::ST_ROAM;
    }
}

pub fn apply_effect(effect: i32, val: i32) {
    use crate::state::{EFF_CLEAR_FLAG, EFF_SET_FLAG, EFF_SET_PROGRESS, ENDING_ID, FLAGS,
        PROGRESS, STATE, ST_ENDING};
    match effect {
        EFF_SET_PROGRESS => unsafe { PROGRESS = PROGRESS.max(val) },
        crate::state::EFF_SET_FLAG => unsafe { FLAGS |= val as u32 },
        EFF_CLEAR_FLAG => unsafe { FLAGS &= !(val as u32) },
        EFF_ENDING => unsafe {
            ENDING_ID = val;
            STATE = ST_ENDING;
        },
        _ => {}
    }
}

pub fn select_choice(idx: i32) {
    let id = cur_node();
    if id < 0 || (id as usize) >= NODES.len() {
        return;
    }
    let n = &NODES[id as usize];
    if idx < 0 || (idx as usize) >= n.choices.len() {
        return;
    }
    let ch = &n.choices[idx as usize];
    let eff = ch.effect;
    let val = ch.val;
    let next = ch.next;
    apply_effect(eff, val);
    bump_progress();
    if eff == EFF_ENDING {
        return;
    }
    if next < 0 {
        close_dialog();
    } else {
        open_node(next);
    }
}

fn bump_progress() {
    use crate::state::PROGRESS;
    let mut p = 0;
    if has(F_TALKED_MATHILDE) {
        p = p.max(1);
    }
    if has(F_KNOWS_RELIC) {
        p = p.max(2);
    }
    if has(F_HAS_RELIC) {
        p = p.max(3);
    }
    if has(F_RODERICK_LINE) || has(F_ASSAULT) || has(F_ACCEPTED_LYRA) {
        p = p.max(4);
    }
    unsafe { PROGRESS = PROGRESS.max(p) };
}

/// Has the player been warned about the cult by an interior NPC?
fn has_been_warned() -> bool {
    has(F_TALKED_MILDRED) || has(F_TALKED_ERIK) || has(F_TALKED_SORA)
}

pub fn start_node_for(kind: i32) -> i32 {
    match kind {
        // 0: Mathilde
        0 => {
            if !has(F_TALKED_MATHILDE) {
                0
            } else {
                2
            }
        }
        // 1: Anselm
        1 => {
            if has(F_TALKED_ANSELM) && has(F_KNOWS_RELIC) {
                6
            } else if has(F_TALKED_ANSELM) {
                5
            } else {
                3
            }
        }
        // 2: Roderick
        2 => {
            // If already holding the line or committed to assault, brief repeat
            if has(F_RODERICK_LINE) || has(F_ASSAULT) {
                45
            // First meeting: always get the full greeting (#8 fix)
            } else if !has(F_TALKED_RODERICK) {
                if has(F_GATE_VISITED_EARLY) {
                    46 // ack that player visited the gate (#4 fix)
                } else {
                    9
                }
            // Returning with relic → holds the line
            } else if has(F_HAS_RELIC) {
                13
            // Returning without relic → plan discussion
            } else {
                11
            }
        }
        // 3: Lyra
        3 => {
            if has(F_TALKED_LYRA) {
                // Repeat: colder if she was refused (#3 fix)
                if has(F_REFUSED_LYRA) {
                    49
                } else {
                    17
                }
            } else if has_been_warned() {
                // First meeting but player has cult warnings → confrontation option (#9/#10 fix)
                47
            } else {
                14
            }
        }
        // 4: Oblivion gate
        4 => gate_node(),
        // 5: Relic pedestal
        5 => {
            if has(F_HAS_RELIC) {
                -1
            } else if has(F_KNOWS_RELIC) {
                22
            } else {
                21
            }
        }
        // 6: Konrad (tavern interior) — reacts to relic (#7 fix)
        6 => {
            if has(F_HAS_RELIC) {
                50
            } else {
                31
            }
        }
        // 7: Mildred (chapel interior) — reacts to relic (#7 fix)
        7 => {
            if has(F_HAS_RELIC) {
                51
            } else {
                35
            }
        }
        // 8: Erik (house 1 interior) — reacts to relic (#7 fix)
        8 => {
            if has(F_HAS_RELIC) {
                52
            } else {
                39
            }
        }
        // 9: Sora (house 2 interior) — reacts to relic (#7 fix)
        9 => {
            if has(F_HAS_RELIC) {
                53
            } else {
                42
            }
        }
        _ => -1,
    }
}

fn gate_node() -> i32 {
    if !has(F_HAS_RELIC) && !has(F_ASSAULT) {
        return 23; // not ready
    }
    // Went with Lyra → betrayal at the gate
    if has(F_TOLD_LYRA) && has(F_ACCEPTED_LYRA) {
        return 26;
    }
    // Told Lyra about the relic but refused her help → she ambushes (#2 fix)
    if has(F_TOLD_LYRA) && !has(F_ACCEPTED_LYRA) {
        return 25;
    }
    // Solo seal (never told Lyra about the relic)
    if has(F_HAS_RELIC) {
        return 24;
    }
    // Assault without relic
    27
}
