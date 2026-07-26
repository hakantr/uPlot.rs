use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, OdakDüzeni, OdakStili, SeriSeçenekleri, UplotHatası,
};
use std::sync::OnceLock;

static DALGA_VERİSİ: OnceLock<Result<HizalıVeri, UplotHatası>> = OnceLock::new();

pub const FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ: &str = r##"let kartlar = focus_cursor_kartları()?;
for (örnek, seçenekler, veri) in kartlar {
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusÖrneği {
    İmleç,
    Dinamik,
    KalınlıkVeRenk,
    Performans300,
}

impl FocusÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::İmleç,
        Self::Dinamik,
        Self::KalınlıkVeRenk,
        Self::Performans300,
    ];
    pub fn kimlik(self) -> &'static str {
        match self {
            Self::İmleç => "focus-cursor",
            Self::Dinamik => "focus-cursor-dynamic",
            Self::KalınlıkVeRenk => "focus-cursor-width-stroke",
            Self::Performans300 => "focus-cursor-performance-300",
        }
    }
    pub fn başlık(self) -> &'static str {
        match self {
            Self::İmleç => "Cursor Focus",
            Self::Dinamik => "Dynamic Focus",
            Self::KalınlıkVeRenk => "Width and stroke color change on focus",
            Self::Performans300 => "Performance test (300 series)",
        }
    }

    pub fn durum(self) -> &'static str {
        match self {
            Self::İmleç => "130.000 nokta · 4 seri · bias 1",
            Self::Dinamik => "aynı 130.000×4 veri · 3 görünür seri · prox 30",
            Self::KalınlıkVeRenk => "10 nokta · 2 seri · macenta + 2 px",
            Self::Performans300 => "10 nokta · 300 seri · retained stil güncellemesi",
        }
    }
}

/// Resmî sayfadaki dört bağımsız yüzeyi kaynak sırasıyla döndürür.
///
/// İlk iki yüzey JavaScript kaynağındaki aynı `data` nesnesi gibi tek
/// immutable hizalı depoyu paylaşır; yalnız seri seçenekleri farklıdır.
pub fn focus_cursor_kartları()
-> Result<Vec<(FocusÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    let dalga = dalga_verisi()?;
    Ok(vec![
        (
            FocusÖrneği::İmleç,
            dalga_seçenekleri(FocusÖrneği::İmleç, OdakStili::Opaklık, 0.3, 1_000_000.0, 1)?,
            dalga.clone(),
        ),
        (
            FocusÖrneği::Dinamik,
            dalga_seçenekleri(FocusÖrneği::Dinamik, OdakStili::OdakDışıSiyah, 1.1, 30.0, 0)?,
            dalga,
        ),
        {
            let (seçenekler, veri) = kalınlık_renk_kartı()?;
            (FocusÖrneği::KalınlıkVeRenk, seçenekler, veri)
        },
        {
            let (seçenekler, veri) = performans_kartı()?;
            (FocusÖrneği::Performans300, seçenekler, veri)
        },
    ])
}

pub fn focus_cursor_kartı(
    örnek: FocusÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        FocusÖrneği::İmleç => Ok((
            dalga_seçenekleri(örnek, OdakStili::Opaklık, 0.3, 1_000_000.0, 1)?,
            dalga_verisi()?,
        )),
        FocusÖrneği::Dinamik => Ok((
            dalga_seçenekleri(örnek, OdakStili::OdakDışıSiyah, 1.1, 30.0, 0)?,
            dalga_verisi()?,
        )),
        FocusÖrneği::KalınlıkVeRenk => kalınlık_renk_kartı(),
        FocusÖrneği::Performans300 => performans_kartı(),
    }
}

fn dalga_verisi() -> Result<HizalıVeri, UplotHatası> {
    DALGA_VERİSİ.get_or_init(dalga_verisini_üret).clone()
}

