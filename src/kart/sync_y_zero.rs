use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri,
};

pub const SYNC_Y_ZERO_KART_TANIM_ÖRNEĞİ: &str = r##"let aşama = SyncYZeroAşaması::SıfırHizalı;
let (seçenekler, veri) = sync_y_zero_kartı(aşama)?;
// Ham, simetrik ve ortak sıfır pikselli aralıklar çekirdekte hesaplanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncYZeroAşaması {
    Ham,
    Simetrik,
    SıfırHizalı,
}

impl SyncYZeroAşaması {
    pub const TÜMÜ: [Self; 3] = [Self::Ham, Self::Simetrik, Self::SıfırHizalı];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Ham => "raw",
            Self::Simetrik => "symmetric",
            Self::SıfırHizalı => "zero-aligned",
        }
    }

    pub const fn açıklama(self) -> &'static str {
        match self {
            Self::Ham => "Bağımsız veri aralıkları",
            Self::Simetrik => "Sıfıra göre simetrik aralıklar",
            Self::SıfırHizalı => "Ortak sıfır pikseline hizalı aralıklar",
        }
    }
}

const HAM_ARALIKLAR: [(f64, f64); 3] = [(-10.0, 100.0), (-150.0, 1_732.0), (3_751.0, 10_000.0)];

pub fn sync_y_zero_aralıkları(aşama: SyncYZeroAşaması) -> Result<[Aralık; 3], UplotHatası> {
    let simetrik = HAM_ARALIKLAR.map(|(en_az, en_çok)| en_az.abs().max(en_çok.abs()));
    let aralıklar = match aşama {
        SyncYZeroAşaması::Ham => HAM_ARALIKLAR,
        SyncYZeroAşaması::Simetrik => simetrik.map(|mutlak| (-mutlak, mutlak)),
        SyncYZeroAşaması::SıfırHizalı => {
            // Kaynaktaki syncRange(): simetrik ölçeklerde bütün ham minimumları
            // ve maksimumları görünür tutan ortak piksel kesişimini bulur.
            let alt_oran = HAM_ARALIKLAR
                .iter()
                .zip(simetrik)
                .map(|((en_az, _), mutlak)| (mutlak - en_az) / (2.0 * mutlak))
                .fold(f64::NEG_INFINITY, f64::max);
            let üst_oran = HAM_ARALIKLAR
                .iter()
                .zip(simetrik)
                .map(|((_, en_çok), mutlak)| (mutlak - en_çok) / (2.0 * mutlak))
                .fold(f64::INFINITY, f64::min);
            simetrik.map(|mutlak| {
                (
                    mutlak - alt_oran * 2.0 * mutlak,
                    mutlak - üst_oran * 2.0 * mutlak,
                )
            })
        }
    };
    let [ilk, ikinci, üçüncü] = aralıklar.map(|(en_az, en_çok)| Aralık::yeni(en_az, en_çok));
    Ok([ilk?, ikinci?, üçüncü?])
}

pub fn sync_y_zero_kartı(
    aşama: SyncYZeroAşaması,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let aralıklar = sync_y_zero_aralıkları(aşama)?;
    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık("Sync Y Zero")
        .x_zaman(false)
        .y_ızgarası_göster(false)
        .birincil_y_eksen_rengi("red")
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y")
                .aralık(aralıklar[0])
                .eksen(true)
                .eksen_rengi("red")
                .ızgara(false),
        )
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y2")
                .aralık(aralıklar[1])
                .eksen(true)
                .eksen_rengi("green")
                .ızgara(false),
        )
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y3")
                .aralık(aralıklar[2])
                .eksen(true)
                .eksen_rengi("blue")
                .ızgara(false),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Data 1").ölçek("y").renk("red"))
        .seri(SeriSeçenekleri::yeni("Data 2").ölçek("y2").renk("green"))
        .seri(SeriSeçenekleri::yeni("Data 3").ölçek("y3").renk("blue"));
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0, 2.0],
        vec![
            vec![Some(-10.0), Some(35.0), Some(100.0)],
            vec![Some(-150.0), Some(1_732.0), Some(-30.0)],
            vec![Some(3_751.0), Some(10_000.0), Some(7_389.0)],
        ],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_veri_ve_üç_bağımsız_ölçek_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = sync_y_zero_kartı(SyncYZeroAşaması::Ham)?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0]);
        assert_eq!(
            seçenekler.seriler.get(1).map(|seri| seri.ölçek.as_str()),
            Some("y2")
        );
        assert_eq!(
            seçenekler.seriler.get(2).map(|seri| seri.ölçek.as_str()),
            Some("y3")
        );
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "blue"))
        );
        let eksen_x_değerleri: Vec<f32> = ["red", "green", "blue"]
            .into_iter()
            .filter_map(|renk| {
                sahne.komutlar().iter().find_map(|komut| match komut {
                    Komut::Metin {
                        konum,
                        renk: metin_rengi,
                        ..
                    } if metin_rengi == renk => Some(konum.x),
                    _ => None,
                })
            })
            .collect();
        assert_eq!(eksen_x_değerleri.len(), 3);
        let [birinci, ikinci, üçüncü] = eksen_x_değerleri.as_slice() else {
            return Err(UplotHatası::YetersizVeri {
                uzunluk: eksen_x_değerleri.len(),
            });
        };
        assert_ne!(birinci, ikinci);
        assert_ne!(ikinci, üçüncü);
        Ok(())
    }

    #[test]
    fn üç_kaynak_aşaması_aynı_sıfır_pikseline_ulaşır() -> Result<(), UplotHatası> {
        let ham = sync_y_zero_aralıkları(SyncYZeroAşaması::Ham)?;
        assert_eq!((ham[2].en_az, ham[2].en_çok), (3_751.0, 10_000.0));
        let simetrik = sync_y_zero_aralıkları(SyncYZeroAşaması::Simetrik)?;
        assert_eq!((simetrik[1].en_az, simetrik[1].en_çok), (-1_732.0, 1_732.0));
        let hizalı = sync_y_zero_aralıkları(SyncYZeroAşaması::SıfırHizalı)?;
        let sıfır_oranları =
            hizalı.map(|aralık| (0.0 - aralık.en_az) / (aralık.en_çok - aralık.en_az));
        assert!((sıfır_oranları[0] - sıfır_oranları[1]).abs() < 1e-12);
        assert!((sıfır_oranları[1] - sıfır_oranları[2]).abs() < 1e-12);
        assert!((hizalı[0].en_az + 10.0).abs() < 1e-12);
        assert!((hizalı[0].en_çok - 100.0).abs() < 1e-12);
        Ok(())
    }
}
