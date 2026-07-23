use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

const VARLIK: &str = "demos/log-scales2.html";
const SABİT_KANIT_ZAMANI: f64 = 1_704_499_200.0;
pub const LOG_SCALES2_KANIT_TOHUMU: u32 = 0x105C_A1E2;

pub const LOG_SCALES2_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) =
    log_scales2_kartı(LogScales2Örneği::GenişLog10)?;
// Logaritmik aralık, bölmeler, ters yön ve etiketler çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogScales2Örneği {
    GenişDoğrusal,
    GenişLog10,
    GenişLog2,
    TersGiriş,
    TersÇıkış,
    PozitifFiltreli,
    SeyrekLog10,
    SeyrekLog2,
    TümüNull,
    ÇokKüçük,
    KısmiBüyük,
    KısmiKüçük,
}

impl LogScales2Örneği {
    pub const TÜMÜ: [Self; 12] = [
        Self::GenişDoğrusal,
        Self::GenişLog10,
        Self::GenişLog2,
        Self::TersGiriş,
        Self::TersÇıkış,
        Self::PozitifFiltreli,
        Self::SeyrekLog10,
        Self::SeyrekLog2,
        Self::TümüNull,
        Self::ÇokKüçük,
        Self::KısmiBüyük,
        Self::KısmiKüçük,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::GenişDoğrusal => "log-scales2-linear-wide",
            Self::GenişLog10 => "log-scales2-log10-wide",
            Self::GenişLog2 => "log-scales2-log2-wide",
            Self::TersGiriş => "log-scales2-inverted-in",
            Self::TersÇıkış => "log-scales2-inverted-out",
            Self::PozitifFiltreli => "log-scales2-positive-filter",
            Self::SeyrekLog10 => "log-scales2-skip-log10",
            Self::SeyrekLog2 => "log-scales2-skip-log2",
            Self::TümüNull => "log-scales2-all-nulls",
            Self::ÇokKüçük => "log-scales2-tiny-values",
            Self::KısmiBüyük => "log-scales2-partial-large",
            Self::KısmiKüçük => "log-scales2-partial-small",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::GenişDoğrusal => "Linear Scale (0.000001 -> 100,000,000)",
            Self::GenişLog10 => "Log10 Y Scale (0.000001 -> 100,000,000)",
            Self::GenişLog2 => "Log2 Y Scale (0.000001 -> 100,000,000)",
            Self::TersGiriş => "Inverted Log10 Y Scale — In",
            Self::TersÇıkış => "Inverted Log10 Y Scale — Out",
            Self::PozitifFiltreli => "Log10 Y Scale (-100 -> 100)",
            Self::SeyrekLog10 => "Skip ticks log10",
            Self::SeyrekLog2 => "Skip ticks log2",
            Self::TümüNull => "Proper range with a all-nulls series",
            Self::ÇokKüçük => "Handle 3e-24 y values",
            Self::KısmiBüyük => "Partial mags (base 10) — large",
            Self::KısmiKüçük => "Partial mags (base 10) — small",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

/// Resmî `log-scales2.html` içindeki on iki çizim yüzeyini aynı sayı
/// dizileriyle üretir. Kaynaktaki saat ve `Math.random` girdileri tekrarlanabilir
/// görsel kanıt için sabitlenmiştir; veri üretme algoritmaları değişmemiştir.
pub fn log_scales2_kartı(
    örnek: LogScales2Örneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        LogScales2Örneği::GenişDoğrusal
        | LogScales2Örneği::GenişLog10
        | LogScales2Örneği::GenişLog2 => geniş_aralık_kartı(örnek),
        LogScales2Örneği::TersGiriş | LogScales2Örneği::TersÇıkış => ters_kart(örnek),
        LogScales2Örneği::PozitifFiltreli => pozitif_filtreli_kart(),
        LogScales2Örneği::SeyrekLog10 | LogScales2Örneği::SeyrekLog2 => {
            seyrek_bölmeli_kart(örnek)
        }
        LogScales2Örneği::TümüNull => tümü_null_kartı(),
        LogScales2Örneği::ÇokKüçük => çok_küçük_kart(),
        LogScales2Örneği::KısmiBüyük | LogScales2Örneği::KısmiKüçük => {
            kısmi_büyüklük_kartı(örnek)
        }
    }
}

fn temel_seçenekler(
    örnek: LogScales2Örneği,
    genişlik: u32,
    yükseklik: u32,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    Ok(GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .etkileşimler(ortak_kart_etkileşimleri()))
}

fn geniş_aralık_verisi() -> Result<HizalıVeri, UplotHatası> {
    let mut değerler = Vec::with_capacity(127);
    for büyüklük in -6..=7 {
        for çarpan in 1..10 {
            let değer = f64::from(çarpan) * 10_f64.powi(büyüklük);
            değerler.push((değer * 1_000_000.0).round() / 1_000_000.0);
        }
    }
    değerler.push(100_000_000.0);
    let x = (1..=değerler.len()).map(|indeks| indeks as f64).collect();
    HizalıVeri::yeni(x, vec![değerler.into_iter().map(Some).collect()])
}

fn geniş_aralık_kartı(
    örnek: LogScales2Örneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (renk, ölçek) = match örnek {
        LogScales2Örneği::GenişDoğrusal => ("magenta", None),
        LogScales2Örneği::GenişLog10 => (
            "orange",
            Some(
                YÖlçekSeçenekleri::yeni("y")
                    .logaritmik(10.0)
                    .etiket_biçimi(YÖlçekEtiketBiçimi::Bilimsel),
            ),
        ),
        LogScales2Örneği::GenişLog2 => (
            "purple",
            Some(
                YÖlçekSeçenekleri::yeni("y")
                    .logaritmik(2.0)
                    .etiket_biçimi(YÖlçekEtiketBiçimi::İkiliÜs),
            ),
        ),
        _ => return Err(kaynak_hatası("geniş aralık kartı türü eşleşmiyor")),
    };
    let mut seçenekler = temel_seçenekler(örnek, 1_600, 600)?
        .x_zaman(false)
        .seri(SeriSeçenekleri::yeni("Value").renk(renk));
    if let Some(ölçek) = ölçek {
        seçenekler = seçenekler.y_ölçeği(ölçek);
    }
    Ok((seçenekler, geniş_aralık_verisi()?))
}

fn ters_kart(
    örnek: LogScales2Örneği
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = [-10_800.0, -7_200.0, -3_600.0, 0.0]
        .into_iter()
        .map(|fark| SABİT_KANIT_ZAMANI + fark)
        .collect();
    let veri = HizalıVeri::yeni(x, vec![vec![Some(0.1), Some(10.0), Some(1.0), Some(100.0)]])?;
    let çıkış = örnek == LogScales2Örneği::TersÇıkış;
    let ölçek = YÖlçekSeçenekleri::yeni("y")
        .logaritmik(10.0)
        .ters_yön(çıkış)
        .eksen_değer_çarpanı(if çıkış { -1.0 } else { 1.0 });
    let seri = SeriSeçenekleri::yeni(if çıkış { "Out" } else { "In" })
        .renk(if çıkış { "blue" } else { "orange" })
        .gösterim_değer_çarpanı(if çıkış { -1.0 } else { 1.0 });
    let seçenekler = temel_seçenekler(örnek, 1_600, 300)?
        .y_ölçeği(ölçek)
        .seri(seri);
    Ok((seçenekler, veri))
}

fn pozitif_filtreli_kart() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = [
        -100.0, -10.0, -1.0, -0.1, -0.01, -0.001, 0.0, 0.001, 0.01, 0.1, 1.0, 10.0, 100.0,
    ];
    let mut değerler = Vec::with_capacity(kaynak.len().saturating_mul(10));
    for _ in 0..10 {
        değerler.extend(kaynak);
    }
    let mut rastgele = KanıtRastgele::yeni(LOG_SCALES2_KANIT_TOHUMU);
    fisher_yates_karıştır(&mut değerler, &mut rastgele)?;
    let x = (0..değerler.len()).map(|indeks| indeks as f64).collect();
    let veri = HizalıVeri::yeni(x, vec![değerler.into_iter().map(Some).collect()])?;
    let seçenekler = temel_seçenekler(LogScales2Örneği::PozitifFiltreli, 1_600, 600)?
        .x_zaman(false)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").logaritmik(10.0))
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("blue")
                .dolgu("rgba(0,0,255,0.1)"),
        );
    Ok((seçenekler, veri))
}

