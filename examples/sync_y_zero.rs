use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, SyncYZeroAşaması, sync_y_zero_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/sync-y-zero"));
    std::fs::create_dir_all(&çıktı)?;

    for aşama in SyncYZeroAşaması::TÜMÜ {
        let (seçenekler, veri) = sync_y_zero_kartı(aşama)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let yol = çıktı.join(format!("sync-y-zero-{}.svg", aşama.kimlik()));
        std::fs::write(&yol, grafik.çiz().svg())?;
        println!("{} üretildi: {}", aşama.açıklama(), yol.display());
    }

    Ok(())
}
