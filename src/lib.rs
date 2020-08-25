mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

static MAX_HEIGHT: u32 = 800;
static MAX_WIDTH: u32 = 1280;


#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    re: f32,
    img: f32,
}


#[wasm_bindgen]
impl Complex {
    pub fn new(re: f32, img: f32) -> Complex {
        Complex{re, img}
    }
}


#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    pixels: Vec<RGBA>,
}


#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let width = if width > MAX_WIDTH {MAX_WIDTH} else {width};
        let height = if height > MAX_HEIGHT {MAX_HEIGHT} else {height};
        let max_pixels = MAX_WIDTH * MAX_HEIGHT;

        let mut pixels: Vec<RGBA> = Vec::with_capacity(max_pixels as usize);
        for _ in 0..width*height {
            pixels.push(RGBA{r:0, g:255, b:0, a:255});
        }
        Universe{width, height, pixels}
    }


    pub fn render(&mut self, anchor: Complex, dx: f32) {
        for (i, e) in self.pixels.iter_mut().enumerate() {
            e.g = 0;
            e.r = 255;
        }
    }


    pub fn pixels(&self) -> *const RGBA {
        self.pixels.as_ptr()
    }
}
