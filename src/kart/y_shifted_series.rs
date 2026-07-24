use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};

pub const Y_SHIFTED_SERIES_KANIT_TOHUMU: u32 = 0x59_53_48_46;
pub const Y_SHIFTED_SERIES_ARALIK_MS: u64 = 2_000;

pub const Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let mut akış = YShiftedSeriesAkışı::yeni()?;
let (seçenekler, veri) = akış.kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;

// Resmî demo her 2 saniyede aynı ham veriyi normal/kaydırılmış kipte yeniden kurar.
let (seçenekler, veri) = akış.ilerlet()?;
grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YShiftedSeriesKipi {
    Kaydırılmış,
    Normal,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YShiftedSeriesAkışı {
    ham_seriler: Vec<Vec<Option<f64>>>,
    kip: YShiftedSeriesKipi,
}

impl YShiftedSeriesAkışı {
    pub fn yeni() -> Result<Self, UplotHatası> {
        let mut rastgele = KanıtRastgele::yeni(Y_SHIFTED_SERIES_KANIT_TOHUMU);
        let ham_seriler = (0..3)
            .map(|_| {
                (0..30)
                    .map(|_| Some((rastgele.sonraki() * 11.0).floor()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        if ham_seriler.iter().flatten().any(|değer| {
            değer.is_none_or(|değer| !değer.is_finite() || !(0.0..=10.0).contains(&değer))
        }) {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "YShiftedSeriesAkışı",
                açıklama: "kanıt verisi 0..10 tamsayı aralığında olmalıdır".to_string(),
            });
        }
        Ok(Self {
            ham_seriler,
            kip: YShiftedSeriesKipi::Kaydırılmış,
        })
    }

    pub const fn kip(&self) -> YShiftedSeriesKipi {
        self.kip
    }

    pub fn ham_seriler(&self) -> &[Vec<Option<f64>>] {
        &self.ham_seriler
    }

    pub fn kartı(&self) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        y_shifted_series_kartı_kipte(self.kip, &self.ham_seriler)
    }

    pub fn ilerlet(&mut self) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        self.kip = match self.kip {
            YShiftedSeriesKipi::Kaydırılmış => YShiftedSeriesKipi::Normal,
            YShiftedSeriesKipi::Normal => YShiftedSeriesKipi::Kaydırılmış,
        };
        self.kartı()
    }
}

pub fn y_shifted_series_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    YShiftedSeriesAkışı::yeni()?.kartı()
}

