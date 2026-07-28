use uplot_rs_gpui_ornekler::{
    Aralık, CustomScaleÖrneği, EtkileşimSeçenekleri, Grafik, HizalıVeri, TekerlekAyarları,
    TekerlekEkseni, TekerlekKipi, UplotHatası, custom_scales_kartı, ortak_kart_etkileşimleri,
    resize_kartı,
};

#[test]
fn resize_kartı_belirlenimci_gpui_sahnesi_üretir() -> Result<(), UplotHatası> {
    let (ilk_seçenekler, ilk_veri) = resize_kartı(100)?;
    let (ikinci_seçenekler, ikinci_veri) = resize_kartı(100)?;
    let ilk = Grafik::yeni(ilk_seçenekler, ilk_veri)?.çiz();
    let ikinci = Grafik::yeni(ikinci_seçenekler, ikinci_veri)?.çiz();

    assert_eq!(ilk, ikinci);
    assert!(
        ilk.komutlar().iter().any(
            |komut| matches!(komut, uplot_rs_gpui_ornekler::diagnostics::Komut::Metin { içerik, .. } if içerik == "Resize")
        )
    );
    assert!(ilk.komutlar().iter().any(
        |komut| matches!(komut, uplot_rs_gpui_ornekler::diagnostics::Komut::Yol { renk, .. } if renk == "red")
    ));
    assert_eq!(ilk.komutlar().len(), 43);
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
    assert!(!varsayılan.tekerlek_odaksız_etkileşim);
    assert!(!varsayılan.görünüm_geçmişi);
    assert!(!varsayılan.dokunma_etkileşimi);

    let ortak_profil = ortak_kart_etkileşimleri();
    assert!(!ortak_profil.tekerlek_etkileşimi);
    assert!(!ortak_profil.tekerlek_odaksız_etkileşim);
    assert!(ortak_profil.seçim_yakınlaştır);
    assert!(ortak_profil.çift_tıkla_tam_görünüm);
    assert!(ortak_profil.görünüm_geçmişi);
    assert!(ortak_profil.dokunma_etkileşimi);
    assert_eq!(ortak_profil.tekerlek_ayarları.kip, TekerlekKipi::Otomatik);

    let odaksız = ortak_profil.tekerlek_odaksız_etkileşim(true);
    assert!(odaksız.tekerlek_odaksız_etkileşim);
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
    let (seçenekler, veri) = resize_kartı(100)?;
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
    assert!(!grafik.tekerlek(0.5, 0.5, 1.0, false)?);
    assert_eq!(grafik.görünür_x_aralığı(), tam);
    Ok(())
}

#[test]
fn tekerlek_x_ve_y_eksenlerini_fare_odağında_yeniden_ölçekler() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = resize_kartı(100)?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let tam_x = grafik.görünür_x_aralığı();
    let tam_y = grafik.görünür_y_aralığı();

    grafik.tekerlek_etkileşimi_ayarla(true);
    assert!(grafik.tekerlek(0.25, 0.75, 1.0, false)?);
    let yakın_x = grafik.görünür_x_aralığı();
    let yakın_y = grafik.görünür_y_aralığı();
    assert!(yakın_x.en_çok - yakın_x.en_az < tam_x.en_çok - tam_x.en_az);
    assert!(yakın_y.en_çok - yakın_y.en_az < tam_y.en_çok - tam_y.en_az);
    assert!(yakın_x.en_az > tam_x.en_az);
    assert!(yakın_y.en_çok < tam_y.en_çok);
    Ok(())
}

#[test]
fn tekerlek_ekseni_çekirdekte_x_ve_y_olarak_ayrılır() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = resize_kartı(100)?;
    let mut yalnız_x = Grafik::yeni(seçenekler.clone(), veri.clone())?;
    let tam_x = yalnız_x.görünür_x_aralığı();
    let tam_y = yalnız_x.görünür_y_aralığı();
    yalnız_x.tekerlek_etkileşimi_ayarla(true);
    assert!(yalnız_x.tekerlek_eksende(0.25, 0.75, 1.0, false, TekerlekEkseni::X)?);
    assert_ne!(yalnız_x.görünür_x_aralığı(), tam_x);
    assert_eq!(yalnız_x.görünür_y_aralığı(), tam_y);

    let mut yalnız_y = Grafik::yeni(seçenekler, veri)?;
    yalnız_y.tekerlek_etkileşimi_ayarla(true);
    assert!(yalnız_y.tekerlek_eksende(0.25, 0.75, 1.0, false, TekerlekEkseni::Y)?);
    assert_eq!(yalnız_y.görünür_x_aralığı(), tam_x);
    assert_ne!(yalnız_y.görünür_y_aralığı(), tam_y);
    Ok(())
}

#[test]
fn yakınlaştırılmış_görünüm_boşluk_sürüklemesi_için_çekirdekte_taşınır() -> Result<(), UplotHatası>
{
    let (seçenekler, veri) = resize_kartı(100)?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    assert!(!grafik.taşımayı_başlat());
    grafik.tekerlek_etkileşimi_ayarla(true);
    assert!(grafik.tekerlek(0.3, 0.4, 1.0, false)?);
    let önceki_x = grafik.görünür_x_aralığı();
    let önceki_y = grafik.görünür_y_aralığı();

    assert!(grafik.taşımayı_başlat());
    assert!(grafik.taşı(-0.1, 0.1)?);
    grafik.taşımayı_bitir();
    assert!(grafik.görünür_x_aralığı().en_az > önceki_x.en_az);
    assert_ne!(grafik.görünür_y_aralığı(), önceki_y);
    assert!(grafik.önceki_görünüm());
    assert_eq!(grafik.görünür_x_aralığı(), önceki_x);
    Ok(())
}

