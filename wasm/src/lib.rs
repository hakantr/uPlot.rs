//! Tarayıcı chart listesinin WASM köprüsü.

#![allow(confusable_idents)]

use uplot_rs::{
    Grafik, UplotHatası, ilk_kart_etkileşimleri, sinüs_kartı, İLK_KART_TANIM_ÖRNEĞİ
};
use wasm_bindgen::prelude::*;

/// Tarayıcı yüzeyinin yalnız olayları ilettiği, bütün kart durumunu çekirdekte
/// tutan ilk kart oturumu.
#[wasm_bindgen]
pub struct IlkKartOturumu {
    grafik: Grafik,
}

#[wasm_bindgen]
impl IlkKartOturumu {
    #[wasm_bindgen(constructor)]
    pub fn yeni(nokta_sayısı: usize) -> Result<IlkKartOturumu, JsValue> {
        let (seçenekler, veri) = sinüs_kartı(nokta_sayısı).map_err(js_hatası)?;
        let grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(Self { grafik })
    }

    pub fn svg(&self, genişlik: u32, yükseklik: u32) -> String {
        self.grafik.çiz_görünür_boyutta(genişlik, yükseklik).svg()
    }

    pub fn tekerlek(
        &mut self,
        odak_oranı: f64,
        delta: f64,
        hassas_girdi: bool,
    ) -> Result<bool, JsValue> {
        self.grafik
            .tekerlek(odak_oranı, delta, hassas_girdi)
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

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn ilk_kart_wasm_svg_üretir() {
        let oturum = IlkKartOturumu::yeni(100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(800, 400);
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("İlk kart · sin(x)"));
        assert_eq!(kart_sayisi(), 1);
        assert!(ilk_kart_tanim_ornegi().contains("GrafikSeçenekleri::yeni"));

        assert!(oturum.secim_yakinlastir(0.15, 0.35).is_ok());
        let yakın = oturum.svg(800, 400);
        assert!(yakın.contains("<circle"));
    }
}
