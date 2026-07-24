use serde_json::Value;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri,
    ÇubukDüzeni, ÇubukYönü,
};

const RESULTS_JSON: &str = include_str!("veri/multi-bars-results.json");

pub const MULTI_BARS_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in MultiBarsÖrneği::TÜMÜ {
    let (seçenekler, veri) = multi_bars_kartı(örnek)?;
    // Metrik sayısı sabit değildir; her SeriSeçenekleri kendi ölçeğini,
    // çizgi/dolgu rengini ve çizgi ya da çubuk davranışını taşır.
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    grafik.seri_görünürlüğünü_ayarla(0, false)?;
    grafik.seri_renklerini_ayarla(1, "#7c3aed", Some("#7c3aed80".into()))?;
    grafik.seri_çubuk_renklerini_ayarla(2, dinamik_dolgular, dinamik_çizgiler)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MultiBarsÖrneği {
    KitaplıklarDikey,
    KitaplıklarYatay,
    DeğişkenRenkler,
    HizasızÇubuklar,
}

impl MultiBarsÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::KitaplıklarDikey,
        Self::KitaplıklarYatay,
        Self::DeğişkenRenkler,
        Self::HizasızÇubuklar,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::KitaplıklarDikey => "multi-bars-libraries-vertical",
            Self::KitaplıklarYatay => "multi-bars-libraries-horizontal",
            Self::DeğişkenRenkler => "multi-bars-variable-colors",
            Self::HizasızÇubuklar => "multi-bars-non-justified",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::KitaplıklarDikey => "Multi Bars · library benchmark · vertical",
            Self::KitaplıklarYatay => "Multi Bars · library benchmark · horizontal",
            Self::DeğişkenRenkler => "Multi Bars · variable colors",
            Self::HizasızÇubuklar => "Multi Bars · non-justified",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn multi_bars_kartı(
    örnek: MultiBarsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        MultiBarsÖrneği::KitaplıklarDikey | MultiBarsÖrneği::KitaplıklarYatay => {
            kitaplık_kartı(örnek)
        }
        MultiBarsÖrneği::DeğişkenRenkler | MultiBarsÖrneği::HizasızÇubuklar => {
            renkli_kart(örnek)
        }
    }
}

fn kitaplık_kartı(
    örnek: MultiBarsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let satırlar: Vec<Vec<Value>> =
        serde_json::from_str(RESULTS_JSON).map_err(|hata| UplotHatası::GeçersizKaynakVeri {
            varlık: "bench/results.json",
            açıklama: hata.to_string(),
        })?;
    let kategoriler = satırlar
        .iter()
        .filter_map(|satır| satır.first()?.as_str().map(str::to_owned))
        .collect::<Vec<_>>();
    let sayı = |satır: &[Value], indeks: usize| satır.get(indeks).and_then(Value::as_f64);
    let toplam = |satır: &[Value], indeks: usize| {
        satır
            .get(indeks)?
            .as_array()?
            .iter()
            .map(Value::as_f64)
            .sum::<Option<f64>>()
    };
    let seriler: Vec<Vec<Option<f64>>> = vec![
        satırlar.iter().map(|s| sayı(s, 1)).collect(),
        satırlar
            .iter()
            .map(|s| toplam(s, 3).map(f64::round))
            .collect(),
        satırlar
            .iter()
            .map(|s| s.get(4)?.as_array()?.first()?.as_f64())
            .collect(),
        satırlar
            .iter()
            .map(|s| s.get(4)?.as_array()?.get(1)?.as_f64())
            .collect(),
        satırlar.iter().map(|s| toplam(s, 5)).collect(),
        satırlar.iter().map(|s| toplam(s, 6)).collect(),
        (0..satırlar.len())
            .map(|indeks| Some(3_000.0 + ((indeks * 677 + 131) % 1_001) as f64))
            .collect(),
    ];
    let ölçek_aralığı = |indeksler: &[usize]| -> Result<Aralık, UplotHatası> {
        let en_çok = indeksler
            .iter()
            .filter_map(|indeks| seriler.get(*indeks))
            .flat_map(|seri| seri.iter().copied().flatten())
            .fold(0.0_f64, f64::max);
        Aralık::uplot_sayısal(0.0, en_çok, 0.05, true)
    };
    let ölçekler = [
        ("size", ölçek_aralığı(&[0])?),
        ("rend", ölçek_aralığı(&[1, 6])?),
        ("mem", ölçek_aralığı(&[2, 3])?),
        ("inter", ölçek_aralığı(&[4])?),
        ("toggle", ölçek_aralığı(&[5])?),
    ];
    let veri = HizalıVeri::yeni((0..kategoriler.len()).map(|i| i as f64).collect(), seriler)?;
    let yatay = örnek == MultiBarsÖrneği::KitaplıklarYatay;
    let düzen = ÇubukDüzeni::yeni(if yatay {
        ÇubukYönü::Yatay
    } else {
        ÇubukYönü::Dikey
    })
    .ters(yatay)
    .genişlik_oranı(0.9)
    .uç_yarıçap_oranı(0.3);
    let (genişlik, yükseklik) = if yatay { (800, 2300) } else { (2300, 800) };
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık("Line Charts (166,650 points)")
        .x_zaman(false)
        .kategoriler(kategoriler)
        .birincil_y_ölçeği("rend")
        .çubuk_düzeni(düzen)
        .etkileşimler(ortak_kart_etkileşimleri());
    for (anahtar, aralık) in ölçekler {
        seçenekler = seçenekler.y_ölçeği(YÖlçekSeçenekleri::yeni(anahtar).aralık(aralık));
    }
    for (etiket, renk, ölçek) in [
        ("Lib Size (KB)", "#33BB55", "size"),
        ("Render Time (ms)", "#B56FAB", "rend"),
        ("Peak Heap (MB)", "#BB1133", "mem"),
        ("Final Heap (MB)", "#F79420", "mem"),
        ("Interact 10s (ms)", "#558AB5", "inter"),
        ("Toggle 6x (ms)", "#FAD55C", "toggle"),
    ] {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk(renk)
                .dolgu(renk)
                .ölçek(ölçek)
                .çizgi_kalınlığı(0.0)
                .çubuk(true),
        );
    }
    seçenekler = seçenekler.seri(
        SeriSeçenekleri::yeni("Random line")
            .renk("purple")
            .ölçek("rend")
            .çizgi_kalınlığı(2.0),
    );
    Ok((seçenekler, veri))
}

