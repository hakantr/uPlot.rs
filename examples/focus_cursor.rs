use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{FocusÖrneği, Grafik, focus_cursor_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/focus-cursor"));
    std::fs::create_dir_all(&dizin)?;
    for örnek in FocusÖrneği::TÜMÜ {
        let (seçenekler, veri) = focus_cursor_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!("Focus Cursor kartları üretildi: {}", dizin.display());
    Ok(())
}
