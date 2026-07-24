use super::ortak_kart_etkileşimleri;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

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
        .duyarlı_boyut(true)
        .x_zaman(false)
        // Kaynak demo kadrajı sabit tutar. SVG yüzeyi CSS pikseline değil
        // aygıt pikseline ölçeklenen canvas'tan farklı olduğu için seri yolunu
        // vektör alt-piksel hassasiyetinde bırakırız.
        .y_aralığı(Aralık::yeni(-1.5, 1.5)?)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("sin(x)")
                .renk("red")
                .piksel_hizası(0.0),
        );
    let veri = HizalıVeri::yeni(x, vec![y])?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    fn resize_yolu(nokta_sayısı: usize) -> Result<Vec<crate::Nokta>, UplotHatası> {
        let (seçenekler, veri) = resize_kartı(nokta_sayısı)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-1.5, 1.5)?);
        Ok(grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Yol { parçalar, .. } => parçalar.first().cloned(),
                _ => None,
            })
            .unwrap_or_default())
    }

    #[test]
    fn yüz_noktalı_kaynak_yolu_css_piksellerine_merdivenlenmez() -> Result<(), UplotHatası> {
        let (seçenekler, _) = resize_kartı(100)?;
        assert!(seçenekler.duyarlı_boyut);
        let yol = resize_yolu(100)?;
        assert_eq!(yol.len(), 100);
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
        assert!(azami_ikinci_fark < 0.5, "{azami_ikinci_fark}");
        Ok(())
    }

    #[test]
    fn bin_noktalı_yoğun_çizgi_de_alt_piksel_hassasiyetini_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = resize_kartı(1_000)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yol = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::Yol { parçalar, .. } => parçalar.first(),
            _ => None,
        });
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
        assert!(
            !sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Daire { .. }))
        );
        Ok(())
    }
}
