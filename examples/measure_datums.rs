use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, measure_datums_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/measure-datums.svg"));
    let (seçenekler, veri) = measure_datums_kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    grafik.ölçüm_datumunu_ayarla(1, 0.25, 0.4);
    grafik.ölçüm_datumunu_ayarla(2, 0.75, 0.6);
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    Ok(())
}
