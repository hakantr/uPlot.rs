//! Stable Rust üzerinde çalışan tek iş parçacıklı GPUI Web girişi.

#[cfg(target_family = "wasm")]
mod web {
    use std::rc::Rc;

    use gpui::{App, AppContext as _, Application, WindowOptions};
    use ortak_bilesenler::{OrtakBilesenAyarlari, baslat};
    use uplot_rs_gpui_katalog::ChartListesi;

    pub fn başlat() {
        console_error_panic_hook::set_once();
        gpui_web::init_logging();

        let uygulama = Application::with_platform(Rc::new(gpui_web::WebPlatform::new(false)))
            .run_embedded(uygulamayı_kur);
        // WebPlatform olay döngüsünü tarayıcı yönetir. Uygulama handle'ı sayfa
        // yaşamı boyunca korunmalıdır.
        std::mem::forget(uygulama);
    }

    fn uygulamayı_kur(cx: &mut App) {
        if let Err(hata) = baslat(OrtakBilesenAyarlari::default(), cx) {
            web_hatası(&format!("Ortak GPUI bileşenleri başlatılamadı: {hata}"));
            return;
        }

        if cx
            .open_window(WindowOptions::default(), |_, cx| cx.new(ChartListesi::yeni))
            .is_err()
        {
            web_hatası("GPUI Web penceresi açılamadı");
            return;
        }
        cx.activate(true);
    }

    fn web_hatası(mesaj: &str) {
        log::error!("{mesaj}");
    }
}

#[cfg(target_family = "wasm")]
fn main() {
    web::başlat();
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    eprintln!("Bu uygulama wasm32-unknown-unknown hedefi için hazırlanmıştır.");
}
