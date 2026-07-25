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
    let yuvarlanmış = (değer * 100.0).round() / 100.0;
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
    use crate::{Grafik, kart::resize_kartı};

    fn resize_bileşeni() -> Result<GpuiGrafik, UplotHatası> {
        let (seçenekler, veri) = resize_kartı(100)?;
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
