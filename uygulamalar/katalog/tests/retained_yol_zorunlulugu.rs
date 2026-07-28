//! Retained yol API'sinin zorunlu olup olmadığını ölçer.
//!
//! Fork, gpui'ye iki şey ekliyor: `Path.vertices`'in `Arc<Vec<_>>` olması ve
//! cihaz ölçeğine çevrilmiş yolu doğrudan gönderen `paint_scaled_path`.
//! Upstream yolu (`paint_path`) her karede `Path::scale` çağırıyor; o da
//! bütün köşeleri yeni bir `Vec`'e tahsis edip tek tek ölçekliyor.
//!
//! Ölçüm, gerçek bir kart geometrisinden üretilen yolla iki maliyeti
//! karşılaştırır: kare başına upstream ölçekleme, kare başına retained
//! gönderim.

use std::hint::black_box;
use std::time::Instant;

use gpui::{Path, PathBuilder, Pixels, point, px};

const TUR: u32 = 200;

/// Yoğun bir veri katmanı yolunu temsil eden çokgen zinciri.
fn yol_kur(nokta_sayısı: usize) -> Option<Path<Pixels>> {
    let mut kurucu = PathBuilder::stroke(px(1.0));
    kurucu.move_to(point(px(0.0), px(0.0)));
    for indeks in 1..nokta_sayısı {
        let x = indeks as f32 * 0.25;
        let y = ((indeks as f32) * 0.13).sin() * 180.0 + 200.0;
        kurucu.line_to(point(px(x), px(y)));
    }
    kurucu.build().ok()
}

#[test]
fn retained_yol_gonderimi_upstream_olceklemeden_ucuz() {
    // Ölçülen kartlarda yüzey başına ~32K tessellate edilmiş köşe görülüyor.
    let yol = yol_kur(8_000);
    assert!(yol.is_some(), "temsil edici yol kurulamadı");
    let Some(yol) = yol else { return };
    let köşe_sayısı = yol.vertices.len();
    assert!(
        köşe_sayısı > 1_000,
        "temsil edici bir yol bekleniyordu, {köşe_sayısı} köşe üretildi"
    );

    // Upstream yolu: her karede Path::scale, yani tam yeniden tahsis.
    let başlangıç = Instant::now();
    for _ in 0..TUR {
        black_box(yol.scale(2.0));
    }
    let upstream = başlangıç.elapsed() / TUR;

    // Retained yol: ölçekleme bir kez yapılır, kareler paylaşılan köşe
    // deposunun ucuz klonunu gönderir.
    let ölçekli = yol.scale(2.0);
    let başlangıç = Instant::now();
    for _ in 0..TUR {
        black_box(ölçekli.clone());
    }
    let retained = başlangıç.elapsed() / TUR;

    // `cached()` isabet ettiğinde gpui `Scene::replay` her primitive'i
    // klonlar. Upstream'de `vertices: Vec<_>` olduğundan bu derin kopyadır;
    // fork'ta `Arc<Vec<_>>` olduğu için sayaç artırımıdır.
    let başlangıç = Instant::now();
    for _ in 0..TUR {
        black_box(ölçekli.vertices.as_ref().clone());
    }
    let derin_kopya = başlangıç.elapsed() / TUR;

    let ns_başına_köşe = |süre: std::time::Duration| süre.as_secs_f64() * 1e9 / köşe_sayısı as f64;
    eprintln!("{köşe_sayısı} köşe · yol başına kare maliyeti:");
    eprintln!(
        "  upstream Path::scale   {upstream:>10?}  ({:.3} ns/köşe)",
        ns_başına_köşe(upstream)
    );
    eprintln!(
        "  upstream replay kopyası{derin_kopya:>10?}  ({:.3} ns/köşe)",
        ns_başına_köşe(derin_kopya)
    );
    eprintln!("  retained gönderim      {retained:>10?}  (paylaşımlı Arc)");

    // Ölçülen en ağır kartta saniyede ~3,08M köşe sunuluyor.
    let ölçülen_köşe_hızı = 3_080_000.0_f64;
    let upstream_yük =
        (ns_başına_köşe(upstream) + ns_başına_köşe(derin_kopya)) * ölçülen_köşe_hızı / 1e9;
    eprintln!(
        "  3,08M köşe/sn'de upstream yolun ek yükü: %{:.2} CPU",
        upstream_yük * 100.0
    );

    assert!(
        retained < upstream,
        "retained gönderim upstream ölçeklemeden ucuz olmalı: {retained:?} >= {upstream:?}"
    );
}
