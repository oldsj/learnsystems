fn main() {
	let (lower, upper, step) = (0.0, 300.0, 20.0);

	let mut fahr = lower;
	while fahr <= upper {
		let celsius = format!("{:.*}", 1, ((5.0/9.0) * (fahr-32.0)));
		println!("{:>3} {:>6}", fahr, celsius);
		fahr += step;
	}
}
