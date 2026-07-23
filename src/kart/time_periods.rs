use std::sync::OnceLock;

use super::ortak_kart_etkileşimleri;
use crate::{GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, İkincilXEksen};

const TRAFİK_JSON: &str = include_str!("veri/time_periods_traffic.json");
const GÜN_SANIYESİ: f64 = 86_400.0;
const OCAK_2019_UTC: f64 = 1_546_300_800.0;

pub const TIME_PERIODS_KART_TANIM_ÖRNEĞİ: &str = r##"for örnek in TimePeriodsÖrneği::TÜMÜ {
    let (seçenekler, veri) = time_periods_kartı(örnek)?;
    // Dönem toplama, ikinci X ekseni ve seri tarih eşlemeleri çekirdektedir.
    let grafik = Grafik::yeni(seçenekler, veri)?;
}"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriodsÖrneği {
    SaatlikKullanıcılar,
    ŞubatOcak2019,
    GünlükKullanıcılar,
}

impl TimePeriodsÖrneği {
    pub const TÜMÜ: [Self; 3] = [
        Self::SaatlikKullanıcılar,
        Self::ŞubatOcak2019,
        Self::GünlükKullanıcılar,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::SaatlikKullanıcılar => "time-periods-hourly-users",
            Self::ŞubatOcak2019 => "time-periods-feb-vs-jan-2019",
            Self::GünlükKullanıcılar => "time-periods-daily-users",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::SaatlikKullanıcılar => "Hourly Users",
            Self::ŞubatOcak2019 => "Feb vs Jan 2019",
            Self::GünlükKullanıcılar => "Daily Users",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

fn trafik_kaynağı() -> Result<&'static [Vec<f64>], UplotHatası> {
    static KAYNAK: OnceLock<Result<Vec<Vec<f64>>, String>> = OnceLock::new();
    match KAYNAK.get_or_init(|| serde_json::from_str(TRAFİK_JSON).map_err(|hata| hata.to_string()))
    {
        Ok(kaynak)
            if kaynak.len() == 4
                && kaynak.first().is_some_and(|x| !x.is_empty())
                && kaynak
                    .first()
                    .is_some_and(|x| kaynak.iter().skip(1).all(|seri| seri.len() >= x.len())) =>
        {
            Ok(kaynak)
        }
        Ok(_) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/traffic.json",
            açıklama: "dört seri bulunmalı ve Y serileri X kadar uzun olmalıdır".to_string(),
        }),
        Err(açıklama) => Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/traffic.json",
            açıklama: açıklama.clone(),
        }),
    }
}

fn kaynak_dilimi(
    kaynak: &[Vec<f64>],
    seri: usize,
    başlangıç: usize,
    bitiş: usize,
) -> Result<Vec<f64>, UplotHatası> {
    kaynak
        .get(seri)
        .and_then(|değerler| değerler.get(başlangıç..bitiş))
        .map(<[f64]>::to_vec)
        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/traffic.json",
            açıklama: format!("{seri}. seri için {başlangıç}..{bitiş} dilimi bulunamadı"),
        })
}

fn saatlik_veri(kaynak: &[Vec<f64>]) -> Result<HizalıVeri, UplotHatası> {
    let x = kaynak
        .first()
        .cloned()
        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/data/traffic.json",
            açıklama: "X serisi bulunamadı".to_string(),
        })?;
    let uzunluk = x.len();
    let mut seriler = Vec::with_capacity(3);
    for seri in 1..=3 {
        seriler.push(
            kaynak_dilimi(kaynak, seri, 0, uzunluk)?
                .into_iter()
                .map(Some)
                .collect(),
        );
    }
    HizalıVeri::yeni(x, seriler)
}

