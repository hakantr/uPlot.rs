use super::ortak_kart_etkileşimleri;
use crate::{
    GradyanDurağı, GradyanEkseni, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası,
    YÖlçekSeçenekleri, ÖlçekGradyanı,
};
use std::sync::OnceLock;

pub const GRADIENTS_KART_TANIM_ÖRNEĞİ: &str = r##"for (örnek, seçenekler, veri) in gradients_kartları()? {
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

static YATAY_VERİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();
static DİKEY_VERİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();
static DOLGU_VERİSİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradientÖrneği {
    YatayÇizgi,
    DikeyÇizgi,
    DikeyArcSinh,
    ÖlçekDolguları,
    GöreliDolgu,
}

impl GradientÖrneği {
    pub const TÜMÜ: [Self; 5] = [
        Self::YatayÇizgi,
        Self::DikeyÇizgi,
        Self::DikeyArcSinh,
        Self::ÖlçekDolguları,
        Self::GöreliDolgu,
    ];

    pub fn kimlik(self) -> &'static str {
        match self {
            Self::YatayÇizgi => "gradients-horizontal-stroke",
            Self::DikeyÇizgi => "gradients-vertical-stroke",
            Self::DikeyArcSinh => "gradients-vertical-arcsinh",
            Self::ÖlçekDolguları => "gradients-scale-fills",
            Self::GöreliDolgu => "gradients-relative-fill",
        }
    }

    pub fn başlık(self) -> &'static str {
        match self {
            Self::YatayÇizgi => "Scale-aligned gradient strokes (hz)",
            Self::DikeyÇizgi | Self::DikeyArcSinh => "Scale-aligned gradient strokes (vt)",
            Self::ÖlçekDolguları => "Scale-aligned gradient fills",
            Self::GöreliDolgu => "Data min/max % relative gradient fills",
        }
    }

    pub fn durum(self) -> &'static str {
        match self {
            Self::YatayÇizgi => "5 nokta · X'e hizalı ayrık duraklar",
            Self::DikeyÇizgi => "5 nokta · Y'ye hizalı mavi/kırmızı",
            Self::DikeyArcSinh => "aynı 5 noktalı veri · ArcSinh · 3 ayrık renk",
            Self::ÖlçekDolguları => "6 nokta · 2 basınç dolgusu",
            Self::GöreliDolgu => "aynı 6×2 veri · görünür min/orta/max",
        }
    }
}

/// Resmî sayfadaki beş bağımsız yüzeyi kaynak sırasıyla döndürür.
///
/// Dikey lineer/ArcSinh yüzeyleri aynı `data2`, iki dolgu yüzeyi aynı `data4`
/// immutable aligned veri deposunu paylaşır.
pub fn gradients_kartları()
-> Result<Vec<(GradientÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    GradientÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            let (seçenekler, veri) = gradients_kartı(örnek)?;
            Ok((örnek, seçenekler, veri))
        })
        .collect()
}

pub fn gradients_kartı(
    örnek: GradientÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        GradientÖrneği::YatayÇizgi => yatay_çizgi(),
        GradientÖrneği::DikeyÇizgi => dikey_çizgi(false),
        GradientÖrneği::DikeyArcSinh => dikey_çizgi(true),
        GradientÖrneği::ÖlçekDolguları => ölçek_dolguları(),
        GradientÖrneği::GöreliDolgu => göreli_dolgu(),
    }
}

fn temel(başlık: &str) -> Result<GrafikSeçenekleri, UplotHatası> {
    Ok(GrafikSeçenekleri::yeni(800, 600)?
        .başlık(başlık)
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri()))
}

fn yoğun(x: Vec<f64>, seriler: Vec<Vec<f64>>) -> Result<HizalıVeri, UplotHatası> {
    HizalıVeri::yeni(
        x,
        seriler
            .into_iter()
            .map(|seri| seri.into_iter().map(Some).collect())
            .collect(),
    )
}

fn sabit_gradyan(
    eksen: GradyanEkseni,
    duraklar: &[(f64, &str)],
    ayrık: bool,
) -> Result<ÖlçekGradyanı, UplotHatası> {
    let duraklar = duraklar
        .iter()
        .map(|(değer, renk)| GradyanDurağı::değer(*değer, *renk))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ÖlçekGradyanı::yeni(eksen, duraklar)?.ayrık(ayrık))
}

