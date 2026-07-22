//! uPlot'un küçük ve hızlı çizim modelini Rust'a taşıyan çekirdek.
//!
//! Çekirdek; GPUI'den bağımsız veri doğrulama, ölçekleme, etkileşim durumu,
//! çizim komutları ve SVG çıktısı sağlar. GPUI ve WASM doğrulama uygulamaları
//! yalnız platform olaylarını çekirdeğe ileten ayrı yüzey adaptörleridir.

#![cfg_attr(feature = "gpui", allow(confusable_idents))]

pub mod cizim;
mod etkilesim;
#[cfg(feature = "gpui")]
pub mod gpui;
pub mod grafik;
pub mod hata;
pub mod kart;
pub mod olcek;
pub mod secenek;
pub mod veri;

#[cfg(feature = "svg")]
pub mod svg;

pub use cizim::{Komut, MetinHizası, Nokta, Sahne};
pub use grafik::Grafik;
pub use hata::UplotHatası;
pub use kart::{
    AREA_FILL_KANIT_TOHUMU, AREA_FILL_KART_TANIM_ÖRNEĞİ, area_fill_kartı, ilk_kart,
    ilk_kart_etkileşimleri, sinüs_kartı, İLK_KART_TANIM_ÖRNEĞİ,
};
pub use olcek::Aralık;
pub use secenek::{
    EtkileşimSeçenekleri, GrafikSeçenekleri, SeriSeçenekleri, TekerlekAyarları, TekerlekKipi,
};
pub use veri::HizalıVeri;
