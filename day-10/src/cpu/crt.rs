use std::fmt;

// CRT display type
#[derive(Clone, PartialEq)]
pub struct CRT {
    width: usize,
    height: usize,
    index: usize,
    pixels: Vec<Vec<bool>>,
}

impl CRT {
    
    // Create a new display with a fixed width and height
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            index: 0,
            pixels: vec![vec![false; width]; height]
        }
    }

    // Iterative draw method. Updates internal draw index
    // pixels turned on if sprite location matches current index (+ tolerance)
    pub fn draw(&mut self, sprite_location: i32) -> () {
        let px = self.index % self.width;
        let py = self.index / self.width;
        let delta = sprite_location - px as i32;
        if delta.abs() <= 1 {            
            self.pixels[py][px] = true;
        }
        self.index += 1;
    }
}

impl fmt::Debug for CRT {
    // Implement custom debug format trait to print output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                write!(f, "{}", if self.pixels[i][j] {'#'} else {','})?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}