#[test]
fn zoom_touch_x_ve_y_eksenlerini_odak_çevresinde_yakınlaştırır() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = resize_kartı(100)?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let tam_x = grafik.görünür_x_aralığı();
    let tam_y = grafik.görünür_y_aralığı();

    assert!(grafik.dokunmayı_başlat());
    assert!(grafik.dokunma_yakınlaştır(0.25, 0.75, 2.0)?);
    grafik.dokunmayı_bitir();
    let yakın_x = grafik.görünür_x_aralığı();
    let yakın_y = grafik.görünür_y_aralığı();
    assert!(yakın_x.en_çok - yakın_x.en_az < tam_x.en_çok - tam_x.en_az);
    assert!(yakın_y.en_çok - yakın_y.en_az < tam_y.en_çok - tam_y.en_az);
    assert!(grafik.önceki_görünüm());
    assert_eq!(grafik.görünür_x_aralığı(), tam_x);
    Ok(())
}

#[test]
fn log_x_tekerlek_seçim_taşıma_ve_dokunmayı_dönüşüm_uzayında_tutar() -> Result<(), UplotHatası> {
    fn log_açıklığı(aralık: Aralık) -> f64 {
        aralık.en_çok.log10() - aralık.en_az.log10()
    }

    let (seçenekler, veri) = custom_scales_kartı(CustomScaleÖrneği::LogLog)?;
    let tam = Grafik::yeni(seçenekler.clone(), veri.clone())?.görünür_x_aralığı();
    let tam_açıklık = log_açıklığı(tam);

    let mut tekerlek = Grafik::yeni(seçenekler.clone(), veri.clone())?;
    tekerlek.tekerlek_etkileşimi_ayarla(true);
    assert!(tekerlek.tekerlek_eksende(0.5, 0.5, 10.0, true, TekerlekEkseni::X,)?);
    let beklenen_tekerlek_oranı = 0.75_f64.powf(0.1);
    assert!(
        (log_açıklığı(tekerlek.görünür_x_aralığı()) / tam_açıklık - beklenen_tekerlek_oranı).abs()
            < 1e-12
    );

    let mut seçim = Grafik::yeni(seçenekler.clone(), veri.clone())?;
    assert!(seçim.seçim_yakınlaştır(0.2, 0.8)?);
    let seçim_açıklığı = log_açıklığı(seçim.görünür_x_aralığı());
    assert!((seçim_açıklığı / tam_açıklık - 0.6).abs() < 1e-12);
    assert!(seçim.taşımayı_başlat());
    assert!(seçim.taşı(0.1, 0.0)?);
    seçim.taşımayı_bitir();
    assert!((log_açıklığı(seçim.görünür_x_aralığı()) - seçim_açıklığı).abs() < 1e-12);

    let mut dokunma = Grafik::yeni(seçenekler, veri)?;
    assert!(dokunma.dokunmayı_başlat());
    assert!(dokunma.dokunma_yakınlaştır(0.5, 0.5, 1.05)?);
    dokunma.dokunmayı_bitir();
    assert!((log_açıklığı(dokunma.görünür_x_aralığı()) / tam_açıklık - 1.0 / 1.05).abs() < 1e-12);
    Ok(())
}

#[test]
fn çok_küçük_doğrusal_değerler_zoom_oranını_değiştirmez() -> Result<(), UplotHatası> {
    let (seçenekler, _) = resize_kartı(3)?;
    let veri = HizalıVeri::yeni(
        vec![1e-30, 2e-30, 3e-30],
        vec![vec![Some(2e-30), Some(3e-30), Some(4e-30)]],
    )?;
    let mut grafik = Grafik::yeni(seçenekler, veri)?;
    let önce_x = grafik.görünür_x_aralığı();
    let önce_y = grafik.görünür_y_aralığı();

    grafik.tekerlek_etkileşimi_ayarla(true);
    assert!(grafik.tekerlek(0.5, 0.5, 10.0, true)?);
    let sonra_x = grafik.görünür_x_aralığı();
    let sonra_y = grafik.görünür_y_aralığı();
    let beklenen = 0.75_f64.powf(0.1);
    let x_oranı = (sonra_x.en_çok - sonra_x.en_az) / (önce_x.en_çok - önce_x.en_az);
    let y_oranı = (sonra_y.en_çok - sonra_y.en_az) / (önce_y.en_çok - önce_y.en_az);
    assert!((x_oranı - beklenen).abs() < 1e-12);
    assert!((y_oranı - beklenen).abs() < 1e-12);
    assert!(grafik.çiz().komutlar().iter().any(|komut| matches!(
        komut,
        uplot_rs_gpui_ornekler::diagnostics::Komut::Yol { parçalar, .. }
            if parçalar.iter().flatten().all(|nokta| {
                nokta.x.is_finite() && nokta.y.is_finite()
            })
    )));
    Ok(())
}