fn dalga_verisini_üret() -> Result<HizalıVeri, UplotHatası> {
    const UZUNLUK: usize = 130_000;
    let mut x = Vec::with_capacity(UZUNLUK);
    let mut sinüs = Vec::with_capacity(UZUNLUK);
    let mut kosinüs = Vec::with_capacity(UZUNLUK);
    let mut logaritma = Vec::with_capacity(UZUNLUK);
    let mut düz = Vec::with_capacity(UZUNLUK);
    for indeks in 0..UZUNLUK {
        let değer = 2.0 * std::f64::consts::PI * indeks as f64 / UZUNLUK as f64;
        x.push(değer);
        sinüs.push(Some(değer.sin()));
        kosinüs.push(Some(değer.cos()));
        // JavaScript `Math.log(0)` için `-Infinity` saklar; çekirdeğin sonlu
        // veri sözleşmesinde bu çizilmeyen kaynak hücresi `None` olur.
        logaritma.push((indeks > 0).then(|| değer.ln()));
        düz.push(Some(1.0));
    }
    HizalıVeri::yeni(x, vec![sinüs, kosinüs, logaritma, düz])
}

fn dalga_seçenekleri(
    örnek: FocusÖrneği,
    stil: OdakStili,
    alfa: f32,
    yakınlık: f32,
    eğilim: i8,
) -> Result<GrafikSeçenekleri, UplotHatası> {
    let odak = OdakDüzeni::yeni(alfa, yakınlık)
        .yön_eğilimi(eğilim)
        .odak_kalınlığı(2.0)
        .stil(stil);
    let mut seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(-2.0, 2.0)?)
        .odak(odak)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(
            SeriSeçenekleri::yeni("sin(x)")
                .renk("#ff0000")
                .dolgu("#ff00001a"),
        )
        .seri(
            SeriSeçenekleri::yeni("cos(x)")
                .renk("#0000ff")
                .dolgu("#0000ff1a"),
        )
        .seri(
            SeriSeçenekleri::yeni("log(x)")
                .renk("#008000")
                .dolgu("#00ff001a"),
        );
    seçenekler = seçenekler.seri(
        SeriSeçenekleri::yeni("flat_one")
            .renk("#800080")
            .göster(örnek == FocusÖrneği::İmleç),
    );
    Ok(seçenekler)
}

fn kalınlık_renk_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        (0..10).map(f64::from).collect(),
        vec![vec![Some(10.0); 10], vec![Some(20.0); 10]],
    )?;
    let seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık(FocusÖrneği::KalınlıkVeRenk.başlık())
        .x_zaman(false)
        .y_aralığı(Aralık::yeni(0.0, 30.0)?)
        .odak(
            OdakDüzeni::yeni(1.0, 1_000_000.0)
                .odak_kalınlığı(2.0)
                .stil(OdakStili::OdaklıMacenta),
        )
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("A").renk("#0000ff"))
        .seri(SeriSeçenekleri::yeni("B").renk("#008000"));
    Ok((seçenekler, veri))
}

