use super::ortak_kart_etkileşimleri;
#[cfg(feature = "gpui-svg")]
use crate::{GpuiSvgKayıtAyarları, Grafik, gpui::GpuiGrafik};
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const GPUI_SVG_EXPORT_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = gpui_svg_export_kartı()?;
let yüzey = GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?);
let ayarlar = GpuiSvgKayıtAyarları::yeni(800, 400)?;
let vektör_kaydı = yüzey.svg_kaydı(ayarlar);
// Kayıtçı yalnız bu açık çağrıda çalışır; normal GPUI paint yolunda maliyet oluşturmaz.
"##;

/// `demos/svg-image.html` içindeki 400×200 "test chart" verisini GPUI yüzeyi
/// ve isteğe bağlı vektör kayıt akışı için kurar.
///
/// Kaynak demo canvas ve DOM katmanlarını sonradan bir görüntüde birleştirir.
/// GPUI portu özel Scene iç alanlarını tersine çevirmek yerine aynı retained
/// komutları yalnız dışa aktarım istendiğinde gerçek vektör SVG'ye kaydeder.
pub fn gpui_svg_export_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(400, 200)?
        .başlık("test chart")
        .x_zaman(false)
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
        .arka_plan_rengi("pink")
        .ızgara_rengi("#00000012")
        .etkileşimler(ortak_kart_etkileşimleri())
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre))
        .seri(SeriSeçenekleri::yeni("Value").renk("blue"));
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0],
        vec![vec![Some(4.0), Some(5.0), Some(6.0)]],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(feature = "gpui-svg")]
pub fn gpui_svg_export_belgesi() -> Result<String, UplotHatası> {
    let (seçenekler, veri) = gpui_svg_export_kartı()?;
    let yüzey = GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?);
    let ayarlar = GpuiSvgKayıtAyarları::yeni(400, 200)?;
    Ok(yüzey.svg_kaydı(ayarlar).stringe_dönüştür())
}

#[cfg(all(test, feature = "gpui-svg"))]
mod testler {
    use super::*;

    #[test]
    fn kaynak_grafik_isteğe_bağlı_gpui_vektör_kaydına_dönüşür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = gpui_svg_export_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (400, 200));
        assert_eq!(seçenekler.başlık, "test chart");
        assert!(!seçenekler.x_zaman);
        assert_eq!(
            seçenekler.x_eksen_etiket_biçimi,
            YÖlçekEtiketBiçimi::ArtımaGöre
        );
        assert_eq!(seçenekler.ızgara_rengi, "#00000012");
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.etiket.as_str()),
            Some("Value")
        );
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0]);
        assert_eq!(
            veri.seriler().first().map(Vec::as_slice),
            Some([Some(4.0), Some(5.0), Some(6.0)].as_slice())
        );
        let svg = gpui_svg_export_belgesi()?;
        assert!(svg.starts_with("<svg"));
        assert!(svg.contains("width=\"400\" height=\"200\""));
        assert!(svg.contains("data-gpui-layer=\"ana\""));
        assert!(svg.contains("test chart"));
        assert!(svg.contains("fill=\"pink\""));
        assert!(svg.contains("stroke=\"blue\""));
        assert!(svg.contains("M64.00 143.00 L220.00 100.00 L376.00 57.00"));
        assert_eq!(svg.matches("a2.00 2.00 0 1 0").count(), 6);
        assert!(!svg.contains("<canvas"));
        assert!(!svg.contains("<foreignObject"));
        assert_eq!(svg, gpui_svg_export_belgesi()?);
        Ok(())
    }
}
