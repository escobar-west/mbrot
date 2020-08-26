mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


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
        let n_pixels = width * height;
        let mut pixels: Vec<RGBA> = Vec::with_capacity(n_pixels as usize);

        for _ in 0..n_pixels {
            pixels.push(RGBA{r:0, g:0, b:0, a:255});
        }
        Universe{width, height, pixels}
    }


    pub fn render(&mut self, center: Complex, dx:f32, max_iter: u32) {
        let mut x: f32;
        let mut y: f32;
        let mut x2: f32;
        let mut y2: f32;
        let mut iter_counter: u32;

        let x_off = center.re - dx * self.width as f32 * 0.5;
        let y_off = center.img - dx * self.height as f32 * 0.5;

        let mut pix_idx: usize = 0;
        let mut pix_coord = Complex{re: x_off,
                                    img: y_off};

        for _row in 0..self.height {
            for _col in 0..self.width { 
                x = 0.0;
                y = 0.0;
                x2 = 0.0;
                y2 = 0.0;
                iter_counter = 0;

                while (x2 + y2 < 4.0) && (iter_counter < max_iter) {
                    y = (x + x) * y + pix_coord.img;
                    x = x2 - y2 + pix_coord.re;
                    x2 = x * x;
                    y2 = y * y;
                    iter_counter += 1;
                }
                unsafe {
                    let pix = self.pixels.get_unchecked_mut(pix_idx);
                    pix.r = if iter_counter < max_iter {155} else {0};
                    pix.b = if iter_counter < max_iter {255} else {0};
                    pix.g = if iter_counter < max_iter {255} else {0};
                }
                pix_coord.re += dx;
                pix_idx += 1;
            }
            pix_coord.re = x_off;
            pix_coord.img += dx;
        }
    }


    pub fn pixels(&self) -> *const RGBA {
        self.pixels.as_ptr()
    }
}
