use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, SoftMinMaxÖrneği, soft_minmax_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/soft-minmax"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in SoftMinMaxÖrneği::TÜMÜ {
        let (seçenekler, veri) = soft_minmax_kartı(örnek, 12.0)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!(
        "Soft Min/Max kartları üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
