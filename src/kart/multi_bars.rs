use serde_json::Value;

use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇubukDüzeni, ÇubukYönü
};

const RESULTS_JSON: &str = include_str!("veri/multi-bars-results.json");

pub const MULTI_BARS_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in MultiBarsÖrneği::TÜMÜ {
    let (seçenekler, veri) = multi_bars_kartı(örnek)?;
    // Gruplu yön, negatif değerler ve nokta başına durum renkleri çekirdekte çözülür.
    let grafik = Grafik::yeni(seçenekler, veri)?;
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
    let seriler = vec![
        satırlar.iter().map(|s| sayı(s, 1)).collect(),
        satırlar.iter().map(|s| toplam(s, 3)).collect(),
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
    ];
    let veri = HizalıVeri::yeni((0..kategoriler.len()).map(|i| i as f64).collect(), seriler)?;
    let yatay = örnek == MultiBarsÖrneği::KitaplıklarYatay;
    let düzen = ÇubukDüzeni::yeni(if yatay {
        ÇubukYönü::Yatay
    } else {
        ÇubukYönü::Dikey
    })
    .ters(yatay)
    .genişlik_oranı(0.7);
    let (genişlik, yükseklik) = if yatay { (800, 2300) } else { (2300, 800) };
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık("Line Charts (166,650 points)")
        .x_zaman(false)
        .kategoriler(kategoriler)
        .çubuk_düzeni(düzen)
        .etkileşimler(ortak_kart_etkileşimleri());
    for (etiket, renk) in [
        ("Lib Size KB", "#33BB55"),
        ("Render Time ms", "#B56FAB"),
        ("Peak Heap MB", "#BB1133"),
        ("Final Heap MB", "#F79420"),
        ("Interact 10s ms", "#558AB5"),
        ("Toggle 6x ms", "#FAD55C"),
    ] {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk(renk)
                .dolgu(renk)
                .çizgi_kalınlığı(0.0),
        );
    }
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
            "orange"
        } else if i % 10 == 0 {
            "red"
        } else {
            "#33BB55"
        }
    });
    let veri = HizalıVeri::yeni(x, vec![değerler])?;
    let seri = SeriSeçenekleri::yeni("Value")
        .renk("#33BB55")
        .dolgu("#33BB55")
        .çizgi_kalınlığı(0.0)
        .çubuk_dolguları(dolgular);
    let mut seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri());
    if örnek == MultiBarsÖrneği::DeğişkenRenkler {
        seçenekler = seçenekler
            .kategoriler(('a'..='z').map(|harf| harf.to_string()))
            .çubuk_düzeni(ÇubukDüzeni::yeni(ÇubukYönü::Dikey).genişlik_oranı(0.5))
            .seri(seri);
    } else {
        seçenekler = seçenekler.seri(seri.çubuk(true).çubuk_boyutu(0.5, f32::INFINITY));
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn dört_kaynak_yüzeyi_çizilir() -> Result<(), UplotHatası> {
        for örnek in MultiBarsÖrneği::TÜMÜ {
            let (seçenekler, veri) = multi_bars_kartı(örnek)?;
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Dikdörtgen { .. })),
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
        assert_eq!(veri.seriler().len(), 6);
        assert!(
            veri.seriler()
                .get(4)
                .and_then(|seri| seri.get(2))
                .is_some_and(Option::is_none)
        );
        Ok(())
    }
}
