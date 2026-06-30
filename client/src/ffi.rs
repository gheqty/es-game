//! Raw `extern "C"` bindings to the browser, plus thin safe wrappers.
//!
//! The WASM module imports these symbols from the `env` object the HTML
//! page passes to `WebAssembly.instantiate`. Symbols not provided at link
//! time are turned into imports via the `--import-undefined` linker flag
//! (set in `build.rs`).

extern "C" {
    pub fn js_clear();
    pub fn js_fill_rect(x: f64, y: f64, w: f64, h: f64);
    pub fn js_fill_circle(x: f64, y: f64, r: f64);
    pub fn js_fill_triangle(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64);
    pub fn js_set_color(r: i32, g: i32, b: i32, a: i32);
    pub fn js_fill_text(ptr: *const u8, len: i32, x: f64, y: f64, size: f64);
    pub fn js_measure_text(ptr: *const u8, len: i32, size: f64) -> f64;
    pub fn js_request_frame();
    pub fn js_now_ms() -> f64;
}

/// Pack an (r,g,b) triple into a 0xRRGGBB u32.
pub fn rgb(r: i32, g: i32, b: i32) -> u32 {
    ((r as u32 & 0xff) << 16) | ((g as u32 & 0xff) << 8) | (b as u32 & 0xff)
}

/// Darken a packed color by subtracting `amount` from each channel.
pub fn darken(c: u32, amount: i32) -> u32 {
    let r = (((c >> 16) & 0xff) as i32 - amount).max(0) as u32;
    let g = (((c >> 8) & 0xff) as i32 - amount).max(0) as u32;
    let b = ((c & 0xff) as i32 - amount).max(0) as u32;
    (r << 16) | (g << 8) | b
}

pub fn now_ms() -> f64 {
    unsafe { js_now_ms() }
}

pub fn clear() {
    unsafe { js_clear() }
}

pub fn rect(x: f64, y: f64, w: f64, h: f64) {
    unsafe { js_fill_rect(x, y, w, h) }
}

pub fn circle(x: f64, y: f64, r: f64) {
    unsafe { js_fill_circle(x, y, r) }
}

pub fn tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64) {
    unsafe { js_fill_triangle(x1, y1, x2, y2, x3, y3) }
}

pub fn set_color(rgb: u32, a: i32) {
    let r = ((rgb >> 16) & 0xff) as i32;
    let g = ((rgb >> 8) & 0xff) as i32;
    let b = (rgb & 0xff) as i32;
    unsafe { js_set_color(r, g, b, a) }
}

pub fn text(s: &str, x: f64, y: f64, size: f64) {
    unsafe { js_fill_text(s.as_ptr(), s.len() as i32, x, y, size) }
}

pub fn measure(s: &str, size: f64) -> f64 {
    unsafe { js_measure_text(s.as_ptr(), s.len() as i32, size) }
}

pub fn request_frame() {
    unsafe { js_request_frame() }
}
