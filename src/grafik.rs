use crate::cizim::{Komut, MetinHizası, Nokta, Sahne};
use crate::etkilesim::EtkileşimDenetleyicisi;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, UplotHatası};

/// Doğrulanmış seçenek ve veriyi taşıyan çizelge örneği.
pub struct Grafik {
    seçenekler: GrafikSeçenekleri,
    veri: HizalıVeri,
    etkileşim: EtkileşimDenetleyicisi,
}

impl Grafik {
    pub fn yeni(seçenekler: GrafikSeçenekleri, veri: HizalıVeri) -> Result<Self, UplotHatası> {
        if seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: seçenekler.seriler.len(),
            });
        }
        let tam = veri
            .x()
            .first()
            .zip(veri.x().last())
            .and_then(|(ilk, son)| Aralık::yeni(*ilk, *son).ok())
            .ok_or(UplotHatası::YetersizVeri {
                uzunluk: veri.x().len(),
            })?;
        let tam_y = seçenekler.y_aralığı.unwrap_or_else(|| {
            Aralık::otomatik(veri.seriler().iter().flat_map(|seri| seri.iter()))
        });
        let etkileşim = EtkileşimDenetleyicisi::yeni(tam, tam_y, seçenekler.etkileşimler);
        Ok(Self {
            seçenekler,
            veri,
            etkileşim,
        })
    }

    pub fn çiz(&self) -> Sahne {
        self.çiz_boyutta_aralıklarla(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            self.etkileşim
                .yakınlaştırılmış()
                .then(|| self.etkileşim.görünür_x()),
            self.etkileşim.görünür_y(),
        )
    }

    pub fn görünür_x_aralığı(&self) -> Aralık {
        self.etkileşim.görünür_x()
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.seçenekler.genişlik, self.seçenekler.yükseklik)
    }

    pub fn yakınlaştırılmış(&self) -> bool {
        self.etkileşim.yakınlaştırılmış()
    }

    pub fn geri_var(&self) -> bool {
        self.etkileşim.geri_var()
    }

    pub fn etkileşim_seçenekleri(&self) -> crate::EtkileşimSeçenekleri {
        self.etkileşim.ayarlar()
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.etkileşim.tekerlek_etkileşimi_ayarla(etkin);
    }

    pub fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas: bool,
    ) -> Result<bool, UplotHatası> {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim
            .tekerlek(yatay_odak_oranı, dikey_odak_oranı, görünür_y, delta, hassas)
    }

    pub fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim
            .seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
    }

    pub fn tam_görünüm(&mut self) -> bool {
        self.etkileşim.tam_görünüm()
    }

    pub fn önceki_görünüm(&mut self) -> bool {
        self.etkileşim.geri()
    }

    pub fn taşımayı_başlat(&mut self) -> bool {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.taşımayı_başlat(görünür_y)
    }

    pub fn taşı(
        &mut self,
        yatay_fark_oranı: f64,
        dikey_fark_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim.taşı(yatay_fark_oranı, dikey_fark_oranı)
    }

    pub fn taşımayı_bitir(&mut self) {
        self.etkileşim.taşımayı_bitir();
    }

    pub fn dokunmayı_başlat(&mut self) -> bool {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.dokunmayı_başlat(görünür_y)
    }

    pub fn dokunma_yakınlaştır(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim
            .dokunma_yakınlaştır(yatay_odak_oranı, dikey_odak_oranı, çarpan)
    }

    pub fn dokunmayı_bitir(&mut self) {
        self.etkileşim.dokunmayı_bitir();
    }

    /// Geçerli X görünümündeki veriden hesaplanan Y aralığını döndürür.
    pub fn görünür_y_aralığı(&self) -> Aralık {
        self.etkileşim
            .görünür_y()
            .unwrap_or_else(|| self.y_aralığı(self.görünür_x_aralığı()))
    }

    /// Geçerli görünümde, normalize edilmiş yatay konuma en yakın seri noktasını bulur.
    pub fn en_yakın_nokta(&self, yatay_oran: f64, seri_indeksi: usize) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let seri = self.veri.seriler().get(seri_indeksi)?;
        let aralık = self.görünür_x_aralığı();
        let hedef = aralık.en_az + yatay_oran.clamp(0.0, 1.0) * (aralık.en_çok - aralık.en_az);
        self.veri
            .x()
            .iter()
            .copied()
            .zip(seri.iter().copied())
            .filter_map(|(x, y)| y.map(|y| (x, y)))
            .filter(|(x, _)| *x >= aralık.en_az && *x <= aralık.en_çok)
            .min_by(|(x_a, _), (x_b, _)| (x_a - hedef).abs().total_cmp(&(x_b - hedef).abs()))
    }

    /// Grafiği belirli bir görünür X aralığında çizer.
    pub fn çiz_aralıkta(&self, görünür_x: Option<Aralık>) -> Sahne {
        self.çiz_boyutta(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            görünür_x,
        )
    }

    /// Etkileşim denetleyicisindeki güncel görünümü hedef yüzey boyutunda çizer.
    pub fn çiz_görünür_boyutta(&self, genişlik_px: u32, yükseklik_px: u32) -> Sahne {
        let görünür = self.yakınlaştırılmış().then(|| self.görünür_x_aralığı());
        self.çiz_boyutta_aralıklarla(
            genişlik_px,
            yükseklik_px,
            görünür,
            self.etkileşim.görünür_y(),
        )
    }

    /// Resize demosundaki gibi hedef yüzey boyutuna göre yeniden yerleşim yapar.
    pub fn çiz_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
    ) -> Sahne {
        self.çiz_boyutta_aralıklarla(genişlik_px, yükseklik_px, görünür_x, None)
    }

    fn çiz_boyutta_aralıklarla(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) -> Sahne {
        let genişlik_px = genişlik_px.max(160);
        let yükseklik_px = yükseklik_px.max(120);
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        sahne.ekle(Komut::ArkaPlan {
            renk: "#ffffff".to_string(),
        });

        let sol = 64.0_f32;
        let sağ = 24.0_f32;
        let üst = 48.0_f32;
        let alt = 48.0_f32;
        let genişlik = genişlik_px as f32 - sol - sağ;
        let yükseklik = yükseklik_px as f32 - üst - alt;

        sahne.ekle(Komut::Metin {
            konum: Nokta::yeni(genişlik_px as f32 / 2.0, 26.0),
            içerik: self.seçenekler.başlık.clone(),
            renk: "#111111".to_string(),
            boyut: 18.0,
            hiza: MetinHizası::Orta,
        });

        let tam_x_aralığı = self
            .veri
            .x()
            .first()
            .zip(self.veri.x().last())
            .and_then(|(ilk, son)| Aralık::yeni(*ilk, *son).ok())
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        let x_aralığı = görünür_x
            .and_then(|aralık| {
                Aralık::yeni(
                    aralık.en_az.max(tam_x_aralığı.en_az),
                    aralık.en_çok.min(tam_x_aralığı.en_çok),
                )
                .ok()
            })
            .unwrap_or(tam_x_aralığı);
        let y_aralığı = görünür_y.unwrap_or_else(|| self.y_aralığı(x_aralığı));

        let y_artımı = uygun_artım(y_aralığı, yükseklik, 30.0);
        for y_değeri in eksen_bölmeleri(y_aralığı, yükseklik, 30.0) {
            let y = üst + yükseklik - y_aralığı.konum(y_değeri, 0.0, yükseklik);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sol + genişlik, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: eksen_değerini_yaz(y_değeri, y_artımı),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });
        }

        let x_artımı = uygun_artım(x_aralığı, genişlik, 50.0);
        for x_değeri in eksen_bölmeleri(x_aralığı, genişlik, 50.0) {
            let x = x_aralığı.konum(x_değeri, sol, genişlik);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(x, üst),
                bitiş: Nokta::yeni(x, üst + yükseklik),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(x, üst + yükseklik + 20.0),
                içerik: eksen_değerini_yaz(x_değeri, x_artımı),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Orta,
            });
        }

        for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
            let Some(seri) = self.seçenekler.seriler.get(seri_indeksi) else {
                continue;
            };
            if !seri.göster {
                continue;
            }
            let mut parçalar = Vec::<Vec<Nokta>>::new();
            let mut parça = Vec::<Nokta>::new();
            let mut görünür_noktalar = Vec::<Nokta>::new();
            for (indeks, değer) in değerler.iter().enumerate() {
                let Some(x_değeri) = self.veri.x().get(indeks) else {
                    continue;
                };
                match değer {
                    Some(y_değeri)
                        if *x_değeri >= x_aralığı.en_az && *x_değeri <= x_aralığı.en_çok =>
                    {
                        let x = x_aralığı.konum(*x_değeri, sol, genişlik);
                        let y = üst + yükseklik - (y_aralığı.konum(*y_değeri, 0.0, yükseklik));
                        let nokta = Nokta::yeni(x, y);
                        parça.push(nokta);
                        if nokta_dikdörtgende(nokta, sol, sol + genişlik, üst, üst + yükseklik)
                        {
                            görünür_noktalar.push(nokta);
                        }
                    }
                    _ if !parça.is_empty() => {
                        parçalar.push(std::mem::take(&mut parça));
                    }
                    _ => {}
                }
            }
            if !parça.is_empty() {
                parçalar.push(parça);
            }
            let parçalar =
                yolu_dikdörtgene_kırp(&parçalar, sol, sol + genişlik, üst, üst + yükseklik);
            sahne.ekle(Komut::Yol {
                parçalar,
                renk: seri.renk.clone(),
                kalınlık: seri.çizgi_kalınlığı,
            });

            // uPlot'un varsayılanı: noktalar ancak ortalama yatay boşluk,
            // nokta çapının iki katını karşılayabildiğinde görünür.
            let ortalama_boşluk = genişlik / görünür_noktalar.len().saturating_sub(1).max(1) as f32;
            if ortalama_boşluk >= 10.0 {
                for nokta in görünür_noktalar {
                    sahne.ekle(Komut::Daire {
                        merkez: nokta,
                        yarıçap: 2.5,
                        dolgu: "#ffffff".to_string(),
                        çizgi: seri.renk.clone(),
                        kalınlık: 1.0,
                    });
                }
            }
        }

        sahne
    }

    fn y_aralığı(&self, x_aralığı: Aralık) -> Aralık {
        self.seçenekler.y_aralığı.unwrap_or_else(|| {
            let görünür = self
                .veri
                .x()
                .iter()
                .enumerate()
                .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                .flat_map(|(indeks, _)| {
                    self.veri
                        .seriler()
                        .iter()
                        .filter_map(move |seri| seri.get(indeks))
                });
            Aralık::otomatik(görünür)
        })
    }
}

