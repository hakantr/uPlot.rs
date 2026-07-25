use super::ortak_kart_etkileşimleri;
use super::veri_uretici::KanıtRastgele;
use crate::{
    Aralık, DağılımDüzeni, DağılımNoktası, DağılımSerisi, GrafikSeçenekleri, HizalıVeri,
    SeriSeçenekleri, UplotHatası, YÖlçekSeçenekleri,
};

pub const SCATTER_KANIT_TOHUMU: u32 = 0x5CA7_7E42;
pub const SCATTER_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = scatter_kartı(ScatterÖrneği::Bubble)?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScatterÖrneği {
    Scatter,
    Bubble,
}
impl ScatterÖrneği {
    pub const TÜMÜ: [Self; 2] = [Self::Scatter, Self::Bubble];
    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Scatter => "scatter-points",
            Self::Bubble => "scatter-bubble",
        }
    }
    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Scatter => "Scatter Plot",
            Self::Bubble => "Bubble Plot",
        }
    }
    pub const fn seri_başı_nokta(self) -> usize {
        match self {
            Self::Scatter => 10_000,
            Self::Bubble => 50,
        }
    }
    pub fn kimlikten(k: &str) -> Option<Self> {
        Self::TÜMÜ.into_iter().find(|o| o.kimlik() == k)
    }
}

