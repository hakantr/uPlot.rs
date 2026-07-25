#[path = "veri/wind_direction.rs"]
mod kaynak_veri;

use kaynak_veri::{RÜZGAR_HIZLARI, RÜZGAR_YÖNLERİ, SICAKLIKLAR, ZAMANLAR};

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, RüzgarYönüDüzeni, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri,
};

pub const WIND_DIRECTION_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = wind_direction_kartı()?;
// Hız konumları ve yön dereceleri çekirdekte 15 px vektörlere dönüştürülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `wind-direction.html` saatlik verisini, iki Y eksenini ve özel
/// yön-vektörü yolunu birebir taşır.
pub fn wind_direction_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .y_eksen_etiketi("Temp °C")
        .birincil_y_eksen_rengi("orangered")
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y2")
                .aralık(Aralık::yeni(0.0, 30.0)?)
                .sağda(true)
                .ızgara(false)
                .eksen(true)
                .eksen_rengi("purple")
                .eksen_etiketi("Wind m/s"),
        )
        .rüzgar_yönü(RüzgarYönüDüzeni::yeni(1, 2, "y2"))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Temp °C").renk("orangered"))
        .seri(SeriSeçenekleri::yeni("Wind m/s").renk("purple").ölçek("y2"))
        .seri(
            SeriSeçenekleri::yeni("Wind dir °")
                .renk("blue")
                .çizgi_kalınlığı(0.0)
                .noktaları_göster(false)
                .otomatik_ölçeğe_katıl(false),
        );
    let veri = HizalıVeri::yeni(
        ZAMANLAR.to_vec(),
        vec![
            SICAKLIKLAR.to_vec(),
            RÜZGAR_HIZLARI.to_vec(),
            RÜZGAR_YÖNLERİ.to_vec(),
        ],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_eksenler_ve_tek_yolda_yön_vektörleri_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = wind_direction_kartı()?;
        assert_eq!(veri.uzunluk(), 143);
        assert_eq!(veri.seriler().len(), 3);
        assert!(seçenekler.başlık.is_empty());
        let yön_serisi = seçenekler
            .seriler
            .get(2)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!(!yön_serisi.otomatik_ölçeğe_katıl);
        assert_eq!(yön_serisi.noktaları_göster, Some(false));
        assert!(
            veri.seriler()
                .iter()
                .all(|seri| seri.iter().filter(|değer| değer.is_none()).count() == 4)
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let ilk_y = grafik.görünür_y_aralığı();
        let sahne = grafik.çiz();
        let yön_yolları = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Yol {
                    parçalar,
                    renk,
                    kalınlık,
                } if renk == "blue" && (*kalınlık - 1.0).abs() <= f32::EPSILON => Some(parçalar),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(yön_yolları.len(), 1);
        let yön_yolu = yön_yolları
            .first()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert_eq!(yön_yolu.len(), 139);
        assert!(yön_yolu.iter().all(|parça| {
            let [başlangıç, bitiş] = parça.as_slice() else {
                return false;
            };
            let dx = bitiş.x - başlangıç.x;
            let dy = bitiş.y - başlangıç.y;
            ((dx * dx + dy * dy).sqrt() - 15.0).abs() < 0.001
        }));

        assert!(grafik.seri_görünürlüğünü_ayarla(2, false)?);
        assert!(
            !grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "blue"))
        );
        assert!(grafik.seri_görünürlüğünü_ayarla(2, true)?);
        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert_ne!(grafik.görünür_y_aralığı(), ilk_y);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 1.0)?);
        assert!(grafik.seri_görünürlüğünü_ayarla(0, true)?);

        assert!(grafik.seçim_yakınlaştır(0.25, 0.75)?);
        let (sol, sağ, _, _) = grafik.çizim_alanı_boyutta(800, 400);
        let yakın_sahne = grafik.çiz();
        let yakın_parçalar = yakın_sahne
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Yol {
                    parçalar, renk, ..
                } if renk == "blue" => Some(parçalar),
                _ => None,
            })
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!(
            yakın_parçalar
                .iter()
                .any(|parça| parça.first().is_some_and(|nokta| nokta.x < sol))
        );
        assert!(
            yakın_parçalar
                .iter()
                .any(|parça| parça.first().is_some_and(|nokta| nokta.x > sağ))
        );
        Ok(())
    }
}
