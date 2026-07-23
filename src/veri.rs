use crate::hata::UplotHatası;

/// uPlot'un sütunlu, ortak x eksenine hizalı veri biçimi.
#[derive(Debug, Clone, PartialEq)]
pub struct HizalıVeri {
    x: Vec<f64>,
    seriler: Vec<Vec<Option<f64>>>,
    hizalama_eksikleri: Vec<Vec<bool>>,
}

/// Hizalı bir seri hücresinin uPlot veri anlamı.
///
/// JavaScript tarafındaki `null` gerçek bir çizim boşluğudur; `undefined`
/// ise `uPlot.join()` ile oluşan hizalama eksikliği gibi yolda atlanır.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HizalıDeğer {
    Değer(f64),
    Boş,
    Tanımsız,
}

/// uPlot `join()` null kipleri.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoşlukKipi {
    /// Açık `null` değerini hizalama artefaktına dönüştürür (`NULL_REMOVE`).
    Kaldır,
    /// Açık `null` değerini korur (`NULL_RETAIN`, varsayılan).
    Koru,
    /// Açık `null` değerini komşu hizalama artefaktlarına yayar (`NULL_EXPAND`).
    Genişlet,
}

impl HizalıVeri {
    /// Veriyi doğrular. X değerleri sonlu ve azalmayan sırada olmalıdır.
    /// uPlot, aynı saniyeye düşen commitler gibi yinelenen X değerlerini kabul eder.
    /// uPlot'un `null`, `[]` ve `[[], []]` girdileri, bütün seri sütunları da
    /// boş olduğunda sıfır uzunluklu hizalı veri olarak korunur.
    pub fn yeni(x: Vec<f64>, seriler: Vec<Vec<Option<f64>>>) -> Result<Self, UplotHatası> {
        for (indeks, değer) in x.iter().enumerate() {
            if !değer.is_finite() {
                return Err(UplotHatası::SonluOlmayanX { indeks });
            }
            if indeks > 0
                && x.get(indeks.saturating_sub(1))
                    .is_some_and(|önceki| önceki > değer)
            {
                return Err(UplotHatası::SırasızX { indeks });
            }
        }

        for (seri, değerler) in seriler.iter().enumerate() {
            if değerler.len() != x.len() {
                return Err(UplotHatası::SeriUzunluğu {
                    seri,
                    beklenen: x.len(),
                    bulunan: değerler.len(),
                });
            }
            for (indeks, değer) in değerler.iter().enumerate() {
                if değer.is_some_and(|sayı| !sayı.is_finite()) {
                    return Err(UplotHatası::SonluOlmayanY { seri, indeks });
                }
            }
        }

        let hizalama_eksikleri = seriler.iter().map(|seri| vec![false; seri.len()]).collect();
        Ok(Self {
            x,
            seriler,
            hizalama_eksikleri,
        })
    }

