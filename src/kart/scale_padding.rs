use super::ortak_kart_etkileşimleri;
use crate::{
    GrafikSeçenekleri, HizalıVeri, SayısalAralıkAyarları, SayısalAralıkParçası, SeriSeçenekleri,
    UplotHatası, YumuşakSınırKipi, YÖlçekSeçenekleri,
};

pub const SCALE_PADDING_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = scale_padding_kartı()?;
// On üç düz seri, uPlot'un sayısal ölçek payı sınamasındaki
// küçük, sıfır çevresi ve büyük değerleri aynen kullanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

/// Resmî `demos/scale-padding.html` kartındaki 10 X değeri ile on üç
/// sabit seriyi kaynak sırasını ve etiketlerini koruyarak oluşturur.
pub fn scale_padding_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    const DÜZEYLER: [f64; 13] = [
        -10_500.0, -10_000.0, -9_500.0, -0.105, -0.100, -0.095, 0.0, 0.095, 0.100, 0.105, 9_500.0,
        10_000.0, 10_500.0,
    ];
    let x = (1_u32..=10).map(f64::from).collect::<Vec<_>>();
    let seriler = DÜZEYLER
        .iter()
        .map(|değer| vec![Some(*değer); x.len()])
        .collect::<Vec<_>>();
    let seçenekler = DÜZEYLER.iter().fold(
        GrafikSeçenekleri::yeni(1600, 600)?
            .başlık("Flat")
            .x_zaman(false)
            .y_ölçeği(YÖlçekSeçenekleri::yeni("y").eksen(true).sayısal_aralık(
                SayısalAralıkAyarları::yeni(
                    SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
                    SayısalAralıkParçası::yeni(0.1, Some(0.0), YumuşakSınırKipi::Koşullu),
                ),
            ))
            .etkileşimler(ortak_kart_etkileşimleri()),
        |seçenekler, değer| {
            seçenekler.seri(SeriSeçenekleri::yeni(değer.to_string()).renk("#ff0000"))
        },
    );
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynak_on_üç_düz_serisi_ve_ölçek_payları_korunur() -> Result<(), UplotHatası> {
        const DÜZEYLER: [f64; 13] = [
            -10_500.0, -10_000.0, -9_500.0, -0.105, -0.100, -0.095, 0.0, 0.095, 0.100, 0.105,
            9_500.0, 10_000.0, 10_500.0,
        ];
        let (seçenekler, veri) = scale_padding_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1600, 600));
        assert_eq!(
            veri.x(),
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]
        );
        assert_eq!(veri.seriler().len(), 13);
        for (seri, düzey) in veri.seriler().iter().zip(DÜZEYLER) {
            assert_eq!(seri, &vec![Some(düzey); 10]);
        }
        assert_eq!(
            seçenekler.seriler.first().map(|seri| seri.etiket.as_str()),
            Some("-10500")
        );
        assert_eq!(
            seçenekler.seriler.last().map(|seri| seri.etiket.as_str()),
            Some("10500")
        );

        let grafik = Grafik::yeni(seçenekler, veri)?;
        let y = grafik.görünür_y_aralığı();
        assert_eq!((y.en_az, y.en_çok), (-13_000.0, 13_000.0));
        let sahne = grafik.çiz();
        let yol_sayısı = sahne
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Yol { .. }))
            .count();
        assert_eq!(yol_sayısı, 13);
        let kaynak_nokta_sayısı = sahne
            .komutlar()
            .iter()
            .filter(|komut| {
                matches!(
                    komut,
                    Komut::Daire {
                        yarıçap,
                        dolgu,
                        çizgi,
                        kalınlık,
                        ..
                    } if *yarıçap == 2.0
                        && dolgu == "#ffffff"
                        && çizgi == "#ff0000"
                        && *kalınlık == 1.0
                )
            })
            .count();
        assert_eq!(kaynak_nokta_sayısı, 130);
        Ok(())
    }

    #[test]
    fn x_yakınlaştırması_sabit_serilerin_kaynak_y_aralığını_değiştirmez() -> Result<(), UplotHatası>
    {
        let (seçenekler, veri) = scale_padding_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;

        assert!(grafik.seçim_yakınlaştır(0.25, 0.75)?);
        let x = grafik.görünür_x_aralığı();
        assert!(x.en_az > 1.0);
        assert!(x.en_çok < 10.0);
        assert_eq!(
            grafik.görünür_y_aralığı(),
            crate::Aralık::yeni(-13_000.0, 13_000.0)?
        );

        assert!(grafik.tam_görünüm());
        assert_eq!(grafik.görünür_x_aralığı(), crate::Aralık::yeni(1.0, 10.0)?);
        Ok(())
    }
}
