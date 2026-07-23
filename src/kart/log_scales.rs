const KAYNAK_PAKETLER: [&str; 13] = include!("veri/log_scales_paket.rs");

use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri
};

const VARLIK: &str = "demos/log-scales.html#msg";
const SUNUCULAR: [(&str, &str); 12] = [
    ("Hypixel", "#d0b283"),
    ("HiveMC", "#ffcb03"),
    ("CubeCraft", "#176093"),
    ("Shotbow", "#bf6889"),
    ("Wynncraft", "#a0a369"),
    ("Mineplex", "#ec8008"),
    ("Cosmic PVP", "#e16914"),
    ("Rewinside", "#7cafb7"),
    ("Timolia", "#eb644d"),
    ("FunCraft", "#b9592d"),
    ("GommeHD", "#d14ead"),
    ("Minehut", "#75505a"),
];

pub const LOG_SCALES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    log_scales_kartı(LogScalesÖrneği::Logaritmik)?;
// Ölçek dönüşümü, eksen bölmeleri ve etkileşimler çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogScalesÖrneği {
    Logaritmik,
    Doğrusal,
}

impl LogScalesÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::Logaritmik, Self::Doğrusal];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Logaritmik => "log-scales-log-y",
            Self::Doğrusal => "log-scales-linear-y",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Logaritmik => "Log Y Scale",
            Self::Doğrusal => "Linear Y Scale",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

