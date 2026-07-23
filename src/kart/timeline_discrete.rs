use super::{ortak_kart_etkileşimleri, veri_uretici::KanıtRastgele};
use crate::{
    Aralık, BoşlukKipi, GrafikSeçenekleri, HizalıDeğer, HizalıVeri, SeriSeçenekleri,
    TimelineDüzeni, TimelineHücresi, UplotHatası, hizalı_verileri_birleştir, ÇizimSırası,
};

pub const TIMELINE_DISCRETE_KANIT_TOHUMU: u32 = 0x5449_4D45;
pub const TIMELINE_DISCRETE_ZAMAN_ÇAPASI: f64 = 1_700_002_800.0;
pub const TIMELINE_DISCRETE_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in TimelineDiscreteÖrneği::TÜMÜ {
    let (seçenekler, veri) = timeline_discrete_kartı(örnek)?;
    // Şerit dağılımı, semantic süreler, null/undefined ve hücre renkleri çekirdektedir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimelineDiscreteÖrneği {
    DurumZamanÇizelgesi,
    PeriyodikDurumGeçmişi,
    YinelenenArdışıkDurumlar,
    BirleştirilmişArdışıkDurumlar,
}

impl TimelineDiscreteÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::DurumZamanÇizelgesi,
        Self::PeriyodikDurumGeçmişi,
        Self::YinelenenArdışıkDurumlar,
        Self::BirleştirilmişArdışıkDurumlar,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::DurumZamanÇizelgesi => "timeline-discrete-state-timeline",
            Self::PeriyodikDurumGeçmişi => "timeline-discrete-periodic-history",
            Self::YinelenenArdışıkDurumlar => "timeline-discrete-repeating-states",
            Self::BirleştirilmişArdışıkDurumlar => "timeline-discrete-merged-states",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::DurumZamanÇizelgesi => {
                "State timeline, duty cycles & transitions (semantic x width)"
            }
            Self::PeriyodikDurumGeçmişi => {
                "Status history, periodic samples - (non-semantic x width)"
            }
            Self::YinelenenArdışıkDurumlar => "Repeating consecutive states",
            Self::BirleştirilmişArdışıkDurumlar => "Merged same consecutive states",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub fn timeline_discrete_kartı(
    örnek: TimelineDiscreteÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (veri, hücreler, zaman, x_aralığı, x_ızgarası) = match örnek {
        TimelineDiscreteÖrneği::DurumZamanÇizelgesi => {
            let veri = durum_zaman_verisi()?;
            let hücreler = semantic_hücreler(&veri, true);
            (veri, hücreler, true, None, true)
        }
        TimelineDiscreteÖrneği::PeriyodikDurumGeçmişi => {
            let veri = periyodik_veri()?;
            let hücreler = periyodik_hücreler(&veri);
            let adım = 3_600.0;
            let aralık = veri
                .x()
                .first()
                .zip(veri.x().last())
                .and_then(|(ilk, son)| Aralık::yeni(*ilk - adım / 2.0, *son + adım / 2.0).ok());
            (veri, hücreler, true, aralık, false)
        }
        TimelineDiscreteÖrneği::YinelenenArdışıkDurumlar => {
            let veri = yinelenen_veri()?;
            let hücreler = semantic_hücreler(&veri, false);
            (veri, hücreler, false, None, true)
        }
        TimelineDiscreteÖrneği::BirleştirilmişArdışıkDurumlar => {
            let veri = ardışıkları_birleştir(&yinelenen_veri()?)?;
            let hücreler = semantic_hücreler(&veri, false);
            (veri, hücreler, false, None, true)
        }
    };

    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 300)?
        .başlık(örnek.başlık())
        .x_zaman(zaman)
        .x_ızgarası_göster(x_ızgarası)
        .y_ekseni_göster(false)
        .y_ızgarası_göster(false)
        .y_aralığı(Aralık::yeni(0.0, 1.0)?)
        .çizim_sırası(ÇizimSırası::SerilerEksenler)
        .timeline_düzeni(TimelineDüzeni::yeni(
            ["Device A", "Device B", "Device C"],
            hücreler,
        ))
        .etkileşimler(ortak_kart_etkileşimleri());
    if let Some(aralık) = x_aralığı {
        seçenekler = seçenekler.x_aralığı(aralık);
    }
    for etiket in ["Device A", "Device B", "Device C"] {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .göster(false)
                .çizgi_kalınlığı(0.0),
        );
    }
    Ok((seçenekler, veri))
}

