use uplot_rs::{Grafik, HizalıVeri, UplotHatası, ilk_kart};

#[test]
fn ilk_kart_belirlenimci_svg_üretir() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = ilk_kart()?;
    let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
    let ilk = sahne.svg();
    let ikinci = sahne.svg();

    assert_eq!(ilk, ikinci);
    assert!(ilk.starts_with("<svg"));
    assert!(ilk.contains("İlk kart · sin(x)"));
    assert!(ilk.contains("stroke=\"#dc2626\""));
    assert_eq!(sahne.komutlar().len(), 23);
    Ok(())
}

#[test]
fn hizalı_veri_sırasız_x_değerini_reddeder() {
    let sonuç = HizalıVeri::yeni(
        vec![0.0, 2.0, 1.0],
        vec![vec![Some(1.0), Some(2.0), Some(3.0)]],
    );
    assert_eq!(sonuç, Err(UplotHatası::SırasızX { indeks: 2 }));
}
