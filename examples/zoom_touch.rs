use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, zoom_touch_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/zoom-touch.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }
    let (seçenekler, veri) = zoom_touch_kartı()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("Pinch Zoom & Pan kartı üretildi: {}", çıktı.display());
    Ok(())
}
