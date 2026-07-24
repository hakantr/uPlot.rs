use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, Grafik, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası,
    ZoomRangerSeçenekleri,
};

pub const ZOOM_WHEEL_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = zoom_wheel_kartı()?;
// Resmî wheelZoomPlugin'in 0.75 katsayılı, fare odaklı X/Y
// yakınlaştırması ve sınır sıkıştırması çekirdekte uygulanır.
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

pub const ZOOM_FETCH_KANIT_ÖRNEĞİ: &str = r##"let mut akış = ZoomFetchAkışı::yeni()?;
let istek = akış.aralık_isteği(0.25, 0.75)?;
akış.kaynak_yanıtını_uygula(istek)?;
akış.tam_aralığı_yükle()?;"##;

pub const ZOOM_RANGER_GRIPS_KANIT_ÖRNEĞİ: &str = r##"let mut ranger = grafik.zoom_ranger_durumu()?;
ranger.pencereyi_taşı(0.25);
ranger.sol_tutamağı_ayarla(2.0);
ranger.sağ_tutamağı_ayarla(5.0);
grafik.zoom_ranger_uygula(ranger);"##;

pub const ZOOM_RANGER_XY_KANIT_ÖRNEĞİ: &str = r##"let mut ranger = grafik.zoom_ranger_durumu()?;
ranger.sol_tutamağı_ayarla(2.0);
ranger.sağ_tutamağı_ayarla(5.0);
ranger.alt_tutamağı_ayarla(-0.5);
ranger.üst_tutamağı_ayarla(0.5);
// dist=10 ve uni=10 hareketi Yok, X, Y veya XY olarak çözümler.
let eksen = ranger.uyarlanabilir_sürükleme_ekseni(18.0, 15.0);
grafik.zoom_ranger_uygula(ranger);"##;

/// Seçim aralığını veri sağlayıcı isteğine dönüştüren, platformdan bağımsız akış.
pub struct ZoomFetchAkışı {
    grafik: Grafik,
}

impl ZoomFetchAkışı {
    pub fn yeni() -> Result<Self, UplotHatası> {
        let veri = tam_zoom_fetch_verisi()?;
        let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
            .başlık("Fetch Zoom")
            .x_zaman(false)
            .etkileşimler(ortak_kart_etkileşimleri().seçim_yakınlaştır(false))
            .seri(SeriSeçenekleri::yeni("Fetched").renk("red"));
        Ok(Self {
            grafik: Grafik::yeni(seçenekler, veri)?,
        })
    }

    pub fn grafik(&self) -> &Grafik {
        &self.grafik
    }

    pub fn aralık_isteği(&self, başlangıç: f64, bitiş: f64) -> Result<Aralık, UplotHatası> {
        self.grafik.x_aralığı_oranlardan(başlangıç, bitiş)
    }

    pub fn kaynak_yanıtını_uygula(&mut self, aralık: Aralık) -> Result<(), UplotHatası> {
        let veri = HizalıVeri::yeni(
            vec![3.0, 4.0, 5.0, 6.0],
            vec![vec![30.0, 23.0, 35.0, 27.0].into_iter().map(Some).collect()],
        )?;
        self.grafik.veriyi_x_aralığında_ayarla(veri, aralık)
    }

    pub fn tam_aralığı_yükle(&mut self) -> Result<(), UplotHatası> {
        self.grafik.veriyi_ayarla(tam_zoom_fetch_verisi()?)
    }
}

fn tam_zoom_fetch_verisi() -> Result<HizalıVeri, UplotHatası> {
    HizalıVeri::yeni(
        vec![1., 2., 3., 4., 5., 6., 7., 9., 10.],
        vec![
            vec![40., 43., 60., 65., 71., 73., 40., 22., 20.]
                .into_iter()
                .map(Some)
                .collect(),
        ],
    )
}

