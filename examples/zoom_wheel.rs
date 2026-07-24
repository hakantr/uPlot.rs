use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, ZoomFetchAkışı, zoom_wheel_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/zoom-wheel.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }
    let (seçenekler, veri) = zoom_wheel_kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let mut ranger = grafik.zoom_ranger_durumu()?;
    ranger.sol_tutamağı_ayarla(2.0);
    ranger.sağ_tutamağı_ayarla(5.0);
    grafik.zoom_ranger_uygula(ranger);
    let svg = grafik.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    let mut fetch = ZoomFetchAkışı::yeni()?;
    let istek = fetch.aralık_isteği(0.25, 0.75)?;
    fetch.kaynak_yanıtını_uygula(istek)?;
    fetch.tam_aralığı_yükle()?;
    println!("Wheel Zoom & Drag kartı üretildi: {}", çıktı.display());
    Ok(())
}
