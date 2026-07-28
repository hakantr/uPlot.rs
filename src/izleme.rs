//! Etkileşim ve kare bütçesi izleme günlüğü.
//!
//! `UPLOT_IZLEME=1` ortam değişkeniyle açılır. Amacı, harici bir CPU
//! örnekleyicisinin çıktısıyla aynı zaman ekseninde "hangi işlem ne kadar
//! yük getirdi" eşlemesi yapabilmek: etkileşim olayları ve kare içindeki
//! ölçüm yuvaları tek satırlık kayıtlar hâlinde stdout'a basılır.
//!
//! Kapalıyken her giriş noktası tek bir `OnceLock` okumasına iner; sıcak
//! yolda ölçülebilir bir maliyeti yoktur. Tanılama içindir; kararlı API
//! değildir.

use std::sync::{Mutex, OnceLock};

use web_time::Instant;

/// Bir kaydırma/fare serisinin kapandığına karar verme eşiği.
const SERİ_SESSİZLİĞİ: f64 = 0.15;
/// Kesintisiz seride ara satır basma aralığı; uzun hareketler tek dev
/// satıra sıkışmasın, saniyelik CPU örnekleriyle hizalanabilsin.
const SERİ_ARA_RAPOR: f64 = 0.5;
/// Kare özetlerinin periyodu.
const ÖZET_PERİYODU: f64 = 1.0;
/// Kart listesi panelinin genişliği; olayın hangi bölgede olduğunu ayırt
/// etmek için kullanılır.
const LİSTE_GENİŞLİĞİ: f32 = 280.0;

static ETKİN: OnceLock<bool> = OnceLock::new();
static BAŞLANGIÇ: OnceLock<Instant> = OnceLock::new();
static DURUM: Mutex<Durum> = Mutex::new(Durum::yeni());

/// Kare içinde ayrı ayrı raporlanan ölçüm noktaları.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Yuva {
    /// Uygulamanın kök görünümünün `render` çağrısı.
    KökRender,
    /// Tek bir grafik varlığının `render` çağrısı.
    GrafikRender,
    /// Odak değişimiyle tetiklenen veri sahnesi yeniden kurulumu.
    VeriSahnesi,
    /// Boyut/ölçek değişimiyle tetiklenen tam sahne yeniden kurulumu.
    TamSahne,
    /// Bir yüzeyin boyanması: yol tessellation'ı ve GPU komut üretimi.
    YüzeyBoyama,
}

impl Yuva {
    const SAYI: usize = 5;
    const HEPSİ: [Self; Self::SAYI] = [
        Self::KökRender,
        Self::GrafikRender,
        Self::VeriSahnesi,
        Self::TamSahne,
        Self::YüzeyBoyama,
    ];

    const fn indeks(self) -> usize {
        self as usize
    }

    const fn ad(self) -> &'static str {
        match self {
            Self::KökRender => "kök render",
            Self::GrafikRender => "grafik render",
            Self::VeriSahnesi => "veri sahnesi",
            Self::TamSahne => "tam sahne",
            Self::YüzeyBoyama => "yüzey boyama",
        }
    }
}

struct Durum {
    kaydırma: Option<Kaydırma>,
    fare: Option<Fare>,
    süreler: [Vec<f64>; Yuva::SAYI],
    fare_olayı: u32,
    fare_sahne_kurdu: u32,
    özet_zamanı: f64,
    pencere: Option<(i32, i32)>,
}

