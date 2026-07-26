use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, GüzelÖlçekDüzeni, HizalıVeri, SeriSeçenekleri, UplotHatası,
    YÖlçekEtiketBiçimi, YÖlçekSeçenekleri,
};

pub const NICE_SCALE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = nice_scale_kartı()?;
// 30 piksellik asgari etiket aralığına göre güzel Y sınırları ve
// artımı, kart her yeni yüzey boyutunda çizildiğinde çekirdekte hesaplanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/nice-scale.html` örneğinin üç serisini ve kaynak
/// `niceScale`/`niceNum` algoritmasını boyuta duyarlı çekirdek ayarıyla kurar.
pub fn nice_scale_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let seriler = vec![
        vec![-5.0, -2.0, 1.0, 7.0, 9.0, 13.0]
            .into_iter()
            .map(Some)
            .collect(),
        vec![-123.0, 4.0, 29.0, 37.0, 217.0, 230.0]
            .into_iter()
            .map(Some)
            .collect(),
        vec![0.5, 0.1, -7.0, -9.0, -13.0, 1.0]
            .into_iter()
            .map(Some)
            .collect(),
    ];
    let güzel = GüzelÖlçekDüzeni::yeni(30.0)?;
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık("Nice Scale & Ticks (resize me)")
        .x_zaman(false)
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y")
                .güzel_ölçek(güzel)
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("1")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("2")
                .renk("#008000")
                .dolgu("#00ff001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("3")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_verisi_renkleri_ve_güzel_ölçek_aralığı_korunur() -> Result<(), UplotHatası> {
        assert!(GüzelÖlçekDüzeni::yeni(0.0).is_err());
        let (seçenekler, veri) = nice_scale_kartı()?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(
            veri.seriler().get(1).map(Vec::as_slice),
            Some(
                [
                    Some(-123.0),
                    Some(4.0),
                    Some(29.0),
                    Some(37.0),
                    Some(217.0),
                    Some(230.0),
                ]
                .as_slice()
            )
        );
        assert_eq!(seçenekler.seriler.len(), 3);
        assert!(seçenekler.etkileşimler.tekerlek_etkileşimi);
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_y_aralığı().en_az, -150.0);
        assert_eq!(grafik.görünür_y_aralığı().en_çok, 250.0);

        let sahne = grafik.çiz();
        assert_eq!(
            sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Yol { .. }))
                .count(),
            3
        );
        assert_eq!(
            sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Alan { .. }))
                .count(),
            3
        );
        Ok(())
    }

    #[test]
    fn yeniden_boyutlandırma_aralığı_ve_izgarayı_birlikte_değiştirir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = nice_scale_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let geniş = grafik.çiz_boyutta(1920, 600, None);
        let dar = grafik.çiz_boyutta(600, 240, None);
        let geniş_etiketler = metinler(&geniş);
        let dar_etiketler = metinler(&dar);
        assert!(geniş_etiketler.contains(&"-150"));
        assert!(geniş_etiketler.contains(&"250"));
        assert!(dar_etiketler.contains(&"-200"));
        assert!(dar_etiketler.contains(&"400"));
        assert!(!dar_etiketler.contains(&"250"));
        Ok(())
    }

    fn metinler(sahne: &crate::Sahne) -> Vec<&str> {
        sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Metin { içerik, .. } => Some(içerik.as_str()),
                _ => None,
            })
            .collect()
    }
}