fn aylık_veri(kaynak: &[Vec<f64>]) -> Result<HizalıVeri, UplotHatası> {
    const OCAK_UZUNLUĞU: usize = 743;
    const ŞUBAT_BAŞI: usize = 744;
    const ŞUBAT_SONU: usize = 1_416;
    let x = kaynak_dilimi(kaynak, 0, ŞUBAT_BAŞI, ŞUBAT_BAŞI + OCAK_UZUNLUĞU)?;
    let mut şubat = kaynak_dilimi(kaynak, 1, ŞUBAT_BAŞI, ŞUBAT_SONU)?;
    şubat.resize(OCAK_UZUNLUĞU, 0.0);
    let ocak = kaynak_dilimi(kaynak, 1, 0, OCAK_UZUNLUĞU)?;
    HizalıVeri::yeni(
        x,
        vec![
            şubat.into_iter().map(Some).collect(),
            ocak.into_iter().map(Some).collect(),
        ],
    )
}

fn günlük_veri(kaynak: &[Vec<f64>]) -> Result<HizalıVeri, UplotHatası> {
    let uzunluk = kaynak.first().map_or(0, Vec::len);
    let gün_sayısı = uzunluk / 24;
    if gün_sayısı == 0 || gün_sayısı * 24 != uzunluk {
        return Err(UplotHatası::GeçersizKaynakVeri {
            varlık: "demos/time-periods.html#aggDays",
            açıklama: format!("{uzunluk} saat tam günlere ayrılamadı"),
        });
    }
    let x = (0..gün_sayısı)
        .map(|gün| OCAK_2019_UTC + gün as f64 * GÜN_SANIYESİ)
        .collect();
    let mut seriler = Vec::with_capacity(3);
    for seri in 1..=3 {
        let saatler = kaynak_dilimi(kaynak, seri, 0, uzunluk)?;
        seriler.push(
            saatler
                .chunks(24)
                .map(|gün| Some(gün.iter().sum()))
                .collect(),
        );
    }
    HizalıVeri::yeni(x, seriler)
}

