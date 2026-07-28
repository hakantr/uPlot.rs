use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
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
        .x_eksen_etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre)
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
    use crate::{Aralık, Grafik, Komut, TekerlekEkseni};

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
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-1_000.0, 1_000.0)?);
        let önce = grafik.çiz();
        let etiketler = önce
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::Metin { içerik, .. } => Some(içerik.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>();
        for beklenen in ["-1000", "-100", "-10", "-1", "0", "1", "10", "100", "1000"] {
            assert!(etiketler.contains(&beklenen), "{beklenen}");
        }
        assert!(!etiketler.contains(&"-184.69"));

        assert!(grafik.y_arcsinh_eşiği_ayarla("y", 1_000.0));
        let geniş_merkez = grafik.çiz();
        for beklenen in ["-1000", "0", "1000"] {
            assert!(
                geniş_merkez.komutlar().iter().any(
                    |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == beklenen)
                )
            );
        }
        assert!(grafik.y_arcsinh_eşiği_ayarla("y", 0.001));
        assert_ne!(grafik.çiz(), önce);
        assert!(!grafik.y_arcsinh_eşiği_ayarla("y", 0.0));

        grafik.tam_görünüm();
        assert!(grafik.fiziksel_seçim_yakınlaştır_eksenlerde(0.0, 0.45, 1.0, 0.55, false, true,)?);
        let merkez = grafik.görünür_y_aralığı();
        assert!(merkez.en_az.abs() < 0.01);
        assert!(merkez.en_çok.abs() < 0.01);
        assert!(grafik.önceki_görünüm());
        // Tekerlek yakınlaştırması varsayılan kapalı; test onu
        // araç olarak kullandığı için açıkça açıyor.
        grafik.tekerlek_etkileşimi_ayarla(true);
        assert!(grafik.tekerlek_eksende(0.5, 0.5, 120.0, true, TekerlekEkseni::Y)?);
        let tekerlek = grafik.görünür_y_aralığı();
        assert!((tekerlek.en_az + tekerlek.en_çok).abs() < 1e-9);

        assert!(grafik.seri_görünürlüğünü_ayarla(0, false)?);
        assert!(
            !grafik
                .çiz()
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "#0000ff"))
        );
        assert!(grafik.seri_görünürlüğünü_ayarla(0, true)?);

        let yeni = HizalıVeri::yeni(
            vec![1.0, 2.0, 3.0],
            vec![vec![Some(-100.0), Some(0.0), Some(100.0)]],
        )?;
        grafik.veriyi_ayarla(yeni)?;
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(-100.0, 100.0)?);
        Ok(())
    }
}
