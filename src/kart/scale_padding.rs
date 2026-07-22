use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const SCALE_PADDING_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = scale_padding_kartı()?;
// On üç düz seri, uPlot'un sayısal ölçek payı sınamasındaki
// küçük, sıfır çevresi ve büyük değerleri aynen kullanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/scale-padding.html` kartındaki 10 X değeri ile on üç
/// sabit seriyi kaynak sırasını ve etiketlerini koruyarak oluşturur.
pub fn scale_padding_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    const DÜZEYLER: [f64; 13] = [
        -10_500.0, -10_000.0, -9_500.0, -0.105, -0.100, -0.095, 0.0, 0.095, 0.100, 0.105, 9_500.0,
        10_000.0, 10_500.0,
    ];
    let x = (1_u32..=10).map(f64::from).collect::<Vec<_>>();
    let seriler = DÜZEYLER
        .iter()
        .map(|değer| vec![Some(*değer); x.len()])
        .collect::<Vec<_>>();
    let seçenekler = DÜZEYLER.iter().fold(
        GrafikSeçenekleri::yeni(1600, 600)?
            .başlık("Flat")
            .x_zaman(false)
            .etkileşimler(ortak_kart_etkileşimleri()),
        |seçenekler, değer| {
            seçenekler.seri(SeriSeçenekleri::yeni(değer.to_string()).renk("#ff0000"))
        },
    );
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_on_üç_düz_serisi_ve_ölçek_payları_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scale_padding_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1600, 600));
        assert_eq!(
            veri.x(),
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(veri.seriler().len(), 13);
        assert_eq!(
            veri.seriler().get(6).and_then(|seri| seri.first()).copied(),
            Some(Some(0.0))
        );
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.etiket.as_str()),
            Some("-10500")
        );
        assert_eq!(
            seçenekler.seriler.last().map(|seri| seri.etiket.as_str()),
            Some("10500")
        );

        let grafik = Grafik::yeni(seçenekler, veri)?;
        let y = grafik.görünür_y_aralığı();
        assert_eq!((y.en_az, y.en_çok), (-12_600.0, 12_600.0));
        let yol_sayısı = grafik
            .çiz()
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Yol { .. }))
            .count();
        assert_eq!(yol_sayısı, 13);
        Ok(())
    }
}
