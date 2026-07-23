use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, StreamDataÖrneği, stream_data_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/stream-data"));
    std::fs::create_dir_all(&çıktı)?;
    for örnek in StreamDataÖrneği::TÜMÜ {
        let (seçenekler, veri) = stream_data_kartı(örnek)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(çıktı.join(format!("{}.svg", örnek.kimlik())), svg)?;
    }
    println!(
        "Data Stream kartları üretildi: {}",
        çıktı.canonicalize()?.display()
    );
    Ok(())
}