pub fn time_periods_kartı(
    örnek: TimePeriodsÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = trafik_kaynağı()?;
    let veri = match örnek {
        TimePeriodsÖrneği::SaatlikKullanıcılar => saatlik_veri(kaynak)?,
        TimePeriodsÖrneği::ŞubatOcak2019 => aylık_veri(kaynak)?,
        TimePeriodsÖrneği::GünlükKullanıcılar => günlük_veri(kaynak)?,
    };
    let mut seçenekler = GrafikSeçenekleri::yeni(1_920, 200)?
        .başlık(örnek.başlık())
        .x_eksen_asgari_etiket_boşluğu(if örnek == TimePeriodsÖrneği::GünlükKullanıcılar {
            100.0
        } else {
            50.0
        })
        .etkileşimler(ortak_kart_etkileşimleri());

    match örnek {
        TimePeriodsÖrneği::ŞubatOcak2019 => {
            let şubat_lejantı = veri
                .seriler()
                .first()
                .map(|seri| {
                    seri.iter()
                        .enumerate()
                        .map(|(indeks, değer)| (indeks < 672).then_some(*değer).flatten())
                        .collect()
                })
                .unwrap_or_default();
            seçenekler = seçenekler
                .ikincil_x_ekseni(İkincilXEksen::yeni(
                    -31.0 * GÜN_SANIYESİ,
                    "rgba(237, 126, 23, 1)",
                ))
                .seri(
                    SeriSeçenekleri::yeni("Feb 2019")
                        .renk("rgba(5, 141, 199, 1)")
                        .dolgu("rgba(5, 141, 199, 0.1)")
                        .lejant_değerleri(şubat_lejantı),
                )
                .seri(
                    SeriSeçenekleri::yeni("Jan 2019")
                        .renk("rgba(237, 126, 23, 1)")
                        .x_zaman_kaydırması(-31.0 * GÜN_SANIYESİ),
                );
        }
        TimePeriodsÖrneği::SaatlikKullanıcılar | TimePeriodsÖrneği::GünlükKullanıcılar => {
            let kaynak_ilk_x = veri.x().first().copied().unwrap_or(OCAK_2019_UTC);
            seçenekler = seçenekler
                .seri(
                    SeriSeçenekleri::yeni("2019")
                        .renk("rgba(5, 141, 199, 1)")
                        .dolgu("rgba(5, 141, 199, 0.1)"),
                )
                .seri(
                    SeriSeçenekleri::yeni("2018")
                        .renk("rgba(237, 126, 23, 1)")
                        .x_zaman_kaydırması(1_514_764_800.0 - kaynak_ilk_x),
                )
                .seri(
                    SeriSeçenekleri::yeni("2017")
                        .renk("rgba(255, 0, 0, 1)")
                        .x_zaman_kaydırması(1_483_228_800.0 - kaynak_ilk_x),
                );
        }
    }
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn üç_kaynak_yüzeyi_aynı_trafik_verisini_kullanır() -> Result<(), UplotHatası> {
        let (saatlik_seçenekler, saatlik) =
            time_periods_kartı(TimePeriodsÖrneği::SaatlikKullanıcılar)?;
        assert_eq!(saatlik.uzunluk(), 8_712);
        assert_eq!(saatlik.x().first(), Some(&1_546_322_400.0));
        assert_eq!(
            saatlik.seriler().first().and_then(|seri| seri.first()),
            Some(&Some(33.0))
        );
        let saatlik_sahne = Grafik::yeni(saatlik_seçenekler, saatlik)?.çiz();
        assert!(saatlik_sahne.komutlar().iter().any(
            |komut| matches!(komut, Komut::Alan { dolgu, .. } if dolgu == "rgba(5, 141, 199, 0.1)")
        ));

        let (aylık_seçenekler, aylık) = time_periods_kartı(TimePeriodsÖrneği::ŞubatOcak2019)?;
        assert_eq!(aylık.uzunluk(), 743);
        assert_eq!(aylık.x().first(), Some(&1_549_000_800.0));
        assert_eq!(
            aylık.seriler().first().and_then(|seri| seri.first()),
            Some(&Some(23.0))
        );
        assert_eq!(
            aylık.seriler().get(1).and_then(|seri| seri.first()),
            Some(&Some(33.0))
        );
        assert!(aylık_seçenekler.ikincil_x_eksen.is_some());
        assert_eq!(
            aylık.seriler().first().and_then(|seri| seri.get(672)),
            Some(&Some(0.0))
        );
        assert_eq!(
            aylık_seçenekler
                .seriler
                .first()
                .and_then(|seri| seri.lejant_değerleri.as_ref())
                .and_then(|değerler| değerler.get(672)),
            Some(&None)
        );
        let aylık_sahne = Grafik::yeni(aylık_seçenekler, aylık)?.çiz();
        assert!(aylık_sahne.komutlar().iter().any(|komut| {
            matches!(
                komut,
                Komut::Metin { konum, renk, .. }
                    if renk == "rgba(237, 126, 23, 1)" && konum.y > 160.0
            )
        }));

        let (_, günlük) = time_periods_kartı(TimePeriodsÖrneği::GünlükKullanıcılar)?;
        assert_eq!(günlük.uzunluk(), 363);
        assert_eq!(günlük.x().first(), Some(&OCAK_2019_UTC));
        assert_eq!(
            günlük.seriler().first().and_then(|seri| seri.first()),
            Some(&Some(1_416.0))
        );
        let toplamlar = günlük
            .seriler()
            .iter()
            .map(|seri| seri.iter().flatten().sum::<f64>())
            .collect::<Vec<_>>();
        assert_eq!(toplamlar, vec![338_421.0, 314_936.0, 323_408.0]);
        assert_eq!(
            günlük.seriler().first().map(|seri| seri
                .iter()
                .rev()
                .take_while(|değer| **değer == Some(0.0))
                .count()),
            Some(29)
        );
        Ok(())
    }

    #[test]
    fn seri_tarih_kaydırmaları_çekirdekten_sorgulanır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = time_periods_kartı(TimePeriodsÖrneği::SaatlikKullanıcılar)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let x = 1_546_322_400.0;
        assert_eq!(grafik.seri_zamanı(0, x), Some(x));
        assert_eq!(grafik.seri_zamanı(1, x), Some(1_514_764_800.0));
        assert_eq!(grafik.seri_zamanı(2, x), Some(1_483_228_800.0));
        assert_eq!(grafik.seri_zamanı(99, x), Some(x));
        assert_eq!(grafik.seri_zamanı(0, f64::NAN), None);
        Ok(())
    }
}
