use std::error::Error;
use std::io::{self, Write};

use rayst::color::Color;

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
            let c = Color::new(r, g, 0.25);

            println!("{}", c);
        }
    }

    write!(&mut stderr, "\nDone.\n")?;

    Ok(())
}
