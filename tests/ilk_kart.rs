use uplot_rs::{
    Aralık, EtkileşimSeçenekleri, Grafik, HizalıVeri, TekerlekAyarları, TekerlekKipi, UplotHatası,
    ilk_kart, ilk_kart_etkileşimleri,
};

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

#[test]
fn tekerlek_yakınlaştırması_farenin_göreli_konumunu_korur() -> Result<(), UplotHatası> {
    let tam = Aralık::yeni(0.0, 100.0)?;
    let mevcut = Aralık::yeni(20.0, 80.0)?;

    let yakın = mevcut.tekerlek_yakınlaştır(tam, 30.0, true)?;
    assert!((yakın.en_az - 22.5).abs() < f64::EPSILON);
    assert!((yakın.en_çok - 67.5).abs() < f64::EPSILON);

    let kenar = mevcut.tekerlek_yakınlaştır(tam, 20.0, true)?;
    assert!((kenar.en_az - 20.0).abs() < f64::EPSILON);
    assert!((kenar.en_çok - 65.0).abs() < f64::EPSILON);
    Ok(())
}

#[test]
fn isteğe_bağlı_etkileşimler_kart_bazında_açılır() {
    let varsayılan = EtkileşimSeçenekleri::default();
    assert!(!varsayılan.tekerlek_etkileşimi);
    assert!(!varsayılan.görünüm_geçmişi);

    let ilk_kart = ilk_kart_etkileşimleri();
    assert!(ilk_kart.tekerlek_etkileşimi);
    assert!(ilk_kart.seçim_yakınlaştır);
    assert!(ilk_kart.çift_tıkla_tam_görünüm);
    assert!(ilk_kart.görünüm_geçmişi);
    assert_eq!(ilk_kart.tekerlek_ayarları.kip, TekerlekKipi::Otomatik);
}

#[test]
fn hassas_tekerlek_delta_büyüklüğüyle_orantılıdır() -> Result<(), UplotHatası> {
    let tam = Aralık::yeni(0.0, 100.0)?;
    let mevcut = Aralık::yeni(20.0, 80.0)?;
    let ayarlar = TekerlekAyarları::default();

    let küçük = mevcut.uyarlanabilir_tekerlek_yakınlaştır(tam, 30.0, 1.0, true, ayarlar)?;
    assert_eq!(küçük, mevcut);

    let hassas = mevcut.uyarlanabilir_tekerlek_yakınlaştır(tam, 30.0, 100.0, true, ayarlar)?;
    let ayrık = mevcut.uyarlanabilir_tekerlek_yakınlaştır(tam, 30.0, 1.0, false, ayarlar)?;
    let büyük_ayrık =
        mevcut.uyarlanabilir_tekerlek_yakınlaştır(tam, 30.0, 3.0, false, ayarlar)?;
    assert!((hassas.en_az - ayrık.en_az).abs() < f64::EPSILON);
    assert!((hassas.en_çok - ayrık.en_çok).abs() < f64::EPSILON);
    assert_eq!(büyük_ayrık, ayrık);

    let onda_bir = mevcut.uyarlanabilir_tekerlek_yakınlaştır(
        tam,
        30.0,
        10.0,
        true,
        TekerlekAyarları::default().kip(TekerlekKipi::Otomatik),
    )?;
    assert!(onda_bir.en_çok - onda_bir.en_az > hassas.en_çok - hassas.en_az);
    assert!(onda_bir.en_çok - onda_bir.en_az < mevcut.en_çok - mevcut.en_az);
    Ok(())
}

#[test]
fn tekerlek_uzaklaştırması_tam_aralıkta_sınırlanır() -> Result<(), UplotHatası> {
    let tam = Aralık::yeni(0.0, 100.0)?;
    let mevcut = Aralık::yeni(20.0, 80.0)?;

    let uzak = mevcut.tekerlek_yakınlaştır(tam, 30.0, false)?;
    assert!((uzak.en_az - 50.0 / 3.0).abs() < f64::EPSILON * 16.0);
    assert!((uzak.en_çok - 290.0 / 3.0).abs() < f64::EPSILON * 16.0);
    assert_eq!(uzak.tekerlek_yakınlaştır(tam, 30.0, false)?, tam);
    Ok(())
}

#[test]
fn grafik_etkileşim_durumunu_çekirdekte_yönetir() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = ilk_kart()?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let tam = grafik.görünür_x_aralığı();

    assert!(grafik.seçim_yakınlaştır(0.25, 0.75)?);
    let seçilen = grafik.görünür_x_aralığı();
    assert!(grafik.yakınlaştırılmış());
    assert!(grafik.geri_var());
    assert!(seçilen.en_az > tam.en_az);
    assert!(seçilen.en_çok < tam.en_çok);

    assert!(grafik.önceki_görünüm());
    assert_eq!(grafik.görünür_x_aralığı(), tam);
    assert!(!grafik.yakınlaştırılmış());

    grafik.tekerlek_etkileşimi_ayarla(false);
    assert!(!grafik.tekerlek(0.5, 1.0, false)?);
    assert_eq!(grafik.görünür_x_aralığı(), tam);
    Ok(())
}