impl Durum {
    const fn yeni() -> Self {
        Self {
            kaydırma: None,
            fare: None,
            süreler: [const { Vec::new() }; Yuva::SAYI],
            fare_olayı: 0,
            fare_sahne_kurdu: 0,
            özet_zamanı: 0.0,
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

struct Fare {
    başlangıç: f64,
    son_olay: f64,
    mesafe: f32,
    olay: u32,
    konum: (f32, f32),
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

/// Rasterleştirici bilgisini bir kez kaydeder. Yazılım fallback'i (llvmpipe
/// gibi) canlı kare süresini başsız ölçümlerin çok üstüne çıkardığı için
/// günlüğün başında görünmesi gerekiyor.
pub fn gpu_bilgisi(ayrıntı: &str) {
    static YAZILDI: OnceLock<()> = OnceLock::new();
    if !etkin() || YAZILDI.set(()).is_err() {
        return;
    }
    yaz("GPU", ayrıntı);
}

/// Kart değişimini kaydeder.
pub fn kart_değişti(önceki: &'static str, yeni: &'static str) {
    if !etkin() {
        return;
    }
    serileri_kapat();
    yaz("KART", &format!("{önceki} → {yeni}"));
}

/// Serbest biçimli tek satırlık olay; açılır/kapanır paneller gibi ayrık
/// etkileşimler için.
pub fn olay(etiket: &str, ayrıntı: &str) {
    if !etkin() {
        return;
    }
    serileri_kapat();
    yaz(etiket, ayrıntı);
}

/// Tek bir tekerlek olayını biriktirir. `dikey`/`yatay` piksel cinsinden
/// ham delta, `x` fare konumunun pencere içindeki yatay bileşenidir.
pub fn kaydırma(dikey: f32, yatay: f32, x: f32, kart: &'static str) {
    if !etkin() {
        return;
    }
    let şimdi = geçen();
    let bölge = bölge_adı(x);
    let mut biten = None;
    if let Ok(mut durum) = DURUM.lock() {
        if durum.kaydırma.as_ref().is_some_and(|k| {
            şimdi - k.son_olay > SERİ_SESSİZLİĞİ
                || şimdi - k.başlangıç > SERİ_ARA_RAPOR
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

/// Fare hareketini biriktirir. Grafik yüzeyleri üzerindeki imleç takibi
/// etkileşim katmanını her harekette yeniden kurduğu için, kaydırmadan
/// bağımsız olarak ayrı izlenmesi gerekiyor.
pub fn fare_hareketi(x: f32, y: f32, kart: &'static str) {
    if !etkin() {
        return;
    }
    let şimdi = geçen();
    let bölge = bölge_adı(x);
    let mut biten = None;
    if let Ok(mut durum) = DURUM.lock() {
        if durum.fare.as_ref().is_some_and(|f| {
            şimdi - f.son_olay > SERİ_SESSİZLİĞİ
                || şimdi - f.başlangıç > SERİ_ARA_RAPOR
                || f.bölge != bölge
                || f.kart != kart
        }) {
            biten = durum.fare.take();
        }
        let f = durum.fare.get_or_insert(Fare {
            başlangıç: şimdi,
            son_olay: şimdi,
            mesafe: 0.0,
            olay: 0,
            konum: (x, y),
            bölge,
            kart,
        });
        f.mesafe += (x - f.konum.0).hypot(y - f.konum.1);
        f.konum = (x, y);
        f.son_olay = şimdi;
        f.olay += 1;
    }
    if let Some(f) = biten {
        fare_satırı(&f);
    }
}

/// Fare düğmesi olayları; sürükleme pencerelerini görünür kılar.
pub fn fare_düğmesi(basıldı: bool, düğme: &str, x: f32, kart: &'static str) {
    if !etkin() {
        return;
    }
    serileri_kapat();
    yaz(
        if basıldı { "BASTI" } else { "BIRAKTI" },
        &format!("{düğme} · {} · {kart}", bölge_adı(x)),
    );
}

/// Bir fare hareketinin ana veri sahnesini yeniden kurdurup kurmadığını
/// sayar. Oran yüksekse imleç takibi her harekette tüm veri yollarını
/// yeniden tessellate ettiriyor demektir.
pub fn fare_sahne_kararı(sahne_kuruldu: bool) {
    if !etkin() {
        return;
    }
    if let Ok(mut durum) = DURUM.lock() {
        durum.fare_olayı = durum.fare_olayı.saturating_add(1);
        if sahne_kuruldu {
            durum.fare_sahne_kurdu = durum.fare_sahne_kurdu.saturating_add(1);
        }
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

/// Bir ölçüm yuvasının süresini tutan kapsam koruyucusu; `Drop` anında
/// birikime yazar, saniyelik özeti zamanlayıcı basar.
pub struct Ölçüm(Option<(Yuva, Instant)>);

impl Ölçüm {
    /// İzleme kapalıysa hiçbir şey ölçmez.
    pub fn başlat(yuva: Yuva) -> Self {
        Self(etkin().then(|| (yuva, Instant::now())))
    }
}

impl Drop for Ölçüm {
    fn drop(&mut self) {
        if let Some((yuva, başlangıç)) = self.0
            && let Ok(mut durum) = DURUM.lock()
            && let Some(birikim) = durum.süreler.get_mut(yuva.indeks())
        {
            birikim.push(başlangıç.elapsed().as_secs_f64());
        }
    }
}

/// Sessizleşen serileri ve saniyelik özetleri boşaltır.
fn zamanlayıcı() {
    let şimdi = geçen();
    let mut biten_kaydırma = None;
    let mut biten_fare = None;
    let mut özet = None;
    if let Ok(mut durum) = DURUM.lock() {
        if durum
            .kaydırma
            .as_ref()
            .is_some_and(|k| şimdi - k.son_olay >= SERİ_SESSİZLİĞİ)
        {
            biten_kaydırma = durum.kaydırma.take();
        }
        if durum
            .fare
            .as_ref()
            .is_some_and(|f| şimdi - f.son_olay >= SERİ_SESSİZLİĞİ)
        {
            biten_fare = durum.fare.take();
        }
        if şimdi - durum.özet_zamanı >= ÖZET_PERİYODU {
            let pencere = şimdi - durum.özet_zamanı;
            durum.özet_zamanı = şimdi;
            let süreler = std::mem::replace(&mut durum.süreler, [const { Vec::new() }; Yuva::SAYI]);
            let fare = (durum.fare_olayı, durum.fare_sahne_kurdu);
            durum.fare_olayı = 0;
            durum.fare_sahne_kurdu = 0;
            if süreler.iter().any(|birikim| !birikim.is_empty()) || fare.0 > 0 {
                özet = Some((süreler, fare, pencere));
            }
        }
    }
    if let Some(k) = biten_kaydırma {
        kaydırma_satırı(&k);
    }
    if let Some(f) = biten_fare {
        fare_satırı(&f);
    }
    if let Some((mut süreler, (fare_olayı, sahne_kurdu), pencere)) = özet {
        for yuva in Yuva::HEPSİ {
            if let Some(birikim) = süreler.get_mut(yuva.indeks())
                && !birikim.is_empty()
            {
                yuva_satırı(yuva, birikim, pencere);
            }
        }
        if fare_olayı > 0 {
            yaz(
                "ODAK",
                &format!(
                    "{fare_olayı} fare olayı · {sahne_kurdu} tanesinde veri sahnesi yeniden kuruldu (%{:.1})",
                    f64::from(sahne_kurdu) / f64::from(fare_olayı) * 100.0
                ),
            );
        }
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

fn fare_satırı(f: &Fare) {
    let süre = (f.son_olay - f.başlangıç).max(0.0);
    yaz(
        "FARE",
        &format!(
            "{:.0}px yol · {} olay · {süre:.2}s · {} · {}",
            f.mesafe, f.olay, f.bölge, f.kart
        ),
    );
}

fn yuva_satırı(yuva: Yuva, süreler: &mut [f64], pencere: f64) {
    süreler.sort_unstable_by(f64::total_cmp);
    let toplam: f64 = süreler.iter().sum();
    let yüzdelik = |oran: f64| {
        let indeks = (süreler.len().saturating_sub(1) as f64 * oran).round() as usize;
        süreler.get(indeks).copied().unwrap_or_default() * 1000.0
    };
    yaz(
        "KARE",
        &format!(
            "{:14} {} kez / {pencere:.1}s · p50 {:.2}ms · p95 {:.2}ms · azami {:.2}ms · toplam {:.0}ms (%{:.1})",
            yuva.ad(),
            süreler.len(),
            yüzdelik(0.5),
            yüzdelik(0.95),
            yüzdelik(1.0),
            toplam * 1000.0,
            toplam / pencere * 100.0
        ),
    );
}

fn serileri_kapat() {
    let (kaydırma, fare) = DURUM
        .lock()
        .map(|mut durum| (durum.kaydırma.take(), durum.fare.take()))
        .unwrap_or((None, None));
    if let Some(k) = kaydırma {
        kaydırma_satırı(&k);
    }
    if let Some(f) = fare {
        fare_satırı(&f);
    }
}

fn bölge_adı(x: f32) -> &'static str {
    if x < LİSTE_GENİŞLİĞİ {
        "kart listesi"
    } else {
        "içerik"
    }
}

fn geçen() -> f64 {
    BAŞLANGIÇ.get_or_init(Instant::now).elapsed().as_secs_f64()
}

fn yaz(etiket: &str, ayrıntı: &str) {
    println!("[izleme] +{:>8.3}s {etiket:<8} {ayrıntı}", geçen());
}
