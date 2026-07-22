use crate::{
    EtkileşimSeçenekleri, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, TekerlekAyarları,
    TekerlekKipi, UplotHatası,
};

/// Masaüstü ve WASM kataloglarında gösterilen, çalıştırılabilir API biçimiyle
/// aynı kalan kart tanım örneği.
pub const İLK_KART_TANIM_ÖRNEĞİ: &str = r##"let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
    .başlık("Resize")
    .x_zaman(false)
    .etkileşimler(EtkileşimSeçenekleri::default()
        .tekerlek_etkileşimi(true)
        .tekerlek_ayarları(TekerlekAyarları::default()
            .kip(TekerlekKipi::Otomatik))
        .seçim_yakınlaştır(true)
        .çift_tıkla_tam_görünüm(true)
        .görünüm_geçmişi(true)
        .dokunma_etkileşimi(true))
    .seri(SeriSeçenekleri::yeni("sin(x)")
        .renk("#dc2626")
        .çizgi_kalınlığı(1.5));

let veri = HizalıVeri::yeni(x, vec![y])?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// <https://github.com/leeoniya/uPlot/blob/0e5812c504430f5c804e0f993376d8999b26cc34/demos/resize.html>
/// çizim davranışını; `zoom-wheel.html` ve `zoom-touch.html` resmî eklenti
/// davranışlarını temel alan ilk uyum kartı.
pub fn ilk_kart() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    sinüs_kartı(100)
}

pub fn ilk_kart_etkileşimleri() -> EtkileşimSeçenekleri {
    EtkileşimSeçenekleri::default()
        .tekerlek_etkileşimi(true)
        .tekerlek_ayarları(TekerlekAyarları::default().kip(TekerlekKipi::Otomatik))
        .seçim_yakınlaştır(true)
        .çift_tıkla_tam_görünüm(true)
        .görünüm_geçmişi(true)
        .dokunma_etkileşimi(true)
}

/// İlk kartın canlı testlerde ayarlanabilir nokta sayılı biçimi.
pub fn sinüs_kartı(
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
        .etkileşimler(ilk_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("sin(x)")
                .renk("#dc2626")
                .çizgi_kalınlığı(1.5),
        );
    let veri = HizalıVeri::yeni(x, vec![y])?;
    Ok((seçenekler, veri))
}
