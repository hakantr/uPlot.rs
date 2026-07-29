//! GPUI için küçük ve hızlı, retained zaman serisi grafik bileşeni.
//!
//! [`gpui::GpuiGrafik`] native ve GPUI Web uygulamalarında aynı veri, ölçek,
//! etkileşim ve retained çizim katmanlarını kullanır. `gpui-svg` feature'ı
//! grafik yüzeyini yalnız istendiğinde gerçek vektör SVG olarak kaydeder.

#![allow(confusable_idents)]

mod cizim;
mod etkilesim;
pub mod gpui;
pub mod grafik;
pub mod hata;
/// Etkileşim ve kare bütçesi tanılaması; `izleme` feature'ıyla açılır.
///
/// Varsayılan yapılandırmada modül boş karşılıklardan oluşur ve çağrılar
/// tümüyle elenir. Feature açıkken `UPLOT_IZLEME=1` ortam değişkeni
/// günlüğü çalıştırır. Kararlı API değildir.
#[cfg(feature = "izleme")]
pub mod izleme;
#[cfg(not(feature = "izleme"))]
#[path = "izleme_kapali.rs"]
pub mod izleme;
pub mod olcek;
pub mod secenek;
pub mod veri;
pub mod yuzey;
mod zaman;

pub use cizim::{DoğrusalGradyan, GradyanRenkDurağı, KöşeYarıçapları, MetinHizası, Nokta};
pub(crate) use cizim::{Komut, Sahne};
pub use gpui::{
    GpuiGrafik, GpuiGrafikGrubu, GpuiGrafikGrupAyarları, GpuiGrafikOlayı, GpuiSeriEşleme,
};
#[cfg(feature = "gpui-svg")]
pub use gpui::{GpuiSvgKaydı, GpuiSvgKayıtAyarları};
pub use grafik::{
    AçıklamaVuruşu, DağılımVuruşu, EksenHedefi, Grafik, NullAtlamaYönü, OransalGörünüm,
    SeriYaşamDöngüsüOlayı, SeçimEylemi, TimelineVuruşu, ZoomRangerDurumu,
    ZoomRangerSürüklemeEkseni, İmleçSeriÖrneği, İmleçÇözümü,
};
pub use hata::UplotHatası;
pub use olcek::{Aralık, SayısalAralıkAyarları, SayısalAralıkParçası, YumuşakSınırKipi};
pub use secenek::{
    AçıklamaDüzeni, AçıklamaHizası, AçıklamaStili, Açıklamaİşareti, BantYönü, BoyutSenkronDüzeni,
    DağılımDüzeni, DağılımNoktası, DağılımSerisi, EnYakınTooltipBilgisi, EnYakınTooltipDüzeni,
    EtkileşimSeçenekleri, GradyanDurağı, GradyanEkseni, GradyanKonumu, GrafikSeçenekleri,
    GüzelÖlçekDüzeni, IsıHaritasıDüzeni, IsıHücresi, IsıHücresiBoyutu, KutuBıyıkDüzeni,
    LejantKonumu, MumDüzeni, NoktaFiltreKipi, NoktaKatmanı, NoktaŞekli, NullİmleçDüzeni,
    OdakDüzeni, OdakStili, RüzgarYönüDüzeni, SeriBandı, SeriSeçenekleri, SeriÇizimTürü,
    TarihAdları, TekerlekAyarları, TekerlekEkseni, TekerlekKipi, TimelineDüzeni, TimelineHücresi,
    TooltipBilgisi, TooltipDüzeni, VARSAYILAN_KATMAN_SIRASI, XÖlçekDağılımı, YÖlçekDağılımı,
    YÖlçekDönüşümFn, YÖlçekEtiketBiçimi, YÖlçekSeçenekleri, ZamanDilimi, ZoomRangerSeçenekleri,
    ZoomSürüklemeKipi, ÇizimKancasıDüzeni, ÇizimKatmanı, ÇubukDüzeni, ÇubukYönü, ÖlçekGradyanı,
    ÖzelYÖlçekDönüşümü, İkincilXEksen, İmleçBağSeçenekleri,
};
pub use veri::{BoşlukKipi, HizalıDeğer, HizalıVeri, hizalı_verileri_birleştir};
pub use yuzey::{
    BilgiKutusuTarafı, BilgiKutusuYerleşimi, YüzeyDikdörtgeni, bilgi_kutusunu_yerleştir,
};

/// Tanılama, doğrulama ve özel dışa aktarım araçları için retained sahne görünümü.
///
/// [`Komut`] ve [`Sahne`] GPUI'ye alternatif, kararlı bir renderer backend API'si
/// değildir. Uygulamalar grafikleri [`Grafik`] ve [`gpui::GpuiGrafik`] üzerinden
/// oluşturmalıdır. Bu türler; testlerin üretilen geometriyi incelemesi, performans
/// ölçümlerinin retained listeyi sayması ve özel tanılama araçlarının sahne
/// dökümü alması için açıkça ayrılmıştır.
///
/// Komutların varyantları ve alanları semver kapsamındaki yüksek seviye grafik
/// seçeneklerinden daha sık değişebilir.
pub mod diagnostics {
    pub use crate::cizim::{Komut, Sahne};
    pub use crate::gpui::GpuiRetainedBoyaÖlçer;
}
