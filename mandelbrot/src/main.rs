fn main() {
    println!("Hello, world!");
}

fn square_loop(mut x: f64) {
    loop {
        x = x * x;
    }
}

use num::Complex;
fn square_add_loop(c: f64){
    let mut z = Complex{ re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}
