use crate::{
    Aralık, EtkileşimSeçenekleri, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası,
    ZamanDilimi, ortak_kart_etkileşimleri,
};

pub const TIMEZONES_DST_KART_TANIM_ÖRNEĞİ: &str = r#"let örnek = TimezonesDstÖrneği::yeni(0)?;
let (seçenekler, veri) = timezones_dst_kartı(örnek)?;
let grafik = Grafik::yeni(seçenekler, veri)?;"#;

const YIL_BAŞI: i64 = 1_704_067_200;
const SAAT: i64 = 3_600;
const KARŞILAŞTIRMA_BAŞLANGIÇLARI: [i64; 4] =
    [1_711_800_000, 1_729_944_000, 1_709_985_600, 1_730_548_800];
const ARTIM_BAŞLANGIÇLARI: [i64; 4] = [1_711_843_200, 1_729_983_600, 1_710_050_400, 1_730_610_000];
const ARTIM_GÜNLERİ: [f64; 7] = [0.3, 0.5, 1.0, 1.5, 2.0, 3.0, 4.0];
const ARTIM_BAŞLIKLARI: [&str; 7] = [
    "1h ticks",
    "2h ticks",
    "3h ticks",
    "4h ticks",
    "6h ticks",
    "8h ticks",
    "12h ticks",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimezonesDstÖrneği(u8);

impl TimezonesDstÖrneği {
    pub const SAYI: usize = 51;

    pub fn yeni(indeks: usize) -> Option<Self> {
        (indeks < Self::SAYI)
            .then(|| u8::try_from(indeks).ok())
            .flatten()
            .map(Self)
    }

    pub fn tümü() -> impl Iterator<Item = Self> {
        (0..Self::SAYI).filter_map(Self::yeni)
    }

    pub fn kimlik(self) -> String {
        format!("timezones-dst-{:02}", self.0 + 1)
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        let sıra = kimlik
            .strip_prefix("timezones-dst-")?
            .parse::<usize>()
            .ok()?;
        sıra.checked_sub(1).and_then(Self::yeni)
    }

    pub fn başlık(self) -> String {
        let indeks = usize::from(self.0);
        if indeks < 12 {
            let sıra = indeks % 3;
            let grup = indeks / 3;
            return karşılaştırma_başlığı(grup, sıra).to_string();
        }
        if indeks < 40 {
            return ARTIM_BAŞLIKLARI
                .get((indeks - 12) % 7)
                .copied()
                .unwrap_or("ticks")
                .to_string();
        }
        match indeks {
            40 => "1-Day",
            41 => "2-Day",
            42 => "3-Day",
            43 => "4-Day",
            44 => "1-Month",
            45 => "2-Month",
            46 => "3-Month",
            47 => "4-Month",
            48 => "6-Month",
            49 => "3-Day (ensure Jan 1)",
            _ => "2-Month (ensure Jan 1)",
        }
        .to_string()
    }

    pub fn bölüm(self) -> &'static str {
        let indeks = usize::from(self.0);
        if indeks < 12 {
            return [
                "London's 2024 \"spring forward\" time range",
                "London's 2024 \"fall back\" time range",
                "Chicago's 2024 \"spring forward\" time range",
                "Chicago's 2024 \"fall back\" time range",
            ]
            .get(indeks / 3)
            .copied()
            .unwrap_or("Timezones & DST");
        }
        if indeks < 40 {
            return [
                "Europe/London (Mar 31, 2024: 1am -> 2am)",
                "Europe/London (Oct 27, 2024: 2am -> 1am)",
                "America/Chicago (Mar 10, 2024: 2am -> 3am)",
                "America/Chicago (Nov 3, 2024: 2am -> 1am)",
            ]
            .get((indeks - 12) / 7)
            .copied()
            .unwrap_or("Timezones & DST");
        }
        match indeks {
            40..=43 => "Days",
            44..=48 => "Months",
            _ => "With start offset",
        }
    }

    pub fn zaman_dilimi(self) -> ZamanDilimi {
        let indeks = usize::from(self.0);
        if indeks < 12 {
            return karşılaştırma_zaman_dilimi(indeks / 3, indeks % 3);
        }
        if indeks < 26 {
            ZamanDilimi::EuropeLondon
        } else if indeks < 40 {
            ZamanDilimi::AmericaChicago
        } else {
            ZamanDilimi::Utc
        }
    }

    pub fn senkron_grubu(self) -> Option<u8> {
        (usize::from(self.0) < 12).then_some(self.0 / 3)
    }

    pub fn eksen_artımı(self) -> Option<f64> {
        let indeks = usize::from(self.0);
        if (12..40).contains(&indeks) {
            return [1.0, 2.0, 3.0, 4.0, 6.0, 8.0, 12.0]
                .get((indeks - 12) % 7)
                .copied()
                .map(|saat| saat * 3_600.0);
        }
        if (40..44).contains(&indeks) {
            return [1.0, 2.0, 3.0, 4.0]
                .get(indeks - 40)
                .copied()
                .map(|gün| gün * 86_400.0);
        }
        if (44..49).contains(&indeks) {
            return [1.0, 2.0, 3.0, 4.0, 6.0]
                .get(indeks - 44)
                .copied()
                .map(|ay| ay * 2_592_000.0);
        }
        match indeks {
            49 => Some(3.0 * 86_400.0),
            50 => Some(2.0 * 2_592_000.0),
            _ => None,
        }
    }
}