pub fn scatter_kartı(
    örnek: ScatterÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let mut rng = KanıtRastgele::yeni(SCATTER_KANIT_TOHUMU ^ örnek as u32);
    let stiller = if örnek == ScatterÖrneği::Scatter {
        [
            ("Value", "red", "red"),
            ("Value", "green", "green"),
            ("Value", "blue", "blue"),
            ("Value", "magenta", "magenta"),
        ]
    } else {
        [
            ("Region A", "red", "rgba(255,0,0,0.3)"),
            ("Region B", "green", "rgba(0,255,0,0.3)"),
            ("Region C", "blue", "rgba(0,0,255,0.3)"),
            ("Region E", "orange", "rgba(255,128,0,0.3)"),
        ]
    };
    let mut düzen = DağılımDüzeni::default().vuruş_etkin(örnek == ScatterÖrneği::Bubble);
    let mut ham_scatter = Vec::new();
    let mut ham = Vec::new();
    let mut azami = 0.0_f64;
    if örnek == ScatterÖrneği::Scatter {
        for seri in 0..5 {
            let noktalar = (0..10_000)
                .map(|_| {
                    DağılımNoktası::yeni(
                        tamsayı(&mut rng, 0, 500),
                        tamsayı(&mut rng, 0, 500),
                        5.0,
                    )
                })
                .collect::<Vec<_>>();
            if seri > 0 {
                ham_scatter.push(noktalar);
            }
        }
    } else {
        for seri in 0..5 {
            let mut ns = Vec::with_capacity(50);
            for _ in 0..50 {
                let x = tamsayı(&mut rng, 0, 500);
                let mut y = tamsayı(&mut rng, 0, 500);
                if seri == 1 && y != 0.0 {
                    y = -y;
                }
                let d = tamsayı(&mut rng, 1, 10_000);
                if seri > 0 {
                    azami = azami.max(d);
                }
                ns.push((x, y, d, etiket(&mut rng)));
            }
            if seri > 0 {
                ham.push(ns);
            }
        }
    }
    for (i, (ad, renk, dolgu)) in stiller.into_iter().enumerate() {
        let ns = if örnek == ScatterÖrneği::Scatter {
            ham_scatter.get(i).cloned().unwrap_or_default()
        } else {
            ham.get(i)
                .into_iter()
                .flatten()
                .map(|(x, y, d, e)| {
                    DağılımNoktası::yeni(
                        *x,
                        *y,
                        (60.0 * (*d / azami).sqrt() as f32).max(f32::EPSILON),
                    )
                    .değer(*d)
                    .etiket(e)
                })
                .collect()
        };
        düzen = düzen.seri(
            DağılımSerisi::yeni(ad, renk)
                .dolgu(if örnek == ScatterÖrneği::Scatter {
                    renk
                } else {
                    dolgu
                })
                .ölçek(if i == 0 && örnek == ScatterÖrneği::Bubble {
                    "y2"
                } else {
                    "y"
                })
                .noktalar(ns),
        );
    }
    let x_aralığı = nokta_aralığı(
        düzen
            .seriler
            .iter()
            .flat_map(|seri| seri.noktalar.iter().map(|nokta| nokta.x)),
    )?;
    let y_aralığı = nokta_aralığı(
        düzen
            .seriler
            .iter()
            .filter(|seri| seri.ölçek == "y")
            .flat_map(|seri| seri.noktalar.iter().map(|nokta| nokta.y)),
    )?;
    let y2_aralığı = (örnek == ScatterÖrneği::Bubble)
        .then(|| {
            nokta_aralığı(
                düzen
                    .seriler
                    .iter()
                    .filter(|seri| seri.ölçek == "y2")
                    .flat_map(|seri| seri.noktalar.iter().map(|nokta| nokta.y)),
            )
        })
        .transpose()?;
    let mut s = GrafikSeçenekleri::yeni(1_920, 600)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .x_eksen_etiketi(if örnek == ScatterÖrneği::Bubble {
            "GDP"
        } else {
            ""
        })
        .y_eksen_etiketi(if örnek == ScatterÖrneği::Bubble {
            "Income 1"
        } else {
            ""
        })
        .x_aralığı(x_aralığı)
        .y_aralığı(y_aralığı)
        .dağılım_düzeni(düzen)
        .etkileşimler(ortak_kart_etkileşimleri());
    if örnek == ScatterÖrneği::Bubble {
        s = s.y_ölçeği(
            YÖlçekSeçenekleri::yeni("y2")
                .aralık(y2_aralığı.unwrap_or(Aralık::yeni(-500.0, 0.0)?))
                .sağda(true)
                .eksen(true)
                .ızgara(false)
                .eksen_rengi("red")
                .eksen_etiketi("Income 2"),
        );
    }
    for (ad, r, _) in stiller {
        s = s.seri(SeriSeçenekleri::yeni(ad).renk(r).göster(false));
    }
    Ok((
        s,
        HizalıVeri::yeni(vec![0.0], (0..4).map(|_| vec![None]).collect())?,
    ))
}
fn tamsayı(r: &mut KanıtRastgele, min: u32, max: u32) -> f64 {
    f64::from(min) + (r.sonraki() * f64::from(max - min + 1)).floor()
}
fn nokta_aralığı(değerler: impl Iterator<Item = f64>) -> Result<Aralık, UplotHatası> {
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for değer in değerler.filter(|değer| değer.is_finite()) {
        en_az = en_az.min(değer);
        en_çok = en_çok.max(değer);
    }
    if en_az == en_çok {
        let fark = en_az.abs().max(100.0);
        return Aralık::yeni(en_az - fark, en_çok + fark);
    }
    Aralık::yeni(en_az, en_çok)
}
fn etiket(r: &mut KanıtRastgele) -> String {
    const C: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";
    let mut değer = (r.sonraki() * 2_821_109_907_456.0).floor() as u64;
    (0..5)
        .filter_map(|_| {
            let indeks = (değer % 36) as usize;
            değer /= 36;
            C.get(indeks).copied().map(char::from)
        })
        .collect()
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn iki_kaynak_yüzeyi_nokta_sayılarını_korur() -> Result<(), UplotHatası> {
        for örnek in ScatterÖrneği::TÜMÜ {
            let (seçenekler, veri) = scatter_kartı(örnek)?;
            assert!(seçenekler.dağılım_düzeni.as_ref().is_some_and(|düzen| {
                düzen.seriler.len() == 4
                    && düzen
                        .seriler
                        .iter()
                        .all(|seri| seri.noktalar.len() == örnek.seri_başı_nokta())
            }));
            let çizilen_nokta_sayısı = Grafik::yeni(seçenekler, veri)?
                .çiz()
                .komutlar()
                .iter()
                .map(|komut| match komut {
                    Komut::Daire { .. } | Komut::Alan { .. } => 1,
                    Komut::Daireler { merkezler, .. } => merkezler.len(),
                    Komut::DeğişkenDaireler { daireler, .. } => daireler.len(),
                    _ => 0,
                })
                .sum::<usize>();
            assert_eq!(çizilen_nokta_sayısı, örnek.seri_başı_nokta() * 4);
            if örnek == ScatterÖrneği::Scatter {
                let toplu_komut_sayısı =
                    Grafik::yeni(scatter_kartı(örnek)?.0, scatter_kartı(örnek)?.1)?
                        .çiz()
                        .komutlar()
                        .iter()
                        .filter(|komut| matches!(komut, Komut::Daireler { .. }))
                        .count();
                assert_eq!(toplu_komut_sayısı, 4);
            }
        }
        Ok(())
    }

    #[test]
    fn bubble_boyut_etiket_ve_vuruş_verisini_korur() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scatter_kartı(ScatterÖrneği::Bubble)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let merkez = grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Daire { merkez, .. } => Some(*merkez),
                Komut::Daireler { merkezler, .. } => merkezler.first().copied(),
                Komut::DeğişkenDaireler { daireler, .. } => {
                    daireler.first().map(|(merkez, _)| *merkez)
                }
                _ => None,
            });
        let Some(merkez) = merkez else {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "scatter bubble",
                açıklama: "çizilmiş balon bulunamadı".to_string(),
            });
        };
        let vuruş = grafik.dağılım_vuruşu_boyutta(1_920, 600, merkez.x, merkez.y);
        assert!(vuruş.is_some_and(|v| v.değer.is_some() && v.etiket.is_some()));
        Ok(())
    }

    #[test]
    fn bubble_seri_başına_tek_kırpılmış_dolgu_ve_stroke_yolu_üretir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scatter_kartı(ScatterÖrneği::Bubble)?;
        let düzen =
            seçenekler
                .dağılım_düzeni
                .as_ref()
                .ok_or(UplotHatası::GeçersizKaynakVeri {
                    varlık: "scatter bubble",
                    açıklama: "dağılım düzeni eksik".to_string(),
                })?;
        let azami_boyut = düzen
            .seriler
            .iter()
            .flat_map(|seri| seri.noktalar.iter().map(|nokta| nokta.boyut))
            .fold(0.0_f32, f32::max);
        assert!((azami_boyut - 60.0).abs() <= f32::EPSILON);
        assert!(düzen.seriler.first().is_some_and(
            |seri| seri.ölçek == "y2" && seri.noktalar.iter().all(|nokta| nokta.y <= 0.0)
        ));

        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let yollar = sahne
            .komutlar()
            .iter()
            .filter_map(|komut| match komut {
                Komut::DeğişkenDaireler {
                    daireler,
                    çizgi,
                    kalınlık,
                    kesme_sınırları,
                    ..
                } => Some((daireler, çizgi, kalınlık, kesme_sınırları)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(yollar.len(), 4);
        assert_eq!(
            yollar
                .iter()
                .map(|(daireler, _, _, _)| daireler.len())
                .sum::<usize>(),
            200
        );
        assert!(
            yollar
                .iter()
                .all(|(_, çizgi, kalınlık, kesme)| **kalınlık == 1.0
                    && !çizgi.is_empty()
                    && kesme.is_some())
        );
        Ok(())
    }

    #[test]
    fn scatter_zoomu_yalnız_merkezi_görünür_noktaları_toplu_yola_alır() -> Result<(), UplotHatası> {
        let (seçenekler, veri) = scatter_kartı(ScatterÖrneği::Scatter)?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let aralık = Aralık::yeni(200.0, 300.0)?;
        assert!(grafik.görünür_aralıkları_ayarla(aralık, aralık, true));
        let (sol, sağ, üst, alt) = grafik.çizim_alanı_boyutta(1_920, 600);
        let sahne = grafik.çiz();
        assert!(sahne.komutlar().iter().all(|komut| match komut {
            Komut::Daireler { merkezler, .. } => merkezler.iter().all(|merkez| {
                (sol..=sağ).contains(&merkez.x) && (üst..=alt).contains(&merkez.y)
            }),
            _ => true,
        }));
        Ok(())
    }

    #[test]
    fn iki_mode2_yüzeyi_zoom_ve_geçmişi_bağımsız_tutar() -> Result<(), UplotHatası> {
        let (scatter_seçenekleri, scatter_veri) = scatter_kartı(ScatterÖrneği::Scatter)?;
        let (bubble_seçenekleri, bubble_veri) = scatter_kartı(ScatterÖrneği::Bubble)?;
        let mut scatter = Grafik::yeni(scatter_seçenekleri, scatter_veri)?;
        let bubble = Grafik::yeni(bubble_seçenekleri, bubble_veri)?;
        let bubble_x = bubble.görünür_x_aralığı();
        let bubble_y = bubble.görünür_y_aralığı();
        assert!(scatter.fiziksel_seçim_yakınlaştır(0.2, 0.2, 0.8, 0.8)?);
        assert_ne!(scatter.görünür_x_aralığı(), bubble_x);
        assert_eq!(bubble.görünür_x_aralığı(), bubble_x);
        assert_eq!(bubble.görünür_y_aralığı(), bubble_y);
        assert!(!bubble.geri_var());
        Ok(())
    }

    #[test]
    fn bubble_hover_en_küçüğü_eşitlikte_en_yakın_son_adayı_seçer_ve_bbox_köşesini_reddeder()
    -> Result<(), UplotHatası> {
        let düzen = DağılımDüzeni::default().vuruş_etkin(true).seri(
            DağılımSerisi::yeni("overlap", "red")
                .dolgu("#ff00004d")
                .noktalar(vec![
                    DağılımNoktası::yeni(50.0, 50.0, 40.0).etiket("large"),
                    DağılımNoktası::yeni(50.0, 50.0, 10.0).etiket("small-first"),
                    DağılımNoktası::yeni(50.0, 50.0, 10.0).etiket("small-last"),
                    DağılımNoktası::yeni(80.0, 80.0, 20.0).etiket("corner"),
                ]),
        );
        let seçenekler = GrafikSeçenekleri::yeni(400, 300)?
            .x_zaman(false)
            .x_aralığı(Aralık::yeni(0.0, 100.0)?)
            .y_aralığı(Aralık::yeni(0.0, 100.0)?)
            .dağılım_düzeni(düzen)
            .seri(SeriSeçenekleri::yeni("placeholder").göster(false));
        let veri = HizalıVeri::yeni(vec![0.0], vec![vec![None]])?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let daireler = grafik
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::DeğişkenDaireler { daireler, .. } => Some(daireler.clone()),
                _ => None,
            })
            .ok_or(UplotHatası::GeçersizKaynakVeri {
                varlık: "scatter bubble hover",
                açıklama: "toplu bubble yolu eksik".to_string(),
            })?;
        let merkez = daireler
            .first()
            .map(|(merkez, _)| *merkez)
            .ok_or(UplotHatası::YetersizVeri { uzunluk: 0 })?;
        let vuruş = grafik
            .dağılım_vuruşu_boyutta(400, 300, merkez.x, merkez.y)
            .ok_or(UplotHatası::GeçersizKaynakVeri {
                varlık: "scatter bubble hover",
                açıklama: "örtüşen bubble vuruşu eksik".to_string(),
            })?;
        assert_eq!(vuruş.indeks, 2);
        assert_eq!(vuruş.etiket.as_deref(), Some("small-last"));

        let (köşe_merkezi, köşe_yarıçapı) =
            daireler.get(3).copied().ok_or(UplotHatası::YetersizVeri {
                uzunluk: daireler.len(),
            })?;
        assert!(
            grafik
                .dağılım_vuruşu_boyutta(
                    400,
                    300,
                    köşe_merkezi.x + köşe_yarıçapı,
                    köşe_merkezi.y + köşe_yarıçapı,
                )
                .is_none()
        );
        Ok(())
    }
}
