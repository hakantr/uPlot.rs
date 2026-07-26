#![cfg(feature = "gpui-svg")]

use uplot_rs_gpui_ornekler::{
    GpuiGrafik, GpuiSvgKayıtAyarları, GradientÖrneği, Grafik, GrafikSeçenekleri, HizalıVeri,
    MultiBarsÖrneği, ScatterÖrneği, TimezonesDstÖrneği, UplotHatası, area_fill_kartı,
    box_whisker_kartı, gradients_kartı, multi_bars_kartı, scatter_kartı, timezones_dst_kartı,
};

fn kart_bileşeni(
    kart: Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası>,
) -> Result<GpuiGrafik, UplotHatası> {
    let (seçenekler, veri) = kart?;
    Ok(GpuiGrafik::yeni(Grafik::yeni(seçenekler, veri)?))
}

#[test]
fn gradyan_tanımları_katmana_özgü_kimliklerle_kaydedilir() -> Result<(), UplotHatası> {
    let bileşen = kart_bileşeni(gradients_kartı(GradientÖrneği::ÖlçekDolguları))?;
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 600)?)
        .stringe_dönüştür();
    assert!(svg.contains("<linearGradient id=\"gpui-ana-uplot-gradyan-"));
    assert!(svg.contains("fill=\"url(#gpui-ana-uplot-gradyan-"));
    assert!(!svg.contains("id=\"uplot-gradyan-"));
    Ok(())
}

#[test]
fn yoğun_dağılım_tek_vektör_yolu_ve_kırpma_olarak_kaydedilir() -> Result<(), UplotHatası> {
    let bileşen = kart_bileşeni(scatter_kartı(ScatterÖrneği::Scatter))?;
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 600)?)
        .stringe_dönüştür();
    assert!(svg.contains("id=\"gpui-ana-uplot-daire-kirpma-"));
    assert!(svg.contains("clip-path=\"url(#gpui-ana-uplot-daire-kirpma-"));
    assert!(svg.contains(" 0 1 0 "));
    assert_eq!(svg.matches("<circle").count(), 0);
    Ok(())
}

#[test]
fn multi_bars_şekil_metin_ve_renkleri_vektör_kalır() -> Result<(), UplotHatası> {
    let bileşen = kart_bileşeni(multi_bars_kartı(MultiBarsÖrneği::KitaplıklarDikey))?;
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_920, 1_080)?)
        .stringe_dönüştür();
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
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(1_920, 600)?)
        .stringe_dönüştür();
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
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(600, 200)?)
        .stringe_dönüştür();
    assert!(svg.contains(">12am</tspan>"));
    assert!(svg.contains("dy=\"1.2em\">3/31/24</tspan>"));
    assert!(!svg.contains("12am\n3/31/24"));
    Ok(())
}

#[test]
fn döndürülmüş_kategori_etiketleri_vektör_dönüşümü_kullanır() -> Result<(), UplotHatası> {
    let bileşen = kart_bileşeni(box_whisker_kartı("01_run1k"))?;
    let svg = bileşen
        .svg_kaydı(GpuiSvgKayıtAyarları::yeni(800, 400)?)
        .stringe_dönüştür();
    assert!(svg.contains("transform=\"rotate(-90.00 "));
    assert!(!svg.contains("<foreignObject"));
    Ok(())
}