fn performans_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let veri = HizalıVeri::yeni(
        (0..10).map(f64::from).collect(),
        (0..300)
            .map(|indeks| vec![Some(indeks as f64); 10])
            .collect(),
    )?;
    let mut seçenekler = GrafikSeçenekleri::yeni(1920, 600)?
        .başlık(FocusÖrneği::Performans300.başlık())
        .x_zaman(false)
        .odak(OdakDüzeni::yeni(0.1, 1_000_000.0))
        .etkileşimler(ortak_kart_etkileşimleri());
    for _ in 0..300 {
        seçenekler = seçenekler.seri(SeriSeçenekleri::yeni("0").renk("#000000"));
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn dört_kaynak_alt_grafiği_ve_odak_stilleri_korunur() -> Result<(), UplotHatası> {
        let kartlar = focus_cursor_kartları()?;
        assert_eq!(
            kartlar
                .iter()
                .map(|(örnek, _, _)| *örnek)
                .collect::<Vec<_>>(),
            FocusÖrneği::TÜMÜ
        );
        let Some((_, ilk_seçenekler, ilk_veri)) = kartlar.first() else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
        };
        let Some((_, ikinci_seçenekler, ikinci_veri)) = kartlar.get(1) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 1 });
        };
        let Some((_, _, üçüncü_veri)) = kartlar.get(2) else {
            return Err(UplotHatası::YetersizVeri { uzunluk: 2 });
        };
        assert!(ilk_veri.aynı_depolamayı_paylaşıyor(ikinci_veri));
        assert!(!ilk_veri.aynı_depolamayı_paylaşıyor(üçüncü_veri));
        assert_eq!(ilk_veri.uzunluk(), 130_000);
        assert_eq!(ilk_veri.seriler().len(), 4);
        assert_eq!(ikinci_veri.seriler().len(), 4);
        assert_eq!(ilk_seçenekler.seriler.len(), 4);
        assert_eq!(ikinci_seçenekler.seriler.len(), 4);
        assert_eq!(
            ikinci_seçenekler.seriler.get(3).map(|seri| seri.göster),
            Some(false)
        );
        assert_eq!(ikinci_seçenekler.odak.map(|odak| odak.alfa), Some(1.1));
        assert_eq!(
            ilk_veri.seriler().get(2).and_then(|seri| seri.first()),
            Some(&None)
        );

        let (seçenekler, veri) = focus_cursor_kartı(FocusÖrneği::KalınlıkVeRenk)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.imleç_odağını_güncelle(0.5, 2.0 / 3.0, 500.0));
        assert_eq!(grafik.odak_serisi(), Some(0));
        assert_eq!(
            grafik.seri_odak_sunumu(0),
            Some(("#ff00ff".to_string(), None, 2.0))
        );
        assert_eq!(
            grafik.seri_odak_sunumu(1),
            Some(("#008000".to_string(), None, 1.0))
        );
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().any(|komut|matches!(komut,Komut::Yol{renk,kalınlık,..} if renk=="#ff00ff"&&(*kalınlık-2.0).abs()<=f32::EPSILON)));
        let (_, performans) = focus_cursor_kartı(FocusÖrneği::Performans300)?;
        assert_eq!(performans.seriler().len(), 300);

        let (seçenekler, dalga) = focus_cursor_kartı(FocusÖrneği::İmleç)?;
        let mut grafik = Grafik::yeni(seçenekler, dalga)?;
        assert!(grafik.imleç_odağını_güncelle(0.0, 0.5, 500.0));
        let sahne = grafik.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "#0000ff4d"))
        );

        let (seçenekler, dalga) = focus_cursor_kartı(FocusÖrneği::Dinamik)?;
        let mut grafik = Grafik::yeni(seçenekler, dalga)?;
        assert!(grafik.imleç_odağını_güncelle(0.0, 0.5, 500.0));
        let sahne = grafik.çiz();
        assert!(
            sahne
                .komutlar()
                .iter()
                .any(|komut| matches!(komut, Komut::Yol { renk, .. } if renk == "#000000"))
        );
        assert!(grafik.imleç_odağını_temizle());
        assert_eq!(grafik.odak_serisi(), None);
        Ok(())
    }

    #[test]
    fn bias_ve_prox_kaynak_semantiğini_korur() -> Result<(), UplotHatası> {
        let veri = HizalıVeri::yeni(
            vec![0.0],
            vec![vec![Some(-1.0)], vec![Some(0.5)], vec![Some(1.5)]],
        )?;
        let seçenekler = GrafikSeçenekleri::yeni(400, 300)?
            .x_zaman(false)
            .y_aralığı(Aralık::yeni(-2.0, 2.0)?)
            .odak(OdakDüzeni::yeni(1.0, 1_000.0).yön_eğilimi(1))
            .seri(SeriSeçenekleri::yeni("negatif"))
            .seri(SeriSeçenekleri::yeni("yakın"))
            .seri(SeriSeçenekleri::yeni("uzak"));
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.imleç_odağını_güncelle(0.5, 0.25, 200.0));
        assert_eq!(grafik.odak_serisi(), Some(2));

        let (seçenekler, veri) = focus_cursor_kartı(FocusÖrneği::Dinamik)?;
        let mut dinamik = Grafik::yeni(seçenekler, veri)?;
        assert!(!dinamik.imleç_odağını_güncelle(0.0, 0.01, 500.0));
        assert_eq!(dinamik.odak_serisi(), None);
        assert!(dinamik.imleç_odağını_güncelle(0.0, 0.25, 500.0));
        assert!(dinamik.odak_serisi().is_some());
        Ok(())
    }
}
