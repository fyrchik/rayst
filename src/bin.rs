use std::io::{self, Write};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	const HEIGHT: u32 = 256;
	const WIDTH: u32 = 256;

	let mut stderr = io::stderr();

	println!("P3\n{} {}\n255", WIDTH, HEIGHT);
	for j in (0..HEIGHT).rev() {
		write!(&mut stderr, "\rScanlines remaining: {}", j)?;

		for i in 0..WIDTH {
			let r = i as f64 / (WIDTH - 1) as f64;
			let g = j as f64 / (HEIGHT - 1) as f64;
			let b = 0.25;

			let ir = (255.999 * r) as u32;
			let ig = (255.999 * g) as u32;
			let ib = (255.999 * b) as u32;

			println!("{} {} {}", ir, ig, ib);
		}
	}

	write!(&mut stderr, "\nDone.\n")?;

	Ok(())
}