//! Etkileşim izleme günlüğü.
//!
//! `UPLOT_IZLEME=1` ortam değişkeniyle açılır. Amacı, harici bir CPU
//! örnekleyicisinin çıktısıyla aynı zaman ekseninde "hangi işlem ne kadar
//! yük getirdi" eşlemesi yapabilmek: kart değişimi, kaydırma, pencere
//! boyutu ve saniyelik kök render özeti tek satırlık kayıtlar hâlinde
//! stdout'a basılır.
//!
//! Kapalıyken her giriş noktası tek bir `OnceLock` okumasına iner; sıcak
//! yolda ölçülebilir bir maliyeti yoktur.

use std::sync::{Mutex, OnceLock};

use web_time::Instant;

/// Bir kaydırma serisinin kapandığına karar verme eşiği.
const KAYDIRMA_SESSİZLİĞİ: f64 = 0.15;
/// Kesintisiz kaydırmada ara satır basma aralığı; uzun kaydırmalar tek
/// dev satıra sıkışmasın, saniyelik CPU örnekleriyle hizalanabilsin.
const KAYDIRMA_ARA_RAPOR: f64 = 0.5;
/// Kök render özetinin periyodu.
const KARE_ÖZET_PERİYODU: f64 = 1.0;
/// Kart listesi panelinin genişliği; kaydırmanın hangi bölgede olduğunu
/// ayırt etmek için kullanılır.
const LİSTE_GENİŞLİĞİ: f32 = 280.0;

static ETKİN: OnceLock<bool> = OnceLock::new();
static BAŞLANGIÇ: OnceLock<Instant> = OnceLock::new();
static DURUM: Mutex<Durum> = Mutex::new(Durum::yeni());

struct Durum {
    kaydırma: Option<Kaydırma>,
    kare_süreleri: Vec<f64>,
    kare_özet_zamanı: f64,
    pencere: Option<(i32, i32)>,
}

impl Durum {
    const fn yeni() -> Self {
        Self {
            kaydırma: None,
            kare_süreleri: Vec::new(),
            kare_özet_zamanı: 0.0,
            pencere: None,
        }
    }
}

struct Kaydırma {
    başlangıç: f64,
    son_olay: f64,
    dikey: f32,
    yatay: f32,
    olay: u32,
    bölge: &'static str,
    kart: &'static str,
}

/// İzleme açık mı? Ortam değişkeni bir kez okunur.
pub fn etkin() -> bool {
    *ETKİN.get_or_init(|| {
        std::env::var("UPLOT_IZLEME").is_ok_and(|değer| !değer.is_empty() && değer != "0")
    })
}

/// Zaman eksenini sıfırlar ve arka plandaki boşaltıcı iş parçacığını açar.
/// Birden çok kez çağrılırsa sonrakiler yok sayılır.
pub fn başlat() {
    if !etkin() || BAŞLANGIÇ.set(Instant::now()).is_err() {
        return;
    }
    yaz("BAŞLADI", "izleme açık · UPLOT_IZLEME=1");
    #[cfg(not(target_family = "wasm"))]
    std::thread::spawn(|| {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(100));
            zamanlayıcı();
        }
    });
}

/// Kart değişimini kaydeder.
pub fn kart_değişti(önceki: &'static str, yeni: &'static str) {
    if !etkin() {
        return;
    }
    kaydırmayı_kapat();
    yaz("KART", &format!("{önceki} → {yeni}"));
}

/// Serbest biçimli tek satırlık olay; açılır/kapanır paneller gibi ayrık
/// etkileşimler için.
pub fn olay(etiket: &str, ayrıntı: &str) {
    if !etkin() {
        return;
    }
    kaydırmayı_kapat();
    yaz(etiket, ayrıntı);
}

/// Tek bir tekerlek olayını biriktirir. `dikey`/`yatay` piksel cinsinden
/// ham delta, `x` fare konumunun pencere içindeki yatay bileşenidir.
pub fn kaydırma(dikey: f32, yatay: f32, x: f32, kart: &'static str) {
    if !etkin() {
        return;
    }
    let şimdi = geçen();
    let bölge = if x < LİSTE_GENİŞLİĞİ {
        "kart listesi"
    } else {
        "içerik"
    };
    let mut biten = None;
    if let Ok(mut durum) = DURUM.lock() {
        if durum.kaydırma.as_ref().is_some_and(|k| {
            şimdi - k.son_olay > KAYDIRMA_SESSİZLİĞİ
                || şimdi - k.başlangıç > KAYDIRMA_ARA_RAPOR
                || k.bölge != bölge
                || k.kart != kart
        }) {
            biten = durum.kaydırma.take();
        }
        let k = durum.kaydırma.get_or_insert(Kaydırma {
            başlangıç: şimdi,
            son_olay: şimdi,
            dikey: 0.0,
            yatay: 0.0,
            olay: 0,
            bölge,
            kart,
        });
        k.son_olay = şimdi;
        k.dikey += dikey;
        k.yatay += yatay;
        k.olay += 1;
    }
    if let Some(k) = biten {
        kaydırma_satırı(&k);
    }
}

