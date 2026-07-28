use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları, SayısalAralıkParçası, SeriSeçenekleri,
    UplotHatası, YumuşakSınırKipi, YÖlçekEtiketBiçimi, YÖlçekSeçenekleri,
};

pub const DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = dependent_scale_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
// Sağ Celsius ekseni, sol Fahrenheit ölçeğinden çekirdekte türetilir.
assert_eq!(grafik.görünür_y_ölçek_aralığı("y"), Some(Aralık::yeni(36.0, 84.0)?));
grafik.seri_görünürlüğünü_ayarla(0, false)?; // uPlot setSeries karşılığı"##;

/// `demos/dependent-scale.html` içindeki Fahrenheit verisini ve `z.from = y`
/// Celsius dönüşümünü aynı 7 noktayla kurar.
pub fn dependent_scale_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=7).map(f64::from).collect();
    let fahrenheit = [40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
        .into_iter()
        .map(Some)
        .collect();
    let varsayılan_y_aralığı = SayısalAralıkAyarları::yeni(
        SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
        SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
    );
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Derived Scale")
        .x_zaman(false)
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y")
                .birim("° F")
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
                .sayısal_aralık(varsayılan_y_aralığı),
        )
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("z")
                .sağda(true)
                .ızgara(false)
                .birim("° C")
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
                .eksen_en_az_etiket_boşluğu(20.0)
                .kaynak_dönüşümü("y", 5.0 / 9.0, -32.0 * 5.0 / 9.0),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("blah").renk("#008000"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![fahrenheit])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Aralık, Grafik, Komut, TekerlekEkseni};

    fn yaklaşık(sol: f64, sağ: f64) {
        assert!((sol - sağ).abs() < 1e-9, "{sol} != {sağ}");
    }

    fn dönüşüm_korunur(grafik: &Grafik) -> bool {
        let (Some(fahrenheit), Some(celsius)) = (
            grafik.görünür_y_ölçek_aralığı("y"),
            grafik.görünür_y_ölçek_aralığı("z"),
        ) else {
            return false;
        };
        yaklaşık(celsius.en_az, (fahrenheit.en_az - 32.0) * 5.0 / 9.0);
        yaklaşık(celsius.en_çok, (fahrenheit.en_çok - 32.0) * 5.0 / 9.0);
        true
    }

    #[test]
    fn fahrenheit_verisi_celsius_eksenine_dönüşür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = dependent_scale_kartı()?;
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied()
                .flatten(),
            Some(40.0)
        );
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.last())
                .copied()
                .flatten(),
            Some(80.0)
        );
        assert_eq!(
            seçenekler
                .y_ölçekleri
                .iter()
                .find(|ölçek| ölçek.anahtar == "z")
                .map(|ölçek| ölçek.eksen_en_az_etiket_boşluğu),
            Some(20.0)
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.görünür_y_ölçek_aralığı("y"),
            Some(Aralık::yeni(36.0, 84.0)?)
        );
        assert!(dönüşüm_korunur(&grafik));
        let sahne = grafik.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "40° F"))
        );
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "4° C"))
        );
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "28° C"))
        );
        assert!(
            !sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.contains(" °")))
        );

        // Tekerlek yakınlaştırması varsayılan kapalı; test onu
        // araç olarak kullandığı için açıkça açıyor.
        grafik.tekerlek_etkileşimi_ayarla(true);
        assert!(grafik.tekerlek_eksende(0.5, 0.5, 120.0, true, TekerlekEkseni::Y)?);
        assert!(dönüşüm_korunur(&grafik));
        assert!(grafik.görünür_aralıkları_ayarla(
            Aralık::yeni(2.0, 6.0)?,
            Aralık::yeni(44.0, 76.0)?,
            true,
        ));
        assert!(dönüşüm_korunur(&grafik));

        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert!(
            !grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "#008000"))
        );
        assert!(grafik.seri_görünürlüğünü_ayarla(0, true)?);
        assert!(
            grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "#008000"))
        );
        Ok(())
    }
}