fn renkli_kart(
    örnek: MultiBarsÖrneği
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = (0..26).map(f64::from).collect::<Vec<_>>();
    // Kaynak demodaki rastgele örneğin sabit tohumlu, yeniden üretilebilir karşılığı.
    let değerler = (0..26)
        .map(|i| {
            let değer = ((i * 37 + 19) % 101) as f64;
            Some(if değer as i32 % 3 == 0 {
                -değer
            } else {
                değer
            })
        })
        .collect::<Vec<_>>();
    let dolgular = (0..26).map(|i| {
        if i % 12 == 0 {
            "#F79420A0"
        } else if i % 10 == 0 {
            "#BB1133A0"
        } else {
            "#33BB55A0"
        }
    });
    let çizgiler = (0..26).map(|i| {
        if i % 12 == 0 {
            "#F79420"
        } else if i % 10 == 0 {
            "#BB1133"
        } else {
            "#33BB55"
        }
    });
    let veri = HizalıVeri::yeni(x, vec![değerler])?;
    let seri = SeriSeçenekleri::yeni("Server #SNAFU")
        .renk("#33BB55")
        .dolgu("#33BB55A0")
        .çizgi_kalınlığı(if örnek == MultiBarsÖrneği::DeğişkenRenkler {
            2.0
        } else {
            1.0
        })
        .çubuk_dolguları(dolgular)
        .çubuk_çizgileri(çizgiler)
        .çubuk(true);
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri());
    if örnek == MultiBarsÖrneği::DeğişkenRenkler {
        seçenekler = seçenekler
            .kategoriler(('a'..='z').map(|harf| harf.to_string()))
            .çubuk_düzeni(
                ÇubukDüzeni::yeni(ÇubukYönü::Dikey)
                    .genişlik_oranı(0.9)
                    .uç_yarıçap_oranı(0.5),
            )
            .y_ölçeği(YÖlçekSeçenekleri::yeni("y").birim("s"))
            .seri(seri);
    } else {
        seçenekler = seçenekler.seri(seri.çubuk_boyutu(0.6, f32::INFINITY));
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, TekerlekEkseni};

    #[test]
    fn dört_kaynak_yüzeyi_çizilir() -> Result<(), UplotHatası> {
        for örnek in MultiBarsÖrneği::TÜMÜ {
            let (seçenekler, veri) = multi_bars_kartı(örnek)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne.komutlar().iter().any(|komut| matches!(
                    komut,
                    Komut::Dikdörtgen { .. } | Komut::YuvarlatılmışDikdörtgen { .. }
                )),
                "{}",
                örnek.kimlik()
            );
        }
        Ok(())
    }

    #[test]
    fn kaynak_benchmark_null_değerlerini_korur() -> Result<(), UplotHatası> {
        let (_, veri) = multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)?;
        assert_eq!(veri.uzunluk(), 13);
        assert_eq!(veri.seriler().len(), 7);
        assert!(
            veri.seriler()
                .get(4)
                .and_then(|seri| seri.get(2))
                .is_some_and(Option::is_none)
        );
        Ok(())
    }

    #[test]
    fn benchmark_metrikleri_kendi_ölçeklerinde_çizilir_ve_çizgi_ayrılır() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let azami_yükseklik = |aranan: &str| {
            sahne
                .komutlar()
                .iter()
                .filter_map(|komut| match komut {
                    Komut::YuvarlatılmışDikdörtgen {
                        dolgu, yükseklik, ..
                    } if dolgu == aranan => Some(*yükseklik),
                    _ => None,
                })
                .fold(0.0_f32, f32::max)
        };
        assert!(azami_yükseklik("#33BB55") > 500.0);
        assert!(azami_yükseklik("#558AB5") > 500.0);
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "purple"))
        );
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == "9869"))
        );
        Ok(())
    }

    #[test]
    fn değişken_renkler_kaynak_alfa_vuruş_ve_uç_yarıçapını_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = multi_bars_kartı(MultiBarsÖrneği::DeğişkenRenkler)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        assert!(sahne.komutlar().iter().any(|komut| {
            matches!(
                komut,
                Komut::YuvarlatılmışDikdörtgen {
                    dolgu,
                    çizgi,
                    kalınlık,
                    yarıçaplar,
                    ..
                } if dolgu == "#F79420A0"
                    && çizgi == "#F79420"
                    && (*kalınlık - 2.0).abs() <= f32::EPSILON
                    && (yarıçaplar.üst_sol > 0.0 || yarıçaplar.alt_sol > 0.0)
            )
        }));
        Ok(())
    }

    #[test]
    fn görünürlük_ve_renkler_çalışma_anında_değiştirilebilir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;

        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert!(!grafik.seri_görünürlüğünü_ayarla(0, false)?);
        let sahne = grafik.çiz();
        assert!(!sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::YuvarlatılmışDikdörtgen { dolgu, .. } if dolgu == "#33BB55")
        ));

        assert!(grafik.seri_renklerini_ayarla(1, "#123456", Some("#abcdef".to_string()))?);
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().any(|komut| matches!(
            komut,
            Komut::YuvarlatılmışDikdörtgen { dolgu, çizgi, .. }
                if dolgu == "#abcdef" && çizgi == "#123456"
        )));

        assert!(grafik.seri_görünürlüğünü_ayarla(6, false)?);
        assert!(
            !grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "purple"))
        );
        Ok(())
    }

    #[test]
    fn tekerlek_bağımsız_metrik_ölçeklerini_ve_çubukları_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = grafik.görünür_x_aralığı();
        assert!(grafik.tekerlek(0.3, 0.5, 120.0, true)?);
        let sonra = grafik.görünür_x_aralığı();
        assert!(sonra.en_çok - sonra.en_az < önce.en_çok - önce.en_az);
        assert!(
            grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::YuvarlatılmışDikdörtgen { .. }))
        );
        for seri in 0..6 {
            let aralık = grafik.seri_görünür_y_aralığı(seri);
            assert!(aralık.is_some(), "metrik ölçeği bulunmalı");
            if let Some(aralık) = aralık {
                assert!(aralık.en_az.is_finite() && aralık.en_çok.is_finite());
                assert!(aralık.en_çok > aralık.en_az);
            }
        }
        Ok(())
    }

    #[test]
    fn y_tekerlek_yakınlaştırmasında_değer_etiketleri_yüzeyden_uçmaz() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        for _ in 0..7 {
            assert!(grafik.tekerlek_eksende(0.5, 0.45, 100.0, true, TekerlekEkseni::Y,)?);
        }
        let sahne = grafik.çiz();
        for komut in sahne.komutlar() {
            if let Komut::Metin { konum, .. } = komut {
                assert!(konum.x >= 0.0 && konum.x <= 2_300.0, "{konum:?}");
                assert!(konum.y >= 0.0 && konum.y <= 800.0, "{konum:?}");
            }
        }
        Ok(())
    }
}
