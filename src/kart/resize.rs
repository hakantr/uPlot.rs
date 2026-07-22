use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

/// Masaüstü ve WASM kataloglarında gösterilen, çalıştırılabilir Resize kartı
/// tanımıyla aynı kalan kod örneği.
pub const RESIZE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = resize_kartı(100)?;
// Seçim, çift tık, tekerlek, touch, taşıma ve görünüm geçmişi
// ortak kart profiliyle çekirdekte hazır gelir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// <https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/resize.html>
/// çizim davranışını ve canlı doğrulamada seçilen nokta sayısını kullanır.
pub fn resize_kartı(
    nokta_sayısı: usize,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let nokta_sayısı = nokta_sayısı.clamp(2, 10_000);
    let mut x = Vec::with_capacity(nokta_sayısı);
    let mut y = Vec::with_capacity(nokta_sayısı);
    for indeks in 0..nokta_sayısı {
        let değer = 2.0 * std::f64::consts::PI * indeks as f64 / nokta_sayısı as f64;
        x.push(değer);
        y.push(Some(değer.sin()));
    }

    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık("Resize")
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("sin(x)")
                .renk("#dc2626")
                .çizgi_kalınlığı(1.5),
        );
    let veri = HizalıVeri::yeni(x, vec![y])?;
    Ok((seçenekler, veri))
}
