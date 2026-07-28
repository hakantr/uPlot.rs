use super::{
    Grafik,
    seri_geometrisi::{bant_yönünde, seri_ara_değeri},
    çubuk_komutu,
};
use crate::cizim::kirpma::çokgeni_dikdörtgene_kırp;
use crate::{Aralık, Komut, Nokta, Sahne};

impl Grafik {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn seri_bantlarını_çiz(
        &self,
        sahne: &mut Sahne,
        seri_indeksi: usize,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let Some(üst_ayarları) = self.seçenekler.seriler.get(seri_indeksi) else {
            return;
        };
        for bant in self
            .seçenekler
            .bantlar
            .iter()
            .filter(|bant| bant.üst_seri == seri_indeksi)
        {
            let Some(üst_değerler) = self.veri.seriler().get(bant.üst_seri) else {
                continue;
            };
            let Some(alt_değerler) = self.veri.seriler().get(bant.alt_seri) else {
                continue;
            };
            let Some(alt_ayarları) = self.seçenekler.seriler.get(bant.alt_seri) else {
                continue;
            };
            let çokgenler = if üst_ayarları.çizim_türü == crate::SeriÇizimTürü::Çubuk
                && alt_ayarları.çizim_türü == crate::SeriÇizimTürü::Çubuk
            {
                self.çubuk_bant_çokgenleri(
                    üst_değerler,
                    alt_değerler,
                    üst_ayarları,
                    bant.yön,
                    x_aralığı,
                    y_aralığı,
                    sol,
                    sağ,
                    üst,
                    alt,
                )
            } else {
                let mut çokgenler = Vec::new();
                let mut koşular = Vec::<Vec<(f64, f64, f64)>>::new();
                let mut koşu = Vec::<(f64, f64, f64)>::new();
                for (indeks, x) in self.veri.x().iter().copied().enumerate() {
                    let üst_değer = üst_değerler.get(indeks).copied().flatten();
                    let alt_değer = alt_değerler.get(indeks).copied().flatten();
                    if let (Some(üst_değer), Some(alt_değer)) = (üst_değer, alt_değer) {
                        koşu.push((x, üst_değer, alt_değer));
                        continue;
                    }
                    let üst_köprülenir = üst_değer.is_some()
                        || self.veri.hizalama_eksiği_mi(bant.üst_seri, indeks)
                        || üst_ayarları.boşlukları_birleştir;
                    let alt_köprülenir = alt_değer.is_some()
                        || self.veri.hizalama_eksiği_mi(bant.alt_seri, indeks)
                        || alt_ayarları.boşlukları_birleştir;
                    if (!üst_köprülenir || !alt_köprülenir) && !koşu.is_empty() {
                        koşular.push(std::mem::take(&mut koşu));
                    }
                }
                if !koşu.is_empty() {
                    koşular.push(koşu);
                }
                for koşu in koşular {
                    let üst_koşu = koşu
                        .iter()
                        .map(|(_, üst, _)| Some(*üst))
                        .collect::<Vec<_>>();
                    let alt_koşu = koşu
                        .iter()
                        .map(|(_, _, alt)| Some(*alt))
                        .collect::<Vec<_>>();
                    let mut örnekler =
                        Vec::with_capacity(koşu.len().saturating_sub(1).saturating_mul(8) + 1);
                    for indeks in 0..koşu.len().saturating_sub(1) {
                        let Some((x0, _, _)) = koşu.get(indeks).copied() else {
                            continue;
                        };
                        let Some((x1, _, _)) = koşu.get(indeks + 1).copied() else {
                            continue;
                        };
                        if x1 < x_aralığı.en_az || x0 > x_aralığı.en_çok {
                            continue;
                        }
                        let ilk_adım = usize::from(!örnekler.is_empty());
                        for adım in ilk_adım..=8 {
                            let t = adım as f64 / 8.0;
                            let x_değeri = x0 + (x1 - x0) * t;
                            let Some(üst_değer) =
                                seri_ara_değeri(&üst_koşu, indeks, t, üst_ayarları.çizim_türü)
                            else {
                                continue;
                            };
                            let Some(alt_değer) =
                                seri_ara_değeri(&alt_koşu, indeks, t, alt_ayarları.çizim_türü)
                            else {
                                continue;
                            };
                            let x = self.x_konumu(x_aralığı, x_değeri, sol, genişlik);
                            let üst_y = alt
                                - self.y_konumu(
                                    &üst_ayarları.ölçek,
                                    y_aralığı,
                                    üst_değer,
                                    0.0,
                                    yükseklik,
                                );
                            let alt_y = alt
                                - self.y_konumu(
                                    &üst_ayarları.ölçek,
                                    y_aralığı,
                                    alt_değer,
                                    0.0,
                                    yükseklik,
                                );
                            örnekler.push((x, üst_y, alt_y, üst_değer - alt_değer));
                        }
                    }
                    for çokgen in bant_örneklerini_birleştir(&örnekler, bant.yön) {
                        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                        if kırpılmış.len() >= 3 {
                            çokgenler.push(kırpılmış);
                        }
                    }
                }
                çokgenler
            };
            if !çokgenler.is_empty() {
                let yuvarlatılmış_çubuk_bandı = üst_ayarları.çizim_türü
                    == crate::SeriÇizimTürü::Çubuk
                    && üst_ayarları.çubuk_uç_yarıçap_oranı > 0.0;
                let çubuk_kenarları = (üst_ayarları.çizim_türü == crate::SeriÇizimTürü::Çubuk
                    && üst_ayarları.çizgi_kalınlığı > 0.0
                    && !yuvarlatılmış_çubuk_bandı)
                    .then(|| {
                        çokgenler
                            .iter()
                            .filter_map(|çokgen| {
                                let ilk = çokgen.first().copied()?;
                                let mut kapalı = çokgen.clone();
                                kapalı.push(ilk);
                                Some(kapalı)
                            })
                            .collect::<Vec<_>>()
                    });
                if yuvarlatılmış_çubuk_bandı {
                    for çokgen in &çokgenler {
                        let Some(ilk) = çokgen.first().copied() else {
                            continue;
                        };
                        let Some(karşı) = çokgen.get(2).copied() else {
                            continue;
                        };
                        let x0 = ilk.x.min(karşı.x);
                        let x1 = ilk.x.max(karşı.x);
                        let y0 = ilk.y.min(karşı.y);
                        let y1 = ilk.y.max(karşı.y);
                        if x1 <= x0 || y1 <= y0 {
                            continue;
                        }
                        sahne.ekle(çubuk_komutu(
                            Nokta::yeni(x0, y0),
                            x1 - x0,
                            y1 - y0,
                            bant.dolgu.clone(),
                            üst_ayarları.renk.clone(),
                            üst_ayarları.çizgi_kalınlığı,
                            üst_ayarları.çubuk_uç_yarıçap_oranı,
                            crate::ÇubukYönü::Dikey,
                            ilk.y > karşı.y,
                        ));
                    }
                } else {
                    sahne.ekle(Komut::Alan {
                        çokgenler,
                        dolgu: bant.dolgu.clone().into(),
                    });
                }
                if let Some(parçalar) = çubuk_kenarları {
                    sahne.ekle(Komut::Yol {
                        parçalar,
                        renk: üst_ayarları.renk.clone().into(),
                        kalınlık: üst_ayarları.çizgi_kalınlığı,
                    });
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn çubuk_bant_çokgenleri(
        &self,
        üst_değerler: &[Option<f64>],
        alt_değerler: &[Option<f64>],
        seri: &crate::SeriSeçenekleri,
        yön: crate::BantYönü,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) -> Vec<Vec<Nokta>> {
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let en_küçük_fark = self
            .veri
            .x()
            .windows(2)
            .filter_map(|çift| çift.first().zip(çift.get(1)))
            .map(|(a, b)| b - a)
            .filter(|fark| *fark > 0.0)
            .min_by(f64::total_cmp)
            .unwrap_or_else(|| {
                (x_aralığı.en_çok - x_aralığı.en_az)
                    / üst_değerler.len().saturating_sub(1).max(1) as f64
            });
        let çubuk_genişliği = (en_küçük_fark / (x_aralığı.en_çok - x_aralığı.en_az)
            * f64::from(genişlik)
            * f64::from(seri.çubuk_genişlik_oranı)) as f32;
        let çubuk_genişliği =
            (çubuk_genişliği.min(seri.azami_çubuk_genişliği) - seri.çubuk_boşluğu_piksel).max(0.0);
        let mut çokgenler = Vec::new();
        for (indeks, x_değeri) in self.veri.x().iter().copied().enumerate() {
            if x_değeri < x_aralığı.en_az || x_değeri > x_aralığı.en_çok {
                continue;
            }
            let Some(üst_değer) = üst_değerler.get(indeks).copied().flatten() else {
                continue;
            };
            let Some(alt_değer) = alt_değerler.get(indeks).copied().flatten() else {
                continue;
            };
            if !bant_yönünde(üst_değer - alt_değer, yön) {
                continue;
            }
            let merkez = self.x_konumu(x_aralığı, x_değeri, sol, genişlik);
            let x0 = (merkez - çubuk_genişliği / 2.0).clamp(sol, sağ);
            let x1 = (merkez + çubuk_genişliği / 2.0).clamp(sol, sağ);
            let y0 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, üst_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            let y1 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, alt_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            if x1 > x0 && (y1 - y0).abs() > f32::EPSILON {
                çokgenler.push(vec![
                    Nokta::yeni(x0, y0),
                    Nokta::yeni(x1, y0),
                    Nokta::yeni(x1, y1),
                    Nokta::yeni(x0, y1),
                ]);
            }
        }
        çokgenler
    }
}

type BantÖrneği = (f32, f32, f32, f64);

fn bant_örneklerini_birleştir(
    örnekler: &[BantÖrneği], yön: crate::BantYönü
) -> Vec<Vec<Nokta>> {
    let mut sonuç = Vec::new();
    let mut üst_yolu = Vec::new();
    let mut alt_yolu = Vec::new();
    for çift in örnekler.windows(2) {
        let Some(a) = çift.first().copied() else {
            continue;
        };
        let Some(b) = çift.get(1).copied() else {
            continue;
        };
        let a_geçerli = bant_yönünde(a.3, yön);
        let b_geçerli = bant_yönünde(b.3, yön);
        match (a_geçerli, b_geçerli) {
            (true, true) => {
                if üst_yolu.is_empty() {
                    üst_yolu.push(Nokta::yeni(a.0, a.1));
                    alt_yolu.push(Nokta::yeni(a.0, a.2));
                }
                üst_yolu.push(Nokta::yeni(b.0, b.1));
                alt_yolu.push(Nokta::yeni(b.0, b.2));
            }
            (true, false) => {
                if üst_yolu.is_empty() {
                    üst_yolu.push(Nokta::yeni(a.0, a.1));
                    alt_yolu.push(Nokta::yeni(a.0, a.2));
                }
                if let Some((üst_kesişim, alt_kesişim)) = bant_kesişimi(a, b) {
                    üst_yolu.push(üst_kesişim);
                    alt_yolu.push(alt_kesişim);
                }
                bant_koşusunu_bitir(&mut sonuç, &mut üst_yolu, &mut alt_yolu);
            }
            (false, true) => {
                bant_koşusunu_bitir(&mut sonuç, &mut üst_yolu, &mut alt_yolu);
                if let Some((üst_kesişim, alt_kesişim)) = bant_kesişimi(a, b) {
                    üst_yolu.push(üst_kesişim);
                    alt_yolu.push(alt_kesişim);
                }
                üst_yolu.push(Nokta::yeni(b.0, b.1));
                alt_yolu.push(Nokta::yeni(b.0, b.2));
            }
            (false, false) => {
                bant_koşusunu_bitir(&mut sonuç, &mut üst_yolu, &mut alt_yolu);
            }
        }
    }
    bant_koşusunu_bitir(&mut sonuç, &mut üst_yolu, &mut alt_yolu);
    sonuç
}

fn bant_kesişimi(a: BantÖrneği, b: BantÖrneği) -> Option<(Nokta, Nokta)> {
    let payda = a.3 - b.3;
    if !payda.is_finite() || payda.abs() <= f64::EPSILON {
        return None;
    }
    let oran = (a.3 / payda).clamp(0.0, 1.0) as f32;
    let x = a.0 + (b.0 - a.0) * oran;
    Some((
        Nokta::yeni(x, a.1 + (b.1 - a.1) * oran),
        Nokta::yeni(x, a.2 + (b.2 - a.2) * oran),
    ))
}

fn bant_koşusunu_bitir(
    sonuç: &mut Vec<Vec<Nokta>>,
    üst_yolu: &mut Vec<Nokta>,
    alt_yolu: &mut Vec<Nokta>,
) {
    if üst_yolu.len() >= 2 && alt_yolu.len() >= 2 {
        let mut çokgen = std::mem::take(üst_yolu);
        çokgen.extend(alt_yolu.drain(..).rev());
        sonuç.push(çokgen);
    } else {
        üst_yolu.clear();
        alt_yolu.clear();
    }
}
