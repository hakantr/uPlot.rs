use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, PixelAlignÖrneği, pixel_align_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı_dizini = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/pixel-align"));
    std::fs::create_dir_all(&çıktı_dizini)?;

    for örnek in PixelAlignÖrneği::TÜMÜ {
        let (seçenekler, veri) = pixel_align_kartı(örnek, 140)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı_dizini.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Pixel Align kartları üretildi: {}", çıktı_dizini.display());
    Ok(())
}
