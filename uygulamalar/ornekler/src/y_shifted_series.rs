use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const Y_SHIFTED_SERIES_KANIT_TOHUMU: u32 = 0x59_53_48_46;
pub const Y_SHIFTED_SERIES_ARALIK_MS: u64 = 2_000;

pub const Y_SHIFTED_SERIES_KART_TANIM_ÖRNEĞİ: &str = r##"let mut akış = YShiftedSeriesAkışı::yeni()?;
let (seçenekler, veri) = akış.kartı()?;
let mut grafik = Grafik::yeni(seçenekler, veri)?;

// Resmî demo gibi aynı grafik örneğinde yalnız setData + dinamik Y sunumu yenilenir.
let güncelleme = akış.ilerlet_güncellemesi()?;
grafik.veriyi_y_sunumunda_ayarla(
    güncelleme.veri,
    güncelleme.y_aralığı,
    güncelleme.y_özel_etiketler,
    güncelleme.dolgu_tabanları,
)?;"##;

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

#[derive(Debug, Clone, PartialEq)]
pub struct YShiftedSeriesGüncellemesi {
    pub veri: HizalıVeri,
    pub y_aralığı: Aralık,
    pub y_özel_etiketler: Vec<(f64, String)>,
    pub dolgu_tabanları: Vec<f64>,
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
        self.kipi_değiştir();
        self.kartı()
    }

    pub fn ilerlet_güncellemesi(&mut self) -> Result<YShiftedSeriesGüncellemesi, UplotHatası> {
        self.kipi_değiştir();
        y_shifted_series_güncellemesi(self.kip, &self.ham_seriler)
    }

    fn kipi_değiştir(&mut self) {
        self.kip = match self.kip {
            YShiftedSeriesKipi::Kaydırılmış => YShiftedSeriesKipi::Normal,
            YShiftedSeriesKipi::Normal => YShiftedSeriesKipi::Kaydırılmış,
        };
    }
}

pub fn y_shifted_series_kartı() -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    YShiftedSeriesAkışı::yeni()?.kartı()
}

fn y_shifted_series_kartı_kipte(
    kip: YShiftedSeriesKipi,
    ham_seriler: &[Vec<Option<f64>>],
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    y_shifted_series_ham_verisini_doğrula(ham_seriler)?;
    let güncelleme = y_shifted_series_güncellemesi(kip, ham_seriler)?;
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık("Y-shifted Series")
        .x_zaman(false)
        .birincil_y_eksen_genişliği(70.0)
        .y_aralığı(güncelleme.y_aralığı)
        .y_ölçeği(YÖlçekSeçenekleri::yeni("y").etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre))
        .etkileşimler(ortak_kart_etkileşimleri());
    seçenekler = seçenekler.y_özel_etiketler(güncelleme.y_özel_etiketler.iter().cloned());
    let renkler = ["red", "green", "blue"];
    let dolgular = [
        "rgba(255,0,0,0.1)",
        "rgba(0,255,0,0.1)",
        "rgba(0,0,255,0.1)",
    ];
    for (indeks, ((ham, renk), dolgu)) in ham_seriler.iter().zip(renkler).zip(dolgular).enumerate()
    {
        let kaydırma =
            güncelleme
                .dolgu_tabanları
                .get(indeks)
                .copied()
                .ok_or(UplotHatası::YetersizVeri {
                    uzunluk: güncelleme.dolgu_tabanları.len(),
                })?;
        let mut seri = SeriSeçenekleri::yeni(format!("Core #{}", indeks + 1))
            .renk(renk)
            .dolgu(dolgu)
            .dolgu_tabanı(kaydırma)
            .lejant_değerleri(ham.clone());
        if indeks == 2 {
            seri = seri.çubuk(true).toplu_çubuk_yolu(true);
        }
        seçenekler = seçenekler.seri(seri);
    }
    Ok((seçenekler, güncelleme.veri))
}

fn y_shifted_series_ham_verisini_doğrula(
    ham_seriler: &[Vec<Option<f64>>],
) -> Result<(), UplotHatası> {
    if ham_seriler.len() != 3 || ham_seriler.iter().any(|seri| seri.len() != 30) {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "YShiftedSeriesAkışı",
            açıklama: "kaynak üç adet 30 noktalı seri gerektirir".to_string(),
        });
    }
    Ok(())
}

