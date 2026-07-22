#![allow(confusable_idents)]

use std::sync::Arc;

mod masaustu;

use gpui::{App, AppContext, Bounds, px, size};
use gpui_platform::application;
use masaustu::ChartListesi;
use ortak_bilesenler::{OrtakBilesenAyarlari, PencereKurulumAyarlari, baslat, pencere_secenekleri};

fn main() {
    application().run(|cx: &mut App| {
        if let Err(hata) = baslat(OrtakBilesenAyarlari::default(), cx) {
            eprintln!("UI kütüphanesi başlatılamadı: {hata}");
            cx.quit();
            return;
        }
        let başlangıç_geometrisi = Bounds::centered(None, size(px(1180.0), px(720.0)), cx);
        let mut seçenekler = pencere_secenekleri(
            cx,
            PencereKurulumAyarlari::yeni("uPlot.rs Charts").sinirlar(başlangıç_geometrisi),
        );
        seçenekler.app_id = Some("io.github.hakantr.uplot-rs".to_string());
        match uygulama_ikonu() {
            Ok(ikon) => seçenekler.icon = Some(ikon),
            Err(hata) => eprintln!("Uygulama ikonu yüklenemedi: {hata}"),
        }
        let sonuç = cx.open_window(seçenekler, |_, cx| cx.new(ChartListesi::yeni));
        if let Err(hata) = sonuç {
            eprintln!("Chart listesi açılamadı: {hata}");
            cx.quit();
            return;
        }
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.activate(true);
    });
}

fn uygulama_ikonu() -> Result<Arc<image::RgbaImage>, image::ImageError> {
    image::load_from_memory_with_format(
        include_bytes!("../../../assets/icons/uplot-rs.png"),
        image::ImageFormat::Png,
    )
    .map(|görsel| Arc::new(görsel.into_rgba8()))
}
