use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

/// Masaüstü ve WASM kataloglarında gösterilen, çalıştırılabilir API biçimiyle
/// aynı kalan kart tanım örneği.
pub const İLK_KART_TANIM_ÖRNEĞİ: &str = r##"let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
    .başlık("İlk kart · sin(x)")
    .x_zaman(false)
    .seri(SeriSeçenekleri::yeni("sin(x)")
        .renk("#dc2626")
        .çizgi_kalınlığı(1.5));

let veri = HizalıVeri::yeni(x, vec![y])?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `../uPlot/demos/resize.html` kaynaklı ilk uyum kartı.
pub fn ilk_kart() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    sinüs_kartı(100)
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
        .başlık("İlk kart · sin(x)")
        .x_zaman(false)
        .seri(
            SeriSeçenekleri::yeni("sin(x)")
                .renk("#dc2626")
                .çizgi_kalınlığı(1.5),
        );
    let veri = HizalıVeri::yeni(x, vec![y])?;
    Ok((seçenekler, veri))
}
