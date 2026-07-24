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
        .başlık("Wind Direction")
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
    fn kaynak_veri_eksenler_ve_yön_vektörleri_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = wind_direction_kartı()?;
        assert_eq!(veri.uzunluk(), 143);
        assert_eq!(veri.seriler().len(), 3);
        assert!(
            veri.seriler()
                .iter()
                .all(|seri| seri.iter().filter(|değer| değer.is_none()).count() == 4)
        );
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yön_çizgileri = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Çizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    ..
                } if renk == "blue" => Some((başlangıç, bitiş)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(yön_çizgileri.len(), 139);
        assert!(yön_çizgileri.iter().all(|(başlangıç, bitiş)| {
            let dx = bitiş.x - başlangıç.x;
            let dy = bitiş.y - başlangıç.y;
            ((dx * dx + dy * dy).sqrt() - 15.0).abs() < 0.001
        }));
        Ok(())
    }
}