fn y_shifted_series_kartı_kipte(
    kip: YShiftedSeriesKipi,
    ham_seriler: &[Vec<Option<f64>>],
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    if ham_seriler.len() != 3 || ham_seriler.iter().any(|seri| seri.len() != 30) {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "YShiftedSeriesAkışı",
            açıklama: "kaynak üç adet 30 noktalı seri gerektirir".to_string(),
        });
    }
    let kaydırılmış = kip == YShiftedSeriesKipi::Kaydırılmış;
    let y_üst = if kaydırılmış { 30.0 } else { 10.0 };
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Y-shifted Series")
        .x_zaman(false)
        .birincil_y_eksen_genişliği(70.0)
        .y_aralığı(Aralık::yeni(0.0, y_üst)?)
        .etkileşimler(ortak_kart_etkileşimleri());
    if kaydırılmış {
        let etiketler = (0..=30).map(|değer| {
            let etiket = if değer % 10 == 0 {
                format!("Core #{}", değer / 10 + 1)
            } else {
                (değer % 10).to_string()
            };
            (f64::from(değer), etiket)
        });
        seçenekler = seçenekler.y_özel_etiketler(etiketler);
    }
    let renkler = ["red", "green", "blue"];
    let dolgular = [
        "rgba(255,0,0,0.1)",
        "rgba(0,255,0,0.1)",
        "rgba(0,0,255,0.1)",
    ];
    let mut çizim_serileri = Vec::with_capacity(3);
    for (indeks, ((ham, renk), dolgu)) in ham_seriler.iter().zip(renkler).zip(dolgular).enumerate()
    {
        let kaydırma = if kaydırılmış {
            indeks as f64 * 10.0
        } else {
            0.0
        };
        let mut seri = SeriSeçenekleri::yeni(format!("Core #{}", indeks + 1))
            .renk(renk)
            .dolgu(dolgu)
            .dolgu_tabanı(kaydırma)
            .lejant_değerleri(ham.clone());
        if indeks == 2 {
            seri = seri.çubuk(true);
        }
        seçenekler = seçenekler.seri(seri);
        çizim_serileri.push(
            ham.iter()
                .map(|değer| değer.map(|değer| değer + kaydırma))
                .collect(),
        );
    }
    let x = (1..=30).map(f64::from).collect();
    Ok((seçenekler, HizalıVeri::yeni(x, çizim_serileri)?))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, SeriÇizimTürü};

    #[test]
    fn iki_kip_aynı_ham_veriyi_farklı_geometriyle_korur() -> Result<(), UplotHatası> {
        let mut akış = YShiftedSeriesAkışı::yeni()?;
        let ham = akış.ham_seriler().to_vec();
        let (kaydırılmış_seçenekler, kaydırılmış_veri) = akış.kartı()?;
        assert_eq!(Y_SHIFTED_SERIES_ARALIK_MS, 2_000);
        assert_eq!(kaydırılmış_veri.uzunluk(), 30);
        assert_eq!(
            kaydırılmış_seçenekler.birincil_y_eksen_genişliği,
            Some(70.0)
        );
        assert!(kaydırılmış_seçenekler.birincil_y_sabit_bölmeler.is_none());
        assert_eq!(
            kaydırılmış_seçenekler
                .birincil_y_özel_etiketler
                .iter()
                .find(|(değer, _)| *değer == 20.0)
                .map(|(_, etiket)| etiket.as_str()),
            Some("Core #3")
        );
        for (indeks, (seri, ham_seri)) in
            kaydırılmış_seçenekler.seriler.iter().zip(&ham).enumerate()
        {
            assert_eq!(seri.dolgu_tabanı, indeks as f64 * 10.0);
            assert_eq!(seri.lejant_değerleri.as_deref(), Some(ham_seri.as_slice()));
        }
        assert_eq!(
            kaydırılmış_seçenekler
                .seriler
                .last()
                .map(|seri| seri.çizim_türü),
            Some(SeriÇizimTürü::Çubuk)
        );
        assert_eq!(
            Grafik::yeni(kaydırılmış_seçenekler, kaydırılmış_veri)?.görünür_y_aralığı(),
            Aralık::yeni(0.0, 30.0)?
        );
        let (normal_seçenekler, normal_veri) = akış.ilerlet()?;
        assert_eq!(akış.kip(), YShiftedSeriesKipi::Normal);
        assert_eq!(normal_veri.seriler(), ham.as_slice());
        assert!(normal_seçenekler.birincil_y_özel_etiketler.is_empty());
        assert!(
            normal_seçenekler
                .seriler
                .iter()
                .all(|seri| seri.dolgu_tabanı == 0.0)
        );
        assert_eq!(
            Grafik::yeni(normal_seçenekler, normal_veri)?.görünür_y_aralığı(),
            Aralık::yeni(0.0, 10.0)?
        );
        let (_, yeniden_kaydırılmış) = akış.ilerlet()?;
        for (indeks, (ham_seri, çizim_serisi)) in
            ham.iter().zip(yeniden_kaydırılmış.seriler()).enumerate()
        {
            assert!(
                ham_seri
                    .iter()
                    .zip(çizim_serisi)
                    .all(|(ham, çizim)| match (ham, çizim) {
                        (Some(ham), Some(çizim)) => {
                            *çizim == *ham + indeks as f64 * 10.0
                        }
                        _ => false,
                    })
            );
        }
        Ok(())
    }
}
