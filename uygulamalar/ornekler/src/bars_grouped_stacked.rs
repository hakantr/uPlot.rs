use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, ÇubukDüzeni, ÇubukYönü,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ÇubukÖrneği {
    ÇokGrupÇokSeriDikeyGruplu,
    ÇokGrupÇokSeriDikeyYığılmış,
    ÇokGrupÇokSeriYatayGruplu,
    ÇokGrupÇokSeriYatayYığılmış,
    ÇokGrupTekSeriGruplu,
    ÇokGrupTekSeriYığılmış,
    TekGrupÇokSeriGruplu,
    TekGrupÇokSeriYığılmış,
    TekGrupTekSeriGruplu,
    TekGrupTekSeriYığılmış,
}

impl ÇubukÖrneği {
    pub const TÜMÜ: [Self; 10] = [
        Self::ÇokGrupÇokSeriDikeyGruplu,
        Self::ÇokGrupÇokSeriDikeyYığılmış,
        Self::ÇokGrupÇokSeriYatayGruplu,
        Self::ÇokGrupÇokSeriYatayYığılmış,
        Self::ÇokGrupTekSeriGruplu,
        Self::ÇokGrupTekSeriYığılmış,
        Self::TekGrupÇokSeriGruplu,
        Self::TekGrupÇokSeriYığılmış,
        Self::TekGrupTekSeriGruplu,
        Self::TekGrupTekSeriYığılmış,
    ];

    pub fn kimlik(self) -> &'static str {
        match self {
            Self::ÇokGrupÇokSeriDikeyGruplu => "bars-multi-group-grouped",
            Self::ÇokGrupÇokSeriDikeyYığılmış => "bars-multi-group-stacked",
            Self::ÇokGrupÇokSeriYatayGruplu => "bars-horizontal-grouped",
            Self::ÇokGrupÇokSeriYatayYığılmış => "bars-horizontal-stacked",
            Self::ÇokGrupTekSeriGruplu => "bars-single-series-grouped",
            Self::ÇokGrupTekSeriYığılmış => "bars-single-series-stacked",
            Self::TekGrupÇokSeriGruplu => "bars-single-group-grouped",
            Self::TekGrupÇokSeriYığılmış => "bars-single-group-stacked",
            Self::TekGrupTekSeriGruplu => "bars-single-bar-grouped",
            Self::TekGrupTekSeriYığılmış => "bars-single-bar-stacked",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }

    pub fn seri_sayısı(self) -> usize {
        if self.tek_seri() { 1 } else { 3 }
    }