pub fn timezones_dst_kartı(
    örnek: TimezonesDstÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (x, y) = kaynak_veri(örnek);
    let mut seçenekler = GrafikSeçenekleri::yeni(600, 200)?
        .başlık(örnek.başlık())
        .x_zaman_dilimi(örnek.zaman_dilimi())
        .y_aralığı(Aralık::yeni(0.0, 1.0)?)
        .y_sabit_bölmeler(vec![0.0, 0.5, 1.0])
        .seri(
            SeriSeçenekleri::yeni("2024")
                .renk("red")
                .noktaları_göster(false),
        )
        .etkileşimler(etkileşimler());
    if let Some(artım) = örnek.eksen_artımı() {
        seçenekler = seçenekler.x_zaman_sabit_artımı(artım);
    }
    Ok((
        seçenekler,
        HizalıVeri::yeni(x, vec![y.into_iter().map(Some).collect()])?,
    ))
}

fn etkileşimler() -> EtkileşimSeçenekleri {
    ortak_kart_etkileşimleri()
}

fn kaynak_veri(örnek: TimezonesDstÖrneği) -> (Vec<f64>, Vec<f64>) {
    let indeks = usize::from(örnek.0);
    if indeks < 12 {
        let başlangıç = KARŞILAŞTIRMA_BAŞLANGIÇLARI
            .get(indeks / 3)
            .copied()
            .unwrap_or(YIL_BAŞI);
        return saatlik_dilim(başlangıç, 49, false);
    }
    if indeks < 40 {
        let yerel = indeks - 12;
        let grup = yerel / 7;
        let gün = ARTIM_GÜNLERİ.get(yerel % 7).copied().unwrap_or(1.0);
        let nokta_sayısı = (24.0 * gün + 1.0).floor() as usize;
        let başlangıç = ARTIM_BAŞLANGIÇLARI.get(grup).copied().unwrap_or(YIL_BAŞI);
        return saatlik_dilim(başlangıç, nokta_sayısı, true);
    }
    if indeks < 44 {
        let günler = [10, 15, 25, 37].get(indeks - 40).copied().unwrap_or(10);
        return saatlik_dilim(YIL_BAŞI, 24 * günler + 1, false);
    }
    if indeks < 49 {
        let sonlar = [
            1_719_792_000,
            1_735_689_600,
            1_767_225_600,
            1_798_761_600,
            1_830_768_000,
        ];
        let son = sonlar.get(indeks - 44).copied().unwrap_or(YIL_BAŞI);
        return (vec![YIL_BAŞI as f64, son as f64], vec![0.5, 0.5]);
    }
    if indeks == 49 {
        return (vec![1_704_240_000.0, 1_706_400_000.0], vec![0.5, 0.5]);
    }
    (vec![1_722_470_400.0, 1_754_006_400.0], vec![0.5, 0.5])
}

