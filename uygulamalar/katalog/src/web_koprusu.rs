use super::KartKimliği;

#[cfg(any(target_family = "wasm", test))]
const KART_PARAMETRESİ: &str = "kart";

#[cfg(any(target_family = "wasm", test))]
pub(super) fn sorgudan_kart(sorgu: &str) -> Option<KartKimliği> {
    sorgu
        .trim_start_matches('?')
        .split('&')
        .filter_map(|parça| parça.split_once('='))
        .find_map(|(anahtar, değer)| {
            (anahtar == KART_PARAMETRESİ)
                .then(|| KartKimliği::slugdan(değer))
                .flatten()
        })
}

#[cfg(any(target_family = "wasm", test))]
fn kart_sorgusunu_yaz(sorgu: &str, slug: &str) -> String {
    let mut parçalar = sorgu
        .trim_start_matches('?')
        .split('&')
        .filter(|parça| !parça.is_empty())
        .filter(|parça| {
            parça
                .split_once('=')
                .map_or(*parça != KART_PARAMETRESİ, |(anahtar, _)| {
                    anahtar != KART_PARAMETRESİ
                })
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();
    parçalar.push(format!("{KART_PARAMETRESİ}={slug}"));
    format!("?{}", parçalar.join("&"))
}

#[cfg(target_family = "wasm")]
pub(super) fn başlangıç_kartı() -> Option<KartKimliği> {
    let sorgu = web_sys::window()?.location().search().ok()?;
    sorgudan_kart(&sorgu)
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn başlangıç_kartı() -> Option<KartKimliği> {
    None
}

#[cfg(target_family = "wasm")]
pub(super) fn kart_url_adresini_güncelle(kart: KartKimliği) -> Result<(), String> {
    use wasm_bindgen::JsValue;

    let pencere = web_sys::window().ok_or_else(|| "Window bulunamadı".to_string())?;
    let konum = pencere.location();
    let mevcut_sorgu = konum
        .search()
        .map_err(|hata| format!("URL sorgusu okunamadı: {hata:?}"))?;
    if sorgudan_kart(&mevcut_sorgu) == Some(kart) {
        return Ok(());
    }
    let sorgu = kart_sorgusunu_yaz(&mevcut_sorgu, kart.slug());
    let yol = konum
        .pathname()
        .map_err(|hata| format!("URL yolu okunamadı: {hata:?}"))?;
    let parça = konum
        .hash()
        .map_err(|hata| format!("URL parçası okunamadı: {hata:?}"))?;
    let url = format!("{yol}{sorgu}{parça}");
    pencere
        .history()
        .map_err(|hata| format!("Tarayıcı geçmişi açılamadı: {hata:?}"))?
        .replace_state_with_url(&JsValue::NULL, "", Some(&url))
        .map_err(|hata| format!("Kart URL'si güncellenemedi: {hata:?}"))
}

#[cfg(not(target_family = "wasm"))]
pub(super) fn kart_url_adresini_güncelle(_kart: KartKimliği) -> Result<(), String> {
    Ok(())
}

#[cfg(target_family = "wasm")]
pub(super) fn svg_indir(svg: &str, dosya_adı: &str) -> Result<(), String> {
    use js_sys::Array;
    use wasm_bindgen::{JsCast as _, JsValue};
    use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

    let parçalar = Array::new();
    parçalar.push(&JsValue::from_str(svg));
    let ayarlar = BlobPropertyBag::new();
    ayarlar.set_type("image/svg+xml;charset=utf-8");
    let blob = Blob::new_with_str_sequence_and_options(&parçalar, &ayarlar)
        .map_err(|hata| format!("SVG Blob oluşturulamadı: {hata:?}"))?;
    let nesne_url = Url::create_object_url_with_blob(&blob)
        .map_err(|hata| format!("SVG nesne URL'si oluşturulamadı: {hata:?}"))?;

    let sonuç = (|| {
        let belge = web_sys::window()
            .and_then(|pencere| pencere.document())
            .ok_or_else(|| "Document bulunamadı".to_string())?;
        let bağlantı = belge
            .create_element("a")
            .map_err(|hata| format!("İndirme bağlantısı oluşturulamadı: {hata:?}"))?
            .dyn_into::<HtmlAnchorElement>()
            .map_err(|_| "İndirme öğesi bağlantıya dönüştürülemedi".to_string())?;
        bağlantı.set_href(&nesne_url);
        bağlantı.set_download(dosya_adı);
        bağlantı.set_hidden(true);
        belge
            .body()
            .ok_or_else(|| "Document body bulunamadı".to_string())?
            .append_child(&bağlantı)
            .map_err(|hata| format!("İndirme bağlantısı DOM'a eklenemedi: {hata:?}"))?;
        bağlantı.click();
        bağlantı.remove();
        Ok(())
    })();

    Url::revoke_object_url(&nesne_url)
        .map_err(|hata| format!("SVG nesne URL'si bırakılamadı: {hata:?}"))?;
    sonuç
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn doğrudan_kart_sorgusu_geçerli_kartı_seçer() {
        assert!(
            sorgudan_kart("?tema=koyu&kart=gpui-svg-export") == Some(KartKimliği::GpuiSvgExport)
        );
        assert!(sorgudan_kart("?kart=line-resize") == Some(KartKimliği::Resize));
        assert!(sorgudan_kart("?kart=bilinmeyen").is_none());
    }

    #[test]
    fn kart_sorgusu_diğer_parametreleri_korur_ve_kartı_tekilleştirir() {
        assert_eq!(
            kart_sorgusunu_yaz("?tema=koyu&kart=resize&dpr=2", "sync-cursor"),
            "?tema=koyu&dpr=2&kart=sync-cursor"
        );
        assert_eq!(
            kart_sorgusunu_yaz("", "gpui-svg-export"),
            "?kart=gpui-svg-export"
        );
    }

    #[test]
    fn bütün_ana_kart_slugları_geri_dönüşümlüdür() {
        let sluglar = [
            "add-del-series",
            "align-data",
            "resize",
            "annotations",
            "area-fill",
            "scale-padding",
            "months",
            "months-ru",
            "nice-scale",
            "no-data",
            "path-gap-clip",
            "pixel-align",
            "points",
            "scales-dir-ori",
            "scatter",
            "scroll-sync",
            "sine-stream",
            "soft-minmax",
            "sparklines-bars",
            "sparklines",
            "sparse",
            "stacked-series",
            "stream-data",
            "gpui-svg-export",
            "sync-cursor",
            "sync-y-zero",
            "thin-bars-stroke-fill",
            "time-periods",
            "timeline-discrete",
            "timeseries-discrete",
            "timezones-dst",
            "tooltips-closest",
            "tooltips",
            "trendlines",
            "update-cursor-select-resize",
            "wind-direction",
            "y-scale-drag",
            "y-shifted-series",
            "cursor-bind",
            "cursor-snap",
            "cursor-tooltip",
            "custom-scales",
            "data-smoothing",
            "draw-hooks",
            "focus-cursor",
            "gradients",
            "grid-over-series",
            "high-low-bands",
            "latency-heatmap",
            "line-paths",
            "log-scales",
            "log-scales2",
            "mass-spectrum",
            "measure-datums",
            "nearest-non-null",
            "missing-data",
            "dependent-scale",
            "arcsinh-scales",
            "axis-control",
            "axis-autosize",
            "axis-indicators",
            "bars-grouped-stacked",
            "bars-values-autosize",
            "box-whisker",
            "candlestick-ohlc",
        ];
        for slug in sluglar {
            let çözülen = KartKimliği::slugdan(slug);
            assert!(çözülen.is_some());
            assert_eq!(çözülen.map(KartKimliği::slug), Some(slug));
        }
        for örnek in uplot_rs_gpui_ornekler::MultiBarsÖrneği::TÜMÜ {
            let kart = KartKimliği::MultiBars(örnek);
            assert_eq!(
                KartKimliği::slugdan(kart.slug()).map(KartKimliği::slug),
                Some(kart.slug())
            );
        }
    }
}
