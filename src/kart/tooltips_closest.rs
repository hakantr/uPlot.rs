use serde::Deserialize;

use crate::{
    Aralık, EnYakınTooltipDüzeni, GrafikSeçenekleri, HizalıVeri, OdakDüzeni, SeriSeçenekleri,
    UplotHatası, ortak_kart_etkileşimleri,
};

pub const TOOLTIPS_CLOSEST_KART_TANIM_ÖRNEĞİ: &str = r#"let (seçenekler, veri) = tooltips_closest_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;
let bilgi = grafik.en_yakın_tooltip(yatay_oran, seri_indeksi);"#;

const RENKLER: [&str; 4] = ["#7cb5ec", "#434348", "#90ed7d", "#f7a35c"];

pub fn tooltips_closest_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak: Kaynak =
        serde_json::from_str(include_str!("veri/rustc-perf.json")).map_err(|hata| {
            UplotHatası::GeçersizVarlıkSatırı {
                varlık: "rustc-perf.json",
                satır: hata.line(),
            }
        })?;
    let Kaynak {
        benchmarks: Benchmarkler {
            summary: Summary { interpolated, opt },
        },
        commits,
    } = kaynak;
    let x = commits.iter().map(|(zaman, _)| *zaman).collect::<Vec<_>>();
    let seriler = vec![
        opt.full,
        opt.incr_unchanged,
        opt.incr_full,
        opt.incr_patched,
    ];
    let etiketler = [
        "full",
        "incr-unchanged",
        "incr-full",
        "incr-patched: println",
    ];
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Summary-opt")
        .x_ızgarası_göster(false)
        .y_eksen_etiketi("Value")
        .y_aralığı(Aralık::yeni(0.0, 1.4)?)
        .odak(OdakDüzeni::yeni(0.3, 5.0))
        .lejant_canlı(false)
        .en_yakın_tooltip(EnYakınTooltipDüzeni::yeni(
            commits.iter().map(|(_, commit)| commit.clone()).collect(),
            interpolated,
            "instructions:u",
        ))
        .etkileşimler(ortak_kart_etkileşimleri().imleç_bilgi_kutusu(true));
    for (indeks, etiket) in etiketler.into_iter().enumerate() {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk(RENKLER.get(indeks).copied().unwrap_or("#000000"))
                .noktaları_göster(false),
        );
    }
    let veri = HizalıVeri::yeni(
        x,
        seriler
            .into_iter()
            .map(|seri| seri.into_iter().map(Some).collect())
            .collect(),
    )?;
    Ok((seçenekler, veri))
}

#[derive(Deserialize)]
struct Kaynak {
    benchmarks: Benchmarkler,
    commits: Vec<(f64, String)>,
}

#[derive(Deserialize)]
struct Benchmarkler {
    #[serde(rename = "Summary")]
    summary: Summary,
}

#[derive(Deserialize)]
struct Summary {
    interpolated: Vec<usize>,
    #[serde(rename = "Opt")]
    opt: Opt,
}

#[derive(Deserialize)]
struct Opt {
    full: Vec<f64>,
    #[serde(rename = "incr-unchanged")]
    incr_unchanged: Vec<f64>,
    #[serde(rename = "incr-full")]
    incr_full: Vec<f64>,
    #[serde(rename = "incr-patched: println")]
    incr_patched: Vec<f64>,
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn rustc_perf_kaynak_verisi_ve_tooltip_bilgisi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = tooltips_closest_kartı()?;
        assert_eq!(veri.uzunluk(), 234);
        assert_eq!(veri.seriler().len(), 4);
        let grafik = crate::Grafik::yeni(seçenekler, veri)?;
        let bilgi = grafik.en_yakın_tooltip(0.0, 0);
        let Some(bilgi) = bilgi else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
        };
        assert_eq!(bilgi.commit.get(..10), Some("567ad7455d"));
        assert!(bilgi.karşılaştırma_url.contains("stat=instructions:u"));
        assert!(!bilgi.interpolasyon);
        assert!(bilgi.metin.contains("since start"));
        let svg = grafik.çiz().svg();
        assert_eq!(svg.matches("<circle").count(), 400);
        assert!(svg.contains("#fcb0f17a"));
        Ok(())
    }
}