    /// Açık `null` ve `undefined` ayrımını koruyarak hizalı veri oluşturur.
    pub fn anlamlı(x: Vec<f64>, seriler: Vec<Vec<HizalıDeğer>>) -> Result<Self, UplotHatası> {
        let değerler = seriler
            .iter()
            .map(|seri| {
                seri.iter()
                    .map(|değer| match değer {
                        HizalıDeğer::Değer(sayı) => Some(*sayı),
                        HizalıDeğer::Boş | HizalıDeğer::Tanımsız => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let maskeler = seriler
            .iter()
            .map(|seri| {
                seri.iter()
                    .map(|değer| matches!(değer, HizalıDeğer::Tanımsız))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Self::hizalama_maskeli(x, değerler, maskeler)
    }

    pub fn x(&self) -> &[f64] {
        &self.x
    }

    pub fn seriler(&self) -> &[Vec<Option<f64>>] {
        &self.seriler
    }

    pub fn uzunluk(&self) -> usize {
        self.x.len()
    }

    /// `None` değerinin kaynak `null` yerine `join()` hizalama artefaktı
    /// (`undefined`) olup olmadığını bildirir.
    pub fn hizalama_eksiği_mi(&self, seri: usize, indeks: usize) -> bool {
        self.hizalama_eksikleri
            .get(seri)
            .and_then(|maske| maske.get(indeks))
            .copied()
            .unwrap_or(false)
    }

    pub(crate) fn seri_ekle(
        &self,
        indeks: usize,
        değerler: Vec<Option<f64>>,
    ) -> Result<Self, UplotHatası> {
        let mut seriler = self.seriler.clone();
        seriler.insert(indeks, değerler);
        let mut maskeler = self.hizalama_eksikleri.clone();
        maskeler.insert(indeks, vec![false; self.x.len()]);
        Self::hizalama_maskeli(self.x.clone(), seriler, maskeler)
    }

    pub(crate) fn seri_sil(&self, indeks: usize) -> Result<Self, UplotHatası> {
        let mut seriler = self.seriler.clone();
        seriler.remove(indeks);
        let mut maskeler = self.hizalama_eksikleri.clone();
        maskeler.remove(indeks);
        Self::hizalama_maskeli(self.x.clone(), seriler, maskeler)
    }

    fn hizalama_maskeli(
        x: Vec<f64>,
        seriler: Vec<Vec<Option<f64>>>,
        hizalama_eksikleri: Vec<Vec<bool>>,
    ) -> Result<Self, UplotHatası> {
        let mut veri = Self::yeni(x, seriler)?;
        let geçerli = hizalama_eksikleri.len() == veri.seriler.len()
            && hizalama_eksikleri
                .iter()
                .zip(veri.seriler.iter())
                .all(|(maske, seri)| maske.len() == seri.len());
        if !geçerli {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "uPlot.join",
                açıklama: "hizalama maskesi veri boyutlarıyla eşleşmiyor".to_string(),
            });
        }
        veri.hizalama_eksikleri = hizalama_eksikleri;
        Ok(veri)
    }
}

/// Ayrı X sütunlarına sahip hizalı tabloları uPlot `join()` algoritmasıyla
/// tek, sıralı X sütununda birleştirir.
pub fn hizalı_verileri_birleştir(
    tablolar: &[HizalıVeri],
    boşluk_kipleri: Option<&[Vec<BoşlukKipi>]>,
) -> Result<HizalıVeri, UplotHatası> {
    let Some(ilk) = tablolar.first() else {
        return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
    };
    if tablolar.iter().all(|tablo| tablo.x() == ilk.x()) {
        let mut seriler = Vec::new();
        let mut maskeler = Vec::new();
        for tablo in tablolar {
            seriler.extend(tablo.seriler.iter().cloned());
            maskeler.extend(tablo.hizalama_eksikleri.iter().cloned());
        }
        return HizalıVeri::hizalama_maskeli(ilk.x().to_vec(), seriler, maskeler);
    }

    let mut x = tablolar
        .iter()
        .flat_map(|tablo| tablo.x().iter().copied())
        .collect::<Vec<_>>();
    x.sort_by(f64::total_cmp);
    x.dedup_by(|sol, sağ| *sol == *sağ);
    let mut birleşik_seriler = Vec::new();
    let mut birleşik_maskeler = Vec::new();

    for (tablo_indeksi, tablo) in tablolar.iter().enumerate() {
        for (seri_indeksi, seri) in tablo.seriler().iter().enumerate() {
            let kip = boşluk_kipleri
                .and_then(|kipler| kipler.get(tablo_indeksi))
                .and_then(|kipler| kipler.get(seri_indeksi))
                .copied()
                .unwrap_or(BoşlukKipi::Koru);
            let mut değerler = vec![None; x.len()];
            let mut hizalama_maskesi = vec![true; x.len()];
            let mut açık_boşluklar = Vec::new();
            for (kaynak_indeksi, x_değeri) in tablo.x().iter().enumerate() {
                let Ok(hedef_indeksi) = x.binary_search_by(|aday| aday.total_cmp(x_değeri)) else {
                    continue;
                };
                let değer = seri.get(kaynak_indeksi).copied().flatten();
                if let Some(değer) = değer {
                    if let Some(hedef) = değerler.get_mut(hedef_indeksi) {
                        *hedef = Some(değer);
                    }
                    if let Some(maske) = hizalama_maskesi.get_mut(hedef_indeksi) {
                        *maske = false;
                    }
                } else if kip != BoşlukKipi::Kaldır {
                    if let Some(maske) = hizalama_maskesi.get_mut(hedef_indeksi) {
                        *maske = false;
                    }
                    if kip == BoşlukKipi::Genişlet {
                        açık_boşluklar.push(hedef_indeksi);
                    }
                }
            }
            if kip == BoşlukKipi::Genişlet {
                for boşluk in açık_boşluklar {
                    let mut sol = boşluk;
                    while let Some(önceki) = sol.checked_sub(1) {
                        if değerler.get(önceki).is_some_and(Option::is_some) {
                            break;
                        }
                        if let Some(maske) = hizalama_maskesi.get_mut(önceki) {
                            *maske = false;
                        }
                        sol = önceki;
                    }
                    let mut sağ = boşluk.saturating_add(1);
                    while değerler.get(sağ).is_some_and(Option::is_none) {
                        if let Some(maske) = hizalama_maskesi.get_mut(sağ) {
                            *maske = false;
                        }
                        sağ = sağ.saturating_add(1);
                    }
                }
            }
            birleşik_seriler.push(değerler);
            birleşik_maskeler.push(hizalama_maskesi);
        }
    }
    HizalıVeri::hizalama_maskeli(x, birleşik_seriler, birleşik_maskeler)
}

#[cfg(test)]
mod birleştirme_testleri {
    use super::*;

    #[test]
    fn yinelenen_x_kabul_edilir_azalan_x_reddedilir() -> Result<(), UplotHatası> {
        let yinelenen = HizalıVeri::yeni(
            vec![1.0, 1.0, 2.0],
            vec![vec![Some(1.0), Some(2.0), Some(3.0)]],
        );
        assert!(yinelenen.is_ok());
        let azalan = HizalıVeri::yeni(vec![1.0, 0.5], vec![vec![Some(1.0), Some(2.0)]]);
        assert!(matches!(azalan, Err(UplotHatası::SırasızX { indeks: 1 })));
        Ok(())
    }

    #[test]
    fn join_sıralı_birleşim_ve_null_expand_maskesini_korur() -> Result<(), UplotHatası> {
        let a = HizalıVeri::yeni(
            vec![3.0, 5.0, 6.0, 7.0, 20.0],
            vec![vec![Some(2.0), Some(3.0), None, Some(10.0), Some(5.0)]],
        )?;
        let b = HizalıVeri::yeni(
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 17.0],
            vec![vec![
                Some(7.0),
                Some(2.0),
                Some(1.0),
                None,
                Some(6.0),
                Some(13.0),
            ]],
        )?;
        let birleşik = hizalı_verileri_birleştir(
            &[a, b],
            Some(&[vec![BoşlukKipi::Genişlet], vec![BoşlukKipi::Koru]]),
        )?;
        assert_eq!(
            birleşik.x(),
            &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 17.0, 20.0]
        );
        assert_eq!(birleşik.seriler().len(), 2);
        assert!(!birleşik.hizalama_eksiği_mi(0, 4));
        assert!(!birleşik.hizalama_eksiği_mi(0, 5));
        assert!(birleşik.hizalama_eksiği_mi(1, 5));
        Ok(())
    }
}
