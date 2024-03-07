
struct LCG {
    m: u64, // Modulus
    a: u64, // Multiplier
    c: u64, // Increment
    s: u64, // Seed (Start Value)
}

impl LCG {

    // Initialize the LCG with parameters and a seed
    fn new(m: u64, a: u64, c: u64, s: u64) -> Self {
        // Return LCG object with initial parameters set
        LCG {
            m,
            a,
            c,
            s,
        }
    }

    // Generate the next random number
    fn next(&mut self) -> u64 {
        self.s = ((self.a * self.s) + self.c) % self.m;
        self.s
    }
}

fn main() {
    // Create lcg object with initial parameters
    // let mut lcg = LCG::new(50000, 40000, 30000, 42);
    let mut lcg = LCG::new(2u64.pow(32), 1664525, 1013904223, 42);

    // Generate some random numbers
    for _ in 0..10 {
        println!("{}", lcg.next() % 10);
    }

}


