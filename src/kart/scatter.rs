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
            ("Region A", "red", "#ff00004d"),
            ("Region B", "green", "#00ff004d"),
            ("Region C", "blue", "#0000ff4d"),
            ("Region E", "orange", "#ff80004d"),
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
            let daire_sayısı = Grafik::yeni(seçenekler, veri)?
                .çiz()
                .komutlar()
                .iter()
                .filter(|komut| matches!(komut, Komut::Daire { .. } | Komut::Alan { .. }))
                .count();
            assert_eq!(daire_sayısı, örnek.seri_başı_nokta() * 4);
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
}
