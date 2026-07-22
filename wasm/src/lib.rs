//! Tarayıcı chart listesinin WASM köprüsü.

#![allow(confusable_idents)]

use uplot_rs::{
    AREA_FILL_KART_TANIM_ÖRNEĞİ, CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, Grafik, MONTHS_KART_TANIM_ÖRNEĞİ,
    RESIZE_KART_TANIM_ÖRNEĞİ, SCALE_PADDING_KART_TANIM_ÖRNEĞİ, UplotHatası,
    ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ, area_fill_kartı, cursor_snap_kartı,
    months_artık_yıllı_kartı, months_artık_yılsız_kartı, ortak_kart_etkileşimleri, resize_kartı,
    scale_padding_kartı, zoom_touch_kartı, zoom_wheel_kartı,
};
use wasm_bindgen::prelude::*;

/// Tarayıcı yüzeyinin yalnız olayları ilettiği, seçilen kartın bütün durumunu
/// çekirdekte tutan ortak oturum.
#[wasm_bindgen]
pub struct KartOturumu {
    grafik: Grafik,
}

#[wasm_bindgen]
impl KartOturumu {
    #[wasm_bindgen(constructor)]
    pub fn yeni(kart_kimliği: &str, nokta_sayısı: usize) -> Result<KartOturumu, JsValue> {
        let (seçenekler, veri) = match kart_kimliği {
            "resize" => resize_kartı(nokta_sayısı),
            "area-fill" => area_fill_kartı(),
            "scale-padding" => scale_padding_kartı(),
            "zoom-wheel" => zoom_wheel_kartı(),
            "zoom-touch" => zoom_touch_kartı(),
            "months-no-leap" => months_artık_yılsız_kartı(),
            "months-leap" => months_artık_yıllı_kartı(),
            "cursor-snap" => cursor_snap_kartı(),
            kimlik => Err(UplotHatası::BilinmeyenKart {
                kimlik: kimlik.to_string(),
            }),
        }
        .map_err(js_hatası)?;
        let grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(Self { grafik })
    }

    pub fn svg(&self, genişlik: u32, yükseklik: u32) -> String {
        self.grafik.çiz_görünür_boyutta(genişlik, yükseklik).svg()
    }

    pub fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas_girdi: bool,
    ) -> Result<bool, JsValue> {
        self.grafik
            .tekerlek(yatay_odak_oranı, dikey_odak_oranı, delta, hassas_girdi)
            .map_err(js_hatası)
    }

    pub fn secim_yakinlastir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, JsValue> {
        self.grafik
            .seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
            .map_err(js_hatası)
    }

    pub fn tasimayi_baslat(&mut self) -> bool {
        self.grafik.taşımayı_başlat()
    }

    pub fn tasi(
        &mut self, yatay_fark_oranı: f64, dikey_fark_oranı: f64
    ) -> Result<bool, JsValue> {
        self.grafik
            .taşı(yatay_fark_oranı, dikey_fark_oranı)
            .map_err(js_hatası)
    }

    pub fn tasimayi_bitir(&mut self) {
        self.grafik.taşımayı_bitir();
    }

    pub fn dokunmayi_baslat(&mut self) -> bool {
        self.grafik.dokunmayı_başlat()
    }

    pub fn dokunma_yakinlastir(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
    ) -> Result<bool, JsValue> {
        self.grafik
            .dokunma_yakınlaştır(yatay_odak_oranı, dikey_odak_oranı, çarpan)
            .map_err(js_hatası)
    }

    pub fn dokunmayi_bitir(&mut self) {
        self.grafik.dokunmayı_bitir();
    }

    pub fn tam_gorunum(&mut self) -> bool {
        self.grafik.tam_görünüm()
    }

    pub fn onceki_gorunum(&mut self) -> bool {
        self.grafik.önceki_görünüm()
    }

    pub fn tekerlek_etkilesimi_ayarla(&mut self, etkin: bool) {
        self.grafik.tekerlek_etkileşimi_ayarla(etkin);
    }

    pub fn gorunur_x_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_x_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn gorunur_y_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_y_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn en_yakin_nokta(&self, yatay_oran: f64) -> Vec<f64> {
        self.grafik
            .en_yakın_nokta(yatay_oran, 0)
            .map_or_else(Vec::new, |(x, y)| vec![x, y])
    }

    pub fn en_yakin_noktalar(&self, yatay_oran: f64) -> Vec<f64> {
        self.grafik
            .en_yakın_noktalar(yatay_oran)
            .map_or_else(Vec::new, |(x, değerler)| {
                let mut sonuç = Vec::with_capacity(değerler.len().saturating_add(1));
                sonuç.push(x);
                sonuç.extend(değerler.into_iter().map(|değer| değer.unwrap_or(f64::NAN)));
                sonuç
            })
    }

    pub fn imlec_oranlarini_uyarla(
        &self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_genişliği: f64,
        çizim_yüksekliği: f64,
    ) -> Vec<f64> {
        self.grafik
            .imleç_oranlarını_uyarla(yatay_oran, dikey_oran, çizim_genişliği, çizim_yüksekliği)
            .map_or_else(Vec::new, |(x, y)| vec![x, y])
    }

    pub fn yakinlastirilmis(&self) -> bool {
        self.grafik.yakınlaştırılmış()
    }

    pub fn geri_var(&self) -> bool {
        self.grafik.geri_var()
    }
}

