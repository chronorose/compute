use std::f64::{INFINITY, consts::PI};

fn f0(x: f64) -> f64 {
    if x == 0.0 {
        return PI;
    } else if x == 1.0 {
        return 5.0 * PI;
    }
    f64::sin(PI * (f64::powi(x, 5))) / (f64::powi(x, 5) * (1.0 - x))
}

fn f1(x: f64) -> f64 {
    if f64::is_infinite(x) {
        return 0.0;
    }
    f64::exp(-f64::sqrt(x) + f64::sin(x / 10.0))
}

fn inf_case(f: Box<dyn Fn(f64) -> f64>) -> Box<impl Fn(f64) -> f64> {
    Box::new(move |x: f64| -> f64 {
        if x == 1.0 {
            return f(INFINITY);
        }
        f(x / (1. - x)) / (f64::powi(1. - x, 2))
    })
}

fn mr_simpson(f: fn(f64) -> f64, a: f64, mut b: f64, n: usize) -> f64 {
    let mut f: Box<dyn Fn(f64) -> f64> = Box::new(f);
    if f64::is_infinite(b) {
        f = inf_case(f);
        b = 1.0;
    }
    let n = n + (n & 1);
    let h = (b - a) / n as f64;
    let points: Vec<_> = (0..=n).map(|i| a + (i as f64) * h).collect();
    let applied: Vec<_> = points.iter().map(|&x| f(x)).collect();
    let odd: f64 = (1..=n).step_by(2).map(|i| applied[i]).sum();
    let even: f64 = (2..n).step_by(2).map(|i| applied[i]).sum();

    h / 3. * (applied[0] + 2. * even + 4. * odd + applied[n])
}

fn order(f: fn(f64) -> f64, exact_value: f64, a: f64, b: f64) {
    let eps = 1e-8;
    for counter in (20..3000).step_by(2) {
        let i = mr_simpson(f, a, b, counter);
        if (i - exact_value).abs() < eps {
            let i2 = mr_simpson(f, a, b, counter * 2);
            let i4 = mr_simpson(f, a, b, counter * 4);
            let error = (i - i2).abs() / (i4 - i2).abs();
            let order = f64::log2(error);
            println!("order of approximation: {}", order);
            println!("result: {}", i);
            break;
        }
        //
    }
}

const EXACT0: f64 = 8.03491057;
const EXACT1: f64 = 2.98100345;

fn main() {
    order(f0, EXACT0, 0., 1.);
    order(f1, EXACT1, 0., INFINITY);
    //println!("{}", mr_simpson(f0, 0., 1., 50));
    //println!("{}", mr_simpson(f1, 0., INFINITY, 50));
}
