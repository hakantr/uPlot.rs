use super::ortak_kart_etkileşimleri;
use crate::{
    GradyanDurağı, GradyanEkseni, GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları,
    SayısalAralıkParçası, SeriSeçenekleri, UplotHatası, YumuşakSınırKipi, YÖlçekSeçenekleri,
    ÇizimSırası, ÖlçekGradyanı,
};

pub const SPARKLINES_BARS_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in SparklinesBarsÖrneği::TÜMÜ {
    let (seçenekler, veri) = sparklines_bars_kartı(örnek)?;
    // Sparkline, yüzen çubuk uçları, gradyan veya nokta başına renk ve
    // Y=0 kesikli ızgarası çekirdekte çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SparklinesBarsÖrneği {
    GradyanÇubuklar,
    AyrıkRenkliÇubuklar,
}

impl SparklinesBarsÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::GradyanÇubuklar, Self::AyrıkRenkliÇubuklar];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::GradyanÇubuklar => "sparklines-bars-gradient",
            Self::AyrıkRenkliÇubuklar => "sparklines-bars-explicit-colors",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::GradyanÇubuklar => "Sparkline + Floating Bars · gradient",
            Self::AyrıkRenkliÇubuklar => "Sparkline + Floating Bars · explicit colors",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn sparklines_bars_kartı(
    örnek: SparklinesBarsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let değerler = [
        5.0, -5.0, 0.0, 1.0, 5.0, 9.0, 10.0, 15.0, 5.0, -10.0, -15.0, -20.0, -20.0, -5.0, 0.0, 5.0,
    ];
    let altlar = değerler.map(|değer| değer - 5.0);
    let üstler = değerler.map(|değer| değer + 5.0);
    let veri = HizalıVeri::yeni(
        (1_u32..=16).map(f64::from).collect(),
        vec![
            değerler.into_iter().map(Some).collect(),
            altlar.into_iter().map(Some).collect(),
            üstler.into_iter().map(Some).collect(),
        ],
    )?;

    let çizgi_dolgusu = ÖlçekGradyanı::yeni(
        GradyanEkseni::Y,
        vec![
            GradyanDurağı::değer(-25.0, "red")?,
            GradyanDurağı::değer(0.0, "white")?,
            GradyanDurağı::değer(20.0, "green")?,
        ],
    )?;
    let ayrık_kırmızı_yeşil = ÖlçekGradyanı::yeni(
        GradyanEkseni::Y,
        vec![
            GradyanDurağı::değer(-25.0, "red")?,
            GradyanDurağı::değer(0.0, "green")?,
        ],
    )?
    .ayrık(true);
    let tam_veri_aralığı = SayısalAralıkAyarları::yeni(
        SayısalAralıkParçası::yeni(0.0, None, YumuşakSınırKipi::SabitPay),
        SayısalAralıkParçası::yeni(0.0, None, YumuşakSınırKipi::SabitPay),
    );

    let mut çubuk = SeriSeçenekleri::yeni("Floating Bars")
        .renk("none")
        .çizgi_kalınlığı(0.0)
        .noktaları_göster(false)
        .çubuk(true)
        .yüzen_çubuk_üst_serisi(2);
    çubuk = match örnek {
        SparklinesBarsÖrneği::GradyanÇubuklar => {
            çubuk.dolgu_gradyanı(ayrık_kırmızı_yeşil.clone())
        }
        SparklinesBarsÖrneği::AyrıkRenkliÇubuklar => {
            let dolgular = altlar.into_iter().zip(üstler).map(|(alt, üst)| {
                if alt < 0.0 || üst < 0.0 {
                    "red"
                } else {
                    "green"
                }
            });
            çubuk.çubuk_dolguları(dolgular)
        }
    };

    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .x_zaman(false)
        .x_ekseni_göster(false)
        .x_ızgarası_göster(false)
        .y_ekseni_göster(false)
        .y_ızgarası_göster(true)
        .y_sabit_bölmeler(vec![0.0])
        .y_ızgara_kesik(3.0)
        .ızgara_rengi("gray")
        .çizim_sırası(ÇizimSırası::SerilerEksenler)
        .etkileşimler(ortak_kart_etkileşimleri())
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").sayısal_aralık(tam_veri_aralığı))
        .seri(
            SeriSeçenekleri::yeni("Value")
                .noktaları_göster(false)
                .dolgu_gradyanı(çizgi_dolgusu)
                .çizgi_gradyanı(ayrık_kırmızı_yeşil),
        )
        .seri(çubuk)
        .seri(
            SeriSeçenekleri::yeni("Bar highs")
                .göster(false)
                .çizgi_kalınlığı(0.0)
                .noktaları_göster(false),
        );

    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn iki_kaynak_yüzeyi_aynı_yüzen_çubuk_verisini_kullanır() -> Result<(), UplotHatası> {
        let (ilk_seçenekler, ilk_veri) =
            sparklines_bars_kartı(SparklinesBarsÖrneği::GradyanÇubuklar)?;
        let (ikinci_seçenekler, ikinci_veri) =
            sparklines_bars_kartı(SparklinesBarsÖrneği::AyrıkRenkliÇubuklar)?;
        assert_eq!(ilk_veri, ikinci_veri);
        assert_eq!(ilk_veri.uzunluk(), 16);
        assert_eq!(ilk_veri.seriler().len(), 3);
        assert_eq!(
            ilk_veri.seriler().get(1).and_then(|seri| seri.first()),
            Some(&Some(0.0))
        );
        assert_eq!(
            ilk_veri.seriler().get(2).and_then(|seri| seri.first()),
            Some(&Some(10.0))
        );
        for (seçenekler, veri) in [(ilk_seçenekler, ilk_veri), (ikinci_seçenekler, ikinci_veri)]
        {
            let grafik = Grafik::yeni(seçenekler, veri)?;
            assert_eq!(grafik.görünür_y_aralığı().en_az, -25.0);
            assert_eq!(grafik.görünür_y_aralığı().en_çok, 20.0);
            assert_eq!(
                grafik.çizim_alanı_boyutta(800, 400),
                (8.0, 792.0, 8.0, 392.0)
            );
        }
        Ok(())
    }

    #[test]
    fn gradyan_ve_nokta_başı_dolgular_kaynak_gibi_ayrılır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = sparklines_bars_kartı(SparklinesBarsÖrneği::GradyanÇubuklar)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yüzen_gradyan = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::GradyanAlan { çokgenler, .. }
                if çokgenler.len() == 16 && çokgenler.iter().all(|çokgen| çokgen.len() == 4) =>
            {
                Some(çokgenler)
            }
            _ => None,
        });
        assert!(yüzen_gradyan.is_some());
        assert!(matches!(
            sahne.komutlar().last(),
            Some(Komut::KesikliÇizgi { renk, kesik, .. })
                if renk == "gray" && (*kesik - 3.0).abs() <= f32::EPSILON
        ));

        let (seçenekler, veri) = sparklines_bars_kartı(SparklinesBarsÖrneği::AyrıkRenkliÇubuklar)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let dolgular = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Dikdörtgen { dolgu, .. } if dolgu == "red" || dolgu == "green" => {
                    Some(dolgu.as_str())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(dolgular.len(), 16);
        assert_eq!(dolgular.iter().filter(|renk| **renk == "red").count(), 9);
        assert_eq!(dolgular.iter().filter(|renk| **renk == "green").count(), 7);
        Ok(())
    }

    #[test]
    fn geçersiz_yüzen_üst_serisi_tipli_hatayla_reddedilir() -> Result<(), UplotHatası> {
        let veri = HizalıVeri::yeni(vec![1.0], vec![vec![Some(0.0)]])?;
        let seçenekler = GrafikSeçenekleri::yeni(800, 400)?.seri(
            SeriSeçenekleri::yeni("Floating")
                .çubuk(true)
                .yüzen_çubuk_üst_serisi(1),
        );
        assert!(Grafik::yeni(seçenekler, veri).is_err());
        Ok(())
    }
}
