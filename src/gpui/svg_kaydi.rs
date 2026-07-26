//! `GpuiGrafik` retained yüzeyinin isteğe bağlı vektör kaydı.
//!
//! Bu modül normal GPUI paint akışına bağlı değildir. Serializer yalnız
//! [`GpuiGrafik::svg_kaydı`] açıkça çağrıldığında çalışır.

use std::fmt::Write as _;

use super::{GpuiGrafik, GpuiYüzeyDönüşümü};
use crate::UplotHatası;

/// GPUI grafik yüzeyinin SVG snapshot boyutu ve katman seçimi.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpuiSvgKayıtAyarları {
    genişlik: u32,
    yükseklik: u32,
    etkileşim_katmanı: bool,
}

impl GpuiSvgKayıtAyarları {
    /// Hedef SVG viewport'unu oluşturur.
    pub fn yeni(genişlik: u32, yükseklik: u32) -> Result<Self, UplotHatası> {
        if genişlik == 0 || yükseklik == 0 {
            return Err(UplotHatası::GeçersizBoyut {
                genişlik,
                yükseklik,
            });
        }
        Ok(Self {
            genişlik,
            yükseklik,
            etkileşim_katmanı: false,
        })
    }

    /// Cursor, seçim ve retained hover işaretlerini kayda dahil eder.
    pub fn etkileşim_katmanı(mut self, dahil: bool) -> Self {
        self.etkileşim_katmanı = dahil;
        self
    }

    pub fn genişlik(&self) -> u32 {
        self.genişlik
    }

    pub fn yükseklik(&self) -> u32 {
        self.yükseklik
    }

    pub fn etkileşim_katmanı_dahil_mi(&self) -> bool {
        self.etkileşim_katmanı
    }
}

/// Düzenlenebilir vektör öğeleri içeren GPUI grafik snapshot'ı.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpuiSvgKaydı {
    içerik: String,
    genişlik: u32,
    yükseklik: u32,
}

impl GpuiSvgKaydı {
    pub fn içerik(&self) -> &str {
        &self.içerik
    }

    pub fn byte_değeri(&self) -> &[u8] {
        self.içerik.as_bytes()
    }

    pub fn stringe_dönüştür(self) -> String {
        self.içerik
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.genişlik, self.yükseklik)
    }
}

impl AsRef<str> for GpuiSvgKaydı {
    fn as_ref(&self) -> &str {
        self.içerik()
    }
}

impl GpuiGrafik {
    /// O anki retained GPUI grafik yüzeyini gerçek vektör SVG'ye kaydeder.
    ///
    /// Bu çağrı yeni grafik geometrisi üretmez ve GPUI paint/frame yoluna
    /// kayıtçı eklemez. Ana sahne yalnız bu yöntem çağrıldığında okunur.
    pub fn svg_kaydı(&self, ayarlar: GpuiSvgKayıtAyarları) -> GpuiSvgKaydı {
        let (kaynak_g, kaynak_y) = self.ana_sahne.boyut();
        let dönüşüm = GpuiYüzeyDönüşümü::hesapla(
            kaynak_g,
            kaynak_y,
            0.0,
            0.0,
            ayarlar.genişlik as f32,
            ayarlar.yükseklik as f32,
        );
        let içerik_g = kaynak_g as f32 * dönüşüm.ölçek;
        let içerik_y = kaynak_y as f32 * dönüşüm.ölçek;

        let mut çıktı = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            ayarlar.genişlik, ayarlar.yükseklik, ayarlar.genişlik, ayarlar.yükseklik
        );
        let _ = writeln!(
            çıktı,
            "  <defs><clipPath id=\"gpui-uplot-yuzey-kirpma\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",
            sayı(dönüşüm.köken_x),
            sayı(dönüşüm.köken_y),
            sayı(içerik_g),
            sayı(içerik_y),
        );
        çıktı.push_str("  <g clip-path=\"url(#gpui-uplot-yuzey-kirpma)\">\n");
        katmanı_yaz(
            &mut çıktı,
            "ana",
            "gpui-ana-",
            &self.ana_sahne.svg_içeriği("gpui-ana-"),
            dönüşüm,
        );
        if ayarlar.etkileşim_katmanı {
            let etkileşim = self.etkileşim_sahnesi();
            katmanı_yaz(
                &mut çıktı,
                "etkilesim",
                "gpui-etkilesim-",
                &etkileşim.svg_içeriği("gpui-etkilesim-"),
                dönüşüm,
            );
        }
        çıktı.push_str("  </g>\n</svg>\n");

