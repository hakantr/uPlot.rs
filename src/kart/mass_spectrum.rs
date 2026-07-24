use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

const KAYNAK_CSV: &str = include_str!("veri/ms_spectrum.csv");
const VARLIK: &str = "demos/data/ms_spectrum.csv";

pub const MASS_SPECTRUM_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = mass_spectrum_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

pub fn mass_spectrum_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = kaynak_veri()?;
    let seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Mass spectrum")
        .x_zaman(false)
        .x_eksen_etiketi("m/z")
        .y_eksen_etiketi("relative abundance (%)")
        .kütle_spektrumu_y_aralığı(true)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Value").renk("#305CDE"));
    Ok((seçenekler, veri))
}

fn kaynak_veri() -> Result<HizalıVeri, UplotHatası> {
    let mut x = Vec::with_capacity(41_986);
    let mut y = Vec::with_capacity(41_986);
    for (indeks, satır) in KAYNAK_CSV.lines().skip(1).enumerate() {
        let satır_no = indeks.saturating_add(2);
        let Some((mz, yoğunluk)) = satır.split_once(',') else {
            return Err(UplotHatası::GeçersizVarlıkSatırı {
                varlık: VARLIK,
                satır: satır_no,
            });
        };
        let mz = sayı(mz, satır_no)?;
        let yoğunluk = sayı(yoğunluk, satır_no)?;
        x.push(mz);
        y.push(Some(yoğunluk));
    }
    HizalıVeri::yeni(x, vec![y])
}

fn sayı(metin: &str, satır: usize) -> Result<f64, UplotHatası> {
    metin
        .parse::<f64>()
        .ok()
        .filter(|değer| değer.is_finite())
        .ok_or(UplotHatası::GeçersizVarlıkSatırı {
            varlık: VARLIK,
            satır,
        })
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Aralık, Grafik};

    #[test]
    fn kaynak_csv_aynen_yüklenir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = mass_spectrum_kartı()?;
        assert_eq!(veri.uzunluk(), 41_986);
        assert_eq!(veri.x().first().copied(), Some(260.01));
        assert_eq!(veri.x().last().copied(), Some(1_100.0));
        assert_eq!(
            veri.seriler().first().and_then(|seri| seri
                .iter()
                .copied()
                .flatten()
                .max_by(f64::total_cmp)),
            Some(100.0)
        );
        assert_eq!(seçenekler.x_eksen_etiketi, "m/z");
        assert_eq!(seçenekler.y_eksen_etiketi, "relative abundance (%)");
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.etiket.as_str()),
            Some("Value")
        );
        assert!(seçenekler.kütle_spektrumu_y_aralığı);
        Ok(())
    }

    #[test]
    fn kaynak_y_aralığı_min_max_ve_düz_değer_kuralını_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = mass_spectrum_kartı()?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 100.0)?);

        let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
            .x_zaman(false)
            .kütle_spektrumu_y_aralığı(true)
            .seri(SeriSeçenekleri::yeni("flat"));
        let sıfır = Grafik::yeni(
            seçenekler.clone(),
            HizalıVeri::yeni(vec![1.0, 2.0], vec![vec![Some(0.0), Some(0.0)]])?,
        )?;
        assert_eq!(sıfır.görünür_y_aralığı(), Aralık::yeni(0.0, 100.0)?);
        let düz = Grafik::yeni(
            seçenekler,
            HizalıVeri::yeni(vec![1.0, 2.0], vec![vec![Some(7.0), Some(7.0)]])?,
        )?;
        assert_eq!(düz.görünür_y_aralığı(), Aralık::yeni(0.0, 14.0)?);
        Ok(())
    }

    #[test]
    fn kaynak_çizgi_ve_eksen_etiketleri_sahneye_çıkar() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = mass_spectrum_kartı()?;
        let svg = Grafik::yeni(seçenekler, veri)?.çiz().svg();
        assert!(svg.contains("Mass spectrum"));
        assert!(svg.contains("m/z"));
        assert!(svg.contains("relative abundance (%)"));
        assert!(svg.contains("#305CDE"));
        Ok(())
    }

    #[test]
    fn yoğun_kaynakta_imleç_ve_zoom_y_aralığı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = mass_spectrum_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let orta = grafik
            .en_yakın_noktalar(0.5)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!((orta.0 - 680.005).abs() < 0.02);
        assert_eq!(orta.1.len(), 1);

        assert!(grafik.seçim_yakınlaştır(0.49, 0.51)?);
        let yakın_y = grafik.görünür_y_aralığı();
        assert!(yakın_y.en_az >= 0.0);
        assert!(yakın_y.en_çok <= 100.0);
        assert!(yakın_y.en_çok > yakın_y.en_az);
        Ok(())
    }
}
