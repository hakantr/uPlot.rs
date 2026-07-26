//! Stable Rust üzerinde çalışan tek iş parçacıklı GPUI Web girişi.

#[cfg(target_family = "wasm")]
mod web {
    use std::rc::Rc;

    use gpui::{App, AppContext as _, Application, WindowOptions};
    use ortak_bilesenler::{OrtakBilesenAyarlari, baslat};
    use uplot_rs_gpui_katalog::ChartListesi;

    pub fn başlat() {
        std::panic::set_hook(Box::new(|bilgi| {
            web_hatası(&format!("GPUI Web/WebGPU paniği: {bilgi}"));
            console_error_panic_hook::hook(bilgi);
        }));
        gpui_web::init_logging();
        web_durumunu_yaz("booting", "GPUI Web ve WebGPU hazırlanıyor…");

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

        if let Err(hata) =
            cx.open_window(WindowOptions::default(), |_, cx| cx.new(ChartListesi::yeni))
        {
            web_hatası(&format!(
                "GPUI Web/WebGPU penceresi açılamadı: {hata:#}. {}",
                tarayıcı_tanısı()
            ));
            return;
        }
        cx.activate(true);
        web_durumunu_yaz("started", "");
    }

    fn web_hatası(mesaj: &str) {
        log::error!("{mesaj}");
        web_durumunu_yaz("failed", &format!("GPUI Web başlatılamadı: {mesaj}"));
    }

    fn tarayıcı_tanısı() -> String {
        let Some(kök) = web_sys::window()
            .and_then(|pencere| pencere.document())
            .and_then(|belge| belge.document_element())
        else {
            return "Tarayıcı yetenekleri okunamadı.".to_string();
        };
        let özellik = |ad: &str| {
            kök.get_attribute(ad)
                .unwrap_or_else(|| "unknown".to_string())
        };
        let güvenli = özellik("data-secure-context");
        let webgpu = özellik("data-webgpu");
        let webgl2 = özellik("data-webgl2");
        let öneri = if güvenli != "true" {
            "WebGPU için HTTPS veya aynı cihazdaki localhost adresini kullanın."
        } else if webgpu != "true" && webgl2 != "true" {
            "Bu tarayıcı WebGPU veya WebGL2 sunmuyor; güncel Chrome/Edge ya da WebGPU destekli Safari kullanın."
        } else if webgpu != "true" {
            "WebGPU sunulmuyor; WebGL2 mevcut olsa da GPUI adaptörü kurulamadı."
        } else {
            "WebGPU sunuluyor fakat GPU adaptörü veya aygıtı oluşturulamadı; donanım hızlandırmasını ve tarayıcı GPU engel listesini denetleyin."
        };
        format!(
            "Tarayıcı tanısı: secureContext={güvenli}, WebGPU={webgpu}, WebGL2={webgl2}. {öneri}"
        )
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