    pub fn başlık(self) -> &'static str {
        match self {
            Self::ÇokGrupÇokSeriDikeyGruplu => "Multi group / multi series · grouped",
            Self::ÇokGrupÇokSeriDikeyYığılmış => "Multi group / multi series · stacked",
            Self::ÇokGrupÇokSeriYatayGruplu => "Horizontal · grouped",
            Self::ÇokGrupÇokSeriYatayYığılmış => "Horizontal · stacked",
            Self::ÇokGrupTekSeriGruplu => "Multi group / single series · grouped",
            Self::ÇokGrupTekSeriYığılmış => "Multi group / single series · stacked",
            Self::TekGrupÇokSeriGruplu => "Single group / multi series · grouped",
            Self::TekGrupÇokSeriYığılmış => "Single group / multi series · stacked",
            Self::TekGrupTekSeriGruplu => "Single group / single series · grouped",
            Self::TekGrupTekSeriYığılmış => "Single group / single series · stacked",
        }
    }

    pub fn açıklama(self) -> &'static str {
        match self {
            Self::ÇokGrupÇokSeriDikeyGruplu => {
                "Dört kategori içinde üç seri, her seri için sabit bir grup yuvası."
            }
            Self::ÇokGrupÇokSeriDikeyYığılmış => {
                "Aynı değerler kümülatif tepelere önceden yığılır; gizlenen seri yeniden yığılmaz."
            }
            Self::ÇokGrupÇokSeriYatayGruplu => {
                "Dikey örneğin yönü çevrilmiş 400×800 bağımsız yüzeyi."
            }
            Self::ÇokGrupÇokSeriYatayYığılmış => {
                "Kümülatif yığının yatay yönelim ve ters kategorik eksen karşılığı."
            }
            Self::ÇokGrupTekSeriGruplu => {
                "Dört kategoride tek seri; grouped dağılımının tek yuvalı sınır durumu."
            }
            Self::ÇokGrupTekSeriYığılmış => {
                "Dört kategoride tek seri; stacked algoritmasının tek serili sınır durumu."
            }
            Self::TekGrupÇokSeriGruplu => "Tek kategoride üç seri, sabit yuvalarla yan yana.",
            Self::TekGrupÇokSeriYığılmış => "Tek kategoride üç serinin kümülatif yığını.",
            Self::TekGrupTekSeriGruplu => "Tek kategori ve tek seri için grouped sınır durumu.",
            Self::TekGrupTekSeriYığılmış => {
                "Tek kategori ve tek seri için stacked sınır durumu."
            }
        }
    }

    pub fn yatay(self) -> bool {
        matches!(
            self,
            Self::ÇokGrupÇokSeriYatayGruplu | Self::ÇokGrupÇokSeriYatayYığılmış
        )
    }

    fn yığılmış(self) -> bool {
        matches!(
            self,
            Self::ÇokGrupÇokSeriDikeyYığılmış
                | Self::ÇokGrupÇokSeriYatayYığılmış
                | Self::ÇokGrupTekSeriYığılmış
                | Self::TekGrupÇokSeriYığılmış
                | Self::TekGrupTekSeriYığılmış
        )
    }

    fn tek_grup(self) -> bool {
        matches!(
            self,
            Self::TekGrupÇokSeriGruplu
                | Self::TekGrupÇokSeriYığılmış
                | Self::TekGrupTekSeriGruplu
                | Self::TekGrupTekSeriYığılmış
        )
    }

    fn tek_seri(self) -> bool {
        matches!(
            self,
            Self::ÇokGrupTekSeriGruplu
                | Self::ÇokGrupTekSeriYığılmış
                | Self::TekGrupTekSeriGruplu
                | Self::TekGrupTekSeriYığılmış
        )
    }
}

pub const BARS_GROUPED_STACKED_KART_TANIM_ÖRNEĞİ: &str = r##"let örnek = ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu;
let (seçenekler, veri) = bars_grouped_stacked_kartı(örnek)?;
// Kategorik eksen, grup dağılımı ve yığma çekirdekte çözülür.
let grafik = Grafik::yeni(seçenekler, veri)?;

// Resmî sayfadaki ilişkili on bağımsız yüzeyi kaynak sırasında birlikte kurar.
let yüzeyler = bars_grouped_stacked_kartları()?;"##;

/// Resmî sayfadaki ilişkili on bağımsız yüzeyi DOM sırasıyla döndürür.
pub fn bars_grouped_stacked_kartları()
-> Result<Vec<(ÇubukÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    ÇubukÖrneği::TÜMÜ
        .into_iter()
        .map(|örnek| {
            bars_grouped_stacked_kartı(örnek).map(|(seçenekler, veri)| (örnek, seçenekler, veri))
        })
        .collect()
}

