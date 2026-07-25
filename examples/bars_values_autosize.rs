use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, bars_values_autosize_kartları, ÇubukYönü};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/bars-values-autosize"));
    std::fs::create_dir_all(&dizin)?;
    for (yön, seçenekler, veri) in bars_values_autosize_kartları()? {
        let ad = if yön == ÇubukYönü::Dikey {
            "vertical"
        } else {
            "horizontal"
        };
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(format!("{ad}.svg")), svg)?;
    }
    println!("İki autosize çubuk grafiği üretildi: {}", dizin.display());
    Ok(())
}