fn js_hatası(hata: UplotHatası) -> JsValue {
    JsValue::from_str(&hata.to_string())
}

#[wasm_bindgen]
pub fn kart_sayisi() -> usize {
    8
}

#[wasm_bindgen]
pub fn resize_kart_tanim_ornegi() -> String {
    RESIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn area_fill_kart_tanim_ornegi() -> String {
    AREA_FILL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn scale_padding_kart_tanim_ornegi() -> String {
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_wheel_kart_tanim_ornegi() -> String {
    ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn zoom_touch_kart_tanim_ornegi() -> String {
    ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn months_kart_tanim_ornegi() -> String {
    MONTHS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_snap_kart_tanim_ornegi() -> String {
    CURSOR_SNAP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn ortak_kart_tekerlek_etkilesimi() -> bool {
    ortak_kart_etkileşimleri().tekerlek_etkileşimi
}

#[wasm_bindgen]
pub fn ortak_kart_secim_yakinlastir() -> bool {
    ortak_kart_etkileşimleri().seçim_yakınlaştır
}

#[wasm_bindgen]
pub fn ortak_kart_cift_tikla_tam_gorunum() -> bool {
    ortak_kart_etkileşimleri().çift_tıkla_tam_görünüm
}

#[wasm_bindgen]
pub fn ortak_kart_gorunum_gecmisi() -> bool {
    ortak_kart_etkileşimleri().görünüm_geçmişi
}

#[wasm_bindgen]
pub fn ortak_kart_dokunma_etkilesimi() -> bool {
    ortak_kart_etkileşimleri().dokunma_etkileşimi
}

#[wasm_bindgen]
pub fn kaynak_commit() -> String {
    "0e5812c504430f5c804e0f993376d8999b26cc34".to_string()
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn resize_kartı_wasm_svg_üretir() {
        let oturum = KartOturumu::yeni("resize", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(800, 400);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("Resize"));
        assert_eq!(kart_sayisi(), 8);
        assert!(resize_kart_tanim_ornegi().contains("resize_kartı(100)"));

        assert!(oturum.secim_yakinlastir(0.15, 0.35).is_ok());
        let yakın = oturum.svg(800, 400);
        assert!(yakın.contains("<circle"));
        assert!(ortak_kart_dokunma_etkilesimi());
        assert!(oturum.dokunmayi_baslat());
        assert!(oturum.dokunma_yakinlastir(0.5, 0.5, 1.25).is_ok());
        oturum.dokunmayi_bitir();
        assert!(oturum.tasimayi_baslat());
        assert!(oturum.tasi(0.05, 0.05).is_ok());
        oturum.tasimayi_bitir();
    }

    #[test]
    fn area_fill_wasm_üç_dolgu_üretir() {
        let oturum = KartOturumu::yeni("area-fill", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(960, 400);
        assert!(svg.contains("Area Fill"));
        assert_eq!(svg.matches("stroke=\"none\"").count(), 3);
        assert_eq!(kart_sayisi(), 8);
    }

    #[test]
    fn scale_padding_wasm_on_üç_seriyi_üretir() {
        let oturum = KartOturumu::yeni("scale-padding", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(960, 400);
        assert!(svg.contains("Flat"));
        assert_eq!(svg.matches("fill=\"none\"").count(), 13);
    }

    #[test]
    fn zoom_wheel_wasm_kaynak_serilerini_üretir() {
        let oturum = KartOturumu::yeni("zoom-wheel", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(600, 400).contains("Wheel Zoom &amp; Drag"));
        assert!(oturum.tekerlek(0.5, 0.5, 1.0, false).is_ok());
        assert!(oturum.yakinlastirilmis());
    }

    #[test]
    fn zoom_touch_wasm_kıstırmayı_çekirdekte_uygular() {
        let oturum = KartOturumu::yeni("zoom-touch", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(960, 400).contains("Pinch Zoom &amp; Pan"));
        assert!(oturum.dokunmayi_baslat());
        assert!(oturum.dokunma_yakinlastir(0.5, 0.5, 1.25).is_ok());
        oturum.dokunmayi_bitir();
        assert!(oturum.yakinlastirilmis());
    }

    #[test]
    fn months_wasm_iki_kaynak_alt_grafiğini_üretir() {
        for kimlik in ["months-no-leap", "months-leap"] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(960, 240);
            assert!(svg.contains("20"));
            assert!(svg.contains("<path"));
        }
    }

    #[test]
    fn cursor_snap_wasm_ızgara_oranını_çekirdekten_alır() {
        let oturum = KartOturumu::yeni("cursor-snap", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert_eq!(
            oturum.imlec_oranlarini_uyarla(0.14, 0.16, 100.0, 100.0),
            vec![0.1, 0.2]
        );
    }
}
