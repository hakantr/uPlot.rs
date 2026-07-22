//! Tarayıcı chart listesinin WASM köprüsü.

#![allow(confusable_idents)]

use uplot_rs::{
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ, ALIGN_DATA_KART_TANIM_ÖRNEĞİ,
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ, AREA_FILL_KART_TANIM_ÖRNEĞİ, AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ, AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ,
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ, BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ,
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ, CANDLESTICK_KART_TANIM_ÖRNEĞİ, CURSOR_BIND_KART_TANIM_ÖRNEĞİ,
    CURSOR_SNAP_KART_TANIM_ÖRNEĞİ, CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ,
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ, CustomScaleÖrneği, DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ,
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ, DRAW_HOOKS_KART_TANIM_ÖRNEĞİ, Grafik,
    MISSING_DATA_KART_TANIM_ÖRNEĞİ, MONTHS_KART_TANIM_ÖRNEĞİ, RESIZE_KART_TANIM_ÖRNEĞİ,
    SCALE_PADDING_KART_TANIM_ÖRNEĞİ, SeriSeçenekleri, SeçimEylemi, SmoothingÖrneği, UplotHatası,
    ZOOM_TOUCH_KART_TANIM_ÖRNEĞİ, ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ, add_del_series_ek_verisi,
    add_del_series_kartı, align_data_maliyet_kartı, align_data_çizgi_çubuk_kartı,
    arcsinh_scales_kartı, area_fill_kartı, axis_autosize_kartı, axis_control_kartı,
    axis_indicators_kartı, bars_grouped_stacked_kartı, bars_values_autosize_kartı,
    box_whisker_kartı, candlestick_ohlc_kartı, cursor_bind_kartı, cursor_snap_kartı,
    cursor_tooltip_kartı, custom_scales_kartı, data_smoothing_kartı, dependent_scale_kartı,
    draw_hooks_kartı, missing_data_null_kartı, missing_data_x_boşluğu_kartı,
    months_artık_yıllı_kartı, months_artık_yılsız_kartı, ortak_kart_etkileşimleri, resize_kartı,
    scale_padding_kartı, zoom_touch_kartı, zoom_wheel_kartı, ÇubukYönü, ÇubukÖrneği,
};
use wasm_bindgen::prelude::*;

/// Tarayıcı yüzeyinin yalnız olayları ilettiği, seçilen kartın bütün durumunu
/// çekirdekte tutan ortak oturum.
#[wasm_bindgen]
pub struct KartOturumu {
    grafik: Grafik,
    dinamik_seri_sayacı: u32,
}

#[wasm_bindgen]
impl KartOturumu {
    #[wasm_bindgen(constructor)]
    pub fn yeni(kart_kimliği: &str, nokta_sayısı: usize) -> Result<KartOturumu, JsValue> {
        let (seçenekler, veri) = match kart_kimliği {
            "add-del-series" => add_del_series_kartı(),
            "align-data-cost" => align_data_maliyet_kartı(),
            "align-data-line-bars" => align_data_çizgi_çubuk_kartı(),
            "resize" => resize_kartı(nokta_sayısı),
            "area-fill" => area_fill_kartı(),
            "scale-padding" => scale_padding_kartı(),
            "zoom-wheel" => zoom_wheel_kartı(),
            "zoom-touch" => zoom_touch_kartı(),
            "months-no-leap" => months_artık_yılsız_kartı(),
            "months-leap" => months_artık_yıllı_kartı(),
            "cursor-bind" => cursor_bind_kartı(),
            "cursor-snap" => cursor_snap_kartı(),
            "cursor-tooltip" => cursor_tooltip_kartı(),
            "custom-scales-linear" => custom_scales_kartı(CustomScaleÖrneği::Doğrusal),
            "custom-scales-log-log" => custom_scales_kartı(CustomScaleÖrneği::LogLog),
            "custom-scales-weibull" => custom_scales_kartı(CustomScaleÖrneği::Weibull),
            "data-smoothing-raw" => data_smoothing_kartı(SmoothingÖrneği::Ham),
            "data-smoothing-sgg" => data_smoothing_kartı(SmoothingÖrneği::SavitzkyGolay),
            "data-smoothing-asap" => data_smoothing_kartı(SmoothingÖrneği::Asap),
            "data-smoothing-moving-average" => {
                data_smoothing_kartı(SmoothingÖrneği::HareketliOrtalama)
            }
            "draw-hooks" => draw_hooks_kartı(),
            "missing-data-null" => missing_data_null_kartı(),
            "missing-data-x-gap" => missing_data_x_boşluğu_kartı(),
            "dependent-scale" => dependent_scale_kartı(),
            "arcsinh-scales" => arcsinh_scales_kartı(),
            "axis-control" => axis_control_kartı(),
            "axis-autosize" => axis_autosize_kartı(1.0),
            "axis-indicators" => axis_indicators_kartı(),
            "bars-values-autosize-vertical" => bars_values_autosize_kartı(ÇubukYönü::Dikey),
            "bars-values-autosize-horizontal" => bars_values_autosize_kartı(ÇubukYönü::Yatay),
            "candlestick-ohlc" => candlestick_ohlc_kartı(),
            kimlik if kimlik.starts_with("box-whisker-") => {
                box_whisker_kartı(kimlik.trim_start_matches("box-whisker-"))
            }
            kimlik => ÇubukÖrneği::kimlikten(kimlik).map_or_else(
                || {
                    Err(UplotHatası::BilinmeyenKart {
                        kimlik: kimlik.to_string(),
                    })
                },
                bars_grouped_stacked_kartı,
            ),
        }
        .map_err(js_hatası)?;
        let grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(Self {
            grafik,
            dinamik_seri_sayacı: 0,
        })
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

    /// 0: değişmedi, 1: yakınlaştırıldı, 2: açıklama metni istenmeli.
    pub fn secimi_bitir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
        açıklama_tuşu: bool,
    ) -> Result<u8, JsValue> {
        self.grafik
            .seçimi_bitir(başlangıç_oranı, bitiş_oranı, açıklama_tuşu)
            .map(|eylem| match eylem {
                SeçimEylemi::Değişmedi => 0,
                SeçimEylemi::Yakınlaştırıldı => 1,
                SeçimEylemi::Açıklamaİstendi => 2,
            })
            .map_err(js_hatası)
    }

