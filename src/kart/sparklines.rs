use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları, SayısalAralıkParçası, SeriSeçenekleri,
    UplotHatası, YumuşakSınırKipi, YÖlçekSeçenekleri,
};

#[path = "veri/sparklines.rs"]
mod kaynak_veri;

use kaynak_veri::{SPARKLINE_KAYITLARI, SparklineKaydı};

pub const SPARKLINES_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in sparklines_kartları()? {
    // Resmî 10 satır × 2 sütun tablosundaki 150×30 yüzeyler, kaynak CSV
    // değerleri ve bağımsız uPlot numeric Y aralıkları birlikte üretilir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SparklineÖlçümü {
    Hacim,
    Kapanış,
}

/// Resmî tabloda bulunan 10 hisse × 2 ölçümden oluşan 20 yüzey.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SparklineÖrneği(u8);

impl SparklineÖrneği {
    pub const İLK: Self = Self(0);

    pub const TÜMÜ: [Self; 20] = [
        Self(0),
        Self(1),
        Self(2),
        Self(3),
        Self(4),
        Self(5),
        Self(6),
        Self(7),
        Self(8),
        Self(9),
        Self(10),
        Self(11),
        Self(12),
        Self(13),
        Self(14),
        Self(15),
        Self(16),
        Self(17),
        Self(18),
        Self(19),
    ];

    pub const SATIRLAR: [(Self, Self); 10] = [
        (Self(0), Self(1)),
        (Self(2), Self(3)),
        (Self(4), Self(5)),
        (Self(6), Self(7)),
        (Self(8), Self(9)),
        (Self(10), Self(11)),
        (Self(12), Self(13)),
        (Self(14), Self(15)),
        (Self(16), Self(17)),
        (Self(18), Self(19)),
    ];

    pub fn kimlik(self) -> &'static str {
        self.kayıt().kimlik
    }

    pub fn başlık(self) -> &'static str {
        self.kayıt().başlık
    }

    pub fn simge(self) -> &'static str {
        self.kaynak().map_or("", |kaynak| kaynak.simge)
    }

    pub fn ölçüm(self) -> &'static str {
        match self.kayıt().ölçüm {
            SparklineÖlçümü::Hacim => "Hacim",
            SparklineÖlçümü::Kapanış => "Kapanış",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        KAYITLAR
            .iter()
            .position(|kayıt| kayıt.kimlik == kimlik)
            .and_then(|indeks| u8::try_from(indeks).ok())
            .map(Self)
    }

    fn kayıt(self) -> &'static SparklineKartKaydı {
        KAYITLAR
            .get(usize::from(self.0))
            .map_or(&VARSAYILAN_KART, |kayıt| kayıt)
    }

    fn kaynak(self) -> Option<&'static SparklineKaydı> {
        SPARKLINE_KAYITLARI.get(self.kayıt().kaynak_indeksi)
    }
}

#[derive(Debug)]
struct SparklineKartKaydı {
    kimlik: &'static str,
    başlık: &'static str,
    kaynak_indeksi: usize,
    ölçüm: SparklineÖlçümü,
}

const VARSAYILAN_KART: SparklineKartKaydı = SparklineKartKaydı {
    kimlik: "sparklines-aapl-volume",
    başlık: "AAPL · Hacim",
    kaynak_indeksi: 0,
    ölçüm: SparklineÖlçümü::Hacim,
};

const KAYITLAR: [SparklineKartKaydı; 20] = [
    SparklineKartKaydı {
        kimlik: "sparklines-aapl-volume",
        başlık: "AAPL · Hacim",
        kaynak_indeksi: 0,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-aapl-close",
        başlık: "AAPL · Kapanış",
        kaynak_indeksi: 0,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-amd-volume",
        başlık: "AMD · Hacim",
        kaynak_indeksi: 1,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-amd-close",
        başlık: "AMD · Kapanış",
        kaynak_indeksi: 1,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-amzn-volume",
        başlık: "AMZN · Hacim",
        kaynak_indeksi: 2,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-amzn-close",
        başlık: "AMZN · Kapanış",
        kaynak_indeksi: 2,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-csco-volume",
        başlık: "CSCO · Hacim",
        kaynak_indeksi: 3,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-csco-close",
        başlık: "CSCO · Kapanış",
        kaynak_indeksi: 3,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-fb-volume",
        başlık: "FB · Hacim",
        kaynak_indeksi: 4,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-fb-close",
        başlık: "FB · Kapanış",
        kaynak_indeksi: 4,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-msft-volume",
        başlık: "MSFT · Hacim",
        kaynak_indeksi: 5,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-msft-close",
        başlık: "MSFT · Kapanış",
        kaynak_indeksi: 5,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-qcom-volume",
        başlık: "QCOM · Hacim",
        kaynak_indeksi: 6,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-qcom-close",
        başlık: "QCOM · Kapanış",
        kaynak_indeksi: 6,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-sbux-volume",
        başlık: "SBUX · Hacim",
        kaynak_indeksi: 7,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-sbux-close",
        başlık: "SBUX · Kapanış",
        kaynak_indeksi: 7,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-tsla-volume",
        başlık: "TSLA · Hacim",
        kaynak_indeksi: 8,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-tsla-close",
        başlık: "TSLA · Kapanış",
        kaynak_indeksi: 8,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-znga-volume",
        başlık: "ZNGA · Hacim",
        kaynak_indeksi: 9,
        ölçüm: SparklineÖlçümü::Hacim,
    },
    SparklineKartKaydı {
        kimlik: "sparklines-znga-close",
        başlık: "ZNGA · Kapanış",
        kaynak_indeksi: 9,
        ölçüm: SparklineÖlçümü::Kapanış,
    },
];

