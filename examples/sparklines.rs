use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, SparklineÖrneği, sparklines_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/sparklines"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in SparklineÖrneği::TÜMÜ {
        let (seçenekler, veri) = sparklines_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!(
        "Sparklines kartları üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
