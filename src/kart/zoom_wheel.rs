use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = zoom_wheel_kartı()?;
// Resmî wheelZoomPlugin'in 0.75 katsayılı, fare odaklı X/Y
// yakınlaştırması ve sınır sıkıştırması çekirdekte uygulanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/zoom-wheel.html` kartının boyutunu, iki serisini ve bütün
/// sayısal veri noktalarını korur. Eklenti davranışı ortak çekirdek profilidir.
pub fn zoom_wheel_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0].to_vec();
    let bir = [40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
        .into_iter()
        .map(Some)
        .collect();
    let iki = [18.0, 24.0, 37.0, 55.0, 55.0, 60.0, 63.0]
        .into_iter()
        .map(Some)
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Wheel Zoom & Drag")
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
    fn kaynak_verisi_ve_resmî_tekerlek_katsayısı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = zoom_wheel_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (600, 400));
        assert_eq!(veri.seriler().len(), 2);
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(40.0))
        );
        assert_eq!(
            veri.seriler().last().and_then(|seri| seri.last()).copied(),
            Some(Some(63.0))
        );
        assert_eq!(
            seçenekler.etkileşimler.tekerlek_ayarları.ayrık_katsayı,
            0.75
        );

        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
        assert!(grafik.yakınlaştırılmış());
        assert!(grafik.görünür_x_aralığı().en_az > 1.0);
        Ok(())
    }
}