fn saatlik_dilim(başlangıç: i64, nokta_sayısı: usize, düz: bool) -> (Vec<f64>, Vec<f64>) {
    let başlangıç_indeksi = (başlangıç - YIL_BAŞI).div_euclid(SAAT);
    let x = (0..nokta_sayısı)
        .map(|indeks| (başlangıç + i64::try_from(indeks).unwrap_or(0) * SAAT) as f64)
        .collect();
    let y = (0..nokta_sayısı)
        .map(|indeks| {
            if düz {
                0.5
            } else if (başlangıç_indeksi + i64::try_from(indeks).unwrap_or(0)).rem_euclid(24) == 0
            {
                1.0
            } else {
                0.0
            }
        })
        .collect();
    (x, y)
}

fn karşılaştırma_zaman_dilimi(grup: usize, sıra: usize) -> ZamanDilimi {
    match (grup, sıra) {
        (_, 0) => ZamanDilimi::Utc,
        (0 | 1, 1) | (2 | 3, 2) => ZamanDilimi::EuropeLondon,
        _ => ZamanDilimi::AmericaChicago,
    }
}

fn karşılaştırma_başlığı(grup: usize, sıra: usize) -> &'static str {
    match (grup, sıra) {
        (_, 0) => "UTC (no DST)",
        (0, 1) => "Europe/London (Mar 31, 2024: 1am -> 2am)",
        (1, 1) => "Europe/London (Oct 27, 2024: 2am -> 1am)",
        (2, 1) => "America/Chicago (Mar 10, 2024: 2am -> 3am)",
        (3, 1) => "America/Chicago (Nov 3, 2024: 2am -> 1am)",
        (0 | 1, 2) => "America/Chicago (no DST switch in this range)",
        _ => "Europe/London (no DST switch in this range)",
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn elli_bir_etkin_kaynak_yüzeyi_benzersizdir() {
        let örnekler = TimezonesDstÖrneği::tümü().collect::<Vec<_>>();
        assert_eq!(örnekler.len(), 51);
        let kimlikler = örnekler
            .iter()
            .map(|örnek| örnek.kimlik())
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(kimlikler.len(), 51);
    }

    #[test]
    fn london_ve_chicago_geçişleri_atlanan_ve_tekrarlanan_saati_gösterir() {
        assert_eq!(
            crate::zaman::zaman_dilimi_ofseti(ZamanDilimi::EuropeLondon, 1_711_846_799.0),
            0
        );
        assert_eq!(
            crate::zaman::zaman_dilimi_ofseti(ZamanDilimi::EuropeLondon, 1_711_846_800.0),
            3_600
        );
        assert_eq!(
            crate::zaman::zaman_dilimi_ofseti(ZamanDilimi::AmericaChicago, 1_730_617_199.0),
            -18_000
        );
        assert_eq!(
            crate::zaman::zaman_dilimi_ofseti(ZamanDilimi::AmericaChicago, 1_730_617_200.0),
            -21_600
        );
    }

    #[test]
    fn london_ilkbahar_bir_saat_ekseninde_01_atlanır() -> Result<(), UplotHatası> {
        let Some(örnek) = TimezonesDstÖrneği::yeni(12) else {
            return Err(UplotHatası::BilinmeyenKart {
                kimlik: "timezones-dst-13".to_string(),
            });
        };
        let (seçenekler, veri) = timezones_dst_kartı(örnek)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let etiketler = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Metin { içerik, .. } => Some(içerik.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert!(etiketler.contains(&"03-31 00:00"));
        assert!(etiketler.contains(&"02:00"));
        assert!(!etiketler.contains(&"01:00"));
        Ok(())
    }

    #[test]
    fn tüm_yüzeyler_paniksiz_çizilir() -> Result<(), UplotHatası> {
        for örnek in TimezonesDstÖrneği::tümü() {
            let (seçenekler, veri) = timezones_dst_kartı(örnek)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "red"))
            );
        }
        Ok(())
    }
}
