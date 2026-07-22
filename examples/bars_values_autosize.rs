use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, bars_values_autosize_kartı, ÇubukYönü};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/bars-values-autosize"));
    std::fs::create_dir_all(&dizin)?;
    for (ad, yön) in [
        ("vertical", ÇubukYönü::Dikey),
        ("horizontal", ÇubukYönü::Yatay),
    ] {
        let (seçenekler, veri) = bars_values_autosize_kartı(yön)?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{ad}.svg")), svg)?;
    }
    println!("İki autosize çubuk grafiği üretildi: {}", dizin.display());
    Ok(())
}
