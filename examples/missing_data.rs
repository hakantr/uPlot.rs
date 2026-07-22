use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, missing_data_null_kartı, missing_data_x_boşluğu_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/missing-data"));
    std::fs::create_dir_all(&dizin)?;
    for (dosya, kurucu) in [
        (
            "null-values.svg",
            missing_data_null_kartı as fn() -> Result<_, _>,
        ),
        ("adjacent-x-gap.svg", missing_data_x_boşluğu_kartı),
    ] {
        let (seçenekler, veri) = kurucu()?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(dosya), svg)?;
    }
    println!("Missing Data kartları üretildi: {}", dizin.display());
    Ok(())
}