/// Resmî `log-scales.html` içindeki iki grafiği, aynı 1.440 zaman damgası ve
/// 12 sunucu serisiyle üretir. Kaynaktaki sıfırlar log ölçeği için 1 yapılır.
pub fn log_scales_kartı(
    örnek: LogScalesÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let zaman_paketi = KAYNAK_PAKETLER
        .first()
        .copied()
        .ok_or_else(|| kaynak_hatası("zaman paketi eksik"))?;
    let x = paket_çöz(zaman_paketi)?
        .into_iter()
        .enumerate()
        .map(|(indeks, değer)| {
            değer.ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
                varlık: VARLIK,
                açıklama: format!("zaman serisinde null değer var: {indeks}"),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut seriler = Vec::with_capacity(SUNUCULAR.len());
    for paket in KAYNAK_PAKETLER.iter().skip(1) {
        let seri = paket_çöz(paket)?
            .into_iter()
            .map(|değer| değer.map(|sayı| if sayı == 0.0 { 1.0 } else { sayı }))
            .collect();
        seriler.push(seri);
    }
    let veri = HizalıVeri::yeni(x, seriler)?;
    let mut seçenekler = GrafikSeçenekleri::yeni(1_600, 600)?
        .başlık(örnek.başlık())
        .etkileşimler(ortak_kart_etkileşimleri());
    for (ad, renk) in SUNUCULAR {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(ad)
                .renk(renk)
                .boşlukları_birleştir(true),
        );
    }
    if örnek == LogScalesÖrneği::Logaritmik {
        seçenekler = seçenekler.y_ölçeği(YÖlçekSeçenekleri::yeni("y").logaritmik(10.0));
    }
    Ok((seçenekler, veri))
}

fn paket_çöz(paket: &str) -> Result<Vec<Option<f64>>, UplotHatası> {
    let baytlar = base64_çöz(paket)?;
    let mut sonuç = Vec::with_capacity(1_440);
    let mut önceki = 0_i64;
    let mut değer = 0_u64;
    let mut kaydırma = 0_u32;
    let mut varint_içinde = false;
    for bayt in baytlar {
        if bayt == 0 && !varint_içinde {
            sonuç.push(None);
            continue;
        }
        varint_içinde = true;
        değer |= u64::from(bayt & 0x7f) << kaydırma;
        if bayt & 0x80 != 0 {
            kaydırma = kaydırma
                .checked_add(7)
                .ok_or_else(|| kaynak_hatası("varint taşması"))?;
            if kaydırma >= 64 {
                return Err(kaynak_hatası("varint 64 biti aşıyor"));
            }
            continue;
        }
        let zigzag = değer
            .checked_sub(1)
            .ok_or_else(|| kaynak_hatası("sıfır varint değeri"))?;
        let delta = if zigzag & 1 == 0 {
            i64::try_from(zigzag / 2).map_err(|_| kaynak_hatası("pozitif delta taşması"))?
        } else {
            let büyüklük = i64::try_from(
                zigzag
                    .checked_add(1)
                    .ok_or_else(|| kaynak_hatası("zigzag delta taşması"))?
                    / 2,
            )
            .map_err(|_| kaynak_hatası("negatif delta taşması"))?;
            -büyüklük
        };
        önceki = önceki
            .checked_add(delta)
            .ok_or_else(|| kaynak_hatası("kaynak değer taşması"))?;
        sonuç.push(Some(önceki as f64));
        değer = 0;
        kaydırma = 0;
        varint_içinde = false;
    }
    if varint_içinde {
        return Err(kaynak_hatası("tamamlanmamış varint"));
    }
    Ok(sonuç)
}

fn base64_çöz(paket: &str) -> Result<Vec<u8>, UplotHatası> {
    let mut sonuç = Vec::with_capacity(paket.len().saturating_mul(3) / 4);
    let mut tampon = 0_u32;
    let mut bit = 0_u8;
    for karakter in paket.bytes() {
        if karakter == b'=' {
            break;
        }
        let altı_bit = match karakter {
            b'A'..=b'Z' => karakter - b'A',
            b'a'..=b'z' => karakter - b'a' + 26,
            b'0'..=b'9' => karakter - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return Err(kaynak_hatası("geçersiz base64 karakteri")),
        };
        tampon = (tampon << 6) | u32::from(altı_bit);
        bit = bit.saturating_add(6);
        if bit >= 8 {
            bit -= 8;
            sonuç.push(((tampon >> bit) & 0xff) as u8);
        }
    }
    Ok(sonuç)
}

fn kaynak_hatası(açıklama: impl Into<String>) -> UplotHatası {
    UplotHatası::GeçersizKaynakVeri {
        varlık: VARLIK,
        açıklama: açıklama.into(),
    }
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, YÖlçekDağılımı};

    #[test]
    fn iki_kaynak_grafiği_aynı_1440_noktalı_on_iki_seriyi_kullanır() -> Result<(), UplotHatası> {
        let beklenen_null = [133, 249, 32, 32, 31, 191, 31, 31, 464, 31, 31, 97];
        let beklenen_toplam = [
            97_834_894.0,
            2_168_405.0,
            3_392_567.0,
            306_426.0,
            1_857_006.0,
            2_036_903.0,
            1_883_014.0,
            277_491.0,
            249_787.0,
            1_523_669.0,
            4_916_648.0,
            9_688_875.0,
        ];
        for örnek in LogScalesÖrneği::TÜMÜ {
            let (seçenekler, veri) = log_scales_kartı(örnek)?;
            assert_eq!(veri.uzunluk(), 1_440);
            assert_eq!(veri.seriler().len(), 12);
            assert_eq!(veri.x().first().copied(), Some(1_594_953_046.0));
            assert_eq!(veri.x().last().copied(), Some(1_595_039_415.0));
            assert!(
                veri.seriler()
                    .iter()
                    .all(|seri| seri.iter().flatten().all(|değer| *değer > 0.0))
            );
            for ((seri, null_sayısı), toplam) in veri
                .seriler()
                .iter()
                .zip(beklenen_null)
                .zip(beklenen_toplam)
            {
                assert_eq!(
                    seri.iter().filter(|değer| değer.is_none()).count(),
                    null_sayısı
                );
                assert_eq!(seri.iter().flatten().sum::<f64>(), toplam);
            }
            let grafik = Grafik::yeni(seçenekler, veri)?;
            if örnek == LogScalesÖrneği::Logaritmik {
                let aralık = grafik.seri_görünür_y_aralığı(0);
                assert_eq!(
                    aralık.map(|aralık| (aralık.en_az, aralık.en_çok)),
                    Some((1.0, 1_000_000.0))
                );
            }
            assert!(grafik.çiz().komutlar().len() > 20);
        }
        Ok(())
    }

    #[test]
    fn yalnız_ilk_kart_log10_y_ölçeğini_etkinleştirir() -> Result<(), UplotHatası> {
        let (log, _) = log_scales_kartı(LogScalesÖrneği::Logaritmik)?;
        assert!(matches!(
            log.y_ölçekleri.first().map(|ölçek| ölçek.dağılım),
            Some(YÖlçekDağılımı::Logaritmik { taban }) if taban == 10.0
        ));
        let (doğrusal, _) = log_scales_kartı(LogScalesÖrneği::Doğrusal)?;
        assert!(doğrusal.y_ölçekleri.is_empty());
        Ok(())
    }
}
