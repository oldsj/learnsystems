#![feature(step_by)]

fn main() {
	println!("Fahrenheit {:^18}", "Celsius");
	for fahr in (0..300).step_by(20) {
		println!("{:>3} {:>18}", fahr, format!("{:.*}", 1, ((5.0/9.0) * (fahr as f64-32.0))));
	}
}