/// `demos/bars-grouped-stacked.html` içindeki on alt grafikten birini üretir.
pub fn bars_grouped_stacked_kartı(
    örnek: ÇubukÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kategoriler = if örnek.tek_grup() {
        vec!["Group A"]
    } else {
        vec!["Group A", "Group B", "Group C", "Group D"]
    };
    let ham = if örnek.tek_grup() {
        vec![vec![1.0], vec![3.0], vec![5.0]]
    } else {
        vec![
            vec![1.0, 2.0, 3.0, 10.0],
            vec![3.0, 2.0, 1.0, 10.0],
            vec![5.0, 9.0, 3.0, 10.0],
        ]
    };
    let seri_sayısı = örnek.seri_sayısı();
    let ham = ham.into_iter().take(seri_sayısı).collect::<Vec<_>>();
    let veri_en_çok = (0..kategoriler.len())
        .map(|indeks| {
            let değerler = ham.iter().filter_map(|seri| seri.get(indeks).copied());
            if örnek.yığılmış() {
                değerler.sum::<f64>()
            } else {
                değerler.fold(0.0_f64, f64::max)
            }
        })
        .fold(0.0_f64, f64::max);
    let kaynak_y_aralığı = Aralık::uplot_sayısal(0.0, veri_en_çok, 0.05, true)?;
    let x = (0..kategoriler.len()).map(|indeks| indeks as f64).collect();
    let veri = HizalıVeri::yeni(
        x,
        ham.into_iter()
            .map(|seri| seri.into_iter().map(Some).collect())
            .collect(),
    )?;
    let yön = if örnek.yatay() {
        ÇubukYönü::Yatay
    } else {
        ÇubukYönü::Dikey
    };
    let düzen = ÇubukDüzeni::yeni(yön)
        .yığılmış(örnek.yığılmış())
        .ters(örnek.yatay())
        .x_kenar_paylı(false)
        .grupları_kenarlara_yay(true);
    let (genişlik, yükseklik) = if örnek.yatay() {
        (400, 800)
    } else {
        (800, 400)
    };
    let etkileşimler = ortak_kart_etkileşimleri();
    let renkler = ["#33BB55", "#B56FAB", "#BB1133"];
    let etiketler = ["Metric 1", "Metric 2", "Metric 3"];
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .y_aralığı(kaynak_y_aralığı)
        .x_aralığı(space_between_x_aralığı(kategoriler.len())?)
        .x_zaman(false)
        .kategoriler(kategoriler)
        .çubuk_düzeni(düzen)
        .lejant_canlı(false)
        .etkileşimler(etkileşimler);
    for indeks in 0..seri_sayısı {
        let renk = renkler.get(indeks).copied().unwrap_or("#6b7280");
        let etiket = etiketler.get(indeks).copied().unwrap_or("Metric");
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(etiket)
                .renk(renk)
                .dolgu(renk)
                .çizgi_kalınlığı(0.0),
        );
    }
    Ok((seçenekler, veri))
}

