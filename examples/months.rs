use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{
    Grafik, months_artık_yıllı_kartı, months_artık_yılsız_kartı, months_rusça_kartı
};

fn main() -> Result<(), Box<dyn Error>> {
    let dizin = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/months"));
    std::fs::create_dir_all(&dizin)?;
    for (dosya, kurucu) in [
        (
            "no-leap-year.svg",
            months_artık_yılsız_kartı as fn() -> Result<_, _>,
        ),
        ("2024-leap-year.svg", months_artık_yıllı_kartı),
        ("russian.svg", months_rusça_kartı),
    ] {
        let (seçenekler, veri) = kurucu()?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        std::fs::write(dizin.join(dosya), svg)?;
    }
    println!("Months kartları üretildi: {}", dizin.display());
    Ok(())
}