fn yatay_çizgi() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = YATAY_VERİ
        .get_or_init(|| {
            yoğun(
                vec![20.0, 30.0, 40.0, 50.0, 60.0],
                vec![vec![20.0, 10.0, 25.0, 50.0, 30.0]],
            )
        })
        .clone()?;
    let gradyan = sabit_gradyan(
        GradyanEkseni::X,
        &[(0.0, "#ff0000"), (30.0, "#ffa500"), (50.0, "#0000ff")],
        true,
    )?;
    let seçenekler = temel(GradientÖrneği::YatayÇizgi.başlık())?.seri(
        SeriSeçenekleri::yeni("Trends")
            .renk("#ff0000")
            .çizgi_kalınlığı(4.0)
            .çizgi_gradyanı(gradyan),
    );
    Ok((seçenekler, veri))
}

fn dikey_çizgi(arcsinh: bool) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = DİKEY_VERİ
        .get_or_init(|| {
            yoğun(
                vec![20.0, 30.0, 40.0, 50.0, 60.0],
                vec![vec![-5.0, 10.0, -2.0, -30.0, 30.0]],
            )
        })
        .clone()?;
    let gradyan = if arcsinh {
        ÖlçekGradyanı::yeni(
            GradyanEkseni::Y,
            vec![
                GradyanDurağı::negatif_sonsuz("#0000ff"),
                GradyanDurağı::değer(-10.0, "#ff0000")?,
                GradyanDurağı::değer(0.0, "#008000")?,
            ],
        )?
        .ayrık(true)
    } else {
        sabit_gradyan(
            GradyanEkseni::Y,
            &[(-100.0, "#0000ff"), (0.0, "#ff0000")],
            true,
        )?
    };
    let mut seçenekler = temel(if arcsinh {
        GradientÖrneği::DikeyArcSinh.başlık()
    } else {
        GradientÖrneği::DikeyÇizgi.başlık()
    })?;
    if arcsinh {
        seçenekler = seçenekler.y_ölçeği(YÖlçekSeçenekleri::yeni("y").arcsinh(1.0));
    }
    seçenekler = seçenekler.seri(
        SeriSeçenekleri::yeni("Over/Under")
            .renk("#ff0000")
            .çizgi_kalınlığı(4.0)
            .çizgi_gradyanı(gradyan),
    );
    Ok((seçenekler, veri))
}

fn ölçek_dolguları() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = dolgu_verisi()?;
    let ilk = sabit_gradyan(
        GradyanEkseni::Y,
        &[(30.0, "#008000"), (50.0, "#ffa500"), (60.0, "#ff0000")],
        false,
    )?;
    let ikinci = sabit_gradyan(
        GradyanEkseni::Y,
        &[(0.0, "#ff0000"), (10.0, "#ffa500"), (20.0, "#008000")],
        false,
    )?;
    let seçenekler = temel(GradientÖrneği::ÖlçekDolguları.başlık())?
        .seri(
            SeriSeçenekleri::yeni("Tank 1 (red = high pressure)")
                .renk("#00ff00")
                .çizgi_kalınlığı(4.0)
                .dolgu_gradyanı(ilk),
        )
        .seri(
            SeriSeçenekleri::yeni("Tank 2 (red = low pressure)")
                .renk("#ff00ff")
                .çizgi_kalınlığı(4.0)
                .dolgu_gradyanı(ikinci),
        );
    Ok((seçenekler, veri))
}

fn göreli_dolgu() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = dolgu_verisi()?;
    let gradyan = ÖlçekGradyanı::yeni(
        GradyanEkseni::Y,
        vec![
            GradyanDurağı::görünür_veri_oranı(0.0, "#008000")?,
            GradyanDurağı::görünür_veri_oranı(0.5, "#ffa500")?,
            GradyanDurağı::görünür_veri_oranı(1.0, "#ff0000")?,
        ],
    )?;
    let seçenekler = temel(GradientÖrneği::GöreliDolgu.başlık())?
        .seri(
            SeriSeçenekleri::yeni("Tank 1")
                .renk("#00ff00")
                .çizgi_kalınlığı(4.0)
                .dolgu_gradyanı(gradyan),
        )
        // Kaynak u5, u4 ile aynı üç sütunlu `data4` nesnesini geçirir; ikinci
        // Y sütunu için seçenek tanımlamadığından çizilmez.
        .seri(SeriSeçenekleri::yeni("Tank 2").göster(false));
    Ok((seçenekler, veri))
}

