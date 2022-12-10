pub use self::crt::CRT;
mod crt;

// CPU instruction set
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Op {
    NoOp,
    AddX(i32)
}

// CPU object
#[derive(Debug, Clone, PartialEq)]
pub struct CPU {
    x: i32,
    y: i32,
    tick: i32,
    carry: i32,        
    breakpoints: Vec<i32>,
    trace: Vec<i32>,
    display: CRT
}

impl CPU {
    
    // Create a new CPU object. Pass in a display to use with the CPU
    pub fn new(display: CRT) -> Self {
        Self {
            x: 1,
            y: 0,
            tick: 0,
            carry: 0,            
            breakpoints: Vec::new(),
            trace: Vec::new(),
            display: display
        }
    }

    // Pass in a list of breakpoints that are trigger at specific cycle counts
    // When a breakpoints is triggered, signal strength is inserted into the trace log
    pub fn set_breakpoints(&mut self, points: Vec<i32>) -> () {        
        self.breakpoints = points;
        self.breakpoints.sort();
    }

    // Returns the trace log created when breakpoints are hit
    pub fn get_trace_log(&self) -> Vec<i32> {
        self.trace.clone()
    }

    // Run a program (list of instructions) and output the result
    // using the display
    pub fn run_program(&mut self, p: &Vec<Op>) -> () {
        let mut iter = p.iter();
        let mut break_iter = self.breakpoints.iter();
        let mut bkpt = break_iter.next();
        while let Some(instruction) = iter.next() {
            while self.carry > 0 {
                self.carry -= 1;
                self.tick += 1;

                // Check break points
                match bkpt {
                    Some(b) => {
                        if *b == self.tick {                 
                            bkpt = break_iter.next();
                            self.trace.push(self.tick * self.x);                            
                        }
                    },
                    _ => (),
                }                

                // Update display
                self.display.draw(self.x);

                // Apply scratch register at end of cycle
                if self.carry == 0 {
                    self.x += self.y;
                }
            }

            // Handle next instruction
            match instruction {
                Op::AddX(x) => {
                    self.carry = 2;
                    self.y = *x; 
                }                    
                _ => {
                    self.carry = 1;
                    self.y = 0
                }
            }            
        }
        // Print program output
        println!("{:?}", self.display);
    }
}