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

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn bin_noktalı_yoğun_çizgi_tam_piksele_merdivenlenmez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = resize_kartı(1_000)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yol = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::Yol { parçalar, .. } => parçalar.first(),
            _ => None,
        });
        assert!(yol.is_some(), "resize çizgi yolu üretilmedi");
        let Some(yol) = yol else {
            return Ok(());
        };
        assert!(yol.len() >= 900);
        assert!(
            yol.iter()
                .filter(|nokta| nokta.x.fract() != 0.0 || nokta.y.fract() != 0.0)
                .count()
                > yol.len() * 9 / 10
        );
        let azami_ikinci_fark = yol
            .windows(3)
            .filter_map(|üçlü| {
                let [ilk, orta, son] = üçlü else {
                    return None;
                };
                Some((son.y - 2.0 * orta.y + ilk.y).abs())
            })
            .fold(0.0_f32, f32::max);
        assert!(azami_ikinci_fark < 0.1, "{azami_ikinci_fark}");
        assert!(
            !sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Daire { .. }))
        );
        Ok(())
    }
}
