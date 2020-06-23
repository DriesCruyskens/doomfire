//! Docs basic structure: [https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
//!
//! An implementation of the fire from the classic video game 'Doom' as explained by [Fabian Sanglard](https://fabiensanglard.net/doom_fire_psx/).
//!
//! This library contains only the algorithm for generating the fire, not displaying it on screen.
//! An `&mut [u8]` pixel buffer (rgba: 4 bytes for a single pixel) is needed for the doomfire to be able to render.
//!
//! # Example
//! ```
//! // Create a doomfire instance with a width of 600 and height of 400.
//! let mut doomfire = Doomfire::new(600, 400);
//! // Ignite the fire to jumpstart the algorithm;
//! doomfire.ignite();
//! // Doomfire copies the color values to the `&mut [u8]` rgba pixel buffer
//! // supplied to the draw function. This is normally done in your render loop function.
//! doomfire.draw(&mut pixel_buffer);
//! // Updates the fire a single step. This is normally done in your render loop function.
//! doomfire.update();
//! // To stop the fire algorithm call extinguish.
//! doomfire.extinguish();
//! ```
use rand::{rngs::ThreadRng, Rng};

/// The rgba color palette with 37 color values from black to red to orange to yellow to white.
pub const PALETTE: [[u8; 4]; 37] = [
    [0x07, 0x07, 0x07, 0xFF],
    [0x1F, 0x07, 0x07, 0xFF],
    [0x2F, 0x0F, 0x07, 0xFF],
    [0x47, 0x0F, 0x07, 0xFF],
    [0x57, 0x17, 0x07, 0xFF],
    [0x67, 0x1F, 0x07, 0xFF],
    [0x77, 0x1F, 0x07, 0xFF],
    [0x8F, 0x27, 0x07, 0xFF],
    [0x9F, 0x2F, 0x07, 0xFF],
    [0xAF, 0x3F, 0x07, 0xFF],
    [0xBF, 0x47, 0x07, 0xFF],
    [0xC7, 0x47, 0x07, 0xFF],
    [0xDF, 0x4F, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x67, 0x0F, 0xFF],
    [0xCF, 0x6F, 0x0F, 0xFF],
    [0xCF, 0x77, 0x0F, 0xFF],
    [0xCF, 0x7F, 0x0F, 0xFF],
    [0xCF, 0x87, 0x17, 0xFF],
    [0xC7, 0x87, 0x17, 0xFF],
    [0xC7, 0x8F, 0x17, 0xFF],
    [0xC7, 0x97, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x37, 0xFF],
    [0xCF, 0xCF, 0x6F, 0xFF],
    [0xDF, 0xDF, 0x9F, 0xFF],
    [0xEF, 0xEF, 0xC7, 0xFF],
    [0xFF, 0xFF, 0xFF, 0xFF],
];

/// Represents the doomfire.
pub struct Doomfire {
    width: usize,
    height: usize,
    /// Returns whether the fire is lit e.g. whether `ignite()` (true) or `extinguish()` (false) was called last.
    pub is_lit: bool,
    fire_pixels: Vec<usize>,
    rng: ThreadRng,
}

impl Doomfire {
    /// Returns a new Doomfire instance with a give width and height.
    /// The width and height needs to be the same as the pixel buffer you'll use.
    /// # Examples
    /// ```
    /// let mut doomfire = Doomfire::new(600, 400);
    /// ```
    pub fn new(width: usize, height: usize) -> Doomfire {
        // Initialze fire pixels to 0 (black).
        let fire_pixels = vec![0; width * height];

        // Initialise random number generator
        let rng = rand::thread_rng();

        Doomfire {
            width,
            height,
            is_lit: false,
            fire_pixels,
            rng,
        }
    }

    /// Updates the fire a single step.
    /// # Examples
    /// ```
    /// let mut doomfire = Doomfire::new(600, 400);
    /// doomfire.update();
    /// ```
    pub fn update(&mut self) {
        // Calculating max index here so it doesn't have to be calculated every iteration.
        let max_idx = self.width * self.height - 1;
        for x in 0..self.width {
            for y in 1..self.height {
                let src_idx = y * self.width + x;
                let src_pixel = self.fire_pixels[src_idx];
                // - width = "1 up"
                let dst_idx = src_idx - self.width;
                // Don't decrease if already 0, otherwise negative overflow.
                if src_pixel == 0 {
                    self.fire_pixels[dst_idx] = 0;
                } else {
                    // Using turbofish syntax to tell round to give f64 to round()
                    // after round converting to usize
                    let rand = self.rng.gen_range::<f64, f64, f64>(0.0, 3.0).round() as usize & 3;
                    // When is_lit: use infite algorithm, when !is_lit: use algorithm that dies out.
                    if self.is_lit {
                        // give dst_idx a random change to go left/right
                        let dst_idx = (src_idx - rand + 1) - self.width;
                        self.fire_pixels[dst_idx] = src_pixel - (rand & 1);
                    } else {
                        // not sure why but this if branch cuts performance in half??
                        let rand2 =
                            self.rng.gen_range::<f64, f64, f64>(0.0, 3.0).round() as usize & 3;
                        let dst_idx = (src_idx - rand + 1) - self.width * rand2;
                        // Clamping the index so no overflow is possible.
                        let dst_idx = if dst_idx > max_idx { max_idx } else { dst_idx };
                        self.fire_pixels[dst_idx] = src_pixel - (rand & 1);
                    }
                }
            }
        }
    }

    /// Copies the color values to the supplied `&mut [u8]` rgba pixel buffer.
    /// The same width and height values are to be used for the fire and pixel buffer.
    /// # Examples
    /// ```
    /// let mut doomfire = Doomfire::new(600, 400);
    /// let pixel_buffer: &mut [u8] = some_pixel_buffer_generator(600, 400) ;
    /// doomfire.draw(pixel_buffer);
    /// ```
    pub fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&PALETTE[self.fire_pixels[i]]);
        }
    }

    /// Sets the bottom row pixels with white so the doomfire algorithm can start.
    pub fn ignite(&mut self) {
        // White values (36) in bottom row.
        for i in 0..self.width {
            self.fire_pixels[(self.height - 1) * self.width + i] = PALETTE.len() - 1;
        }

        self.is_lit = true;
    }

    /// Sets the bottom row pixels to black so the doomfire algorithm dies out.
    pub fn extinguish(&mut self) {
        // White values (36) in bottom row.
        /* for i in 0..self.width {
            self.fire_pixels[(self.height - 1) * self.width + i] = 0;
        } */

        self.is_lit = false;
    }
}

/// Returns a new Doomfire instance width a width of 600 and height of 400.
impl Default for Doomfire {
    fn default() -> Self {
        Doomfire::new(600, 400)
    }
}
