use std::error::Error;

use bathy::BathySurface;

mod bathy;

fn main() -> Result<(), Box<dyn Error>> {
    let mut surface = BathySurface::from_path("data.csv")?;

    print!("{:?}", surface);
    println!("{:?}", surface.get_bounds());
    Ok(())
}
