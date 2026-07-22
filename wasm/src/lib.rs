//! Tarayıcı chart listesinin WASM köprüsü.

use uplot_rs::{
    Aralık, Grafik, ilk_kart_etkileşimleri, sinüs_kartı, İLK_KART_TANIM_ÖRNEĞİ
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn ilk_kart_svg(nokta_sayısı: usize) -> String {
    let sonuç = sinüs_kartı(nokta_sayısı)
        .and_then(|(seçenekler, veri)| Grafik::yeni(seçenekler, veri))
        .map(|grafik| grafik.çiz().svg());
    sonuç.unwrap_or_else(|hata| hata_svg(&hata.to_string()))
}

#[wasm_bindgen]
pub fn ilk_kart_svg_aralik(
    nokta_sayısı: usize,
    genişlik: u32,
    yükseklik: u32,
    x_en_az: f64,
    x_en_çok: f64,
) -> String {
    let sonuç = sinüs_kartı(nokta_sayısı).and_then(|(seçenekler, veri)| {
        let aralık = Aralık::yeni(x_en_az, x_en_çok)?;
        Grafik::yeni(seçenekler, veri)
            .map(|grafik| grafik.çiz_boyutta(genişlik, yükseklik, Some(aralık)).svg())
    });
    sonuç.unwrap_or_else(|hata| hata_svg(&hata.to_string()))
}

#[wasm_bindgen]
pub fn tekerlek_x_araligi(
    mevcut_en_az: f64,
    mevcut_en_çok: f64,
    tam_en_az: f64,
    tam_en_çok: f64,
    odak: f64,
    yakınlaştır: bool,
) -> Result<Vec<f64>, JsValue> {
    let mevcut = Aralık::yeni(mevcut_en_az, mevcut_en_çok)
        .map_err(|hata| JsValue::from_str(&hata.to_string()))?;
    let tam =
        Aralık::yeni(tam_en_az, tam_en_çok).map_err(|hata| JsValue::from_str(&hata.to_string()))?;
    mevcut
        .tekerlek_yakınlaştır(tam, odak, yakınlaştır)
        .map(|aralık| vec![aralık.en_az, aralık.en_çok])
        .map_err(|hata| JsValue::from_str(&hata.to_string()))
}

#[wasm_bindgen]
pub fn uyarlanabilir_tekerlek_x_araligi(
    mevcut_en_az: f64,
    mevcut_en_çok: f64,
    tam_en_az: f64,
    tam_en_çok: f64,
    odak: f64,
    delta: f64,
    hassas_girdi: bool,
) -> Result<Vec<f64>, JsValue> {
    let mevcut = Aralık::yeni(mevcut_en_az, mevcut_en_çok)
        .map_err(|hata| JsValue::from_str(&hata.to_string()))?;
    let tam =
        Aralık::yeni(tam_en_az, tam_en_çok).map_err(|hata| JsValue::from_str(&hata.to_string()))?;
    mevcut
        .uyarlanabilir_tekerlek_yakınlaştır(
            tam,
            odak,
            delta,
            hassas_girdi,
            ilk_kart_etkileşimleri().tekerlek_ayarları,
        )
        .map(|aralık| vec![aralık.en_az, aralık.en_çok])
        .map_err(|hata| JsValue::from_str(&hata.to_string()))
}

#[wasm_bindgen]
pub fn kart_sayisi() -> usize {
    1
}

#[wasm_bindgen]
pub fn ilk_kart_tanim_ornegi() -> String {
    İLK_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn ilk_kart_tekerlek_etkilesimi() -> bool {
    ilk_kart_etkileşimleri().tekerlek_etkileşimi
}

#[wasm_bindgen]
pub fn ilk_kart_tekerlek_hareket_birlestirme_ms() -> u32 {
    ilk_kart_etkileşimleri()
        .tekerlek_ayarları
        .hareket_birleştirme_ms
        .min(u64::from(u32::MAX)) as u32
}

#[wasm_bindgen]
pub fn ilk_kart_secim_yakinlastir() -> bool {
    ilk_kart_etkileşimleri().seçim_yakınlaştır
}

#[wasm_bindgen]
pub fn ilk_kart_cift_tikla_tam_gorunum() -> bool {
    ilk_kart_etkileşimleri().çift_tıkla_tam_görünüm
}

#[wasm_bindgen]
pub fn ilk_kart_gorunum_gecmisi() -> bool {
    ilk_kart_etkileşimleri().görünüm_geçmişi
}

#[wasm_bindgen]
pub fn kaynak_commit() -> String {
    "0e5812c504430f5c804e0f993376d8999b26cc34".to_string()
}

fn hata_svg(metin: &str) -> String {
    let güvenli = metin
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"800\" height=\"400\"><rect width=\"100%\" height=\"100%\" fill=\"#fff\"/><text x=\"24\" y=\"48\" fill=\"#b91c1c\">{güvenli}</text></svg>"
    )
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn ilk_kart_wasm_svg_üretir() {
        let svg = ilk_kart_svg(100);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("İlk kart · sin(x)"));
        assert_eq!(kart_sayisi(), 1);
        assert!(ilk_kart_tanim_ornegi().contains("GrafikSeçenekleri::yeni"));

        let yakın = ilk_kart_svg_aralik(100, 800, 400, 1.0, 2.0);
        assert!(yakın.contains("<circle"));
    }
}
