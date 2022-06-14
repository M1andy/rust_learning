use std::fmt;

#[derive(Debug)]
struct ComplexNum {
    real: f64,
    imag: f64,
}

impl fmt::Display for com {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "real:{}, imag:{}", self.real, self.imag)
    }
}

fn main() {
    let c1 = ComplexNum {
        real: 3.3,
        imag: 5.5,
    };
    println!("Compare structures:");
    println!("Display: {}", c1);
    println!("Debug: {:?}", c1)

}
