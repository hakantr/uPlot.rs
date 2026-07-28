//! `izleme` feature'ı kapalıyken devreye giren boş karşılık.
//!
//! Tanılama günlüğü kütüphanenin varsayılan yüzeyinde yer almaz; çağrı
//! noktaları `src/gpui.rs` içinde `#[cfg]` kalabalığı yaratmasın diye
//! imzalar burada birebir korunur ve gövdeler boşaltılır. Derleyici bu
//! çağrıları tümüyle eler.
//!
//! Feature açıkken gerçek uygulama [`crate::izleme`] modülündedir.

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

/// Ölçüm kapsamının boş karşılığı.
pub struct Ölçüm;

impl Ölçüm {
    /// Feature kapalıyken hiçbir şey ölçmez.
    pub const fn başlat(_yuva: Yuva) -> Self {
        Self
    }
}

/// Feature kapalıyken izleme daima kapalıdır.
pub const fn etkin() -> bool {
    false
}

/// Zaman eksenini başlatmanın boş karşılığı.
pub const fn başlat() {}

/// Rasterleştirici bilgisi kaydının boş karşılığı.
pub fn gpu_bilgisi(_ayrıntı: &str) {}

/// Kart değişimi kaydının boş karşılığı.
pub fn kart_değişti(_önceki: &'static str, _yeni: &'static str) {}

/// Serbest biçimli olay kaydının boş karşılığı.
pub fn olay(_etiket: &str, _ayrıntı: &str) {}

/// Tekerlek olayı birikiminin boş karşılığı.
pub fn kaydırma(_dikey: f32, _yatay: f32, _x: f32, _kart: &'static str) {}

/// Fare hareketi birikiminin boş karşılığı.
pub fn fare_hareketi(_x: f32, _y: f32, _kart: &'static str) {}

/// Fare düğmesi kaydının boş karşılığı.
pub fn fare_düğmesi(_basıldı: bool, _düğme: &str, _x: f32, _kart: &'static str) {}

/// Yol/köşe sayacının boş karşılığı.
pub const fn yol_boyandı(_köşe: usize) {}

/// Odak kararı sayacının boş karşılığı.
pub const fn fare_sahne_kararı(_sahne_kuruldu: bool) {}

/// Pencere boyutu kaydının boş karşılığı.
pub fn pencere_boyutu(_genişlik: f32, _yükseklik: f32) {}
