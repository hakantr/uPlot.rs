use crate::{
    GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, TooltipDüzeni, UplotHatası,
    ortak_kart_etkileşimleri,
};

pub const TOOLTIPS_KART_TANIM_ÖRNEĞİ: &str = r#"let (seçenekler, veri) = tooltips_kartı()?;
let grafik = Grafik::yeni(seçenekler, veri)?;
let bilgi_kutuları = grafik.tooltip_bilgileri(yatay_oran, dikey_oran);"#;

pub fn tooltips_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Tooltips")
        .x_zaman(false)
        .tooltip(
            TooltipDüzeni::yeni()
                .imleç_durumunu_koru(true)
                .yeniden_kurma_ms(2_000),
        )
        .etkileşimler(ortak_kart_etkileşimleri().imleç_bilgi_kutusu(true))
        .seri(SeriSeçenekleri::yeni("One").renk("red"))
        .seri(SeriSeçenekleri::yeni("Two").renk("blue").göster(false));
    let veri = HizalıVeri::yeni(
        vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        vec![
            vec![40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
                .into_iter()
                .map(Some)
                .collect(),
            vec![18.0, 24.0, 37.0, 55.0, 55.0, 60.0, 63.0]
                .into_iter()
                .map(Some)
                .collect(),
        ],
    )?;
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kaynak_veri_gizli_seri_ve_tooltip_davranışı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = tooltips_kartı()?;
        assert_eq!(veri.x(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        assert_eq!(veri.seriler().len(), 2);
        assert!(seçenekler.seriler.first().is_some_and(|seri| seri.göster));
        assert!(seçenekler.seriler.get(1).is_some_and(|seri| !seri.göster));

        let grafik = crate::Grafik::yeni(seçenekler, veri)?;
        let düzen = grafik
            .tooltip_düzeni()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!(düzen.imleç_durumunu_koru);
        assert_eq!(düzen.yeniden_kurma_ms, Some(2_000));

        let bilgiler = grafik.tooltip_bilgileri(0.5, 0.5);
        assert_eq!(bilgiler.len(), 2);
        assert_eq!(bilgiler.first().map(|bilgi| bilgi.seri), Some(None));
        assert!(
            bilgiler
                .first()
                .is_some_and(|bilgi| bilgi.metin.starts_with("(4.00, "))
        );
        assert_eq!(bilgiler.get(1).map(|bilgi| bilgi.seri), Some(Some(0)));
        assert_eq!(
            bilgiler.get(1).map(|bilgi| bilgi.metin.as_str()),
            Some("(4, 65)")
        );
        assert_eq!(
            bilgiler.get(1).map(|bilgi| bilgi.metin_rengi.as_str()),
            Some("red")
        );
        Ok(())
    }
}