fn durum_zaman_verisi() -> Result<HizalıVeri, UplotHatası> {
    let durumlar = [
        vec![Some(0.0), Some(1.0), Some(2.0), Some(0.0), Some(3.0)],
        vec![Some(4.0), Some(5.0), Some(4.0), Some(5.0)],
        vec![Some(4.0), None, Some(4.0), None, Some(4.0), None, Some(4.0)],
    ];
    let mut rastgele = KanıtRastgele::yeni(TIMELINE_DISCRETE_KANIT_TOHUMU);
    let mut tablolar = Vec::with_capacity(durumlar.len());
    for durum in durumlar {
        let uzunluk = durum.len();
        let mut x = (0..uzunluk)
            .map(|indeks| {
                TIMELINE_DISCRETE_ZAMAN_ÇAPASI
                    - ((uzunluk - indeks) as f64 * 3_600.0 * rastgele.sonraki()).floor()
            })
            .collect::<Vec<_>>();
        x.sort_by(f64::total_cmp);
        tablolar.push(HizalıVeri::yeni(x, vec![durum])?);
    }
    hizalı_verileri_birleştir(
        &tablolar,
        Some(&[
            vec![BoşlukKipi::Genişlet],
            vec![BoşlukKipi::Genişlet],
            vec![BoşlukKipi::Genişlet],
        ]),
    )
}

fn periyodik_veri() -> Result<HizalıVeri, UplotHatası> {
    let x = (0..30)
        .map(|indeks| TIMELINE_DISCRETE_ZAMAN_ÇAPASI - (30 - indeks) as f64 * 3_600.0)
        .collect::<Vec<_>>();
    let mut a = vec![Some(1.0); 30];
    let mut b = vec![Some(4.0); 30];
    let mut c = vec![Some(5.0); 30];
    if let Some(değer) = a.get_mut(7) {
        *değer = None;
    }
    if let Some(değer) = a.get_mut(25) {
        *değer = Some(3.0);
    }
    if let Some(değer) = b.get_mut(13) {
        *değer = Some(0.0);
    }
    for (indeks, değer) in [(15, Some(0.0)), (18, Some(0.0)), (19, None)] {
        if let Some(hedef) = c.get_mut(indeks) {
            *hedef = değer;
        }
    }
    HizalıVeri::yeni(x, vec![a, b, c])
}

fn yinelenen_veri() -> Result<HizalıVeri, UplotHatası> {
    let tablolar = [
        HizalıVeri::yeni(
            vec![
                0.0, 18.0, 37.0, 55.0, 74.0, 92.0, 111.0, 129.0, 148.0, 166.0, 185.0, 203.0, 222.0,
                240.0,
            ],
            vec![vec![
                Some(0.0),
                Some(0.0),
                Some(1.0),
                Some(1.0),
                Some(1.0),
                Some(1.0),
                Some(2.0),
                Some(0.0),
                Some(0.0),
                Some(3.0),
                Some(3.0),
                Some(3.0),
                Some(3.0),
                Some(3.0),
            ]],
        )?,
        HizalıVeri::yeni(
            vec![
                0.0, 16.0, 32.0, 48.0, 64.0, 80.0, 96.0, 112.0, 128.0, 144.0, 160.0, 176.0, 192.0,
                208.0, 224.0, 240.0,
            ],
            vec![vec![
                None,
                None,
                Some(4.0),
                Some(4.0),
                Some(4.0),
                None,
                None,
                Some(4.0),
                None,
                None,
                Some(4.0),
                None,
                Some(4.0),
                Some(4.0),
                Some(4.0),
                Some(4.0),
            ]],
        )?,
        HizalıVeri::yeni(
            vec![0.0, 48.0, 96.0, 144.0, 192.0, 240.0],
            vec![vec![Some(1.0), None, Some(0.0), None, Some(1.0), Some(0.0)]],
        )?,
    ];
    hizalı_verileri_birleştir(&tablolar, None)
}

