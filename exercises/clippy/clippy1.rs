const X1: f64 = 0.01;

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if (y -x).abs() > X1 {
        println!("Success!");
    }
}