fn dolgu_verisi() -> Result<HizalıVeri, UplotHatası> {
    DOLGU_VERİSİ
        .get_or_init(|| {
            yoğun(
                vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0],
                vec![
                    vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0],
                    vec![1.0, 3.0, 5.0, 8.0, 15.0, 20.0],
                ],
            )
        })
        .clone()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn beş_kaynak_grafiği_ve_ölçek_durakları_korunur() -> Result<(), UplotHatası> {
        let kartlar = gradients_kartları()?;
        assert_eq!(
            kartlar
                .iter()
                .map(|(örnek, _, _)| *örnek)
                .collect::<Vec<_>>(),
            GradientÖrneği::TÜMÜ
        );
        let Some((_, _, dikey)) = kartlar.get(1) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 1 });
        };
        let Some((_, _, arcsinh)) = kartlar.get(2) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 2 });
        };
        let Some((_, _, dolgular)) = kartlar.get(3) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 3 });
        };
        let Some((_, göreli_seçenekler, göreli_veri)) = kartlar.get(4) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 4 });
        };
        assert!(dikey.aynı_depolamayı_paylaşıyor(arcsinh));
        assert!(dolgular.aynı_depolamayı_paylaşıyor(göreli_veri));
        assert_eq!(göreli_veri.seriler().len(), 2);
        assert_eq!(
            göreli_seçenekler.seriler.get(1).map(|seri| seri.göster),
            Some(false)
        );

        for (_, seçenekler, veri) in kartlar {
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(sahne.komutlar().iter().any(|komut| matches!(
                komut,
                Komut::GradyanYol { .. } | Komut::GradyanAlan { .. }
            )));
        }
        let (_, veri) = gradients_kartı(GradientÖrneği::ÖlçekDolguları)?;
        assert_eq!(veri.x(), &[0.0, 1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(veri.seriler().len(), 2);

        let (seçenekler, veri) = gradients_kartı(GradientÖrneği::YatayÇizgi)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.seri_imleç_rengi(0, 20.0, 20.0).as_deref(),
            Some("#ff0000")
        );
        assert_eq!(
            grafik.seri_imleç_rengi(0, 30.0, 10.0).as_deref(),
            Some("#ffa500")
        );
        assert_eq!(
            grafik.seri_imleç_rengi(0, 50.0, 50.0).as_deref(),
            Some("#0000ff")
        );

        let (seçenekler, veri) = gradients_kartı(GradientÖrneği::DikeyArcSinh)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(
            grafik.seri_imleç_rengi(0, 20.0, -30.0).as_deref(),
            Some("#0000ff")
        );
        assert_eq!(
            grafik.seri_imleç_rengi(0, 30.0, -10.0).as_deref(),
            Some("#ff0000")
        );
        assert_eq!(
            grafik.seri_imleç_rengi(0, 40.0, 0.0).as_deref(),
            Some("#008000")
        );
        Ok(())
    }

    #[test]
    fn göreli_duraklar_yakınlaştırılan_veri_aralığıyla_yeniden_çözülür() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = gradients_kartı(GradientÖrneği::GöreliDolgu)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let ilk = grafik.çiz();
        let ilk_gradyan = ilk.komutlar().iter().find_map(|komut| match komut {
            Komut::GradyanAlan { gradyan, .. } => Some(gradyan),
            _ => None,
        });
        let Some(ilk_gradyan) = ilk_gradyan else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
        };
        assert_eq!(
            ilk_gradyan
                .duraklar
                .iter()
                .map(|durak| durak.renk.as_ref())
                .collect::<Vec<_>>(),
            ["#008000", "#ffa500", "#ff0000"]
        );
        assert_eq!(
            ilk_gradyan
                .duraklar
                .iter()
                .map(|durak| durak.oran)
                .collect::<Vec<_>>(),
            [0.0, 0.5, 1.0]
        );
        let ilk_başlangıç = ilk_gradyan.başlangıç.y;
        // Tekerlek yakınlaştırması varsayılan kapalı; test onu
        // araç olarak kullandığı için açıkça açıyor.
        grafik.tekerlek_etkileşimi_ayarla(true);
        assert!(grafik.tekerlek(0.7, 0.5, 1.0, false)?);
        let yakın = grafik.çiz();
        let yakın_başlangıç = yakın.komutlar().iter().find_map(|komut| match komut {
            Komut::GradyanAlan { gradyan, .. } => Some(gradyan.başlangıç.y),
            _ => None,
        });
        assert_ne!(Some(ilk_başlangıç), yakın_başlangıç);
        Ok(())
    }
}