/// Pencere boyutu değiştiyse kaydeder.
pub fn pencere_boyutu(genişlik: f32, yükseklik: f32) {
    if !etkin() {
        return;
    }
    let yeni = (genişlik.round() as i32, yükseklik.round() as i32);
    let önceki = {
        let Ok(mut durum) = DURUM.lock() else {
            return;
        };
        if durum.pencere == Some(yeni) {
            return;
        }
        durum.pencere.replace(yeni)
    };
    let (genişlik, yükseklik) = yeni;
    match önceki {
        Some((eski_g, eski_y)) => yaz(
            "PENCERE",
            &format!("{eski_g}×{eski_y} → {genişlik}×{yükseklik}"),
        ),
        None => yaz("PENCERE", &format!("{genişlik}×{yükseklik} (ilk)")),
    }
}

/// Kök render süresini ölçen kapsam koruyucusu; `Drop` anında birikime
/// yazar, saniyelik özeti zamanlayıcı basar.
pub struct KareÖlçümü(Option<Instant>);

impl KareÖlçümü {
    /// İzleme kapalıysa hiçbir şey ölçmez.
    pub fn başlat() -> Self {
        Self(etkin().then(Instant::now))
    }
}

impl Drop for KareÖlçümü {
    fn drop(&mut self) {
        if let Some(başlangıç) = self.0
            && let Ok(mut durum) = DURUM.lock()
        {
            durum.kare_süreleri.push(başlangıç.elapsed().as_secs_f64());
        }
    }
}

/// Sessizleşen kaydırmaları ve saniyelik kare özetini boşaltır.
fn zamanlayıcı() {
    let şimdi = geçen();
    let mut biten = None;
    let mut özet = None;
    if let Ok(mut durum) = DURUM.lock() {
        if durum
            .kaydırma
            .as_ref()
            .is_some_and(|k| şimdi - k.son_olay >= KAYDIRMA_SESSİZLİĞİ)
        {
            biten = durum.kaydırma.take();
        }
        if şimdi - durum.kare_özet_zamanı >= KARE_ÖZET_PERİYODU {
            let pencere = şimdi - durum.kare_özet_zamanı;
            durum.kare_özet_zamanı = şimdi;
            if !durum.kare_süreleri.is_empty() {
                özet = Some((std::mem::take(&mut durum.kare_süreleri), pencere));
            }
        }
    }
    if let Some(k) = biten {
        kaydırma_satırı(&k);
    }
    if let Some((mut süreler, pencere)) = özet {
        kare_satırı(&mut süreler, pencere);
    }
}

fn kaydırma_satırı(k: &Kaydırma) {
    let (yön, miktar) = if k.dikey.abs() >= k.yatay.abs() {
        // Kaydırma ofseti `[-azami, 0]` aralığına kırpılıyor: aşağı
        // kaydırma negatif delta üretir.
        (if k.dikey < 0.0 { "aşağı" } else { "yukarı" }, k.dikey)
    } else {
        (if k.yatay < 0.0 { "sağa" } else { "sola" }, k.yatay)
    };
    let süre = (k.son_olay - k.başlangıç).max(0.0);
    yaz(
        "KAYDIR",
        &format!(
            "{yön} {:.0}px · {} olay · {süre:.2}s · {} · {}",
            miktar.abs(),
            k.olay,
            k.bölge,
            k.kart
        ),
    );
}

fn kare_satırı(süreler: &mut [f64], pencere: f64) {
    süreler.sort_unstable_by(f64::total_cmp);
    let toplam: f64 = süreler.iter().sum();
    let yüzdelik = |oran: f64| {
        let indeks = ((süreler.len() - 1) as f64 * oran).round() as usize;
        süreler.get(indeks).copied().unwrap_or_default() * 1000.0
    };
    yaz(
        "KARE",
        &format!(
            "{} kök render / {pencere:.1}s · p50 {:.2}ms · p95 {:.2}ms · azami {:.2}ms · toplam {:.0}ms (%{:.1})",
            süreler.len(),
            yüzdelik(0.5),
            yüzdelik(0.95),
            yüzdelik(1.0),
            toplam * 1000.0,
            toplam / pencere * 100.0
        ),
    );
}

fn kaydırmayı_kapat() {
    let biten = DURUM
        .lock()
        .ok()
        .and_then(|mut durum| durum.kaydırma.take());
    if let Some(k) = biten {
        kaydırma_satırı(&k);
    }
}

fn geçen() -> f64 {
    BAŞLANGIÇ.get_or_init(Instant::now).elapsed().as_secs_f64()
}

fn yaz(etiket: &str, ayrıntı: &str) {
    println!("[izleme] +{:>8.3}s {etiket:<8} {ayrıntı}", geçen());
}
