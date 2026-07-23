use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const PIXEL_ALIGN_KANIT_TOHUMU: u32 = 0x5049_5845;
pub const PIXEL_ALIGN_PENCERE_MS: f64 = 120_000.0;
pub const PIXEL_ALIGN_ARALIK_MS: f64 = 1_000.0;
const KANIT_BAŞLANGICI_MS: f64 = 1_700_000_000_000.0;

pub const PIXEL_ALIGN_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    pixel_align_kartı(PixelAlignÖrneği::Kapalı, 121)?;
// pxAlign çekirdekte çözülür; GPUI, SVG ve WASM aynı alt-piksel
// sahne geometrisini tüketir.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixelAlignÖrneği {
    Varsayılan,
    Kapalı,
}

impl PixelAlignÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::Varsayılan, Self::Kapalı];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Varsayılan => "pixel-align-default",
            Self::Kapalı => "pixel-align-off",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Varsayılan => "pxAlign: 1 (default)",
            Self::Kapalı => "pxAlign: 0 (off)",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

/// `demos/pixel-align.html` içindeki 120 saniyelik pencereyi, bir saniyelik
/// örnekleme aralığını ve üç rastgele seri üretecini yeniden kurar.
///
/// Tarayıcıdaki `Date.now()` yerine görsel kanıtların tekrar üretilebilmesi
/// için sabit bir başlangıç; `Math.random()` yerine açık bir kanıt tohumu
/// kullanılır. Üreteç ve parametreler kaynakla aynıdır.
pub fn pixel_align_kartı(
    örnek: PixelAlignÖrneği,
    adım_sayısı: usize,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let adım_sayısı = adım_sayısı.clamp(1, 10_000);
    let atlanacak = adım_sayısı.saturating_sub(1_000);
    let başlangıç_indeksi = atlanacak;
    let mut rastgele = KanıtRastgele::yeni(PIXEL_ALIGN_KANIT_TOHUMU);
    let mut x = Vec::with_capacity(adım_sayısı.min(1_000));
    let mut seriler = [
        Vec::with_capacity(adım_sayısı.min(1_000)),
        Vec::with_capacity(adım_sayısı.min(1_000)),
        Vec::with_capacity(adım_sayısı.min(1_000)),
    ];

    for indeks in 0..adım_sayısı {
        let değerler = [
            0.0 - rastgele.sonraki(),
            -1.0 - rastgele.sonraki(),
            -2.0 - rastgele.sonraki(),
        ];
        if indeks >= başlangıç_indeksi {
            x.push(KANIT_BAŞLANGICI_MS + indeks as f64 * PIXEL_ALIGN_ARALIK_MS);
            for (seri, değer) in seriler.iter_mut().zip(değerler) {
                seri.push(Some(değer));
            }
        }
    }

    let şimdi = KANIT_BAŞLANGICI_MS + adım_sayısı.saturating_sub(1) as f64 * PIXEL_ALIGN_ARALIK_MS;
    let hizalama = match örnek {
        PixelAlignÖrneği::Varsayılan => 1.0,
        PixelAlignÖrneği::Kapalı => 0.0,
    };
    let ortak_seri = |etiket: &str, renk: &str| {
        SeriSeçenekleri::yeni(etiket)
            .renk(renk)
            .boşlukları_birleştir(true)
            .noktaları_göster(true)
            .piksel_hizası(hizalama)
    };
    let seçenekler = GrafikSeçenekleri::yeni(1_200, 400)?
        .başlık(örnek.başlık())
        .x_zaman(true)
        .x_zaman_milisaniye(true)
        .x_aralığı(Aralık::yeni(şimdi - PIXEL_ALIGN_PENCERE_MS, şimdi)?)
        .y_aralığı(Aralık::yeni(-3.5, 1.5)?)
        .piksel_hizası(hizalama)
        .seri(ortak_seri("Linear", "red"))
        .seri(ortak_seri("Spline", "blue").eğri())
        .seri(ortak_seri("Stepped", "purple").basamak_sonra())
        .etkileşimler(ortak_kart_etkileşimleri());
    Ok((seçenekler, HizalıVeri::yeni(x, seriler.into())?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    fn doğrusal_yol_noktaları(grafik: Grafik) -> Vec<crate::Nokta> {
        grafik
            .çiz()
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Yol {
                    parçalar, renk, ..
                } if renk == "red" => Some(parçalar),
                _ => None,
            })
            .flat_map(|parçalar| parçalar.iter().flatten().copied())
            .collect()
    }

    #[test]
    fn kaynak_penceresi_ve_üç_yol_türü_korunur() -> Result<(), UplotHatası> {
        for örnek in PixelAlignÖrneği::TÜMÜ {
            let (seçenekler, veri) = pixel_align_kartı(örnek, 140)?;
            assert_eq!(veri.uzunluk(), 140);
            assert_eq!(veri.seriler().len(), 3);
            assert_eq!(seçenekler.y_aralığı, Some(Aralık::yeni(-3.5, 1.5)?));
            assert!(
                seçenekler
                    .seriler
                    .iter()
                    .all(|seri| seri.noktaları_göster == Some(true))
            );
        }
        Ok(())
    }

    #[test]
    fn varsayılan_tam_piksele_hizalanır_kapalı_alt_pikseli_korur() -> Result<(), UplotHatası> {
        let (hizalı_seçenekler, hizalı_veri) =
            pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 121)?;
        let hizalı = doğrusal_yol_noktaları(Grafik::yeni(hizalı_seçenekler, hizalı_veri)?);
        assert!(!hizalı.is_empty());
        assert!(
            hizalı
                .iter()
                .all(|nokta| nokta.x.fract() == 0.0 && nokta.y.fract() == 0.0)
        );

        let (serbest_seçenekler, serbest_veri) = pixel_align_kartı(PixelAlignÖrneği::Kapalı, 121)?;
        let serbest = doğrusal_yol_noktaları(Grafik::yeni(serbest_seçenekler, serbest_veri)?);
        assert!(
            serbest
                .iter()
                .any(|nokta| nokta.x.fract() != 0.0 || nokta.y.fract() != 0.0)
        );
        Ok(())
    }

    #[test]
    fn aynı_adım_sayısı_aynı_kanıt_verisini_üretir() -> Result<(), UplotHatası> {
        let (_, sol) = pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 257)?;
        let (_, sağ) = pixel_align_kartı(PixelAlignÖrneği::Kapalı, 257)?;
        assert_eq!(sol, sağ);
        Ok(())
    }

    #[test]
    fn kaynak_döngüsü_en_fazla_bin_örneği_tutar() -> Result<(), UplotHatası> {
        let (_, veri) = pixel_align_kartı(PixelAlignÖrneği::Varsayılan, 1_200)?;
        assert_eq!(veri.uzunluk(), 1_000);
        Ok(())
    }
}