    pub fn ctrl_aciklama_etkin(&self) -> bool {
        self.grafik.etkileşim_seçenekleri().ctrl_açıklama
    }

    pub fn add_del_seri_ekle(&mut self) -> Result<bool, JsValue> {
        let değerler = add_del_series_ek_verisi(self.dinamik_seri_sayacı);
        self.grafik
            .seri_ekle(
                1,
                SeriSeçenekleri::yeni("Orange")
                    .renk("#ffa500")
                    .dolgu("#ffa5001a"),
                değerler,
            )
            .map_err(js_hatası)?;
        self.dinamik_seri_sayacı = self.dinamik_seri_sayacı.wrapping_add(1);
        Ok(true)
    }

    pub fn add_del_seri_sil(&mut self) -> Result<bool, JsValue> {
        if self.grafik.seri_seçenekleri().len() < 2 {
            return Ok(false);
        }
        self.grafik.seri_sil(1).map_err(js_hatası)?;
        Ok(true)
    }

    pub fn seri_sayisi(&self) -> usize {
        self.grafik.seri_seçenekleri().len()
    }

    pub fn seri_etiketleri(&self) -> Vec<String> {
        self.grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.etiket.clone())
            .collect()
    }

    pub fn seri_renkleri(&self) -> Vec<String> {
        self.grafik
            .seri_seçenekleri()
            .iter()
            .map(|seri| seri.renk.clone())
            .collect()
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

    pub fn bosluklari_birlestir_ayarla(&mut self, etkin: bool) {
        self.grafik.boşlukları_birleştir_ayarla(etkin);
    }

    pub fn gorunur_x_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_x_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn gorunur_y_araligi(&self) -> Vec<f64> {
        let aralık = self.grafik.görünür_y_aralığı();
        vec![aralık.en_az, aralık.en_çok]
    }

    pub fn seri_gorunur_y_araligi(&self, seri_indeksi: usize) -> Vec<f64> {
        self.grafik
            .seri_görünür_y_aralığı(seri_indeksi)
            .map_or_else(Vec::new, |aralık| vec![aralık.en_az, aralık.en_çok])
    }

    pub fn seri_y_konum_orani(&self, seri_indeksi: usize, değer: f64) -> f64 {
        self.grafik
            .seri_y_konum_oranı(seri_indeksi, değer)
            .unwrap_or(f64::NAN)
    }

    pub fn x_konum_orani(&self, değer: f64) -> f64 {
        self.grafik.x_konum_oranı(değer).unwrap_or(f64::NAN)
    }

    pub fn y_arcsinh_esigi_ayarla(&mut self, anahtar: &str, eşik: f64) -> bool {
        self.grafik.y_arcsinh_eşiği_ayarla(anahtar, eşik)
    }

    pub fn axis_autosize_carpani_ayarla(&mut self, çarpan: f64) -> Result<(), JsValue> {
        let (seçenekler, veri) = axis_autosize_kartı(çarpan).map_err(js_hatası)?;
        self.grafik = Grafik::yeni(seçenekler, veri).map_err(js_hatası)?;
        Ok(())
    }

    pub fn eksen_gostergeleri_etkin(&self) -> bool {
        self.grafik.eksen_göstergeleri_etkin()
    }

    pub fn cubuk_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> Vec<f64> {
        self.grafik
            .çubuk_vuruşu(genişlik, yükseklik, x, y)
            .map_or_else(
                Vec::new,
                |(seri, indeks, konum, çubuk_g, çubuk_y, değer)| {
                    vec![
                        seri as f64,
                        indeks as f64,
                        f64::from(konum.x),
                        f64::from(konum.y),
                        f64::from(çubuk_g),
                        f64::from(çubuk_y),
                        değer,
                    ]
                },
            )
    }

    pub fn kutu_biyik_vurusu(&self, genişlik: u32, yükseklik: u32, x: f32, y: f32) -> Vec<f64> {
        self.grafik
            .kutu_bıyık_vuruşu(genişlik, yükseklik, x, y)
            .map_or_else(Vec::new, |(indeks, konum, kutu_g, kutu_y, değerler)| {
                let mut sonuç = vec![
                    indeks as f64,
                    f64::from(konum.x),
                    f64::from(konum.y),
                    f64::from(kutu_g),
                    f64::from(kutu_y),
                ];
                sonuç.extend(değerler);
                sonuç
            })
    }

    pub fn cizim_alani(&self, genişlik: u32, yükseklik: u32) -> Vec<f64> {
        let (sol, sağ, üst, alt) = self.grafik.çizim_alanı_boyutta(genişlik, yükseklik);
        vec![
            f64::from(sol),
            f64::from(sağ),
            f64::from(üst),
            f64::from(alt),
        ]
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
    58
}

#[wasm_bindgen]
pub fn draw_hooks_kart_tanim_ornegi() -> String {
    DRAW_HOOKS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn data_smoothing_kart_tanim_ornegi() -> String {
    DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn custom_scales_kart_tanim_ornegi() -> String {
    CUSTOM_SCALES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_tooltip_kart_tanim_ornegi() -> String {
    CURSOR_TOOLTIP_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn align_data_kart_tanim_ornegi() -> String {
    ALIGN_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn resize_kart_tanim_ornegi() -> String {
    RESIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn add_del_series_kart_tanim_ornegi() -> String {
    ADD_DEL_SERIES_KART_TANIM_ÖRNEĞİ.to_string()
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
pub fn missing_data_kart_tanim_ornegi() -> String {
    MISSING_DATA_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn dependent_scale_kart_tanim_ornegi() -> String {
    DEPENDENT_SCALE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn arcsinh_scales_kart_tanim_ornegi() -> String {
    ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_control_kart_tanim_ornegi() -> String {
    AXIS_CONTROL_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_autosize_kart_tanim_ornegi() -> String {
    AXIS_AUTOSIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn axis_indicators_kart_tanim_ornegi() -> String {
    AXIS_INDICATORS_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn bars_grouped_stacked_kart_tanim_ornegi() -> String {
    BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn bars_values_autosize_kart_tanim_ornegi() -> String {
    BARS_VALUES_AUTOSIZE_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn box_whisker_kart_tanim_ornegi() -> String {
    BOX_WHISKER_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn candlestick_kart_tanim_ornegi() -> String {
    CANDLESTICK_KART_TANIM_ÖRNEĞİ.to_string()
}

#[wasm_bindgen]
pub fn cursor_bind_kart_tanim_ornegi() -> String {
    CURSOR_BIND_KART_TANIM_ÖRNEĞİ.to_string()
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
        assert_eq!(kart_sayisi(), 58);
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
        assert_eq!(kart_sayisi(), 58);
    }

    #[test]
    fn add_del_series_wasm_seriyi_atomik_günceller() {
        let oturum = KartOturumu::yeni("add-del-series", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert_eq!(oturum.seri_sayisi(), 3);
        assert!(matches!(oturum.add_del_seri_ekle(), Ok(true)));
        assert_eq!(oturum.seri_sayisi(), 4);
        assert_eq!(
            oturum.seri_etiketleri().get(1).map(String::as_str),
            Some("Orange")
        );
        assert!(oturum.svg(960, 400).contains("#ffa500"));
        assert!(matches!(oturum.add_del_seri_sil(), Ok(true)));
        assert_eq!(oturum.seri_sayisi(), 3);
        assert!(add_del_series_kart_tanim_ornegi().contains("seri_ekle"));
    }

    #[test]
    fn align_data_wasm_join_ve_karma_yolu_üretir() {
        let maliyet = KartOturumu::yeni("align-data-cost", 100);
        assert!(maliyet.is_ok());
        let Ok(mut maliyet) = maliyet else {
            return;
        };
        assert_eq!(maliyet.seri_sayisi(), 25);
        let ayrı = maliyet.svg(960, 400);
        maliyet.bosluklari_birlestir_ayarla(true);
        assert_ne!(maliyet.svg(960, 400), ayrı);

        let karma = KartOturumu::yeni("align-data-line-bars", 100);
        assert!(karma.is_ok());
        let Ok(karma) = karma else {
            return;
        };
        let svg = karma.svg(960, 400);
        assert!(svg.contains("#ff0000"));
        assert_eq!(svg.matches("fill=\"#0000ff1a\"").count(), 4);
        assert!(align_data_kart_tanim_ornegi().contains("align_data_maliyet_kartı"));
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

    #[test]
    fn cursor_bind_wasm_ctrl_seçimini_yakınlaştırmadan_ayırır() {
        let oturum = KartOturumu::yeni("cursor-bind", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        assert!(oturum.svg(1_920, 600).contains("Cursor Bind"));
        assert!(oturum.ctrl_aciklama_etkin());
        assert_eq!(oturum.secimi_bitir(0.2, 0.6, true), Ok(2));
        assert!(!oturum.yakinlastirilmis());
        assert_eq!(oturum.secimi_bitir(0.2, 0.6, false), Ok(1));
        assert!(oturum.yakinlastirilmis());
        assert!(cursor_bind_kart_tanim_ornegi().contains("cursor_bind_kartı"));
    }

    #[test]
    fn cursor_tooltip_wasm_kaynak_verisini_üretir() {
        let oturum = KartOturumu::yeni("cursor-tooltip", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("placement.js tooltip"));
        assert!(svg.contains("#008000"));
        assert_eq!(oturum.en_yakin_noktalar(0.5), vec![4.0, 65.0]);
        assert!(cursor_tooltip_kart_tanim_ornegi().contains("cursor_tooltip_kartı"));
    }

    #[test]
    fn custom_scales_wasm_üç_farklı_geometri_üretir() {
        let mut svgler = Vec::new();
        for kimlik in [
            "custom-scales-linear",
            "custom-scales-log-log",
            "custom-scales-weibull",
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else { return };
            let svg = oturum.svg(800, 800);
            assert!(svg.contains("#ffa50030"));
            assert_eq!(svg.matches("fill=\"#000000\"").count(), 20);
            assert!(svg.contains("stroke-dasharray=\"10.00 5.00\""));
            svgler.push(svg);
        }
        assert_ne!(svgler.first(), svgler.get(1));
        assert_ne!(svgler.get(1), svgler.get(2));
        assert!(custom_scales_kart_tanim_ornegi().contains("CustomScaleÖrneği"));
    }

    #[test]
    fn data_smoothing_wasm_dört_kaynak_alt_grafiğini_üretir() {
        for (kimlik, başlık) in [
            ("data-smoothing-raw", "Taxi Trips (raw)"),
            ("data-smoothing-sgg", "Savitzky-Golay"),
            ("data-smoothing-asap", "Taxi Trips (ASAP FFT)"),
            (
                "data-smoothing-moving-average",
                "Taxi Trips (Moving Avg 300)",
            ),
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok());
            let Ok(oturum) = oturum else { return };
            let svg = oturum.svg(960, 300);
            assert!(svg.contains(başlık));
            assert!(svg.contains("#ff0000"));
        }
        assert!(data_smoothing_kart_tanim_ornegi().contains("SmoothingÖrneği::Asap"));
    }

    #[test]
    fn draw_hooks_wasm_kaynak_çizim_katmanlarını_üretir() {
        let oturum = KartOturumu::yeni("draw-hooks", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else { return };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("Draw Hooks"));
        assert!(svg.contains("Time to Draw: 0ms"));
        assert!(svg.contains("#ff333333"));
        assert_eq!(svg.matches("fill=\"#ff3333\"").count(), 9);
        assert!(draw_hooks_kart_tanim_ornegi().contains("draw_hooks_kartı"));
    }

    #[test]
    fn missing_data_wasm_iki_kaynak_alt_grafiğini_üretir() {
        let ana = KartOturumu::yeni("missing-data-null", 100);
        assert!(ana.is_ok());
        let Ok(ana) = ana else {
            return;
        };
        let svg = ana.svg(960, 400);
        assert!(svg.contains("Missing Data (null values)"));
        assert!(svg.contains("MB"));

        let boşluk = KartOturumu::yeni("missing-data-x-gap", 100);
        assert!(boşluk.is_ok());
        let Ok(boşluk) = boşluk else {
            return;
        };
        assert!(boşluk.svg(960, 400).contains("adjacent points"));
    }

    #[test]
    fn dependent_scale_wasm_iki_sıcaklık_eksenini_üretir() {
        let oturum = KartOturumu::yeni("dependent-scale", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(600, 400);
        assert!(svg.contains("° F"));
        assert!(svg.contains("° C"));
    }

    #[test]
    fn arcsinh_wasm_eşiği_çekirdekte_değiştirir() {
        let oturum = KartOturumu::yeni("arcsinh-scales", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let önce = oturum.svg(960, 400);
        assert!(oturum.y_arcsinh_esigi_ayarla("y", 0.001));
        assert_ne!(oturum.svg(960, 400), önce);
    }

    #[test]
    fn axis_control_wasm_seyrek_sahne_ve_eksenleri_üretir() {
        let oturum = KartOturumu::yeni("axis-control", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1048, 600);
        assert!(svg.contains("X Axis Label"));
        assert!(svg.contains("Y Axis Label"));
        assert!(svg.len() < 500_000);
    }

    #[test]
    fn axis_autosize_wasm_dinamik_çarpanda_eksenleri_yeniden_ölçer() {
        let oturum = KartOturumu::yeni("axis-autosize", 100);
        assert!(oturum.is_ok());
        let Ok(mut oturum) = oturum else {
            return;
        };
        let önceki = oturum.cizim_alani(1048, 600);
        assert!(oturum.axis_autosize_carpani_ayarla(1e9).is_ok());
        let sonraki = oturum.cizim_alani(1048, 600);
        assert!(
            sonraki
                .first()
                .zip(önceki.first())
                .is_some_and(|(yeni, eski)| yeni > eski)
        );
        assert!(oturum.svg(1048, 600).contains("500000000000.00"));
    }

    #[test]
    fn axis_indicators_wasm_üç_ölçeği_ve_göstergeyi_üretir() {
        let oturum = KartOturumu::yeni("axis-indicators", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        assert!(oturum.eksen_gostergeleri_etkin());
        assert_eq!(oturum.svg(1200, 600).matches("fill=\"none\"").count(), 3);
        assert_eq!(oturum.seri_gorunur_y_araligi(2).len(), 2);
    }

    #[test]
    fn bars_grouped_stacked_wasm_on_alt_grafiği_üretir() {
        for örnek in ÇubukÖrneği::TÜMÜ {
            let oturum = KartOturumu::yeni(örnek.kimlik(), 100);
            assert!(oturum.is_ok(), "{}", örnek.kimlik());
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(800, 500);
            assert!(svg.contains("Group A"));
            assert!(svg.matches("<rect").count() >= 2);
        }
    }

    #[test]
    fn bars_values_autosize_wasm_iki_yönü_üretir() {
        for kimlik in [
            "bars-values-autosize-vertical",
            "bars-values-autosize-horizontal",
        ] {
            let oturum = KartOturumu::yeni(kimlik, 100);
            assert!(oturum.is_ok(), "{kimlik}");
            let Ok(oturum) = oturum else {
                continue;
            };
            let svg = oturum.svg(1_275, 600);
            assert!(svg.contains("#00ff0022"));
            assert!(svg.matches("#00000033").count() >= 12);
        }
    }

    #[test]
    fn box_whisker_wasm_kaynak_kutusunu_ve_vurgu_sütununu_üretir() {
        let oturum = KartOturumu::yeni("box-whisker-01_run1k", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(800, 400);
        assert!(svg.contains("stroke-dasharray=\"4.00 4.00\""));
        let vuruş = oturum.kutu_biyik_vurusu(800, 400, 76.0, 120.0);
        assert!(vuruş.is_empty() || vuruş.len() == 10);
    }

    #[test]
    fn candlestick_wasm_ohlc_ve_hacmi_üretir() {
        let oturum = KartOturumu::yeni("candlestick-ohlc", 100);
        assert!(oturum.is_ok());
        let Ok(oturum) = oturum else {
            return;
        };
        let svg = oturum.svg(1_920, 600);
        assert!(svg.contains("#4ab650"));
        assert!(svg.contains("#e54245"));
        assert_eq!(oturum.kutu_biyik_vurusu(1_920, 600, 76.0, 100.0).len(), 10);
    }
}
