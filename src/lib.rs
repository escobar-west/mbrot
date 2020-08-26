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
        for (i, e) in self.pixels.iter_mut().enumerate() {
            let i = i as u32;
            let row = (i / self.width) as f32;
            let col = (i % self.width) as f32;
            let x_off = center.re - dx * self.width as f32 / 2.0;
            let y_off = center.img - dx * self.height as f32 / 2.0;

            let pix_coord = Complex::new(x_off + col * dx, y_off + row * dx);
            let mut x = 0.0;
            let mut y = 0.0;
            let mut x2 = 0.0;
            let mut y2 = 0.0;
            let mut counter = 0;

            while (x2 + y2 < 4.0) && (counter < max_iter) {
                y = (x + x) * y + pix_coord.img;
                x = x2 - y2 + pix_coord.re;
                x2 = x * x;
                y2 = y * y;
                counter += 1;
            }

            e.r = if counter < max_iter {0} else {255};
            e.b = if counter < max_iter {255} else {0};
            e.g = if counter < max_iter {0} else {255};
        }
    }


    pub fn pixels(&self) -> *const RGBA {
        self.pixels.as_ptr()
    }
}
