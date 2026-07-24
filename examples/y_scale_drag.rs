use std::error::Error;
use std::path::PathBuf;

use uplot_rs::{Grafik, y_scale_drag_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/y-scale-drag.svg"));
    let (seçenekler, veri) = y_scale_drag_kartı()?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!("Draggable x & y scales üretildi: {}", çıktı.display());
    Ok(())
}
