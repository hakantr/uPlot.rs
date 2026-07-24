use uplot_rs::{Grafik, Komut, UplotHatası, area_fill_kartı};

#[test]
fn area_fill_kaynak_verisini_ve_dolgularını_korur() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = area_fill_kartı()?;
    assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1920, 600));
    assert_eq!(veri.x().first().copied(), Some(1.0));
    assert_eq!(veri.x().last().copied(), Some(30.0));
    assert_eq!(veri.seriler().len(), 3);
    assert!(veri.seriler().iter().all(|seri| seri.len() == 30));

    let grafik = Grafik::yeni(seçenekler, veri)?;
    let boşta = grafik.boşta_lejant_değerleri();
    assert!(boşta.is_some(), "kaynak fmtVal son değerleri bulunamadı");
    let boşta = boşta.unwrap_or_default();
    assert_eq!(boşta.len(), 3);
    assert!(boşta.iter().all(Option::is_some));
    let sahne = grafik.çiz();
    assert_eq!(
        sahne
            .komutlar()
            .iter()
            .filter(|komut| matches!(komut, Komut::Alan { .. }))
            .count(),
        3
    );

    let svg = sahne.svg();
    assert_eq!(svg.matches("stroke=\"none\"").count(), 3);
    assert!(svg.contains("fill=\"#ff00001a\""));
    assert!(svg.contains("fill=\"#00ff001a\""));
    assert!(svg.contains("fill=\"#0000ff1a\""));
    Ok(())
}

#[test]
fn area_fill_ortak_yakinlastirma_ve_gecmis_davranislarini_devralir() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = area_fill_kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let tam = grafik.görünür_x_aralığı();

    assert!(grafik.seçim_yakınlaştır(0.2, 0.8)?);
    assert!(grafik.yakınlaştırılmış());
    assert!(grafik.geri_var());
    assert!(grafik.önceki_görünüm());
    assert_eq!(grafik.görünür_x_aralığı(), tam);

    assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
    assert!(grafik.yakınlaştırılmış());
    assert!(grafik.tam_görünüm());
    assert_eq!(grafik.görünür_x_aralığı(), tam);

    assert!(grafik.dokunmayı_başlat());
    assert!(grafik.dokunma_yakınlaştır(0.5, 0.5, 1.25)?);
    grafik.dokunmayı_bitir();
    assert!(grafik.yakınlaştırılmış());
    Ok(())
}

#[test]
fn area_fill_yakinlastirmada_dolguyu_dikey_seritlere_bolmez() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = area_fill_kartı()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    for _ in 0..4 {
        assert!(grafik.tekerlek(0.5, 0.45, 1.0, false)?);
    }

    let sahne = grafik.çiz_görünür_boyutta(1200, 600);
    let alanlar = sahne.komutlar().iter().filter_map(|komut| {
        if let Komut::Alan { çokgenler, .. } = komut {
            Some(çokgenler)
        } else {
            None
        }
    });
    let mut alan_sayısı = 0;
    for çokgenler in alanlar {
        alan_sayısı += 1;
        assert_eq!(çokgenler.len(), 1);
        assert!(çokgenler.iter().flatten().all(|nokta| {
            (64.0..=1176.0).contains(&nokta.x) && (48.0..=552.0).contains(&nokta.y)
        }));
    }
    assert_eq!(alan_sayısı, 3);
    Ok(())
}
