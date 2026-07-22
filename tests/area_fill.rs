use uplot_rs::{Grafik, Komut, UplotHatası, area_fill_kartı};

#[test]
fn area_fill_kaynak_verisini_ve_dolgularını_korur() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = area_fill_kartı()?;
    assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1920, 600));
    assert_eq!(veri.x().first().copied(), Some(1.0));
    assert_eq!(veri.x().last().copied(), Some(30.0));
    assert_eq!(veri.seriler().len(), 3);
    assert!(veri.seriler().iter().all(|seri| seri.len() == 30));

    let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
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
