use serde::Deserialize;

use crate::{
    EnYakınTooltipDüzeni, GrafikSeçenekleri, HizalıVeri, OdakDüzeni, SeriSeçenekleri, UplotHatası,
    ortak_kart_etkileşimleri,
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
    let veri_en_çok = seriler
        .iter()
        .flatten()
        .copied()
        .filter(|değer| değer.is_finite())
        .max_by(f64::total_cmp)
        .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
    let y_aralığı = crate::Aralık::uplot_sayısal(0.0, veri_en_çok, 0.2, true)?;
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
        .y_aralığı(y_aralığı)
        .odak(OdakDüzeni::yeni(0.3, 5.0))
        .lejant_canlı(false)
        .en_yakın_tooltip(EnYakınTooltipDüzeni::yeni(
            commits.iter().map(|(_, commit)| commit.clone()).collect(),
            interpolated,
            "instructions:u",
        ))
        .etkileşimler(
            ortak_kart_etkileşimleri()
                .seçim_xy_yakınlaştır(true)
                .imleç_bilgi_kutusu(true),
        );
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
    use crate::Komut;

    #[test]
    fn rustc_perf_kaynak_verisi_ve_tooltip_bilgisi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = tooltips_closest_kartı()?;
        assert!(seçenekler.etkileşimler.seçim_xy_yakınlaştır);
        let y_aralığı = seçenekler
            .y_aralığı
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert_eq!(y_aralığı.en_az, 0.0);
        assert!((y_aralığı.en_çok - 1.5).abs() < 1e-12);
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
        assert!(bilgi.metin.ends_with("1 (0.00% since start)"));
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().any(|komut| {
            matches!(
                komut,
                Komut::Yol { parçalar, renk, .. }
                    if renk == "#fcb0f17a" && parçalar.len() == 100
            )
        }));
        let svg = sahne.test_svg();
        assert_eq!(svg.matches("<circle").count(), 0);
        assert_eq!(svg.matches("#fcb0f17a").count(), 1);
        Ok(())
    }

    #[test]
    fn gizli_seri_odak_ve_tooltip_adayı_olamaz() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = tooltips_closest_kartı()?;
        let mut grafik = crate::Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.imleç_odağını_seriye_ayarla(Some(0)));
        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert_eq!(grafik.odak_serisi(), None);
        assert!(!grafik.imleç_odağını_güncelle(0.0, 2.0 / 3.0, 600.0));
        assert_ne!(grafik.odak_serisi(), Some(0));
        Ok(())
    }

    #[test]
    fn tek_eksenli_xy_seçim_diğer_aralığı_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = tooltips_closest_kartı()?;
        let mut grafik = crate::Grafik::yeni(seçenekler, veri)?;
        let başlangıç_x = grafik.görünür_x_aralığı();
        let başlangıç_y = grafik.görünür_y_aralığı();
        assert!(grafik.fiziksel_seçim_yakınlaştır_eksenlerde(0.2, 0.5, 0.8, 0.5, true, false,)?);
        assert_eq!(grafik.görünür_y_aralığı(), başlangıç_y);
        let x = grafik.görünür_x_aralığı();
        assert!(x.en_çok - x.en_az < başlangıç_x.en_çok - başlangıç_x.en_az);
        Ok(())
    }
}