fn space_between_x_aralığı(grup_sayısı: usize) -> Result<Aralık, UplotHatası> {
    let grup_sayısı = grup_sayısı.max(1) as f64;
    let ham_açıklık = (grup_sayısı - 1.0).max(1.0);
    let ilk_merkez_oranı = 0.9 / grup_sayısı / 2.0;
    let büyütme = 1.0 / (1.0 - ilk_merkez_oranı * 2.0);
    let ofset = (büyütme * ham_açıklık - ham_açıklık) / 2.0;
    Aralık::yeni(-ofset, ham_açıklık + ofset)
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn kaynaktaki_on_düzen_ve_tek_noktalı_veriler_çizilir() -> Result<(), UplotHatası> {
        let kartlar = bars_grouped_stacked_kartları()?;
        assert_eq!(kartlar.len(), 10);
        for (örnek, seçenekler, _veri) in kartlar {
            assert!(seçenekler.başlık.is_empty(), "{}", örnek.kimlik());
            assert!(!seçenekler.lejant_canlı, "{}", örnek.kimlik());
            let (seçenekler, veri) = bars_grouped_stacked_kartı(örnek)?;
            let beklenen = veri.uzunluk() * veri.seriler().len();
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            let çubuklar = sahne
                .komutlar()
                .iter()
                .filter(
                    |komut| matches!(komut, Komut::Dikdörtgen { kalınlık, .. } if *kalınlık == 0.0),
                )
                .count();
            assert_eq!(çubuklar, beklenen, "{}", örnek.kimlik());
        }
        Ok(())
    }

    #[test]
    fn kaynak_range_num_ve_set_series_yerleşimi_korunur() -> Result<(), UplotHatası> {
        let (gruplu_seçenekler, gruplu_veri) =
            bars_grouped_stacked_kartı(ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu)?;
        assert_eq!(
            gruplu_seçenekler.y_aralığı,
            Some(Aralık {
                en_az: 0.0,
                en_çok: 11.0
            })
        );
        let mut gruplu = Grafik::yeni(gruplu_seçenekler, gruplu_veri)?;
        let kırmızı_x = |grafik: &Grafik| {
            grafik
                .çiz()
                .komutlar()
                .iter()
                .find_map(|komut| match komut {
                    Komut::Dikdörtgen {
                        konum,
                        dolgu,
                        kalınlık,
                        ..
                    } if dolgu == "#BB1133" && *kalınlık == 0.0 => Some(konum.x),
                    _ => None,
                })
        };
        let önceki_kırmızı_x = kırmızı_x(&gruplu);
        assert!(gruplu.seri_görünürlüğünü_ayarla(1, false)?);
        assert_eq!(kırmızı_x(&gruplu), önceki_kırmızı_x);

        let (yığılmış_seçenekler, yığılmış_veri) =
            bars_grouped_stacked_kartı(ÇubukÖrneği::ÇokGrupÇokSeriDikeyYığılmış)?;
        assert_eq!(
            yığılmış_seçenekler.y_aralığı,
            Some(Aralık {
                en_az: 0.0,
                en_çok: 32.0
            })
        );
        let mut yığılmış = Grafik::yeni(yığılmış_seçenekler, yığılmış_veri)?;
        assert!(yığılmış.seri_görünürlüğünü_ayarla(1, false)?);
        let kırmızı_merkez = yığılmış
            .çiz()
            .komutlar()
            .iter()
            .find_map(|komut| match komut {
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu,
                    kalınlık,
                    ..
                } if dolgu == "#BB1133" && *kalınlık == 0.0 => {
                    Some((konum.x + genişlik / 2.0, konum.y + yükseklik / 2.0))
                }
                _ => None,
            });
        assert!(kırmızı_merkez.is_some_and(|(x, y)| {
            yığılmış
                .çubuk_vuruşu(800, 400, x, y)
                .is_some_and(|(_, _, _, _, _, değer)| değer == 9.0)
        }));
        Ok(())
    }

    #[test]
    fn çubuk_vuruşu_kaynak_hover_dikdörtgenini_döndürür() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            bars_grouped_stacked_kartı(ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu)?;
        let grafik = Grafik::yeni(seçenekler, veri)?;
        let sahne = grafik.çiz();
        let merkez = sahne.komutlar().iter().find_map(|komut| match komut {
            Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                kalınlık,
                ..
            } if *kalınlık == 0.0 => {
                Some((konum.x + *genişlik / 2.0, konum.y + *yükseklik / 2.0))
            }
            _ => None,
        });
        assert!(merkez.is_some());
        let Some((x, y)) = merkez else {
            return Ok(());
        };
        let vuruş = grafik.çubuk_vuruşu(800, 400, x, y);
        assert!(
            vuruş.is_some_and(|(seri, indeks, _, _, _, değer)| {
                seri == 0 && indeks == 0 && değer == 1.0
            }),
            "{vuruş:?}"
        );
        Ok(())
    }

    #[test]
    fn tüm_çubuk_yüzeyleri_ortak_zoom_ve_taşıma_profilini_devralır() -> Result<(), UplotHatası> {
        for örnek in ÇubukÖrneği::TÜMÜ {
            let (seçenekler, _) = bars_grouped_stacked_kartı(örnek)?;
            let etkileşimler = seçenekler.etkileşimler;
            assert!(etkileşimler.tekerlek_etkileşimi, "{}", örnek.kimlik());
            assert!(etkileşimler.seçim_yakınlaştır, "{}", örnek.kimlik());
            assert!(etkileşimler.çift_tıkla_tam_görünüm, "{}", örnek.kimlik());
            assert!(etkileşimler.dokunma_etkileşimi, "{}", örnek.kimlik());
            assert!(etkileşimler.görünüm_geçmişi, "{}", örnek.kimlik());
        }
        Ok(())
    }

    #[test]
    fn dikey_ve_yatay_çubuklar_zoom_sonrası_kırpılır_vurulur_ve_tam_görünüme_döner()
    -> Result<(), UplotHatası> {
        for (örnek, genişlik, yükseklik) in [
            (ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu, 800_u32, 400_u32),
            (ÇubukÖrneği::ÇokGrupÇokSeriYatayYığılmış, 400_u32, 800_u32),
        ] {
            let (seçenekler, veri) = bars_grouped_stacked_kartı(örnek)?;
            let mut grafik = Grafik::yeni(seçenekler, veri)?;
            let tam_x = grafik.görünür_x_aralığı();
            assert!(grafik.tekerlek(0.5, 0.5, 1.0, false)?, "{}", örnek.kimlik());
            let sahne = grafik.çiz_görünür_boyutta(genişlik, yükseklik);
            let merkez = sahne.komutlar().iter().find_map(|komut| match komut {
                Komut::Dikdörtgen {
                    konum,
                    genişlik: çubuk_genişliği,
                    yükseklik: çubuk_yüksekliği,
                    kalınlık,
                    ..
                } if *kalınlık == 0.0 && *çubuk_genişliği > 0.0 && *çubuk_yüksekliği > 0.0 =>
                {
                    assert!(konum.x >= 0.0 && konum.y >= 0.0, "{}", örnek.kimlik());
                    assert!(
                        konum.x + *çubuk_genişliği <= genişlik as f32
                            && konum.y + *çubuk_yüksekliği <= yükseklik as f32,
                        "{}",
                        örnek.kimlik()
                    );
                    Some((
                        konum.x + *çubuk_genişliği / 2.0,
                        konum.y + *çubuk_yüksekliği / 2.0,
                    ))
                }
                _ => None,
            });
            assert!(
                merkez.is_some(),
                "zoom sonrası görünür çubuk yok: {}",
                örnek.kimlik()
            );
            let Some((x, y)) = merkez else {
                continue;
            };
            assert!(
                grafik.çubuk_vuruşu(genişlik, yükseklik, x, y).is_some(),
                "{}",
                örnek.kimlik()
            );
            assert!(grafik.tam_görünüm(), "{}", örnek.kimlik());
            assert_eq!(grafik.görünür_x_aralığı(), tam_x, "{}", örnek.kimlik());
        }
        Ok(())
    }

    #[test]
    fn yığılmış_etiketler_kaynaktaki_kümülatif_tepeyi_gösterir() -> Result<(), UplotHatası> {
        let (seçenekler, veri) =
            bars_grouped_stacked_kartı(ÇubukÖrneği::ÇokGrupÇokSeriDikeyYığılmış)?;
        let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
        let etiketler = sahne.komutlar().iter().filter_map(|komut| match komut {
            Komut::Metin { içerik, .. } => Some(içerik.as_str()),
            _ => None,
        });

        assert!(etiketler.clone().any(|etiket| etiket == "4"));
        assert!(etiketler.clone().any(|etiket| etiket == "9"));
        assert!(etiketler.clone().any(|etiket| etiket == "30"));
        Ok(())
    }

    #[test]
    fn space_between_ilk_ve_son_grubu_çizim_kenarlarına_yaslar() -> Result<(), UplotHatası> {
        for (örnek, beklenen_baş, beklenen_son, yatay) in [
            (ÇubukÖrneği::ÇokGrupÇokSeriDikeyGruplu, 64.0, 776.0, false),
            (ÇubukÖrneği::ÇokGrupÇokSeriYatayGruplu, 48.0, 752.0, true),
        ] {
            let (seçenekler, veri) = bars_grouped_stacked_kartı(örnek)?;
            assert!(
                seçenekler
                    .çubuk_düzeni
                    .is_some_and(|düzen| düzen.grupları_kenarlara_yay && !düzen.x_kenar_paylı)
            );
            let sahne = Grafik::yeni(seçenekler, veri)?.çiz();
            let sınırlar = sahne.komutlar().iter().filter_map(|komut| match komut {
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    kalınlık,
                    ..
                } if *kalınlık == 0.0 => Some(if yatay {
                    (konum.y, konum.y + yükseklik)
                } else {
                    (konum.x, konum.x + genişlik)
                }),
                _ => None,
            });
            let (en_az, en_çok) = sınırlar
                .fold((f32::INFINITY, f32::NEG_INFINITY), |sonuç, sınır| {
                    (sonuç.0.min(sınır.0), sonuç.1.max(sınır.1))
                });
            assert!((en_az - beklenen_baş).abs() <= 0.01, "{}", örnek.kimlik());
            assert!((en_çok - beklenen_son).abs() <= 0.01, "{}", örnek.kimlik());
        }
        Ok(())
    }
}
