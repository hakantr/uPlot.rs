use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, cursor_bind_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/cursor-bind.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }
    let (seçenekler, veri) = cursor_bind_kartı()?;
    let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    println!("Cursor Bind kartı üretildi: {}", çıktı.display());
    Ok(())
}