        GpuiSvgKaydı {
            içerik: çıktı,
            genişlik: ayarlar.genişlik,
            yükseklik: ayarlar.yükseklik,
        }
    }

    /// Native hedefte o anki vektör snapshot'ını bir dosyaya yazar.
    #[cfg(not(target_family = "wasm"))]
    pub fn svg_dosyasına_yaz(
        &self,
        yol: impl AsRef<std::path::Path>,
        ayarlar: GpuiSvgKayıtAyarları,
    ) -> std::io::Result<()> {
        std::fs::write(yol, self.svg_kaydı(ayarlar).byte_değeri())
    }
}

fn katmanı_yaz(
    çıktı: &mut String,
    katman: &str,
    kimlik_öneki: &str,
    gövde: &str,
    dönüşüm: GpuiYüzeyDönüşümü,
) {
    let _ = writeln!(
        çıktı,
        "    <g id=\"{kimlik_öneki}katman\" data-gpui-layer=\"{katman}\" transform=\"translate({} {}) scale({})\">",
        sayı(dönüşüm.köken_x),
        sayı(dönüşüm.köken_y),
        sayı(dönüşüm.ölçek),
    );
    çıktı.push_str(gövde);
    çıktı.push_str("    </g>\n");
}

fn sayı(değer: f32) -> String {
    let güvenli = if değer.is_finite() {
        f64::from(değer)
    } else {
        0.0
    };
    let yuvarlanmış = (güvenli * 100.0).round() / 100.0;
    let yuvarlanmış = if yuvarlanmış == 0.0 {
        0.0
    } else {
        yuvarlanmış
    };
    format!("{yuvarlanmış:.2}")
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{
        Grafik,
        kart::{
            GradientÖrneği, MultiBarsÖrneği, ScatterÖrneği, TimezonesDstÖrneği, area_fill_kartı,
            box_whisker_kartı, cursor_snap_kartı, gradients_kartı, multi_bars_kartı, resize_kartı,
            scatter_kartı, timezones_dst_kartı,
        },
    };

    fn resize_bileşeni() -> Result<GpuiGrafik, UplotHatası> {
        let (seçenekler, veri) = resize_kartı(100)?;
        Ok(GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?))
    }

    fn kart_bileşeni(
        kart: Result<(crate::GrafikSeçenekleri, crate::HizalıVeri), UplotHatası>,
    ) -> Result<GpuiGrafik, UplotHatası> {
        let (seçenekler, veri) = kart?;
        Ok(GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?))
    }

    #[test]
    fn kayıt_gerçek_vektör_ve_gpui_aspect_fit_dönüşümü_üretir() -> Result<(), UplotHatası> {
        let bileşen = resize_bileşeni()?;
        let ayarlar = GpuiSvgKayıtAyarları::yeni(1_000, 1_000)?;
        let kayıt = bileşen.svg_kaydı(ayarlar);
        let svg = kayıt.içerik();

        assert_eq!(kayıt.boyut(), (1_000, 1_000));
        assert!(svg.contains("transform=\"translate(0.00 250.00) scale(1.25)\""));
        assert!(svg.contains("<path"));
        assert!(svg.contains("<text"));
        assert!(svg.contains("<rect width=\"800\" height=\"400\""));
        assert!(!svg.contains("width=\"100%\""));
        assert!(!svg.contains("<image"));
        assert!(!svg.contains("vector-effect"));
        Ok(())
    }

    #[test]
    fn çok_küçük_hedef_gpui_minimum_ölçeğini_korur() -> Result<(), UplotHatası> {
        let bileşen = resize_bileşeni()?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(1, 1)?);

        assert!(
            kayıt
                .içerik()
                .contains("transform=\"translate(-3.50 -1.50) scale(0.01)\"")
        );
        Ok(())
    }

    #[test]
    fn kayıt_belirlenimlidir_ve_ana_sahneyi_değiştirmez() -> Result<(), UplotHatası> {
        let bileşen = resize_bileşeni()?;
        let ana_sahne = bileşen.ana_sahne.clone();
        let ayarlar = GpuiSvgKayıtAyarları::yeni(1_200, 600)?;

        let ilk = bileşen.svg_kaydı(ayarlar);
        let ikinci = bileşen.svg_kaydı(ayarlar);

        assert_eq!(ilk, ikinci);
        assert!(std::rc::Rc::ptr_eq(&bileşen.ana_sahne, &ana_sahne));
        Ok(())
    }

    #[test]
    fn etkileşim_katmanı_yalnız_istendiğinde_serileştirilir() -> Result<(), UplotHatası> {
        let bileşen = resize_bileşeni()?;
        let yalın = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 400)?);
        let etkileşimli =
            bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 400)?.etkileşim_katmanı(true));

        assert!(!yalın.içerik().contains("data-gpui-layer=\"etkilesim\""));
        assert!(
            etkileşimli
                .içerik()
                .contains("data-gpui-layer=\"etkilesim\"")
        );
        Ok(())
    }

    #[test]
    fn gradyan_tanımları_katmana_özgü_kimliklerle_kaydedilir() -> Result<(), UplotHatası> {
        let bileşen = kart_bileşeni(gradients_kartı(GradientÖrneği::ÖlçekDolguları))?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 600)?);
        let svg = kayıt.içerik();

        assert!(svg.contains("<linearGradient id=\"gpui-ana-uplot-gradyan-"));
        assert!(svg.contains("fill=\"url(#gpui-ana-uplot-gradyan-"));
        assert!(!svg.contains("id=\"uplot-gradyan-"));
        Ok(())
    }

    #[test]
    fn yoğun_dağılım_tek_vektör_yolu_ve_kırpma_olarak_kaydedilir() -> Result<(), UplotHatası> {
        let bileşen = kart_bileşeni(scatter_kartı(ScatterÖrneği::Scatter))?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 600)?);
        let svg = kayıt.içerik();

        assert!(svg.contains("id=\"gpui-ana-uplot-daire-kirpma-"));
        assert!(svg.contains("clip-path=\"url(#gpui-ana-uplot-daire-kirpma-"));
        assert!(svg.contains(" 0 1 0 "));
        assert_eq!(svg.matches("<circle").count(), 0);
        Ok(())
    }

    #[test]
    fn multi_bars_şekil_metin_ve_renkleri_vektör_kalır() -> Result<(), UplotHatası> {
        let bileşen = kart_bileşeni(multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey))?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_920, 1_080)?);
        let svg = kayıt.içerik();

        assert!(svg.contains("<path"));
        assert!(svg.contains("<text"));
        assert!(svg.contains(" Q"));
        assert!(!svg.contains("<foreignObject"));
        assert!(!svg.contains("<image"));
        Ok(())
    }

    #[test]
    fn area_fill_üç_serinin_çizgi_ve_dolgularını_korur() -> Result<(), UplotHatası> {
        let bileşen = kart_bileşeni(area_fill_kartı())?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_920, 600)?);
        let svg = kayıt.içerik();

        for dolgu in ["#ff00001a", "#00ff001a", "#0000ff1a"] {
            assert!(svg.contains(&format!("fill=\"{dolgu}\"")));
        }
        for çizgi in ["#ff0000", "#008000", "#0000ff"] {
            assert!(svg.contains(&format!("stroke=\"{çizgi}\"")));
        }
        assert!(svg.matches("stroke=\"none\"").count() >= 3);
        Ok(())
    }

    #[test]
    fn timezone_çok_satırlı_etiketi_tspan_olarak_kaydeder() -> Result<(), UplotHatası> {
        let örnek = TimezonesDstÖrneği::yeni(12).ok_or(UplotHatası::BilinmeyenKart {
            kimlik: "timezones-dst-13".to_string(),
        })?;
        let bileşen = kart_bileşeni(timezones_dst_kartı(örnek))?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(600, 200)?);
        let svg = kayıt.içerik();

        assert!(svg.contains(">12am</tspan>"));
        assert!(svg.contains("dy=\"1.2em\">3/31/24</tspan>"));
        assert!(!svg.contains("12am\n3/31/24"));
        Ok(())
    }

    #[test]
    fn döndürülmüş_kategori_etiketleri_vektör_dönüşümü_kullanır() -> Result<(), UplotHatası> {
        let bileşen = kart_bileşeni(box_whisker_kartı("01_run1k"))?;
        let kayıt = bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 400)?);
        let svg = kayıt.içerik();

        assert!(svg.contains("transform=\"rotate(-90.00 "));
        assert!(!svg.contains("<foreignObject"));
        Ok(())
    }

    #[test]
    fn cursor_snap_kaydı_etkileşim_ve_bileşen_durumunu_korur() -> Result<(), UplotHatası> {
        let mut bileşen = kart_bileşeni(cursor_snap_kartı())?;
        let ham = crate::Nokta::yeni(123.4, 234.5);
        let oturmuş =
            bileşen
                .imleç_ızgarasına_oturt(ham)
                .ok_or(UplotHatası::GeçersizKaynakVeri {
                    varlık: "cursor-snap",
                    açıklama: "10x10 imleç ızgarası uygulanamadı".to_string(),
                })?;
        assert_ne!(oturmuş, ham);
        bileşen.canlı_imleci_yenile(oturmuş);
        bileşen.seçim = Some((
            crate::Nokta::yeni(120.0, 140.0),
            crate::Nokta::yeni(360.0, 280.0),
        ));
        bileşen.taşıma_başlangıcı = Some(crate::Nokta::yeni(200.0, 200.0));
        bileşen.boşluk_basılı = true;

        let ana_sahne = bileşen.ana_sahne.clone();
        let imleç_öncesi = bileşen
            .imleç
            .as_ref()
            .map(|imleç| (imleç.fare, imleç.veri_x, imleç.seri_değerleri.clone()));
        let seçim_öncesi = bileşen.seçim;
        let taşıma_öncesi = bileşen.taşıma_başlangıcı;
        let kayıt =
            bileşen.svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_920, 600)?.etkileşim_katmanı(true));

        assert!(kayıt.içerik().contains("data-gpui-layer=\"etkilesim\""));
        assert!(kayıt.içerik().contains("stroke-dasharray="));
        assert_eq!(
            bileşen.imleç.as_ref().map(|imleç| (
                imleç.fare,
                imleç.veri_x,
                imleç.seri_değerleri.clone()
            )),
            imleç_öncesi
        );
        assert_eq!(bileşen.seçim, seçim_öncesi);
        assert_eq!(bileşen.taşıma_başlangıcı, taşıma_öncesi);
        assert!(bileşen.boşluk_basılı);
        assert!(std::rc::Rc::ptr_eq(&bileşen.ana_sahne, &ana_sahne));
        Ok(())
    }

    #[test]
    fn sıfır_boyut_reddedilir() {
        assert_eq!(
            GpuiSvgKayıtAyarları::yeni(0, 400),
            Err(UplotHatası::GeçersizBoyut {
                genişlik: 0,
                yükseklik: 400
            })
        );
    }
}
