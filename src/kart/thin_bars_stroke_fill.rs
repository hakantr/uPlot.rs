use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇubukDüzeni, ÇubukYönü,
};

pub const THIN_BARS_STROKE_FILL_KART_TANIM_ÖRNEĞİ: &str = r##"let örnek = ThinBarsÖrneği::Geometri {
    hizalama: 0, genişlik_yüzdesi: 60, boşluk: 0, yön: 1, vuruş: 1,
};
let (seçenekler, veri) = thin_bars_stroke_fill_kartı(örnek)?;
// Vuruş/dolgu düşümü, hizalama, boşluk ve ters yön çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinBarsYoğunluk {
    Normal30,
    VuruşsuzKırmızıDolgu200,
    VuruşSıfırNormalDolgu200,
    İnceVuruşDolguyaDönüşür200,
    VuruşVeDolgu1000,
    VuruşVeDolgu1200,
    VuruşVeDolgu1600,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinBarsÖrneği {
    Yoğunluk(ThinBarsYoğunluk),
    Geometri {
        hizalama: i8,
        genişlik_yüzdesi: u8,
        boşluk: u8,
        yön: i8,
        vuruş: u8,
    },
}

impl ThinBarsÖrneği {
    pub fn tümü() -> Vec<Self> {
        let mut sonuç = vec![
            Self::Yoğunluk(ThinBarsYoğunluk::Normal30),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşsuzKırmızıDolgu200),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşSıfırNormalDolgu200),
            Self::Yoğunluk(ThinBarsYoğunluk::İnceVuruşDolguyaDönüşür200),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1000),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1200),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1600),
        ];
        for (hizalama, genişlik_yüzdesi, boşluk) in [
            (0, 60, 0),
            (0, 60, 16),
            (0, 100, 0),
            (0, 100, 16),
            (1, 60, 0),
            (1, 60, 16),
            (1, 100, 0),
            (1, 100, 16),
            (-1, 60, 0),
            (-1, 60, 16),
            (-1, 100, 0),
            (-1, 100, 16),
        ] {
            for vuruş in [1, 4] {
                for yön in [1, -1] {
                    sonuç.push(Self::Geometri {
                        hizalama,
                        genişlik_yüzdesi,
                        boşluk,
                        yön,
                        vuruş,
                    });
                }
            }
        }
        sonuç
    }

    pub fn kimlik(self) -> String {
        match self {
            Self::Yoğunluk(örnek) => format!("thin-bars-{}", örnek.kimlik()),
            Self::Geometri {
                hizalama,
                genişlik_yüzdesi,
                boşluk,
                yön,
                vuruş,
            } => format!("thin-bars-a{hizalama}-w{genişlik_yüzdesi}-g{boşluk}-d{yön}-s{vuruş}"),
        }
    }

    pub fn başlık(self) -> String {
        match self {
            Self::Yoğunluk(örnek) => örnek.başlık().to_string(),
            Self::Geometri {
                hizalama,
                genişlik_yüzdesi,
                boşluk,
                yön,
                vuruş,
            } => {
                let genişlik = f32::from(genişlik_yüzdesi) / 100.0;
                format!(
                    "{{align: {hizalama}, width: {genişlik:.1}, gap: {boşluk}, dir: {yön}, stroke: {vuruş}}}"
                )
            }
        }
    }

    pub const fn boyut(self) -> (u32, u32) {
        match self {
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1000) => (1_000, 200),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1200) => (1_200, 200),
            Self::Yoğunluk(ThinBarsYoğunluk::VuruşVeDolgu1600) => (1_600, 200),
            Self::Yoğunluk(_) => (800, 200),
            Self::Geometri { .. } => (400, 200),
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::tümü()
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

impl ThinBarsYoğunluk {
    const fn kimlik(self) -> &'static str {
        match self {
            Self::Normal30 => "normal-30",
            Self::VuruşsuzKırmızıDolgu200 => "no-stroke-red-fill-200",
            Self::VuruşSıfırNormalDolgu200 => "width-zero-normal-fill-200",
            Self::İnceVuruşDolguyaDönüşür200 => "thin-stroke-fallback-200",
            Self::VuruşVeDolgu1000 => "stroke-fill-1000",
            Self::VuruşVeDolgu1200 => "stroke-fill-1200",
            Self::VuruşVeDolgu1600 => "stroke-fill-1600",
        }
    }

    const fn başlık(self) -> &'static str {
        match self {
            Self::Normal30 => "Normal",
            Self::VuruşsuzKırmızıDolgu200 => "No stroke, red fill",
            Self::VuruşSıfırNormalDolgu200 => "Too thin, but stroke width 0, use normal fill",
            Self::İnceVuruşDolguyaDönüşür200 => "Too thin, use stroke color for fill",
            Self::VuruşVeDolgu1000 | Self::VuruşVeDolgu1200 | Self::VuruşVeDolgu1600 => {
                "Still stroke & fill"
            }
        }
    }
}

