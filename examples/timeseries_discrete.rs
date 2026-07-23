use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, TimeseriesDiscreteÖrneği, timeseries_discrete_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/timeseries-discrete"));
    std::fs::create_dir_all(&çıktı)?;

    for örnek in TimeseriesDiscreteÖrneği::TÜMÜ {
        let (seçenekler, veri) = timeseries_discrete_kartı(örnek)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let yol = çıktı.join(format!("{}.svg", örnek.kimlik()));
        std::fs::write(&yol, grafik.çiz().svg())?;
        println!("{} üretildi: {}", örnek.başlık(), yol.display());
    }

    Ok(())
}
