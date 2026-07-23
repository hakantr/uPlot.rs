use super::ortak_kart_etkileşimleri;
#[cfg(feature = "svg")]
use crate::Grafik;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const SVG_IMAGE_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = svg_image_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;
let bağımsız_svg = grafik.çiz().svg();
// Başlık, eksenler ve seri tek bir taşınabilir SVG belgesindedir.
"##;

/// `demos/svg-image.html` içindeki 400×200 "test chart" yüzeyini kurar.
///
/// Kaynak demo canvas ile DOM katmanlarını sonradan bir görüntüde birleştirir.
/// Rust portunda aynı içerik zaten tek bir bağımsız SVG sahnesi olarak üretilir.
pub fn svg_image_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(400, 200)?
        .başlık("test chart")
        .x_zaman(false)
        .arka_plan_rengi("pink")
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Value").renk("blue"));
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0],
        vec![vec![Some(4.0), Some(5.0), Some(6.0)]],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(feature = "svg")]
pub fn svg_image_belgesi() -> Result<String, UplotHatası> {
    let (seçenekler, veri) = svg_image_kartı()?;
    Ok(Grafik::yeni(seçenekler, veri)?.çiz().svg())
}

#[cfg(all(test, feature = "svg"))]
mod testler {
    use super::*;

    #[test]
    fn kaynak_grafik_ve_bağımsız_svg_belgesi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = svg_image_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (400, 200));
        assert_eq!(seçenekler.başlık, "test chart");
        assert!(!seçenekler.x_zaman);
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.etiket.as_str()),
            Some("Value")
        );
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0]);
        assert_eq!(
            veri.seriler().first().map(Vec::as_slice),
            Some([Some(4.0), Some(5.0), Some(6.0)].as_slice())
        );
        let svg = svg_image_belgesi()?;
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("width=\"400\" height=\"200\""));
        assert!(svg.contains("test chart"));
        assert!(svg.contains("fill=\"pink\""));
        assert!(svg.contains("stroke=\"blue\""));
        Ok(())
    }
}
