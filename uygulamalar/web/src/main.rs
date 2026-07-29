//! Stable Rust üzerinde çalışan tek iş parçacıklı GPUI Web girişi.

#[cfg(target_family = "wasm")]
mod web {
    use std::rc::Rc;
    use std::sync::Arc;

    use gpui::{App, AppContext as _, Application, WindowOptions};
    use gpui_web::WebPlatform;
    use ortak_bilesenler::baslat;
    use uplot_rs_gpui_katalog::{ChartListesi, başlat as katalog_başlat};

    /// `gpui_platform::single_threaded_web` ile aynı kurulum, doğrudan
    /// `gpui_web` üzerinden. Aradaki crate `gpui_web`'i varsayılan
    /// feature'larıyla çektiğinden `multithreaded` kapatılamıyordu; kabuk
    /// zaten tek iş parçacıklı yolu kullanıyor.
    fn single_threaded_web() -> Application {
        let platform = Rc::new(WebPlatform::new(false));
        let http_client = Arc::new(platform.fetch_http_client());
        Application::with_platform(platform).with_http_client(http_client)
    }

    /// `gpui_platform::web_init` karşılığı: panic kancası ve konsol günlüğü.
    fn web_init() {
        console_error_panic_hook::set_once();
        gpui_web::init_logging();
    }

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
        if let Err(hata) = baslat(uplot_rs_gpui_katalog::ortak_bileşen_ayarları(), cx) {
            web_hatası(&format!("Ortak GPUI bileşenleri başlatılamadı: {hata}"));
            return;
        }
        katalog_başlat(cx);

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
        if durum == "started"
            && let Some(durum_öğesi) = belge.get_element_by_id("boot-status")
        {
            durum_öğesi.remove();
        }
        if let Some(mesaj_öğesi) = belge.get_element_by_id("boot-message") {
            mesaj_öğesi.set_text_content(Some(mesaj));
            let _ = mesaj_öğesi
                .set_attribute("role", if durum == "failed" { "alert" } else { "status" });
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

#[cfg(test)]
mod testler {
    const WEB_KABUĞU: &str = include_str!("../index.html");

    #[test]
    fn linux_hatası_gpui_çözümünü_ve_yerel_uygulamayı_gösterir() {
        assert!(WEB_KABUĞU.contains(r#"id="boot-solutions""#));
        assert!(WEB_KABUĞU.contains("cargo run -p uplot-rs-chart-listesi --release"));
        assert!(WEB_KABUĞU.contains("actions/workflows/nightly-builds.yml"));
        assert!(WEB_KABUĞU.contains("vulkaninfo --summary"));
        assert!(WEB_KABUĞU.contains("glxinfo -B"));
    }

    #[test]
    fn web_kabuğu_ikinci_renderer_veya_svg_yedeği_sunmaz() {
        assert!(!WEB_KABUĞU.contains("svg-fallback"));
        assert!(!WEB_KABUĞU.contains("renderer=svg"));
        assert!(!WEB_KABUĞU.contains("GPU’suz SVG"));
    }
}
