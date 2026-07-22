use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, OdakDüzeni, OdakStili, SeriSeçenekleri, UplotHatası,
};

pub const FOCUS_CURSOR_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = focus_cursor_kartı(FocusÖrneği::Dinamik)?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

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
}

pub fn focus_cursor_kartı(
    örnek: FocusÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        FocusÖrneği::İmleç => {
            dalga_kartı(örnek, true, OdakStili::Opaklık, 0.3, 1_000_000.0, 1)
        }
        FocusÖrneği::Dinamik => {
            dalga_kartı(örnek, false, OdakStili::OdakDışıSiyah, 1.0, 30.0, 0)
        }
        FocusÖrneği::KalınlıkVeRenk => kalınlık_renk_kartı(),
        FocusÖrneği::Performans300 => performans_kartı(),
    }
}

fn dalga_kartı(
    örnek: FocusÖrneği,
    düz_seri: bool,
    stil: OdakStili,
    alfa: f32,
    yakınlık: f32,
    eğilim: i8,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
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
        logaritma.push((indeks > 0).then(|| değer.ln()));
        düz.push(Some(1.0));
    }
    let mut seriler = vec![sinüs, kosinüs, logaritma];
    if düz_seri {
        seriler.push(düz);
    }
    let veri = HizalıVeri::yeni(x, seriler)?;
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
    if düz_seri {
        seçenekler = seçenekler.seri(SeriSeçenekleri::yeni("flat_one").renk("#800080"));
    }
    Ok((seçenekler, veri))
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
        let (seçenekler, veri) = focus_cursor_kartı(FocusÖrneği::KalınlıkVeRenk)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.imleç_odağını_güncelle(0.5, 2.0 / 3.0, 500.0));
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().any(|komut|matches!(komut,Komut::Yol{renk,kalınlık,..} if renk=="#ff00ff"&&(*kalınlık-2.0).abs()<=f32::EPSILON)));
        let (_, performans) = focus_cursor_kartı(FocusÖrneği::Performans300)?;
        assert_eq!(performans.seriler().len(), 300);
        let (_, dalga) = focus_cursor_kartı(FocusÖrneği::İmleç)?;
        assert_eq!(dalga.uzunluk(), 130_000);
        assert_eq!(dalga.seriler().len(), 4);
        assert_eq!(
            dalga.seriler().get(2).and_then(|seri| seri.first()),
            Some(&None)
        );

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
        Ok(())
    }
}