/// Resmî `demos/zoom-wheel.html` kartının boyutunu, iki serisini ve bütün
/// sayısal veri noktalarını korur. Eklenti davranışı ortak çekirdek profilidir.
pub fn zoom_wheel_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0].to_vec();
    let bir = [40.0, 43.0, 60.0, 65.0, 71.0, 73.0, 80.0]
        .into_iter()
        .map(Some)
        .collect();
    let iki = [18.0, 24.0, 37.0, 55.0, 55.0, 60.0, 63.0]
        .into_iter()
        .map(Some)
        .collect();
    let seçenekler = GrafikSeçenekleri::yeni(600, 400)?
        .başlık("Wheel Zoom & Drag")
        .x_zaman(false)
        .etkileşimler(
            ortak_kart_etkileşimleri().zoom_ranger(
                ZoomRangerSeçenekleri::default()
                    .etkin(true)
                    .eksenler(true, true)
                    .sürükleme_eşikleri(10.0, 10.0),
            ),
        )
        .seri(SeriSeçenekleri::yeni("One").renk("#ff0000"))
        .seri(SeriSeçenekleri::yeni("Two").renk("#0000ff"));
    Ok((seçenekler, HizalıVeri::yeni(x, vec![bir, iki])?))
}

/// `zoom-ranger-xy` kaynağındaki 100 örnekli sinüsü ve başlangıç X seçimini
/// ayrı katalog kartı üretmeden, ortak ranger doğrulama yüzeyi için hazırlar.
pub fn zoom_ranger_xy_grafiği() -> Result<Grafik, UplotHatası> {
    let nokta_sayısı = 100;
    let x: Vec<f64> = (0..nokta_sayısı)
        .map(|indeks| 2.0 * std::f64::consts::PI * indeks as f64 / nokta_sayısı as f64)
        .collect();
    let y = x.iter().map(|değer| Some(değer.sin())).collect();
    let seçenekler = GrafikSeçenekleri::yeni(800, 400)?
        .x_zaman(false)
        .etkileşimler(
            ortak_kart_etkileşimleri().zoom_ranger(
                ZoomRangerSeçenekleri::default()
                    .etkin(true)
                    .eksenler(true, true)
                    .sürükleme_eşikleri(10.0, 10.0),
            ),
        )
        .seri(SeriSeçenekleri::yeni("sin(x)").renk("purple"));
    let mut grafik = Grafik::yeni(seçenekler, HizalıVeri::yeni(x, vec![y])?)?;
    let mut ranger = grafik.zoom_ranger_durumu()?;
    ranger.sol_tutamağı_ayarla(1.0);
    ranger.sağ_tutamağı_ayarla(4.5);
    grafik.zoom_ranger_uygula(ranger);
    Ok(grafik)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn kaynak_verisi_ve_resmî_tekerlek_katsayısı_korunur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = zoom_wheel_kartı()?;
        assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (600, 400));
        assert_eq!(veri.seriler().len(), 2);
        assert_eq!(
            veri.seriler()
                .first()
                .and_then(|seri| seri.first())
                .copied(),
            Some(Some(40.0))
        );
        assert_eq!(
            veri.seriler().last().and_then(|seri| seri.last()).copied(),
            Some(Some(63.0))
        );
        assert_eq!(
            seçenekler.etkileşimler.tekerlek_ayarları.ayrık_katsayı,
            0.75
        );

        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
        assert!(grafik.yakınlaştırılmış());
        assert!(grafik.görünür_x_aralığı().en_az > 1.0);
        Ok(())
    }

    #[test]
    fn zoom_fetch_seçimi_isteğe_dönüşür_veri_ve_görünüm_atomik_güncellenir()
    -> Result<(), UplotHatası> {
        let mut akış = ZoomFetchAkışı::yeni()?;
        let istek = akış.aralık_isteği(0.25, 0.75)?;
        assert_eq!(istek, Aralık::yeni(3.25, 7.75)?);
        akış.kaynak_yanıtını_uygula(istek)?;
        assert_eq!(akış.grafik().görünür_x_aralığı(), istek);
        assert_eq!(akış.grafik().en_yakın_nokta(0.0, 0), Some((4.0, 23.0)));
        akış.tam_aralığı_yükle()?;
        assert_eq!(akış.grafik().görünür_x_aralığı(), Aralık::yeni(1.0, 10.0)?);
        Ok(())
    }

    #[test]
    fn ranger_taşıma_tutamaklar_ve_çift_yönlü_senkronu_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = zoom_wheel_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.seçim_yakınlaştır(0.0, 0.5)?);
        let mut ranger = grafik.zoom_ranger_durumu()?;
        assert_eq!(ranger.seçim_oranları(), (0.0, 0.5));
        assert!(ranger.pencereyi_taşı(10.0));
        assert_eq!(ranger.seçim_aralığı(), Aralık::yeni(4.0, 7.0)?);
        assert!(ranger.sol_tutamağı_ayarla(5.0));
        assert!(ranger.sağ_tutamağı_ayarla(6.0));
        assert!(grafik.zoom_ranger_uygula(ranger));
        assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(5.0, 6.0)?);
        let mut ranger = grafik.zoom_ranger_durumu()?;
        assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
        assert!(ranger.ana_görünümle_senkronla(grafik.görünür_x_aralığı())?);
        assert_eq!(ranger.seçim_aralığı(), grafik.görünür_x_aralığı());
        Ok(())
    }

    #[test]
    fn ranger_xy_seçimi_adaptif_sürükleme_ve_iki_yönlü_senkronu_korur() -> Result<(), UplotHatası> {
        use crate::ZoomRangerSürüklemeEkseni;

        let (seçenekler, veri) = zoom_wheel_kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let mut ranger = grafik.zoom_ranger_durumu()?;
        let tam_y = ranger.y_tam_aralık();
        let y_uzunluğu = tam_y.en_çok - tam_y.en_az;
        assert_eq!(
            ranger.uyarlanabilir_sürükleme_ekseni(3.0, 4.0),
            ZoomRangerSürüklemeEkseni::Yok
        );
        assert_eq!(
            ranger.uyarlanabilir_sürükleme_ekseni(24.0, 2.0),
            ZoomRangerSürüklemeEkseni::X
        );
        assert_eq!(
            ranger.uyarlanabilir_sürükleme_ekseni(2.0, 24.0),
            ZoomRangerSürüklemeEkseni::Y
        );
        assert_eq!(
            ranger.uyarlanabilir_sürükleme_ekseni(18.0, 15.0),
            ZoomRangerSürüklemeEkseni::XY
        );
        assert!(ranger.alt_tutamağı_ayarla(tam_y.en_az + y_uzunluğu * 0.25));
        assert!(ranger.üst_tutamağı_ayarla(tam_y.en_az + y_uzunluğu * 0.75));
        assert!(grafik.zoom_ranger_uygula(ranger));
        let (alt, üst) = grafik.zoom_ranger_durumu()?.y_seçim_oranları();
        assert!((alt - 0.25).abs() < 1e-12);
        assert!((üst - 0.75).abs() < 1e-12);

        assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?);
        let görünür_x = grafik.görünür_x_aralığı();
        let görünür_y = grafik.görünür_y_aralığı();
        let ranger = grafik.zoom_ranger_durumu()?;
        assert_eq!(ranger.seçim_aralığı(), görünür_x);
        assert_eq!(ranger.y_seçim_aralığı(), görünür_y);
        Ok(())
    }

    #[test]
    fn ranger_xy_resmî_sinüs_verisini_ve_ilk_x_seçimini_korur() -> Result<(), UplotHatası> {
        let grafik = zoom_ranger_xy_grafiği()?;
        assert_eq!(grafik.boyut(), (800, 400));
        assert_eq!(grafik.görünür_x_aralığı(), Aralık::yeni(1.0, 4.5)?);
        let ilk_x = 2.0 * std::f64::consts::PI * 16.0 / 100.0;
        let son_x = 2.0 * std::f64::consts::PI * 71.0 / 100.0;
        assert_eq!(grafik.en_yakın_nokta(0.0, 0), Some((ilk_x, ilk_x.sin())));
        assert_eq!(grafik.en_yakın_nokta(1.0, 0), Some((son_x, son_x.sin())));
        Ok(())
    }
}
