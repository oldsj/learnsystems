fn main() {
	let (lower, upper, step) = (0.0, 300.0, 20.0);

	let mut celsius = lower;
	println!("Celsius {:>18}", "Fahrenheit");
	while celsius <= upper {
		let fahr = format!("{:.*}", 1, (celsius * (9.0/5.0) + 32.0));
		println!("{:>3} {:>19}", celsius, fahr);
		celsius += step;
	}
}
