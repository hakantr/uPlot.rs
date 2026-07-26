//! Stable Rust üzerinde çalışan tek iş parçacıklı GPUI Web girişi.

#[cfg(target_family = "wasm")]
mod web {
    use gpui::{App, AppContext as _, WindowOptions};
    use gpui_platform::{single_threaded_web, web_init};
    use ortak_bilesenler::{OrtakBilesenAyarlari, baslat};
    use uplot_rs_gpui_katalog::ChartListesi;

    pub fn başlat() {
        web_init();
        gpui_uygulamasını_başlat();
    }

    fn gpui_uygulamasını_başlat() {
        web_durumunu_yaz("booting", "GPUI Web ve WebGPU hazırlanıyor…");
        let uygulama = single_threaded_web().run_embedded(uygulamayı_kur);
        // WebPlatform olay döngüsünü tarayıcı yönetir. Uygulama handle'ı sayfa
        // yaşamı boyunca korunmalıdır.
        std::mem::forget(uygulama);
    }

    fn uygulamayı_kur(cx: &mut App) {
        if let Err(hata) = baslat(OrtakBilesenAyarlari::default(), cx) {
            web_hatası(&format!("Ortak GPUI bileşenleri başlatılamadı: {hata}"));
            return;
        }

        if let Err(hata) =
            cx.open_window(WindowOptions::default(), |_, cx| cx.new(ChartListesi::yeni))
        {
            let mesaj = format!("GPUI Web penceresi açılamadı: {hata:#}");
            web_hatası(&mesaj);
            return;
        }
        cx.activate(true);
        web_durumunu_yaz("started", "");
    }

    fn web_hatası(mesaj: &str) {
        log::error!("{mesaj}");
        web_durumunu_yaz("failed", &format!("GPUI Web başlatılamadı: {mesaj}"));
    }

    fn web_durumunu_yaz(durum: &str, mesaj: &str) {
        let Some(belge) = web_sys::window().and_then(|pencere| pencere.document()) else {
            return;
        };
        if let Some(kök) = belge.document_element() {
            let _ = kök.set_attribute("data-gpui-uplot", durum);
        }
        if let Some(durum_öğesi) = belge.get_element_by_id("boot-status") {
            if durum == "started" {
                durum_öğesi.remove();
            } else {
                durum_öğesi.set_text_content(Some(mesaj));
                let _ = durum_öğesi
                    .set_attribute("role", if durum == "failed" { "alert" } else { "status" });
            }
        }
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
