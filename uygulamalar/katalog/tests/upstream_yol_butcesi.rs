//! Upstream yol gönderiminin kare başına maliyetini bütçeye bağlar.
//!
//! Veri katmanı yolları kareler arası saklanıyor, ama cihaz ölçeklemesi
//! `Window::paint_path` içinde her gönderimde tekrarlanıyor ve `cached()`
//! alt ağacı yeniden kullanıldığında `Scene::replay` köşe vektörünü derin
//! kopyalıyor. İkisi de köşe sayısıyla doğrusal.
//!
//! Bu maliyet bilinçli olarak kabul edildi: gpui'ye ölçeklenmiş yolu
//! paylaşımlı gönderen bir API eklemek onu neredeyse sıfıra indiriyordu,
//! ama ölçülen kazanç en ağır yükte ~%0,6 CPU'da kaldı ve her upstream
//! senkronunda fork'u yeniden harmanlamanın bakım yükünü karşılamadı.
//! Test, kabul edilen maliyetin sessizce büyümesini engeller.
//!
//! Karar 2026-07-30'da bu makinede yeniden ölçüldü: 47.994 köşede
//! `Path::scale` 0,560 ns/köşe, replay kopyası 0,550 ns/köşe, en ağır
//! yükte toplam pay **%0,34 CPU**. gpui'de bilinçli sapma artık koşullu
//! olarak mümkün (`../gpui/AGENTS.md`), ama o sürecin ilk koşulu gerçek
//! bir sınır olması; %0,34 CPU sınır değil, kabul edilmiş bir maliyettir.

use std::hint::black_box;
use std::time::Instant;

use gpui::{Path, PathBuilder, Pixels, point, px};

const TUR: u32 = 200;
/// Köşe başına kabul edilen üst sınır. Ölçülen değerler ~0,9 ns; sınır
/// gürültüye yer bırakacak kadar geniş, bir büyüklük sıçramasını yakalayacak
/// kadar dar.
const KÖŞE_BAŞINA_AZAMİ_NS: f64 = 4.0;
/// Canlı ölçümde en ağır kartın saniyede sunduğu köşe sayısı.
const ÖLÇÜLEN_KÖŞE_HIZI: f64 = 3_080_000.0;

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
fn upstream_yol_gonderimi_kare_butcesinde_kalir() {
    // Bütçe optimize edilmiş kod için konuldu; `cargo test` varsayılanı olan
    // debug profilinde aynı döngü ~30 kat yavaş ölçülüyor ve test gerçek bir
    // gerileme olmadan patlıyordu. Ölçüm yalnız release profilinde anlamlı.
    if cfg!(debug_assertions) {
        eprintln!("debug profili: kare bütçesi ölçümü atlandı (release ile çalıştırın)");
        return;
    }
    let yol = yol_kur(8_000);
    assert!(yol.is_some(), "temsil edici yol kurulamadı");
    let Some(yol) = yol else { return };
    let köşe_sayısı = yol.vertices.len();
    assert!(
        köşe_sayısı > 1_000,
        "temsil edici bir yol bekleniyordu, {köşe_sayısı} köşe üretildi"
    );

    // `Window::paint_path` her gönderimde bunu yapıyor.
    let başlangıç = Instant::now();
    for _ in 0..TUR {
        black_box(yol.scale(2.0));
    }
    let ölçekleme = başlangıç.elapsed() / TUR;

    // `cached()` isabet ettiğinde `Scene::replay` primitive'i klonluyor.
    let ölçekli = yol.scale(2.0);
    let başlangıç = Instant::now();
    for _ in 0..TUR {
        black_box(ölçekli.clone());
    }
    let replay_kopyası = başlangıç.elapsed() / TUR;

    let ns_başına_köşe = |süre: std::time::Duration| süre.as_secs_f64() * 1e9 / köşe_sayısı as f64;
    let ölçekleme_ns = ns_başına_köşe(ölçekleme);
    let kopya_ns = ns_başına_köşe(replay_kopyası);
    let cpu_payı = (ölçekleme_ns + kopya_ns) * ÖLÇÜLEN_KÖŞE_HIZI / 1e9;

    eprintln!("{köşe_sayısı} köşe · yol başına kare maliyeti:");
    eprintln!("  Path::scale      {ölçekleme:>10?}  ({ölçekleme_ns:.3} ns/köşe)");
    eprintln!("  replay kopyası   {replay_kopyası:>10?}  ({kopya_ns:.3} ns/köşe)");
    eprintln!(
        "  {ÖLÇÜLEN_KÖŞE_HIZI:.0} köşe/sn'de toplam pay: %{:.2} CPU",
        cpu_payı * 100.0
    );

    assert!(
        ölçekleme_ns <= KÖŞE_BAŞINA_AZAMİ_NS,
        "Path::scale köşe başına {ölçekleme_ns:.3} ns, bütçe {KÖŞE_BAŞINA_AZAMİ_NS} ns"
    );
    assert!(
        kopya_ns <= KÖŞE_BAŞINA_AZAMİ_NS,
        "replay kopyası köşe başına {kopya_ns:.3} ns, bütçe {KÖŞE_BAŞINA_AZAMİ_NS} ns"
    );
}
