use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const Y_SCALE_DRAG_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = y_scale_drag_kartı()?;
// Eksen sürükleme çekirdekte çözülür; yüzey yalnız konum ve Shift durumunu iletir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `y-scale-drag.html` verisini, bağımsız Y ölçeklerini ve eksen
/// sürükleme eklentisini taşır.
pub fn y_scale_drag_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(1_200, 600)?
        .başlık("Draggable x & y scales (hold shift to grow/contract)")
        .x_zaman(false)
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
        .birincil_y_ölçeği("meter")
        .y_eksen_etiketi("km/h")
        .birincil_y_eksen_rengi("red")
        .otomatik_y_eksen_genişliği(true)
        .otomatik_y_eksen_genişliği_hesabı(25.0, 6.0)
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("meter")
                .eksen(true)
                .eksen_rengi("red")
                .eksen_etiketi("km/h")
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
        )
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("km/h")
                .sağda(true)
                .ızgara(false)
                .eksen(true)
                .eksen_rengi("blue")
                .eksen_etiketi("meter")
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
        )
        .etkileşimler(ortak_kart_etkileşimleri().eksen_sürükleme(true))
        .seri(SeriSeçenekleri::yeni("Price").renk("red").ölçek("meter"))
        .seri(SeriSeçenekleri::yeni("Amount").renk("blue").ölçek("km/h"));
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0, 2.0, 3.0, 4.0],
        vec![
            vec![Some(1.0), Some(3.0), Some(2.0), Some(4.0), Some(3.0)],
            vec![Some(6.0), Some(8.0), Some(3.0), Some(7.0), Some(9.0)],
        ],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{EksenHedefi, Grafik};

    #[test]
    fn kaynak_veri_ve_iki_bağımsız_y_ölçeği_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = y_scale_drag_kartı()?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0, 3.0, 4.0]);
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let (sol, _, üst, alt) = grafik.çizim_alanı_boyutta(1_200, 600);
        assert_eq!(sol, 49.0);
        assert_eq!(alt - üst, 504.0);
        assert_eq!(
            grafik.eksen_vuruşu_boyutta(1_200, 600, 20.0, 300.0),
            Some(EksenHedefi::Y("meter".to_string()))
        );
        assert_eq!(
            grafik.eksen_vuruşu_boyutta(1_200, 600, 1_180.0, 300.0),
            Some(EksenHedefi::Y("km/h".to_string()))
        );

        let meter_önce = grafik.seri_görünür_y_aralığı(0);
        let kmh_önce = grafik.seri_görünür_y_aralığı(1);
        assert!(grafik.eksen_sürüklemeyi_başlat(1_200, 600, 20.0, 300.0));
        assert!(grafik.eksen_sürükle(20.0, 340.0, false)?);
        grafik.eksen_sürüklemeyi_bitir();
        let meter_sonra = grafik.seri_görünür_y_aralığı(0);
        let beklenen_kaydırma = 40.0 * (4.3 - 0.7) / 504.0;
        assert!(meter_sonra.is_some_and(|aralık| {
            (aralık.en_az - (0.7 + beklenen_kaydırma)).abs() < 1e-12
                && (aralık.en_çok - (4.3 + beklenen_kaydırma)).abs() < 1e-12
        }));
        assert_eq!(grafik.seri_görünür_y_aralığı(1), kmh_önce);
        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert_eq!(
            grafik.seri_görünür_y_aralığı(0),
            Some(crate::Aralık::yeni(0.0, 1.0)?)
        );
        assert!(grafik.seri_görünürlüğünü_ayarla(0, true)?);
        assert_eq!(grafik.seri_görünür_y_aralığı(0), meter_önce);
        assert!(grafik.eksen_sürüklemeyi_başlat(1_200, 600, 1_180.0, 300.0));
        assert!(grafik.eksen_sürükle(1_180.0, 340.0, false)?);
        grafik.eksen_sürüklemeyi_bitir();
        let kmh_sonra = grafik
            .seri_görünür_y_aralığı(1)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let kmh_kaydırma = 40.0 * (9.6 - 2.4) / 504.0;
        assert!((kmh_sonra.en_az - (2.4 + kmh_kaydırma)).abs() < 1e-12);
        assert!((kmh_sonra.en_çok - (9.6 + kmh_kaydırma)).abs() < 1e-12);
        Ok(())
    }

    #[test]
    fn shift_iki_uçtan_büyütür_ve_geçersiz_girdi_görünümü_bozmaz() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = y_scale_drag_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = grafik.görünür_x_aralığı();
        assert!(grafik.eksen_sürüklemeyi_başlat(1_200, 600, 400.0, 580.0));
        assert!(grafik.eksen_sürükle(500.0, 580.0, false)?);
        let kaydırılmış = grafik.görünür_x_aralığı();
        let (sol, sağ, _, _) = grafik.çizim_alanı_boyutta(1_200, 600);
        let beklenen_kaydırma = -100.0 * 4.0 / f64::from(sağ - sol);
        assert!(kaydırılmış.en_az < önce.en_az);
        assert!(kaydırılmış.en_çok < önce.en_çok);
        assert!((kaydırılmış.en_az - (önce.en_az + beklenen_kaydırma)).abs() < 1e-12);
        assert!((kaydırılmış.en_çok - (önce.en_çok + beklenen_kaydırma)).abs() < 1e-12);
        assert_eq!(
            kaydırılmış.en_çok - kaydırılmış.en_az,
            önce.en_çok - önce.en_az
        );
        assert!(grafik.tam_görünüm());
        assert!(grafik.eksen_sürüklemeyi_başlat(1_200, 600, 400.0, 580.0));
        assert!(grafik.eksen_sürükle(500.0, 580.0, true)?);
        let sonra = grafik.görünür_x_aralığı();
        assert!((sonra.en_az - (önce.en_az - beklenen_kaydırma)).abs() < 1e-12);
        assert!((sonra.en_çok - (önce.en_çok + beklenen_kaydırma)).abs() < 1e-12);
        assert!(!grafik.eksen_sürükle(f32::NAN, 580.0, false)?);
        Ok(())
    }
}
