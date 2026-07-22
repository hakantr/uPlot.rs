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
