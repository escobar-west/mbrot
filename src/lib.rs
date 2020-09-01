mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub struct Complex {
    re: f64,
    img: f64,
}


#[wasm_bindgen]
impl Complex {
    pub fn new(re: f64, img: f64) -> Complex {
        Complex{re, img}
    }


    pub fn real(&self) -> f64 {
        self.re
    }


    pub fn imag(&self) -> f64 {
        self.img
    }
}


#[wasm_bindgen]
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


    pub fn render(&mut self, center: &Complex, dx: f64, max_iter: u32) {
        let mut x: f64;
        let mut y: f64;
        let mut x2: f64;
        let mut y2: f64;
        let mut iter_counter: u32;

        let x_off = center.re - dx * self.width as f64 * 0.5;
        let y_off = center.img - dx * self.height as f64 * 0.5;

        let mut pix: *mut RGBA;
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
                    pix = self.pixels.get_unchecked_mut(pix_idx);
                    if iter_counter < max_iter {
                        *pix = get_gradient(iter_counter);
                    }
                    else {
                       (*pix).r = 0;
                       (*pix).b = 0;
                       (*pix).g = 0;
                    }
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


fn get_gradient(iter: u32) -> RGBA {
    let grad_length: u8= 15;
    let color_step: u8 = 17;
    let n_colors: u8 = 90;

    let iter = (iter % n_colors as u32) as u8;
    let mut output = RGBA{r:0, g:0, b:0, a:255};

    if iter < grad_length { // red to yellow
        output.r = 255;
        output.g = color_step * iter;
    }
    else if iter < 2 * grad_length { // yellow to green
        output.g = 255;
        output.r = 255 - color_step * (iter - grad_length);
    }
    else if iter < 3 * grad_length {// green to cyan
        output.g = 255;
        output.b = color_step * (iter - 2 * grad_length);
    }
    else if iter < 4 * grad_length {// cyan to blue
        output.b = 255;
        output.g = 255 - color_step * (iter - 3 * grad_length);
    }
    else if iter < 5 * grad_length {// blue to violet
        output.b = 255;
        output.r = color_step * (iter - 4 * grad_length);
    }
    else if iter < 6 * grad_length {// violet to red
        output.r = 255;
        output.b = 255 - color_step * (iter - 5 * grad_length);
    }
    return output;
}