fn fisher_yates_karıştır(
    değerler: &mut [f64],
    rastgele: &mut KanıtRastgele,
) -> Result<(), UplotHatası> {
    let mut kalan = değerler.len();
    while kalan > 0 {
        let hedef = (rastgele.sonraki() * kalan as f64).floor() as usize;
        kalan = kalan.saturating_sub(1);
        güvenli_takas(değerler, kalan, hedef)?;
    }
    Ok(())
}

fn güvenli_takas(değerler: &mut [f64], sol: usize, sağ: usize) -> Result<(), UplotHatası> {
    if sol >= değerler.len() || sağ >= değerler.len() {
        return Err(kaynak_hatası("Fisher–Yates indeksi veri sınırını aştı"));
    }
    if sol == sağ {
        return Ok(());
    }
    let (küçük, büyük, ters) = if sol < sağ {
        (sol, sağ, false)
    } else {
        (sağ, sol, true)
    };
    let (önce, sonra) = değerler.split_at_mut(büyük);
    let Some(küçük_değer) = önce.get_mut(küçük) else {
        return Err(kaynak_hatası("Fisher–Yates küçük indeksi eksik"));
    };
    let Some(büyük_değer) = sonra.first_mut() else {
        return Err(kaynak_hatası("Fisher–Yates büyük indeksi eksik"));
    };
    if ters {
        std::mem::swap(büyük_değer, küçük_değer);
    } else {
        std::mem::swap(küçük_değer, büyük_değer);
    }
    Ok(())
}

