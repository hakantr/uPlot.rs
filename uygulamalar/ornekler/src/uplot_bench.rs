//! uPlot'un resmî karşılaştırma grafiği.
//!
//! Kaynak: `bench/uPlot.html` ve `bench/data.json`. uPlot bu veri setiyle on
//! iki rakip kütüphaneye karşı ölçüm yayınlıyor (`bench/results.json`,
//! `bench/table.md`, `perf.png`) ve README'de kendi sonucunu şöyle veriyor:
//! 47,9 KB paket, **34 ms ilk çizim**, 10 saniyelik fare hareketinde 218 ms
//! JS. Donanım README'de kayıtlı (Ryzen 7 PRO 5850U, Chrome 113, 1.5 dpr).
//!
//! Bu modül aynı veriyi aynı dönüşümle kurar ki bizim ölçümümüz onların
//! yayınladığı sayının karşılığı olsun. Katalog kartı değildir: uyum
//! sözleşmesi katalogda uPlot'un 73 demosunu birebir tutuyor, benchmark ise
//! demo değil. Yalnız performans bütçesinden çağrılır.

use std::sync::OnceLock;

use uplot_rs::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

use crate::ortak_kart_etkileşimleri;

const KAYNAK_JSON: &str = include_str!("veri/uplot_bench_dstat.json");
const VARLIK: &str = "bench/data.json";
/// `bench/uPlot.html` yorumu: 55.550 nokta × 3 seri = 166.650.
const BEKLENEN_NOKTA: usize = 55_550;

static KAYNAK_VERİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();

pub const UPLOT_BENCH_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = uplot_bench_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// uPlot'un benchmark sayfasındaki grafiğin birebir karşılığı.
pub fn uplot_bench_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = kaynak_veri()?;
    // `bench/uPlot.html` grafiği 3 seriyi tek Y ölçeğinde çiziyor; genişlik ve
    // yükseklik oradaki `width`/`height` değerleridir.
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("uPlot bench · 166.650 nokta")
        .x_zaman(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("CPU").renk("#a6cee3"))
        .seri(SeriSeçenekleri::yeni("RAM").renk("#b2df8a"))
        .seri(SeriSeçenekleri::yeni("TCP Out").renk("#33a02c"));
    Ok((seçenekler, veri))
}

fn kaynak_veri() -> Result<HizalıVeri, UplotHatası> {
    KAYNAK_VERİ.get_or_init(çöz).clone()
}

/// `bench/uPlot.html` içindeki `prepData` dönüşümünün birebir karşılığı.
///
/// Paket düzeni: `[alan_sayısı, ...alan_adları, ...kayıtlar]`. Alanlar
/// `epoch, idl, recv, send, writ, used, free`.
fn çöz() -> Result<HizalıVeri, UplotHatası> {
    let paket: Vec<serde_json::Value> =
        serde_json::from_str(KAYNAK_JSON).map_err(|hata| UplotHatası::GeçersizKaynakVeri {
            varlık: VARLIK,
            açıklama: format!("paket JSON olarak ayrıştırılamadı: {hata}"),
        })?;
    let alan_sayısı = paket
        .first()
        .and_then(serde_json::Value::as_u64)
        .and_then(|değer| usize::try_from(değer).ok())
        .filter(|değer| *değer > 0)
        .ok_or(UplotHatası::GeçersizKaynakVeri {
            varlık: VARLIK,
            açıklama: "ilk eleman alan sayısı olmalıydı".to_string(),
        })?;
    let gövde =
        paket
            .get(alan_sayısı.saturating_add(1)..)
            .ok_or(UplotHatası::GeçersizKaynakVeri {
                varlık: VARLIK,
                açıklama: "başlıktan sonra kayıt kalmadı".to_string(),
            })?;
    let kayıt_sayısı = gövde.len() / alan_sayısı;
    if kayıt_sayısı != BEKLENEN_NOKTA {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: VARLIK,
            açıklama: format!("{BEKLENEN_NOKTA} kayıt bekleniyordu, {kayıt_sayısı} bulundu"),
        });
    }

    let sayı = |değer: Option<&serde_json::Value>| -> Result<f64, UplotHatası> {
        değer
            .and_then(serde_json::Value::as_f64)
            .ok_or(UplotHatası::GeçersizKaynakVeri {
                varlık: VARLIK,
                açıklama: "kayıt alanı sayısal değil".to_string(),
            })
    };

    let mut x = Vec::with_capacity(kayıt_sayısı);
    let mut cpu = Vec::with_capacity(kayıt_sayısı);
    let mut ram = Vec::with_capacity(kayıt_sayısı);
    let mut tcp_out = Vec::with_capacity(kayıt_sayısı);
    for kayıt in gövde.chunks_exact(alan_sayısı) {
        let epoch = sayı(kayıt.first())?;
        let idl = sayı(kayıt.get(1))?;
        let send = sayı(kayıt.get(3))?;
        let used = sayı(kayıt.get(5))?;
        let free = sayı(kayıt.get(6))?;

        x.push(epoch * 60.0);
        cpu.push(Some(yuvarla(100.0 - idl, 3)));
        let toplam = used + free;
        ram.push(Some(if toplam == 0.0 {
            0.0
        } else {
            yuvarla(100.0 * used / toplam, 2)
        }));
        tcp_out.push(Some(send));
    }

    HizalıVeri::yeni(x, vec![cpu, ram, tcp_out])
}

/// `bench/uPlot.html` içindeki `round2`/`round3` yardımcılarının karşılığı.
fn yuvarla(değer: f64, basamak: u32) -> f64 {
    let çarpan = 10_f64.powi(basamak as i32);
    (değer * çarpan).round() / çarpan
}
