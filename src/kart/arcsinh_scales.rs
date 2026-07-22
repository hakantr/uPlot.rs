use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri,
};

pub const ARCSINH_SCALES_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = arcsinh_scales_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
// İsteğe bağlı canlı eşik değişimi tamamen çekirdekte çözülür.
grafik.y_arcsinh_eşiği_ayarla("y", 0.1);"##;

/// `demos/arcsinh-scales.html` içindeki −1000…1000 simetrik değer dizisini
/// ve başlangıç `asinh: 1` eşiğini kurar.
pub fn arcsinh_scales_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut pozitif = Vec::with_capacity(55);
    for kuvvet in -3..=2 {
        for katsayı in 1..10 {
            pozitif.push(yuvarla6(f64::from(katsayı) * 10_f64.powi(kuvvet)));
        }
    }
    pozitif.push(1_000.0);
    let mut y = pozitif
        .iter()
        .rev()
        .map(|değer| Some(-değer))
        .collect::<Vec<_>>();
    y.push(Some(0.0));
    y.extend(pozitif.into_iter().map(Some));
    let x = (1..=y.len()).map(|indeks| indeks as f64).collect();

    let seçenekler = GrafikSeçenekleri::yeni(1600, 600)?
        .başlık("ArcSinh Y Scale")
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(-1_000.0, 1_000.0)?)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").arcsinh(1.0))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Value")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    Ok((seçenekler, HizalıVeri::yeni(x, vec![y])?))
}

fn yuvarla6(değer: f64) -> f64 {
    (değer * 1_000_000.0).round() / 1_000_000.0
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_değerleri_ve_canlı_eşik_değişimi_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = arcsinh_scales_kartı()?;
        assert_eq!(veri.uzunluk(), 111);
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied()
                .flatten(),
            Some(-1_000.0)
        );
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.get(55))
                .copied()
                .flatten(),
            Some(0.0)
        );
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.last())
                .copied()
                .flatten(),
            Some(1_000.0)
        );
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let önce = grafik.çiz();
        assert!(grafik.y_arcsinh_eşiği_ayarla("y", 0.001));
        assert_ne!(grafik.çiz(), önce);
        assert!(!grafik.y_arcsinh_eşiği_ayarla("y", 0.0));
        Ok(())
    }
}