fn ardışıkları_birleştir(veri: &HizalıVeri) -> Result<HizalıVeri, UplotHatası> {
    let seriler = veri
        .seriler()
        .iter()
        .enumerate()
        .map(|(seri_indeksi, değerler)| {
            let mut önceki = None;
            değerler
                .iter()
                .enumerate()
                .map(|(indeks, değer)| {
                    if veri.hizalama_eksiği_mi(seri_indeksi, indeks) {
                        HizalıDeğer::Tanımsız
                    } else if let Some(değer) = değer {
                        if önceki == Some(*değer) {
                            HizalıDeğer::Tanımsız
                        } else {
                            önceki = Some(*değer);
                            HizalıDeğer::Değer(*değer)
                        }
                    } else {
                        önceki = None;
                        HizalıDeğer::Boş
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    HizalıVeri::anlamlı(veri.x().to_vec(), seriler)
}

fn semantic_hücreler(veri: &HizalıVeri, sayısal: bool) -> Vec<TimelineHücresi> {
    let mut hücreler = Vec::new();
    let Some(son_x) = veri.x().last().copied() else {
        return hücreler;
    };
    for (seri_indeksi, değerler) in veri.seriler().iter().enumerate() {
        for (indeks, değer) in değerler.iter().enumerate() {
            let Some(değer) = değer else {
                continue;
            };
            let Some(başlangıç) = veri.x().get(indeks).copied() else {
                continue;
            };
            let sonraki = ((indeks + 1)..değerler.len())
                .find(|aday| !veri.hizalama_eksiği_mi(seri_indeksi, *aday))
                .and_then(|aday| veri.x().get(aday))
                .copied()
                .unwrap_or(son_x);
            if sonraki <= başlangıç {
                continue;
            }
            let (dolgu, çizgi, etiket) = durum_stili(seri_indeksi, *değer, sayısal);
            hücreler.push(
                TimelineHücresi::yeni(
                    seri_indeksi,
                    indeks,
                    başlangıç,
                    sonraki,
                    etiket,
                    dolgu,
                    çizgi,
                )
                .çizgi_kalınlığı(4.0),
            );
        }
    }
    hücreler
}

fn periyodik_hücreler(veri: &HizalıVeri) -> Vec<TimelineHücresi> {
    let yarı = 3_600.0 * 0.9 / 2.0;
    veri.seriler()
        .iter()
        .enumerate()
        .flat_map(|(seri_indeksi, değerler)| {
            değerler
                .iter()
                .enumerate()
                .filter_map(move |(indeks, değer)| {
                    let değer = (*değer)?;
                    let x = veri.x().get(indeks).copied()?;
                    let (dolgu, çizgi, etiket) = durum_stili(seri_indeksi, değer, true);
                    Some(
                        TimelineHücresi::yeni(
                            seri_indeksi,
                            indeks,
                            x - yarı,
                            x + yarı,
                            etiket,
                            dolgu,
                            çizgi,
                        )
                        .çizgi_kalınlığı(4.0)
                        .azami_genişlik(100.0),
                    )
                })
        })
        .collect()
}

fn durum_stili(seri: usize, değer: f64, sayısal: bool) -> (&'static str, &'static str, String) {
    let (dolgu, çizgi) = if sayısal {
        match (seri, değer as i32) {
            (0, 0) => ("rgba(255, 0, 0, 0.2)", "red"),
            (0, 1) => ("rgba(0, 255, 0, 0.2)", "green"),
            (0, 2) => ("rgba(0, 0, 255, 0.2)", "blue"),
            (0, 3) => ("rgba(0, 255, 255, 0.2)", "cyan"),
            (1 | 2, 0) => ("rgba(255, 0, 0, 0.2)", "red"),
            (1 | 2, 4) => ("rgba(255, 165, 0, 0.4)", "orange"),
            (1 | 2, 5) => ("rgba(255, 255, 0, 0.3)", "yellow"),
            _ => ("rgba(0, 0, 0, 0.1)", "black"),
        }
    } else {
        match (seri, değer as i32) {
            (0, 0) => ("red", "red"),
            (0, 1) => ("green", "green"),
            (0, 2) => ("blue", "blue"),
            (0, 3) => ("cyan", "cyan"),
            (1, 4) => ("magenta", "magenta"),
            (2, 1) => ("green", "green"),
            (2, 0) => ("red", "red"),
            _ => ("black", "black"),
        }
    };
    let etiket = if sayısal {
        format!("{}", değer as i32)
    } else {
        match (seri, değer as i32) {
            (0, 0) => "a".to_string(),
            (0, 1) => "b".to_string(),
            (0, 2) => "c".to_string(),
            (0, 3) => "d".to_string(),
            (1, 4) => "e".to_string(),
            (2, 1) => "true".to_string(),
            (2, 0) => "false".to_string(),
            _ => "?".to_string(),
        }
    };
    (dolgu, çizgi, etiket)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn dört_kaynak_yüzeyi_şerit_geometrisini_korur() -> Result<(), UplotHatası> {
        for örnek in TimelineDiscreteÖrneği::TÜMÜ {
            let (seçenekler, veri) = timeline_discrete_kartı(örnek)?;
            assert_eq!(seçenekler.genişlik, 1_920);
            assert_eq!(seçenekler.yükseklik, 300);
            assert_eq!(veri.seriler().len(), 3);
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            assert!(
                sahne
                    .komutlar()
                    .iter()
                    .any(|komut| matches!(komut, Komut::Dikdörtgen { .. }))
            );
        }
        Ok(())
    }

    #[test]
    fn periyodik_kaynak_otuz_saat_ve_null_hücreleri_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            timeline_discrete_kartı(TimelineDiscreteÖrneği::PeriyodikDurumGeçmişi)?;
        assert_eq!(veri.uzunluk(), 30);
        assert_eq!(
            veri.seriler().first().and_then(|seri| seri.get(7)),
            Some(&None)
        );
        assert_eq!(
            veri.seriler().get(2).and_then(|seri| seri.get(19)),
            Some(&None)
        );
        assert_eq!(
            seçenekler
                .timeline_düzeni
                .as_ref()
                .map(|düzen| düzen.hücreler.len()),
            Some(88)
        );
        Ok(())
    }

    #[test]
    fn ardışık_birleştirme_null_ve_tanımsız_ayrımını_korur() -> Result<(), UplotHatası> {
        let yinelenen = yinelenen_veri()?;
        let birleşik = ardışıkları_birleştir(&yinelenen)?;
        assert_eq!(birleşik.uzunluk(), 28);
        assert_eq!(birleşik.x().first(), Some(&0.0));
        assert_eq!(birleşik.x().last(), Some(&240.0));
        assert!(birleşik.hizalama_eksiği_mi(0, 2));
        assert!(!birleşik.hizalama_eksiği_mi(1, 0));
        assert_eq!(
            semantic_hücreler(&birleşik, false)
                .iter()
                .filter(|hücre| hücre.seri_indeksi == 0)
                .count(),
            5
        );
        Ok(())
    }

    #[test]
    fn semantic_hover_tüm_hücre_dikdörtgenini_döndürür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            timeline_discrete_kartı(TimelineDiscreteÖrneği::BirleştirilmişArdışıkDurumlar)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let vuruşlar = grafik.timeline_vuruşları(0.2);
        assert!(!vuruşlar.is_empty());
        assert!(vuruşlar.iter().all(|vuruş| vuruş.bitiş > vuruş.başlangıç));
        Ok(())
    }
}