pub fn sparklines_kartı(
    örnek: SparklineÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kayıt = örnek.kayıt();
    let Some(kaynak) = örnek.kaynak() else {
        return Err(UplotHatası::BilinmeyenKart {
            kimlik: örnek.kimlik().to_string(),
        });
    };
    let değerler = match kayıt.ölçüm {
        SparklineÖlçümü::Hacim => kaynak.hacim,
        SparklineÖlçümü::Kapanış => kaynak.kapanış,
    };
    let veri = HizalıVeri::yeni(
        (0_u32..22).map(f64::from).collect(),
        vec![değerler.into_iter().map(Some).collect()],
    )?;
    let seçenekler = GrafikSeçenekleri::kompakt(150, 30)?
        .arka_plan_rengi("pink")
        .x_zaman(false)
        .x_ekseni_göster(false)
        .x_ızgarası_göster(false)
        .y_ekseni_göster(false)
        .y_ızgarası_göster(false)
        .piksel_hizası(0.0)
        .etkileşimler(ortak_kart_etkileşimleri())
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni("y").sayısal_aralık(SayısalAralıkAyarları::yeni(
                SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
                SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
            )),
        )
        .seri(
            SeriSeçenekleri::yeni(örnek.ölçüm())
                .renk("#03a9f4")
                .dolgu("#b3e5fc")
                .noktaları_göster(false),
        );
    Ok((seçenekler, veri))
}

/// Resmî tablonun 10 hisse × (hacim, kapanış) yüzeylerini satır sırasıyla
/// tek bir kaynak grubu olarak üretir.
pub fn sparklines_kartları()
-> Result<Vec<(SparklineÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    SparklineÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            let (seçenekler, veri) = sparklines_kartı(örnek)?;
            Ok((örnek, seçenekler, veri))
        })
        .collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn yirmi_yüzey_ve_kaynak_csv_değerleri_korunur() -> Result<(), UplotHatası> {
        assert_eq!(SparklineÖrneği::TÜMÜ.len(), 20);
        for örnek in SparklineÖrneği::TÜMÜ {
            let (seçenekler, veri) = sparklines_kartı(örnek)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (150, 30));
            assert!(seçenekler.kompakt_yüzey);
            assert_eq!(veri.uzunluk(), 22);
            let grafik = Grafik::yeni(seçenekler, veri)?;
            assert_eq!(grafik.çizim_alanı_boyutta(150, 30), (0.0, 150.0, 0.0, 30.0));
        }
        let (_, aapl_hacim) = sparklines_kartı(SparklineÖrneği::TÜMÜ[0])?;
        assert_eq!(
            aapl_hacim
                .seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(25_093_670.0))
        );
        let (_, znga_kapanış) = sparklines_kartı(SparklineÖrneği::TÜMÜ[19])?;
        assert_eq!(
            znga_kapanış
                .seriler()
                .first()
                .and_then(|seri| seri.get(21))
                .copied(),
            Some(Some(6.24))
        );
        Ok(())
    }

    #[test]
    fn on_satır_iki_sütun_tek_kaynak_grubunda_korunur() -> Result<(), UplotHatası> {
        let kartlar = sparklines_kartları()?;
        assert_eq!(kartlar.len(), 20);
        for (satır, ((hacim, kapanış), çift)) in SparklineÖrneği::SATIRLAR
            .into_iter()
            .zip(kartlar.chunks_exact(2))
            .enumerate()
        {
            assert_eq!(çift.first().map(|kart| kart.0), Some(hacim));
            assert_eq!(çift.get(1).map(|kart| kart.0), Some(kapanış));
            assert_eq!(hacim.simge(), kapanış.simge());
            assert_eq!(hacim.ölçüm(), "Hacim");
            assert_eq!(kapanış.ölçüm(), "Kapanış");
            assert_eq!(usize::from(hacim.0) / 2, satır);
        }
        Ok(())
    }

    #[test]
    fn aapl_y_aralıkları_kaynağın_range_num_davranışını_korur() -> Result<(), UplotHatası> {
        let (hacim_seçenekleri, hacim_verisi) = sparklines_kartı(SparklineÖrneği::TÜMÜ[0])?;
        let hacim = crate::Grafik::yeni(hacim_seçenekleri, hacim_verisi)?;
        let hacim_aralığı = hacim.görünür_y_aralığı();
        assert_eq!(
            (hacim_aralığı.en_az, hacim_aralığı.en_çok),
            (15_000_000.0, 40_000_000.0)
        );

        let (kapanış_seçenekleri, kapanış_verisi) = sparklines_kartı(SparklineÖrneği::TÜMÜ[1])?;
        let kapanış = crate::Grafik::yeni(kapanış_seçenekleri, kapanış_verisi)?;
        let kapanış_aralığı = kapanış.görünür_y_aralığı();
        assert_eq!(
            (kapanış_aralığı.en_az, kapanış_aralığı.en_çok),
            (232.0, 269.0)
        );
        Ok(())
    }

    #[test]
    fn kompakt_sahne_kaynak_renklerini_ve_alt_pikseli_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = sparklines_kartı(SparklineÖrneği::TÜMÜ[0])?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(sahne.svg().contains("width=\"150\" height=\"30\""));
        assert!(matches!(
            sahne.komutlar().first(),
            Some(Komut::ArkaPlan { renk }) if renk == "pink"
        ));
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "#b3e5fc"))
        );
        Ok(())
    }

    #[test]
    fn geçersiz_kompakt_boyut_tipli_hata_döndürür() {
        assert!(matches!(
            GrafikSeçenekleri::kompakt(1, 30),
            Err(UplotHatası::GeçersizBoyut { .. })
        ));
    }
}
