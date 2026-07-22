use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = zoom_touch_kartı()?;
// Resmî touchZoomPlugin'in kıstırarak X/Y yakınlaştırması ve tek
// parmak taşıması çekirdekte hazırdır; yüzey yalnız olayları iletir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/zoom-touch.html` kartının iki serisini ve sayısal verisini
/// korur. Kaynaktaki `screen.width`, hedef yüzeyde duyarlı çizimle karşılanır.
pub fn zoom_touch_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0].to_vec();
    let bir = [40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
        .into_iter()
        .map(Some)
        .collect();
    let iki = [18.0, 24.0, 37.0, 55.0, 55.0, 60.0, 63.0]
        .into_iter()
        .map(Some)
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(1920, 400)?
        .başlık("Pinch Zoom & Pan")
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("One").renk("#ff0000"))
        .seri(SeriSeçenekleri::yeni("Two").renk("#0000ff"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![bir, iki])?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_verisi_ve_touch_yakınlaştırması_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = zoom_touch_kartı()?;
        assert_eq!(seçenekler.yükseklik, 400);
        assert!(seçenekler.etkileşimler.dokunma_etkileşimi);
        assert_eq!(veri.seriler().len(), 2);
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.dokunmayı_başlat());
        assert!(grafik.dokunma_yakınlaştır(0.5, 0.5, 1.25)?);
        grafik.dokunmayı_bitir();
        assert!(grafik.yakınlaştırılmış());
        assert!(grafik.görünür_x_aralığı().en_az > 1.0);
        Ok(())
    }
}
