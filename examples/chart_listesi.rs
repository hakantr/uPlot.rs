use gpui::{App, AppContext, Bounds, px, size};
use gpui_platform::application;
use ortak_bilesenler::{OrtakBilesenAyarlari, PencereKurulumAyarlari, baslat, pencere_secenekleri};
use uplot_rs::masaustu::ChartListesi;

fn main() {
    application().run(|cx: &mut App| {
        if let Err(hata) = baslat(OrtakBilesenAyarlari::default(), cx) {
            eprintln!("UI kütüphanesi başlatılamadı: {hata}");
            cx.quit();
            return;
        }
        let başlangıç_geometrisi = Bounds::centered(None, size(px(1180.0), px(720.0)), cx);
        let seçenekler = pencere_secenekleri(
            cx,
            PencereKurulumAyarlari::yeni("uPlot.rs Charts").sinirlar(başlangıç_geometrisi),
        );
        let sonuç = cx.open_window(seçenekler, |_, cx| cx.new(ChartListesi::yeni));
        if let Err(hata) = sonuç {
            eprintln!("Chart listesi açılamadı: {hata}");
            std::process::exit(1);
        }
        cx.on_window_closed(|cx, _| cx.quit()).detach();
        cx.activate(true);
    });
}
