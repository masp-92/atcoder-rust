use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let h = x / (a*a);
    let area = h * a;

    if h > (b / 2.0) {
        // 平行四辺形
        let x = (2.0 * area / a) - b;

        let theta_radians = ((b - x) / a).atan();
        let theta_degrees = theta_radians * 180.0 / PI;
        println!("{}", theta_degrees);
    } else {
        // 三角形
        let x = (2.0 * area) / b;
        let theta_radians = (b / x).atan();
        let theta_degrees = theta_radians * 180.0 / PI;
        println!("{}", theta_degrees);
    }
}