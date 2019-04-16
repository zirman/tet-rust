#![deny(bare_trait_objects)]
extern crate cfg_if;
extern crate rand;
extern crate wasm_bindgen;

mod grid;
mod linear;
mod tetrust;

use cfg_if::cfg_if;
use grid::{Grid, Tile};
use linear::M4;
use tetrust::Regulator;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub struct Scene {
    pub changed: bool,
    pub clip_matrix: *const M4,
    pub grid_width: u32,
    pub grid_height: u32,
    pub grid: *const Tile,
}

// statics hold data in heap to be passed to JavaScript
static mut CLIP_MATRIX: Option<M4> = None;
static mut GRID: Option<Grid> = None;
static mut REGULATOR: Option<Regulator> = None;

const GRID_WIDTH: u32 = 48;
const GRID_HEIGHT: u32 = 27;

#[wasm_bindgen]
pub fn render(t: f32, wd: u32, ht: u32) -> Scene {
    let regulator: &mut Regulator = Regulator::get(t);
    *regulator = regulator.iterate(t);

    let clip_matrix: &mut M4 = get_clip_matrix();
    let clip_matrix_n = calc_clip_matrix(wd, ht);

    let grid: &mut Grid = get_grid();
    let grid_n = regulator.game.grid();

    let clip_matrix_changed = *clip_matrix != clip_matrix_n;
    *clip_matrix = clip_matrix_n;

    let grid_changed = *grid != grid_n;
    *grid = grid_n;

    Scene {
        changed: clip_matrix_changed || grid_changed,
        clip_matrix,
        grid_width: grid.width(),
        grid_height: grid.height(),
        grid: grid.as_ptr(),
    }
}

#[wasm_bindgen]
pub fn on_key_down(t: f32, key_code: u32, _key: &str) {
    Regulator::get(t).controls.key_down(key_code);
}

#[wasm_bindgen]
pub fn on_key_up(t: f32, key_code: u32, _key: &str) {
    Regulator::get(t).controls.key_up(key_code);
}

#[wasm_bindgen]
pub fn on_mouse_move(_f: f32, _x: u32, _y: u32) {
    // log(&format!("on_mouse_move {} {}", x, y));
}

#[wasm_bindgen]
pub fn on_mouse_down(_f: f32, _x: u32, _y: u32) {
    // log(&format!("on_mouse_down {} {}", x, y));
}

#[wasm_bindgen]
pub fn on_mouse_up(_f: f32, _x: u32, _y: u32) {
    // log(&format!("on_mouse_up {} {}", x, y));
}

fn get_clip_matrix() -> &'static mut M4 {
    unsafe { CLIP_MATRIX.get_or_insert_with(|| M4::identity()) }
}

fn calc_clip_matrix(wd: u32, ht: u32) -> M4 {
    let display_ratio = wd as f32 / ht as f32;
    let grid_ratio = GRID_WIDTH as f32 / GRID_HEIGHT as f32;

    if display_ratio > grid_ratio {
        let ratio = display_ratio / grid_ratio;
        let offset = (1.0 - ratio) / 2.0;

        M4::orthographic_proj_2d(
            GRID_WIDTH as f32 * offset,
            GRID_WIDTH as f32 * (ratio + offset),
            0.0,
            GRID_HEIGHT as f32,
        )
    } else if display_ratio < grid_ratio {
        let ratio = grid_ratio / display_ratio;
        let offset = (1.0 - ratio) / 2.0;

        M4::orthographic_proj_2d(
            0.0,
            GRID_WIDTH as f32,
            GRID_HEIGHT as f32 * offset,
            GRID_HEIGHT as f32 * (ratio + offset),
        )
    } else {
        M4::orthographic_proj_2d(0.0, GRID_WIDTH as f32, 0.0, GRID_HEIGHT as f32)
    }
}

fn get_grid() -> &'static mut Grid {
    unsafe { GRID.get_or_insert_with(|| Grid::new(GRID_WIDTH, GRID_HEIGHT)) }
}