fn seyrek_bölmeli_kart(
    örnek: LogScales2Örneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (taban, en_az, en_çok, renk, biçim) = match örnek {
        LogScales2Örneği::SeyrekLog10 => (10.0, 1e-14, 1e14, "red", YÖlçekEtiketBiçimi::Bilimsel),
        LogScales2Örneği::SeyrekLog2 => (
            2.0,
            2_f64.powi(-10),
            2_f64.powi(20),
            "blue",
            YÖlçekEtiketBiçimi::İkiliŞapka,
        ),
        _ => return Err(kaynak_hatası("seyrek bölmeli kart türü eşleşmiyor")),
    };
    let veri = HizalıVeri::yeni(vec![0.0, 1.0], vec![vec![Some(en_az), Some(en_çok)]])?;
    let ölçek = YÖlçekSeçenekleri::yeni("y")
        .logaritmik(taban)
        .aralık(Aralık::yeni(en_az, en_çok)?)
        .etiket_biçimi(biçim);
    let seçenekler = temel_seçenekler(örnek, 800, 300)?
        .x_zaman(false)
        .y_ölçeği(ölçek)
        .seri(SeriSeçenekleri::yeni("Value").renk(renk));
    Ok((seçenekler, veri))
}

fn tümü_null_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0, 2.0],
        vec![
            vec![Some(100.0), Some(200.0), Some(1_000.0)],
            vec![None, None, None],
        ],
    )?;
    let seçenekler = temel_seçenekler(LogScales2Örneği::TümüNull, 800, 400)?
        .x_zaman(false)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").logaritmik(10.0))
        .seri(SeriSeçenekleri::yeni("Value").renk("red"))
        .seri(SeriSeçenekleri::yeni("All null").renk("blue"));
    Ok((seçenekler, veri))
}

fn çok_küçük_kart() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0],
        vec![vec![Some(3.1992e-16), Some(4.9047e-13)]],
    )?;
    let seçenekler = temel_seçenekler(LogScales2Örneği::ÇokKüçük, 800, 600)?
        .x_zaman(false)
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y")
                .logaritmik(10.0)
                .etiket_biçimi(YÖlçekEtiketBiçimi::Bilimsel),
        )
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("red")
                .dolgu("rgba(255,0,0,0.1)"),
        );
    Ok((seçenekler, veri))
}

