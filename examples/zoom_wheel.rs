use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{
    Grafik, ZoomFetchAkışı, ZoomRangerSeçenekleri, ZoomSürüklemeKipi, zoom_ranger_xy_grafiği,
    zoom_wheel_kartı,
};

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
    let tam_y = ranger.y_tam_aralık();
    let y_uzunluğu = tam_y.en_çok - tam_y.en_az;
    ranger.alt_tutamağı_ayarla(tam_y.en_az + y_uzunluğu * 0.2);
    ranger.üst_tutamağı_ayarla(tam_y.en_az + y_uzunluğu * 0.8);
    grafik.zoom_ranger_uygula(ranger);
    let svg = grafik.çiz().svg();
    std::fs::write(&çıktı, svg)?;
    let xy_çıktı = çıktı.with_file_name("zoom-ranger-xy.svg");
    std::fs::write(&xy_çıktı, zoom_ranger_xy_grafiği()?.çiz().svg())?;
    let mut fetch = ZoomFetchAkışı::yeni()?;
    let istek = fetch.aralık_isteği(0.25, 0.75)?;
    fetch.kaynak_yanıtını_uygula(istek)?;
    fetch.tam_aralığı_yükle()?;
    let varyasyon_sayısı = ZoomSürüklemeKipi::TÜMÜ
        .into_iter()
        .flat_map(|kip| {
            [0.0, 10.0].map(move |dist| ZoomRangerSeçenekleri::zoom_varyasyonu(kip, dist))
        })
        .count();
    println!(
        "Wheel Zoom & Drag, XY ranger ve {varyasyon_sayısı} drag varyasyonu kanıtlandı: {}, {}",
        çıktı.display(),
        xy_çıktı.display()
    );
    Ok(())
}
