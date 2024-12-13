use std::f64::consts::PI;

fn main() {
    print!("{}", make_it_fractional(1425.));
}

fn make_it_fractional(num: f64) -> String {
    let mut counter = 0.;
    let rnum = {
        loop {
            counter += 1.;
            if is_natural_number(num * counter) {
                break;
            }
        }
        num * counter
    };
    format!("{rnum}/{counter}")
}

fn is_natural_number(n: f64) -> bool {
    n > 0.0 && n.fract() == 0.0
}

