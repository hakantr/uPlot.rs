use super::{
    Grafik,
    seri_geometrisi::{bant_dilim_çokgeni, bant_yönünde, seri_ara_değeri},
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
                for indeks in 0..self.veri.x().len().saturating_sub(1) {
                    let Some(x0) = self.veri.x().get(indeks).copied() else {
                        continue;
                    };
                    let Some(x1) = self.veri.x().get(indeks + 1).copied() else {
                        continue;
                    };
                    if x1 < x_aralığı.en_az || x0 > x_aralığı.en_çok {
                        continue;
                    }
                    if üst_değerler.get(indeks).copied().flatten().is_none()
                        || üst_değerler.get(indeks + 1).copied().flatten().is_none()
                        || alt_değerler.get(indeks).copied().flatten().is_none()
                        || alt_değerler.get(indeks + 1).copied().flatten().is_none()
                    {
                        continue;
                    }
                    let mut örnekler = Vec::with_capacity(9);
                    for adım in 0..=8 {
                        let t = adım as f64 / 8.0;
                        let x_değeri = x0 + (x1 - x0) * t;
                        let Some(üst_değer) =
                            seri_ara_değeri(üst_değerler, indeks, t, üst_ayarları.çizim_türü)
                        else {
                            continue;
                        };
                        let Some(alt_değer) =
                            seri_ara_değeri(alt_değerler, indeks, t, alt_ayarları.çizim_türü)
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
                    for çift in örnekler.windows(2) {
                        let Some(a) = çift.first().copied() else {
                            continue;
                        };
                        let Some(b) = çift.get(1).copied() else {
                            continue;
                        };
                        if let Some(çokgen) = bant_dilim_çokgeni(a, b, bant.yön) {
                            let kırpılmış =
                                çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                            if kırpılmış.len() >= 3 {
                                çokgenler.push(kırpılmış);
                            }
                        }
                    }
                }
                çokgenler
            };
            if !çokgenler.is_empty() {
                let çubuk_kenarları = (üst_ayarları.çizim_türü == crate::SeriÇizimTürü::Çubuk
                    && üst_ayarları.çizgi_kalınlığı > 0.0)
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
                sahne.ekle(Komut::Alan {
                    çokgenler,
                    dolgu: bant.dolgu.clone(),
                });
                if let Some(parçalar) = çubuk_kenarları {
                    sahne.ekle(Komut::Yol {
                        parçalar,
                        renk: üst_ayarları.renk.clone(),
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
        let çubuk_genişliği = çubuk_genişliği.min(seri.azami_çubuk_genişliği);
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
