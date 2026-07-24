use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const MEASURE_DATUMS_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = measure_datums_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
grafik.ölçüm_datumunu_ayarla(1, 0.25, 0.40);
grafik.ölçüm_datumunu_ayarla(2, 0.75, 0.60);"##;

pub fn measure_datums_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![
            vec![0.0, 100.0, 30.0, 25.0, 7.0]
                .into_iter()
                .map(Some)
                .collect(),
        ],
    )?;
    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .x_zaman(false)
        .ölçüm_datumları(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Data 1")
                .renk("red")
                .dolgu("rgba(255,0,0,0.1)"),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_ve_datum_durum_makinesi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = measure_datums_kartı()?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.ölçüm_datumunu_ayarla(1, 0.25, 0.4));
        assert!(grafik.ölçüm_datumunu_ayarla(2, 0.75, 0.6));
        let [bir, iki] = grafik.ölçüm_datumları();
        assert!(bir.is_some());
        assert!(iki.is_some());
        let sahne = grafik.çiz();
        assert_eq!(
            sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(
                    komut,
                    Komut::Daire { çizgi, .. } if çizgi == "blue" || çizgi == "orange"
                ))
                .count(),
            2
        );
        assert!(sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik.starts_with("dx: "))
        ));
        assert!(grafik.ölçüm_datumlarını_temizle());
        assert_eq!(grafik.ölçüm_datumları(), [None, None]);
        Ok(())
    }

    #[test]
    fn geçersiz_tuş_ve_imleç_konumu_durumu_değiştirmez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = measure_datums_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(!grafik.ölçüm_datumunu_ayarla(0, 0.5, 0.5));
        assert!(!grafik.ölçüm_datumunu_ayarla(1, -0.1, 0.5));
        assert!(!grafik.ölçüm_datumunu_ayarla(2, 0.5, f64::NAN));
        assert_eq!(grafik.ölçüm_datumları(), [None, None]);
        Ok(())
    }
}