/// uPlot'un sayısal eksen yaklaşımı gibi görünür aralık ve piksel yoğunluğuna
/// göre 1/2/2.5/5 × 10ⁿ ailesinden uygun artımı seçer.
fn uygun_artım(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> f64 {
    let uzunluk = aralık.en_çok - aralık.en_az;
    if !uzunluk.is_finite() || uzunluk <= 0.0 || !boyut.is_finite() || boyut <= 0.0 {
        return 1.0;
    }
    let hedef = uzunluk * f64::from(en_az_boşluk.max(1.0)) / f64::from(boyut);
    if !hedef.is_finite() || hedef <= 0.0 {
        return 1.0;
    }
    let taban = 10_f64.powf(hedef.log10().floor());
    for çarpan in [1.0_f64, 2.0, 2.5, 5.0, 10.0] {
        let aday = taban * çarpan;
        if aday >= hedef && aday.is_finite() {
            return aday;
        }
    }
    hedef
}

fn eksen_bölmeleri(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> Vec<f64> {
    let artım = uygun_artım(aralık, boyut, en_az_boşluk);
    let tolerans = artım.abs() * 1e-9;
    let mut değer = ((aralık.en_az - tolerans) / artım).ceil() * artım;
    let mut bölmeler = Vec::new();
    for _ in 0..1_000 {
        if değer > aralık.en_çok + tolerans {
            break;
        }
        let yuvarlanmış = artıma_yuvarla(değer, artım);
        bölmeler.push(if yuvarlanmış.abs() <= tolerans {
            0.0
        } else {
            yuvarlanmış
        });
        değer += artım;
    }
    bölmeler
}

fn artıma_yuvarla(değer: f64, artım: f64) -> f64 {
    let basamak = ondalık_basamak(artım);
    let kuvvet = 10_f64.powf(f64::from(basamak));
    (değer * kuvvet).round() / kuvvet
}

fn ondalık_basamak(artım: f64) -> u32 {
    let mut ölçekli = artım.abs();
    for basamak in 0..=12 {
        if (ölçekli - ölçekli.round()).abs() <= 1e-9 {
            return basamak;
        }
        ölçekli *= 10.0;
    }
    12
}

fn eksen_değerini_yaz(değer: f64, artım: f64) -> String {
    let basamak = usize::try_from(ondalık_basamak(artım).max(2)).unwrap_or(12);
    format!("{değer:.basamak$}")
}

fn nokta_dikdörtgende(nokta: Nokta, sol: f32, sağ: f32, üst: f32, alt: f32) -> bool {
    (sol..=sağ).contains(&nokta.x) && (üst..=alt).contains(&nokta.y)
}

fn yolu_dikdörtgene_kırp(
    parçalar: &[Vec<Nokta>],
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Vec<Vec<Nokta>> {
    let mut sonuç = Vec::new();
    for parça in parçalar {
        if parça.len() == 1 {
            if let Some(nokta) = parça.first().copied()
                && nokta_dikdörtgende(nokta, sol, sağ, üst, alt)
            {
                sonuç.push(vec![nokta]);
            }
            continue;
        }
        let mut kırpılmış = Vec::new();
        let mut noktalar = parça.iter().copied();
        let Some(mut önceki) = noktalar.next() else {
            continue;
        };
        for sonraki in noktalar {
            if let Some((başlangıç, bitiş)) =
                doğru_parçasını_kırp(önceki, sonraki, sol, sağ, üst, alt)
            {
                if kırpılmış.last().is_some_and(|son| *son != başlangıç) {
                    if kırpılmış.len() >= 2 {
                        sonuç.push(std::mem::take(&mut kırpılmış));
                    } else {
                        kırpılmış.clear();
                    }
                }
                if kırpılmış.is_empty() {
                    kırpılmış.push(başlangıç);
                }
                if kırpılmış.last().is_none_or(|son| *son != bitiş) {
                    kırpılmış.push(bitiş);
                }
            } else if kırpılmış.len() >= 2 {
                sonuç.push(std::mem::take(&mut kırpılmış));
            } else {
                kırpılmış.clear();
            }
            önceki = sonraki;
        }
        if kırpılmış.len() >= 2 {
            sonuç.push(kırpılmış);
        }
    }
    sonuç
}

fn doğru_parçasını_kırp(
    başlangıç: Nokta,
    bitiş: Nokta,
    sol: f32,
    sağ: f32,
    üst: f32,
    alt: f32,
) -> Option<(Nokta, Nokta)> {
    let dx = bitiş.x - başlangıç.x;
    let dy = bitiş.y - başlangıç.y;
    let mut en_az_t = 0.0_f32;
    let mut en_çok_t = 1.0_f32;
    if !kırpma_parametresini_uygula(-dx, başlangıç.x - sol, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(dx, sağ - başlangıç.x, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(-dy, başlangıç.y - üst, &mut en_az_t, &mut en_çok_t)
        || !kırpma_parametresini_uygula(dy, alt - başlangıç.y, &mut en_az_t, &mut en_çok_t)
    {
        return None;
    }
    Some((
        Nokta::yeni(başlangıç.x + en_az_t * dx, başlangıç.y + en_az_t * dy),
        Nokta::yeni(başlangıç.x + en_çok_t * dx, başlangıç.y + en_çok_t * dy),
    ))
}

fn kırpma_parametresini_uygula(p: f32, q: f32, en_az_t: &mut f32, en_çok_t: &mut f32) -> bool {
    if p.abs() <= f32::EPSILON {
        return q >= 0.0;
    }
    let oran = q / p;
    if p < 0.0 {
        *en_az_t = en_az_t.max(oran);
    } else {
        *en_çok_t = en_çok_t.min(oran);
    }
    *en_az_t <= *en_çok_t
}

#[cfg(test)]
mod eksen_testleri {
    use super::*;

    #[test]
    fn bölmeler_sıfıra_hizalanır_ve_yakınlaştıkça_ondalık_detayı_artırır() {
        let tam = Aralık {
            en_az: -1.2,
            en_çok: 1.2,
        };
        let yakın = Aralık {
            en_az: -0.011,
            en_çok: 0.013,
        };
        let tam_artım = uygun_artım(tam, 304.0, 30.0);
        let yakın_artım = uygun_artım(yakın, 304.0, 30.0);
        let yakın_bölmeler = eksen_bölmeleri(yakın, 304.0, 30.0);
        let hizalı_tam_bölmeler = eksen_bölmeleri(tam, 593.0, 30.0);

        assert!(yakın_artım < tam_artım);
        assert!(yakın_bölmeler.contains(&0.0));
        assert!(hizalı_tam_bölmeler.contains(&-1.2));
        assert!(hizalı_tam_bölmeler.contains(&1.2));
        assert_eq!(eksen_değerini_yaz(0.0, yakın_artım), "0.0000");
        assert!(yakın_bölmeler.windows(2).all(|çift| {
            çift
                .first()
                .zip(çift.get(1))
                .is_some_and(|(sol, sağ)| sol < sağ)
        }));
    }

    #[test]
    fn veri_yolu_grafik_dikdörtgeninin_üst_ve_alt_sınırında_kesilir() {
        let parçalar = vec![vec![
            Nokta::yeni(0.0, 5.0),
            Nokta::yeni(5.0, -10.0),
            Nokta::yeni(10.0, 20.0),
            Nokta::yeni(15.0, 5.0),
        ]];
        let kırpılmış = yolu_dikdörtgene_kırp(&parçalar, 0.0, 15.0, 0.0, 10.0);

        assert!(!kırpılmış.is_empty());
        assert!(
            kırpılmış.iter().flatten().all(|nokta| {
                (0.0..=15.0).contains(&nokta.x) && (0.0..=10.0).contains(&nokta.y)
            })
        );
        assert!(kırpılmış.iter().flatten().any(|nokta| nokta.y == 0.0));
        assert!(kırpılmış.iter().flatten().any(|nokta| nokta.y == 10.0));
    }
}