fn kısmi_büyüklük_kartı(
    örnek: LogScales2Örneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let küçük = örnek == LogScales2Örneği::KısmiKüçük;
    let a_bölen = if küçük { 10_000_000.0 } else { 1.0 };
    let b_bölen = if küçük { 1_000_000.0 } else { 1.0 };
    let a = [1_000_001.0, 1_000_000.0, 990_000.0]
        .into_iter()
        .map(|değer| Some(değer / a_bölen))
        .collect();
    let b = [99_000.0, 100_000.0, 100_000.001]
        .into_iter()
        .map(|değer| Some(değer / b_bölen))
        .collect();
    let veri = HizalıVeri::yeni(
        vec![1_704_326_400.0, 1_704_412_800.0, 1_704_499_200.0],
        vec![a, b],
    )?;
    let y0 = YÖlçekSeçenekleri::yeni("y0")
        .logaritmik_kısmi(10.0)
        .etiket_biçimi(YÖlçekEtiketBiçimi::Kompakt);
    let y1 = YÖlçekSeçenekleri::yeni("y1")
        .logaritmik_kısmi(10.0)
        .etiket_biçimi(YÖlçekEtiketBiçimi::Kompakt)
        .sağda(true)
        .eksen(true);
    let seçenekler = temel_seçenekler(örnek, 600, 300)?
        .birincil_y_ölçeği("y0")
        .y_ölçeği(y0)
        .y_ölçeği(y1)
        .seri(SeriSeçenekleri::yeni("A").ölçek("y0").renk("#f7931a"))
        .seri(SeriSeçenekleri::yeni("B").ölçek("y1").renk("#000000"));
    Ok((seçenekler, veri))
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
    fn on_iki_kaynak_yüzeyi_paniksiz_çizilir() -> Result<(), UplotHatası> {
        for örnek in LogScales2Örneği::TÜMÜ {
            let (seçenekler, veri) = log_scales2_kartı(örnek)?;
            let grafik = Grafik::yeni(seçenekler, veri)?;
            assert!(grafik.çiz().komutlar().len() > 10, "{}", örnek.kimlik());
        }
        Ok(())
    }

    #[test]
    fn geniş_veri_kaynak_sınırlarını_ve_uzunluğunu_korur() -> Result<(), UplotHatası> {
        let (_, veri) = log_scales2_kartı(LogScales2Örneği::GenişLog10)?;
        assert_eq!(veri.uzunluk(), 127);
        assert_eq!(veri.x().first().copied(), Some(1.0));
        assert_eq!(veri.x().last().copied(), Some(127.0));
        let seri = veri
            .seriler()
            .first()
            .ok_or_else(|| kaynak_hatası("seri eksik"))?;
        assert_eq!(seri.first().copied().flatten(), Some(0.000001));
        assert_eq!(seri.last().copied().flatten(), Some(100_000_000.0));
        Ok(())
    }

    #[test]
    fn log2_ters_yön_ve_gösterim_dönüşümü_seçeneklerde_kalır() -> Result<(), UplotHatası> {
        let (log2, _) = log_scales2_kartı(LogScales2Örneği::GenişLog2)?;
        assert!(matches!(
            log2.y_ölçekleri.first().map(|ölçek| ölçek.dağılım),
            Some(YÖlçekDağılımı::Logaritmik { taban }) if taban == 2.0
        ));
        assert_eq!(
            log2.y_ölçekleri.first().map(|ölçek| ölçek.etiket_biçimi),
            Some(YÖlçekEtiketBiçimi::İkiliÜs)
        );

        let (çıkış, veri) = log_scales2_kartı(LogScales2Örneği::TersÇıkış)?;
        let ölçek = çıkış
            .y_ölçekleri
            .first()
            .ok_or_else(|| kaynak_hatası("ters ölçek eksik"))?;
        assert!(ölçek.ters_yön);
        assert_eq!(ölçek.eksen_değer_çarpanı, -1.0);
        let grafik = Grafik::yeni(çıkış, veri)?;
        assert_eq!(
            grafik
                .en_yakın_noktalar(0.0)
                .and_then(|(_, değerler)| { değerler.first().copied().flatten() }),
            Some(-0.1)
        );
        Ok(())
    }

    #[test]
    fn negatifler_aralığı_büyütmez_ve_tümü_null_seri_yoksayılır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = log_scales2_kartı(LogScales2Örneği::PozitifFiltreli)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let aralık = grafik.seri_görünür_y_aralığı(0);
        assert_eq!(
            aralık.map(|aralık| (aralık.en_az, aralık.en_çok)),
            Some((0.001, 100.0))
        );

        let (seçenekler, veri) = log_scales2_kartı(LogScales2Örneği::TümüNull)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let aralık = grafik.seri_görünür_y_aralığı(0);
        assert_eq!(
            aralık.map(|aralık| (aralık.en_az, aralık.en_çok)),
            Some((100.0, 1_000.0))
        );
        Ok(())
    }

    #[test]
    fn kısmi_büyüklükler_tam_onluklara_genişlemez() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = log_scales2_kartı(LogScales2Örneği::KısmiBüyük)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let a = grafik.seri_görünür_y_aralığı(0);
        let b = grafik.seri_görünür_y_aralığı(1);
        assert_eq!(
            a.map(|aralık| (aralık.en_az, aralık.en_çok)),
            Some((900_000.0, 2_000_000.0))
        );
        assert_eq!(
            b.map(|aralık| (aralık.en_az, aralık.en_çok)),
            Some((90_000.0, 200_000.0))
        );
        Ok(())
    }
}
