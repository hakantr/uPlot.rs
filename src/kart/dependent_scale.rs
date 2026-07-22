use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri
};

pub const DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = dependent_scale_kartı()?;
// Sağ Celsius ekseni, sol Fahrenheit ölçeğinden çekirdekte türetilir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// `demos/dependent-scale.html` içindeki Fahrenheit verisini ve `z.from = y`
/// Celsius dönüşümünü aynı 7 noktayla kurar.
pub fn dependent_scale_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (1_u32..=7).map(f64::from).collect();
    let fahrenheit = [40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
        .into_iter()
        .map(Some)
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Derived Scale")
        .x_zaman(false)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").birim("° F"))
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("z")
                .sağda(true)
                .ızgara(false)
                .birim("° C")
                .kaynak_dönüşümü("y", 5.0 / 9.0, -32.0 * 5.0 / 9.0),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("blah").renk("#008000"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![fahrenheit])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

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
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.ends_with("° F"))
            )
        );
        assert!(
            sahne.komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.ends_with("° C"))
            )
        );
        Ok(())
    }
}
