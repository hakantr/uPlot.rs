//! Stable Rust üzerinde çalışan tek iş parçacıklı GPUI Web girişi.

#[cfg(target_family = "wasm")]
mod web {
    use std::rc::Rc;

    use gpui::{App, AppContext as _, Application, WindowOptions};
    use uplot_rs::{Grafik, gpui::GpuiGrafik, resize_kartı};

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
        let grafik =
            resize_kartı(100).and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri));
        let Ok(grafik) = grafik else {
            web_hatası("Resize GPUI grafiği oluşturulamadı");
            return;
        };

        if cx
            .open_window(WindowOptions::default(), move |_, cx| {
                cx.new(|_| GpuiGrafik::yeni(grafik))
            })
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
