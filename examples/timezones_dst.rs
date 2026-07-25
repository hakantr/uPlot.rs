use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, timezones_dst_kartları};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/timezones-dst"));
    std::fs::create_dir_all(&çıktı)?;

    for (örnek, seçenekler, veri) in timezones_dst_kartları()? {
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let yol = çıktı.join(format!("{}.svg", örnek.kimlik()));
        std::fs::write(&yol, grafik.çiz().svg())?;
        println!(
            "{} / {} üretildi: {}",
            örnek.bölüm(),
            örnek.başlık(),
            yol.display()
        );
    }

    Ok(())
}
