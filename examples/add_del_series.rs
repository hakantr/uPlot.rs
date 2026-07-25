use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{
    Grafik, add_del_series_ek_seçeneği, add_del_series_ek_verisi, add_del_series_kartı,
};

fn main() -> Result<(), Box<dyn Error>> {
    let çıktı = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/add-del-series.svg"));
    if let Some(üst) = çıktı.parent() {
        std::fs::create_dir_all(üst)?;
    }
    let (seçenekler, veri) = add_del_series_kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    grafik.seri_ekle(
        1,
        add_del_series_ek_seçeneği(0),
        add_del_series_ek_verisi(0),
    )?;
    std::fs::write(&çıktı, grafik.çiz().svg())?;
    println!(
        "Add/Delete Series ekleme kanıtı üretildi: {}",
        çıktı.display()
    );
    Ok(())
}
