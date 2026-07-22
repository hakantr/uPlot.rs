//! uPlot'un küçük ve hızlı çizim modelini Rust'a taşıyan çekirdek.
//!
//! İlk altyapı dilimi, GPUI'den bağımsız veri doğrulama, ölçekleme, çizim
//! komutları ve SVG çıktısı sağlar. GPUI yüzeyi aynı sahne komutlarını tüketen
//! ayrı bir adaptör olarak sonraki fazda eklenecektir.

pub mod cizim;
pub mod grafik;
pub mod hata;
pub mod kart;
#[cfg(feature = "desktop")]
pub mod masaustu;
pub mod olcek;
pub mod secenek;
pub mod veri;

pub use cizim::{Komut, MetinHizası, Nokta, Sahne};
pub use grafik::Grafik;
pub use hata::UplotHatası;
pub use kart::{ilk_kart, sinüs_kartı};
pub use olcek::Aralık;
pub use secenek::{GrafikSeçenekleri, SeriSeçenekleri};
pub use veri::HizalıVeri;
