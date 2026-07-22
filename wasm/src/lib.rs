//! Tarayıcı chart listesinin WASM köprüsü.

use uplot_rs::{Aralık, Grafik, sinüs_kartı};
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
pub fn kart_sayisi() -> usize {
    1
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

        let yakın = ilk_kart_svg_aralik(100, 800, 400, 1.0, 2.0);
        assert!(yakın.contains("<circle"));
    }
}