fn y_shifted_series_güncellemesi(
    kip: YShiftedSeriesKipi,
    ham_seriler: &[Vec<Option<f64>>],
) -> Result<YShiftedSeriesGüncellemesi, UplotHatası> {
    y_shifted_series_ham_verisini_doğrula(ham_seriler)?;
    let kaydırılmış = kip == YShiftedSeriesKipi::Kaydırılmış;
    let dolgu_tabanları = (0..3)
        .map(|indeks| {
            if kaydırılmış {
                indeks as f64 * 10.0
            } else {
                0.0
            }
        })
        .collect::<Vec<_>>();
    let y_özel_etiketler = if kaydırılmış {
        (0..=30)
            .map(|değer| {
                let etiket = if değer % 10 == 0 {
                    format!("Core #{}", değer / 10 + 1)
                } else {
                    (değer % 10).to_string()
                };
                (f64::from(değer), etiket)
            })
            .collect()
    } else {
        Vec::new()
    };
    let çizim_serileri = ham_seriler
        .iter()
        .zip(&dolgu_tabanları)
        .map(|(ham, kaydırma)| {
            ham.iter()
                .map(|değer| değer.map(|değer| değer + kaydırma))
                .collect()
        })
        .collect();
    Ok(YShiftedSeriesGüncellemesi {
        veri: HizalıVeri::yeni((1..=30).map(f64::from).collect(), çizim_serileri)?,
        y_aralığı: Aralık::yeni(0.0, if kaydırılmış { 30.0 } else { 10.0 })?,
        y_özel_etiketler,
        dolgu_tabanları,
    })
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut, SeriÇizimTürü};

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
        let mut grafik = Grafik::yeni(kaydırılmış_seçenekler, kaydırılmış_veri)?;
        let grafik_adresi = std::ptr::from_ref(&grafik);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 30.0)?);
        let çözüm = grafik
            .imleç_çözümü(0.0, 1_000.0)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let ham_lejant = grafik
            .en_yakın_noktalar(0.0)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?
            .1;
        let üçüncü_geometri = çözüm
            .seriler
            .get(2)
            .copied()
            .flatten()
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?
            .değer;
        assert_eq!(
            üçüncü_geometri,
            ham_lejant
                .get(2)
                .copied()
                .flatten()
                .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?
                + 20.0
        );
        let (sol, sağ, üst, alt) = grafik.çizim_alanı_boyutta(1_920, 600);
        assert!(sağ > sol);
        let taban_20 = alt
            - grafik
                .seri_y_konum_oranı(2, 20.0)
                .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })? as f32
                * (alt - üst);
        let kaydırılmış_sahne = grafik.çiz();
        let mavi_çubuklar = kaydırılmış_sahne
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Alan { çokgenler, dolgu } if dolgu == "rgba(0,0,255,0.1)" => {
                    Some(çokgenler)
                }
                _ => None,
            })
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!(!mavi_çubuklar.is_empty());
        assert!(mavi_çubuklar.iter().all(|çokgen| {
            let taban = çokgen
                .iter()
                .map(|nokta| nokta.y)
                .fold(f32::NEG_INFINITY, f32::max);
            çokgen.len() == 4 && (taban - taban_20).abs() < 0.001
        }));
        assert_eq!(
            kaydırılmış_sahne
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "rgba(0,0,255,0.1)"))
                .count(),
            1
        );

        assert!(grafik.seri_görünürlüğünü_ayarla(1, false)?);
        let güncelleme = akış.ilerlet_güncellemesi()?;
        assert_eq!(akış.kip(), YShiftedSeriesKipi::Normal);
        assert_eq!(güncelleme.veri.seriler(), ham.as_slice());
        assert!(güncelleme.y_özel_etiketler.is_empty());
        assert!(güncelleme.dolgu_tabanları.iter().all(|taban| *taban == 0.0));
        grafik.veriyi_y_sunumunda_ayarla(
            güncelleme.veri,
            güncelleme.y_aralığı,
            güncelleme.y_özel_etiketler,
            güncelleme.dolgu_tabanları,
        )?;
        assert_eq!(std::ptr::from_ref(&grafik), grafik_adresi);
        assert_eq!(grafik.görünür_y_aralığı(), Aralık::yeni(0.0, 10.0)?);
        assert!(!grafik.seri_görünür_mü(1));
        let (_, _, normal_üst, normal_alt) = grafik.çizim_alanı_boyutta(1_920, 600);
        let normal_mavi_çubuklar = grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Alan { çokgenler, dolgu } if dolgu == "rgba(0,0,255,0.1)" => {
                    Some(çokgenler.clone())
                }
                _ => None,
            })
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        assert!(normal_mavi_çubuklar.iter().all(|çokgen| {
            let taban = çokgen
                .iter()
                .map(|nokta| nokta.y)
                .fold(f32::NEG_INFINITY, f32::max);
            (taban - normal_alt).abs() < 0.001 && taban >= normal_üst
        }));

        let yeniden = akış.ilerlet_güncellemesi()?;
        assert_eq!(akış.kip(), YShiftedSeriesKipi::Kaydırılmış);
        for (indeks, (ham_seri, çizim_serisi)) in ham.iter().zip(yeniden.veri.seriler()).enumerate()
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
        assert_eq!(yeniden.dolgu_tabanları, vec![0.0, 10.0, 20.0]);
        Ok(())
    }
}
