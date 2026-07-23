use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ortak_kart_etkileşimleri,
    ÇizimKancasıDüzeni,
};

pub const TRENDLINES_KART_TANIM_ÖRNEĞİ: &str = r#"let (seçenekler, veri) = trendlines_kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;
grafik.seçim_yakınlaştır(0.15, 0.82)?;
let sahne = grafik.çiz();"#;

pub fn trendlines_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(800, 600)?
        .başlık("Trendlines")
        .x_zaman(false)
        .x_aralığını_veriye_yapıştır(true)
        .çizim_kancaları(ÇizimKancasıDüzeni::default().seri_uç_trendleri(5.0))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("Data 1")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("Data 2")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        );
    let veri = HizalıVeri::yeni(
        (0..100).map(f64::from).collect(),
        vec![
            vec![
                309, 317, 322, 304, 305, 317, 319, 321, 317, 321, 322, 329, 319, 313, 313, 321,
                308, 308, 300, 303, 313, 310, 307, 305, 299, 293, 287, 283, 291, 285, 281, 269,
                276, 261, 263, 274, 276, 268, 255, 261, 248, 239, 254, 244, 237, 230, 222, 233,
                229, 221, 222, 243, 247, 233, 247, 228, 229, 231, 232, 235, 237, 225, 195, 186,
                193, 186, 193, 182, 182, 182, 184, 159, 178, 170, 173, 170, 153, 151, 153, 158,
                145, 166, 173, 178, 177, 166, 177, 168, 164, 153, 167, 168, 182, 177, 179, 167,
                161, 179, 182, 173,
            ]
            .into_iter()
            .map(|değer| Some(f64::from(değer)))
            .collect(),
            vec![
                293, 291, 281, 258, 257, 265, 252, 258, 242, 246, 240, 242, 227, 221, 227, 227,
                258, 241, 260, 262, 254, 257, 261, 246, 238, 229, 233, 241, 243, 248, 268, 274,
                277, 285, 275, 280, 262, 258, 263, 252, 265, 270, 249, 233, 242, 233, 223, 215,
                209, 200, 210, 213, 216, 224, 222, 223, 230, 237, 229, 241, 255, 260, 259, 264,
                259, 246, 253, 240, 240, 233, 228, 237, 247, 235, 238, 243, 236, 240, 254, 269,
                259, 272, 266, 258, 281, 282, 280, 280, 277, 277, 297, 301, 310, 313, 305, 306,
                298, 308, 317, 290,
            ]
            .into_iter()
            .map(|değer| Some(f64::from(değer)))
            .collect(),
        ],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kaynak_veri_trend_kancası_ve_veriye_yapışan_aralık_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = trendlines_kartı()?;
        assert_eq!(veri.uzunluk(), 100);
        assert_eq!(veri.seriler().len(), 2);
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied()
                .flatten(),
            Some(309.0)
        );
        assert_eq!(
            veri.seriler()
                .get(1)
                .and_then(|seri| seri.last())
                .copied()
                .flatten(),
            Some(290.0)
        );
        let mut grafik = crate::Grafik::yeni(seçenekler, veri)?;
        let tam_sahne = grafik.çiz();
        assert_eq!(
            tam_sahne
                .komutlar()
                .iter()
                .filter(|komut| {
                    matches!(
                        komut,
                        crate::Komut::KesikliÇizgi { kesik, .. } if (*kesik - 5.0).abs() <= f32::EPSILON
                    )
                })
                .count(),
            2
        );

        assert!(grafik.seçim_yakınlaştır(0.151, 0.817)?);
        let görünür = grafik.görünür_x_aralığı();
        assert_eq!(görünür.en_az, 15.0);
        assert_eq!(görünür.en_çok, 81.0);

        assert!(grafik.tam_görünüm());
        assert!(grafik.tekerlek(0.37, 0.5, 1.0, false)?);
        let tekerlek_aralığı = grafik.görünür_x_aralığı();
        assert_eq!(tekerlek_aralığı.en_az.fract(), 0.0);
        assert_eq!(tekerlek_aralığı.en_çok.fract(), 0.0);
        Ok(())
    }
}
