use super::ortak_kart_etkileşimleri;
use crate::{
    BoşlukKipi, GrafikSeçenekleri, HizalıVeri, NullİmleçDüzeni, SeriSeçenekleri, UplotHatası,
    hizalı_verileri_birleştir,
};

pub const NEAREST_NON_NULL_KART_TANIM_ÖRNEĞİ: &str = r##"// Kaynak sayfadaki beş ilişkili yüzeyi birlikte kurun.
let grafikler = NearestNonNullÖrneği::TÜMÜ
    .into_iter()
    .map(nearest_non_null_kartı)
    .collect::<Result<Vec<_>, _>>()?;
// Null ile uPlot.join() hizalama eksiği farklı anlam taşır; cursor politikası
// her yüzeyin GrafikSeçenekleri::null_imleç_düzeni alanında tanımlıdır."##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NearestNonNullÖrneği {
    XDeğerineGöre,
    OtuzPikselYakınlık,
    NullİseOnBeşPiksel,
    ÖncekiSeri,
    ÖncekiİmleçVeSeri,
}

impl NearestNonNullÖrneği {
    pub const TÜMÜ: [Self; 5] = [
        Self::XDeğerineGöre,
        Self::OtuzPikselYakınlık,
        Self::NullİseOnBeşPiksel,
        Self::ÖncekiSeri,
        Self::ÖncekiİmleçVeSeri,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::XDeğerineGöre => "nearest-non-null-x-value",
            Self::OtuzPikselYakınlık => "nearest-non-null-30px",
            Self::NullİseOnBeşPiksel => "nearest-non-null-null-15px",
            Self::ÖncekiSeri => "nearest-non-null-previous-series",
            Self::ÖncekiİmleçVeSeri => "nearest-non-null-previous-cursor",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::XDeğerineGöre => "Nearest Non-Null (by nearest x scale value)",
            Self::OtuzPikselYakınlık => "Nearest Non-Null (by 30px proximity)",
            Self::NullİseOnBeşPiksel => {
                "Nearest Non-Null (by 15px proximity only when hovered val === null)"
            }
            Self::ÖncekiSeri => "Snap hover points to previous non-null (cursor.dataIdx)",
            Self::ÖncekiİmleçVeSeri => {
                "Snap hover points and cursor to previous non-null (cursor.move)"
            }
        }
    }

    pub const fn kısa_açıklama(self) -> &'static str {
        match self {
            Self::XDeğerineGöre => {
                "Her seri, null koşusunda X ölçeğinde en yakın gerçek örneği seçer."
            }
            Self::OtuzPikselYakınlık => {
                "Joined seriler yalnız 30 CSS piksel içindeki dolu noktayı gösterir."
            }
            Self::NullİseOnBeşPiksel => {
                "15 px sınırı yalnız gerçek null hücrede uygulanır; hizalama eksiği farklıdır."
            }
            Self::ÖncekiSeri => "Nokta ve seri değeri geriye yapışır; dikey cursor farede kalır.",
            Self::ÖncekiİmleçVeSeri => {
                "Nokta, seri değeri ve dikey cursor birlikte geriye yapışır."
            }
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

const CPU: &str = "6.54,6.15,6.16,6.15,6.19,6.26,6.32,6.15,6.15,6.28,6.29,6.33,6.18,6.17,6.17,6.33,6.32,6.23,6.15,6.15,6.24,6.29,6.16,6.17,6.17,6.45,6.28,6.16,6.17,6.17,6.24,6.3,6.19,6.19,6.17,6.24,6.29,6.22,6.18,6.28,6.26,6.17,6.3,6.16,6.21,6.24,6.16,6.29,6.17,6.18,6.27,6.16,6.33,6.16,6.19,6.24,6.18,1.7,6.18,6.16,6.34,6.18,6.28,6.18,6.17,6.34,6.17,6.28,6.17,6.22,6.23,6.14,6.29,6.2,6.16,6.24,6.16,6.17,6.29,6.17,6.38,6.2,6.16,6.3,6.18,6.31,6.16,6.16,6.29,6.15,6.31,6.17,6.17,6.3,6.2,6.27,6.16,6.15,6.3,6.41,6.28,6.16,6.15,6.28,6.15,6.33,6.18,6.18,6.17,5.2,6.23,6.17,6.16,6.16,6.33,6.3,6.19,6.16,6.92,6.28,6.33,6.17,6.21,6.15,6.32,6.61,1.48,6.15,6.17,6.44,6.23,6.19,6.17,6.24,6.33,6.26,6.17,6.17,6.17,6.27,6.25,6.11,6.1,6.07,6.09,6.37,6.08,6.09,6.08,6.07,6.34,6.07,6.08,6.07,6.07,6.37,6.07,6.07,6.07,6.25,6.26,6.06,6.1,6.07,6.09,6.31,6.08,6.11,6.11,6.12,6.26,6.08,6.09,6.07,6.1,6.25,6.06,6.06,6.07,6.07,6.36,6.26,6.08,6.08,6.09,6.16,6.23,6.07,6.09,6.21,6.22,6.21,6.09,6.07,6.09,6.22,6.3,6.12,6.07,6.11";
const RAM: &str = "14.02,14.01,14.01,14.01,14.01,14.03,14.03,14.02,14.02,14.03,14.03,14.04,14.03,14.03,14.03,14.02,14.04,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.04,14.04,14.03,14.03,14.03,14.04,14.03,14.04,14.04,14.04,14.05,14.03,14.03,14.03,14.04,14.04,14.05,14.03,14.03,14.03,14.03,14.05,14.04,14.04,14.04,14.03,14.04,14.03,14.04,14.04,14.03,14.04,14.03,14.03,14.03,14.03,14.02,14.02,14.02,14.01,14.01,14.02,14.02,14.02,14.02,14.02,14.02,14.01,14.01,14.02,14.02,14.02,14.03,14.02,14,14.02,14.02,14.04,14.03,14.02,14.02,14.02,14.03,14.02,14.03,14.03,14.03,14.03,14.02,14.02,14.02,14.02,14.02,14.02,14.02,14.02,14.02,14.03,14.03,14.02,14.02,14.02,14.02,14.36,14.08,14.08,14.08,14.08,14.04,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14,14,14,14,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.03,14.04,14.04,14.03,14.01,14.01,14.01,14.01,14.03,14.01,14.01,14.01,14.01,14.04,14.03,14.04,14.04,14.04,14.02,14.01,14.01,14.01,14.03,14.04,14.03,14.03,14.03,14.04,14.04,14.03,14.03,14.03,14.04,14.05,14.04,14.03,14.03,14.03,14.03,14.04,14.04,14.04,14.03,14.03,14.04,14.03,14.04,14.04,14.04,14.04,14.03,14.03,14.04,14.04,14.05,14.04,14.04,14.04,14.02,14.05,14.04,14.03,14.03";

fn kaynak_seri(kaynak: &str) -> Vec<Option<f64>> {
    kaynak
        .split(',')
        .map(|değer| değer.parse::<f64>().ok())
        .collect()
}

fn x_değerine_göre_veri() -> Result<HizalıVeri, UplotHatası> {
    let x = (0..200)
        .map(|i| 1_566_453_600.0 + f64::from(i) * 60.0)
        .collect::<Vec<_>>();
    let mut cpu = kaynak_seri(CPU);
    let mut ram = kaynak_seri(RAM);
    for değer in cpu.iter_mut().take(42).skip(35) {
        *değer = None;
    }
    for indeks in [79, 80, 91, 125, 126, 127] {
        if let Some(değer) = ram.get_mut(indeks) {
            *değer = None;
        }
    }
    HizalıVeri::yeni(x, vec![cpu, ram])
}

fn otuz_piksel_verisi() -> Result<HizalıVeri, UplotHatası> {
    let değerler = vec![
        vec![
            Some(30.0),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(30.0),
            None,
            None,
            None,
            None,
            None,
            Some(30.0),
        ],
        vec![Some(5.0); 33],
        vec![Some(18.0); 32],
        vec![Some(25.0); 31],
    ];
    let tablolar = değerler
        .into_iter()
        .map(|seri| {
            let uzunluk = seri.len();
            let x = (0..uzunluk)
                .map(|i| i as f64 / uzunluk as f64 * 100.0)
                .collect();
            HizalıVeri::yeni(x, vec![seri])
        })
        .collect::<Result<Vec<_>, _>>()?;
    hizalı_verileri_birleştir(&tablolar, None)
}

fn null_ise_on_beş_piksel_verisi() -> Result<HizalıVeri, UplotHatası> {
    let a = HizalıVeri::yeni(
        vec![
            1_626_235_540_319.0,
            1_626_235_584_004.0,
            1_626_235_627_689.0,
            1_626_235_671_374.0,
            1_626_235_715_059.0,
            1_626_235_758_744.0,
            1_626_235_802_429.0,
            1_626_235_846_114.0,
            1_626_235_889_799.0,
            1_626_235_933_484.0,
            1_626_235_977_169.0,
        ],
        vec![vec![
            Some(1.0),
            Some(20.0),
            Some(90.0),
            Some(30.0),
            Some(5.0),
            Some(0.0),
            Some(100.0),
            Some(20.0),
            None,
            Some(100.0),
            Some(20.0),
        ]],
    )?;
    let b = HizalıVeri::yeni(
        vec![
            1_626_235_540_319.0,
            1_626_235_613_128.0,
            1_626_235_685_937.0,
            1_626_235_758_746.0,
            1_626_235_831_555.0,
            1_626_235_904_364.0,
            1_626_235_977_173.0,
        ],
        vec![vec![
            Some(1.0),
            Some(20.0),
            Some(90.0),
            Some(30.0),
            Some(5.0),
            None,
            Some(0.0),
        ]],
    )?;
    hizalı_verileri_birleştir(
        &[a, b],
        Some(&[vec![BoşlukKipi::Genişlet], vec![BoşlukKipi::Genişlet]]),
    )
}

fn önceki_verisi() -> Result<HizalıVeri, UplotHatası> {
    HizalıVeri::yeni(
        (0..10).map(f64::from).collect(),
        vec![(0..10).map(|i| (i != 5).then_some(5.0)).collect()],
    )
}

pub fn nearest_non_null_kartı(
    örnek: NearestNonNullÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (genişlik, yükseklik, veri, adlar, renkler, dolgular, düzen) = match örnek {
        NearestNonNullÖrneği::XDeğerineGöre => (
            1920,
            600,
            x_değerine_göre_veri()?,
            vec!["CPU", "RAM"],
            vec!["red", "blue"],
            vec![Some("rgba(255,0,0,0.05)"), Some("rgba(0,0,255,0.05)")],
            NullİmleçDüzeni::EnYakınX,
        ),
        NearestNonNullÖrneği::OtuzPikselYakınlık => (
            1920,
            600,
            otuz_piksel_verisi()?,
            vec!["Sparse", "5", "18", "25"],
            vec!["red", "blue", "green", "orange"],
            vec![None, None, None, None],
            NullİmleçDüzeni::PikselYakınlığı { piksel: 30.0 },
        ),
        NearestNonNullÖrneği::NullİseOnBeşPiksel => (
            1920,
            600,
            null_ise_on_beş_piksel_verisi()?,
            vec!["Series 1", "Series 2"],
            vec!["red", "blue"],
            vec![Some("rgba(255,0,0,0.1)"), Some("rgba(0,0,255,0.1)")],
            NullİmleçDüzeni::YalnızNullsaPiksel { piksel: 15.0 },
        ),
        NearestNonNullÖrneği::ÖncekiSeri => (
            600,
            300,
            önceki_verisi()?,
            vec!["foo"],
            vec!["red"],
            vec![None],
            NullİmleçDüzeni::ÖncekiSeri,
        ),
        NearestNonNullÖrneği::ÖncekiİmleçVeSeri => (
            600,
            300,
            önceki_verisi()?,
            vec!["foo"],
            vec!["blue"],
            vec![None],
            NullİmleçDüzeni::ÖncekiİmleçVeSeri,
        ),
    };
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(matches!(
            örnek,
            NearestNonNullÖrneği::XDeğerineGöre | NearestNonNullÖrneği::NullİseOnBeşPiksel
        ))
        .x_zaman_milisaniye(örnek == NearestNonNullÖrneği::NullİseOnBeşPiksel)
        .null_imleç_düzeni(düzen)
        .imleç_y_göster(!matches!(
            örnek,
            NearestNonNullÖrneği::ÖncekiSeri | NearestNonNullÖrneği::ÖncekiİmleçVeSeri
        ))
        .etkileşimler(ortak_kart_etkileşimleri());
    for ((ad, renk), dolgu) in adlar.into_iter().zip(renkler).zip(dolgular) {
        let mut seri = SeriSeçenekleri::yeni(ad)
            .renk(renk)
            .boşlukları_birleştir(örnek != NearestNonNullÖrneği::XDeğerineGöre);
        if let Some(dolgu) = dolgu {
            seri = seri.dolgu(dolgu);
        }
        seçenekler = seçenekler.seri(seri);
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn beş_kaynak_yüzeyi_çizilir() -> Result<(), UplotHatası> {
        for örnek in NearestNonNullÖrneği::TÜMÜ {
            let (seçenekler, veri) = nearest_non_null_kartı(örnek)?;
            assert!(!Grafik::yeni(seçenekler, veri)?.çiz().komutlar().is_empty());
        }
        Ok(())
    }

    #[test]
    fn data_idx_ve_cursor_move_ayrımı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::ÖncekiSeri)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let çözüm = grafik.imleç_çözümü(5.8 / 9.0, 540.0);
        assert!(çözüm.is_some());
        let Some(çözüm) = çözüm else {
            return Ok(());
        };
        assert_eq!(
            çözüm
                .seriler
                .first()
                .copied()
                .flatten()
                .map(|örnek| örnek.indeks),
            Some(4)
        );
        assert!(çözüm.imleç_x > 5.0);

        let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::ÖncekiİmleçVeSeri)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let çözüm = grafik.imleç_çözümü(5.8 / 9.0, 540.0);
        assert!(çözüm.is_some());
        let Some(çözüm) = çözüm else {
            return Ok(());
        };
        assert_eq!(çözüm.imleç_x, 4.0);
        Ok(())
    }

    #[test]
    fn joined_veri_null_ile_hizalama_eksiğini_ayırır() -> Result<(), UplotHatası> {
        let (_, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::NullİseOnBeşPiksel)?;
        let açık_null = veri.x().iter().position(|x| *x == 1_626_235_889_799.0);
        assert!(açık_null.is_some());
        let Some(açık_null) = açık_null else {
            return Ok(());
        };
        let hizalama_eksiği = veri.x().iter().position(|x| *x == 1_626_235_584_004.0);
        assert!(hizalama_eksiği.is_some());
        let Some(hizalama_eksiği) = hizalama_eksiği else {
            return Ok(());
        };
        assert!(!veri.hizalama_eksiği_mi(0, açık_null));
        assert!(veri.hizalama_eksiği_mi(1, hizalama_eksiği));
        Ok(())
    }

    #[test]
    fn piksel_eşiği_çizim_genişliğinde_ve_null_türüne_göre_uygulanır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::OtuzPikselYakınlık)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let çözüm = grafik.imleç_çözümü(0.40, 1_920.0);
        assert!(çözüm.is_some());
        let Some(çözüm) = çözüm else {
            return Ok(());
        };
        assert!(çözüm.seriler.first().is_some_and(Option::is_none));
        assert!(çözüm.seriler.get(1).is_some_and(Option::is_some));

        let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::NullİseOnBeşPiksel)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let aralık = grafik.görünür_x_aralığı();
        let oran = (1_626_235_889_799.0 - aralık.en_az) / (aralık.en_çok - aralık.en_az);
        let çözüm = grafik.imleç_çözümü(oran, 200_000.0);
        assert!(çözüm.is_some());
        let Some(çözüm) = çözüm else {
            return Ok(());
        };
        assert!(
            çözüm.seriler.first().is_some_and(Option::is_none),
            "gerçek null 15 px ile sınırlı"
        );

        let oran = (1_626_235_584_004.0 - aralık.en_az) / (aralık.en_çok - aralık.en_az);
        let çözüm = grafik.imleç_çözümü(oran, 200_000.0);
        assert!(çözüm.is_some());
        let Some(çözüm) = çözüm else {
            return Ok(());
        };
        assert!(
            çözüm.seriler.get(1).is_some_and(Option::is_some),
            "alignment missing kaynak callback'te 15 px sınırına girmez"
        );
        Ok(())
    }
}