pub fn thin_bars_stroke_fill_kartı(
    örnek: ThinBarsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (uzunluk, değerler, genişlik_oranı, boşluk, hizalama, ters, vuruş, renk, dolgu) =
        match örnek {
            ThinBarsÖrneği::Yoğunluk(yoğunluk) => {
                let uzunluk = if yoğunluk == ThinBarsYoğunluk::Normal30 {
                    30
                } else {
                    200
                };
                let (vuruş, renk, dolgu) = match yoğunluk {
                    ThinBarsYoğunluk::VuruşsuzKırmızıDolgu200 => (0.0, "none", "red"),
                    ThinBarsYoğunluk::VuruşSıfırNormalDolgu200 => {
                        (0.0, "red", "rgba(255,0,0,0.2)")
                    }
                    _ => (1.0, "red", "rgba(255,0,0,0.2)"),
                };
                (
                    uzunluk,
                    vec![5.0; uzunluk],
                    0.6,
                    0.0,
                    0,
                    false,
                    vuruş,
                    renk,
                    dolgu,
                )
            }
            ThinBarsÖrneği::Geometri {
                hizalama,
                genişlik_yüzdesi,
                boşluk,
                yön,
                vuruş,
            } => (
                5,
                vec![0.0, 1.0, 2.0, 3.0, 4.0],
                f32::from(genişlik_yüzdesi) / 100.0,
                f32::from(boşluk),
                hizalama,
                yön < 0,
                f32::from(vuruş),
                "#00f",
                "rgb(0,0,255,0.1)",
            ),
        };
    let x = (0..uzunluk).map(|indeks| indeks as f64).collect();
    let veri = HizalıVeri::yeni(x, vec![değerler.into_iter().map(Some).collect()])?;
    let (genişlik, yükseklik) = örnek.boyut();
    let y_aralığı = match örnek {
        ThinBarsÖrneği::Yoğunluk(_) => Aralık::yeni(0.0, 10.0)?,
        ThinBarsÖrneği::Geometri { .. } => Aralık::yeni(0.0, 4.0)?,
    };
    let noktalar_görünür = matches!(
        örnek,
        ThinBarsÖrneği::Yoğunluk(ThinBarsYoğunluk::Normal30) | ThinBarsÖrneği::Geometri { .. }
    );
    let düzen = ÇubukDüzeni::yeni(ÇubukYönü::Dikey)
        .ters(ters)
        .genişlik_oranı(genişlik_oranı)
        .ek_boşluk(boşluk)
        .hizalama(hizalama)
        .x_kenar_paylı(false)
        .değer_etiketleri(false);
    let seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .y_aralığı(y_aralığı)
        .çubuk_düzeni(düzen)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk(renk)
                .dolgu(dolgu)
                .çizgi_kalınlığı(vuruş)
                .noktaları_göster(noktalar_görünür)
                .nokta_stili(5.0, 1.0, Some("#ffffff")),
        );
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn elli_beş_kaynak_yüzeyi_boyut_veri_ve_geometriyle_çizilir() -> Result<(), UplotHatası> {
        let örnekler = ThinBarsÖrneği::tümü();
        assert_eq!(örnekler.len(), 55);
        for örnek in örnekler {
            let (seçenekler, veri) = thin_bars_stroke_fill_kartı(örnek)?;
            let beklenen_y = match örnek {
                ThinBarsÖrneği::Yoğunluk(_) => Aralık::yeni(0.0, 10.0)?,
                ThinBarsÖrneği::Geometri { .. } => Aralık::yeni(0.0, 4.0)?,
            };
            assert_eq!(seçenekler.y_aralığı, Some(beklenen_y));
            let beklenen = match örnek {
                ThinBarsÖrneği::Yoğunluk(ThinBarsYoğunluk::Normal30) => 30,
                ThinBarsÖrneği::Yoğunluk(_) => 200,
                ThinBarsÖrneği::Geometri { .. } => 4,
            };
            let beklenen_nokta = match örnek {
                ThinBarsÖrneği::Yoğunluk(ThinBarsYoğunluk::Normal30) => 30,
                ThinBarsÖrneği::Yoğunluk(_) => 0,
                ThinBarsÖrneği::Geometri { .. } => 5,
            };
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            let çubuklar = sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Dikdörtgen { .. }))
                .count();
            let noktalar = sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Daire { .. }))
                .count();
            assert_eq!(çubuklar, beklenen, "{}", örnek.kimlik());
            assert_eq!(noktalar, beklenen_nokta, "{}", örnek.kimlik());
        }
        Ok(())
    }

    #[test]
    fn ince_vuruş_kaynak_kuralıyla_vuruş_rengine_düşer() -> Result<(), UplotHatası> {
        let örnek = ThinBarsÖrneği::Yoğunluk(ThinBarsYoğunluk::İnceVuruşDolguyaDönüşür200);
        let (seçenekler, veri) = thin_bars_stroke_fill_kartı(örnek)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz_boyutta(600, 200, None);
        assert!(sahne.komutlar().iter().all(|komut| match komut {
            Komut::Dikdörtgen {
                dolgu, kalınlık, ..
            } => dolgu == "red" && *kalınlık == 0.0,
            _ => true,
        }));
        Ok(())
    }

    #[test]
    fn hizalama_boşluk_yön_ve_vuruş_birleşimleri_benzersizdir() {
        let kimlikler = ThinBarsÖrneği::tümü()
            .into_iter()
            .map(ThinBarsÖrneği::kimlik)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(kimlikler.len(), 55);
    }
}
