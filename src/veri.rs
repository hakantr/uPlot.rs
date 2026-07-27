use std::{collections::HashMap, sync::Arc};

use crate::hata::UplotHatası;

/// uPlot'un sütunlu, ortak x eksenine hizalı veri biçimi.
#[derive(Debug, Clone, PartialEq)]
pub struct HizalıVeri {
    iç: Arc<HizalıVeriİç>,
}

#[derive(Debug, Clone, PartialEq)]
struct HizalıVeriİç {
    x: Vec<f64>,
    seriler: Vec<Vec<Option<f64>>>,
    hizalama_eksikleri: Option<Vec<Vec<bool>>>,
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

        Ok(Self {
            iç: Arc::new(HizalıVeriİç {
                x,
                seriler,
                hizalama_eksikleri: None,
            }),
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
        &self.iç.x
    }

    pub fn seriler(&self) -> &[Vec<Option<f64>>] {
        &self.iç.seriler
    }

    pub fn uzunluk(&self) -> usize {
        self.iç.x.len()
    }

    /// İki hizalı verinin aynı doğrulanmış immutable sütun deposunu paylaşıp
    /// paylaşmadığını bildirir. `clone()` O(1) ve kopyala-yaz yaklaşımındadır.
    pub fn aynı_depolamayı_paylaşıyor(&self, diğer: &Self) -> bool {
        Arc::ptr_eq(&self.iç, &diğer.iç)
    }

    /// `None` değerinin kaynak `null` yerine `join()` hizalama artefaktı
    /// (`undefined`) olup olmadığını bildirir.
    pub fn hizalama_eksiği_mi(&self, seri: usize, indeks: usize) -> bool {
        self.iç
            .hizalama_eksikleri
            .as_ref()
            .and_then(|maskeler| maskeler.get(seri))
            .and_then(|maske| maske.get(indeks))
            .copied()
            .unwrap_or(false)
    }

    pub(crate) fn seri_ekle(
        &self,
        indeks: usize,
        değerler: Vec<Option<f64>>,
    ) -> Result<Self, UplotHatası> {
        let mut seriler = self.iç.seriler.clone();
        seriler.insert(indeks, değerler);
        if let Some(mut maskeler) = self.iç.hizalama_eksikleri.clone() {
            maskeler.insert(indeks, vec![false; self.iç.x.len()]);
            Self::hizalama_maskeli(self.iç.x.clone(), seriler, maskeler)
        } else {
            Self::yeni(self.iç.x.clone(), seriler)
        }
    }

    pub(crate) fn seri_sil(&self, indeks: usize) -> Result<Self, UplotHatası> {
        let mut seriler = self.iç.seriler.clone();
        seriler.remove(indeks);
        if let Some(mut maskeler) = self.iç.hizalama_eksikleri.clone() {
            maskeler.remove(indeks);
            Self::hizalama_maskeli(self.iç.x.clone(), seriler, maskeler)
        } else {
            Self::yeni(self.iç.x.clone(), seriler)
        }
    }

    fn hizalama_maskeli(
        x: Vec<f64>,
        seriler: Vec<Vec<Option<f64>>>,
        hizalama_eksikleri: Vec<Vec<bool>>,
    ) -> Result<Self, UplotHatası> {
        let mut veri = Self::yeni(x, seriler)?;
        let geçerli = hizalama_eksikleri.len() == veri.iç.seriler.len()
            && hizalama_eksikleri
                .iter()
                .zip(veri.iç.seriler.iter())
                .all(|(maske, seri)| maske.len() == seri.len());
        if !geçerli {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "uPlot.join",
                açıklama: "hizalama maskesi veri boyutlarıyla eşleşmiyor".to_string(),
            });
        }
        Arc::make_mut(&mut veri.iç).hizalama_eksikleri = hizalama_eksikleri
            .iter()
            .flatten()
            .any(|eksik| *eksik)
            .then_some(hizalama_eksikleri);
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
        let maskeli = tablolar
            .iter()
            .any(|tablo| tablo.iç.hizalama_eksikleri.is_some());
        let mut maskeler = maskeli.then(Vec::new);
        for tablo in tablolar {
            seriler.extend(tablo.iç.seriler.iter().cloned());
            if let Some(birleşik) = maskeler.as_mut() {
                if let Some(kaynak) = tablo.iç.hizalama_eksikleri.as_ref() {
                    birleşik.extend(kaynak.iter().cloned());
                } else {
                    birleşik.extend(tablo.iç.seriler.iter().map(|seri| vec![false; seri.len()]));
                }
            }
        }
        if let Some(maskeler) = maskeler {
            return HizalıVeri::hizalama_maskeli(ilk.x().to_vec(), seriler, maskeler);
        }
        return HizalıVeri::yeni(ilk.x().to_vec(), seriler);
    }

    let mut x = tablolar
        .iter()
        .flat_map(|tablo| tablo.x().iter().copied())
        .collect::<Vec<_>>();
    x.sort_by(f64::total_cmp);
    x.dedup_by(|sol, sağ| *sol == *sağ);
    // uPlot bir kez `Map<X, index>` kurup bütün kaynak hücrelerini O(1)
    // ortalama maliyetle yerleştirir. Sonlu f64 değerlerini bit anahtarıyla
    // taşırken `-0.0 == 0.0` eşitliğini de koruyoruz.
    let x_indeksleri = x
        .iter()
        .enumerate()
        .map(|(indeks, değer)| {
            let anahtar = if *değer == 0.0 {
                0_u64
            } else {
                değer.to_bits()
            };
            (anahtar, indeks)
        })
        .collect::<HashMap<_, _>>();
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
            for (kaynak_indeksi, x_değeri) in tablo.x().iter().enumerate() {
                let anahtar = if *x_değeri == 0.0 {
                    0_u64
                } else {
                    x_değeri.to_bits()
                };
                let Some(&hedef_indeksi) = x_indeksleri.get(&anahtar) else {
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
                } else if kip != BoşlukKipi::Kaldır
                    && let Some(maske) = hizalama_maskesi.get_mut(hedef_indeksi)
                {
                    *maske = false;
                }
            }
            if kip == BoşlukKipi::Genişlet {
                // Her None koşusunu yalnız bir kez tara. Koşuda en az bir açık
                // `null` varsa komşu hizalama `undefined` hücreleri de null'a
                // genişler. Böylece uzun null koşularında tekrarlı sol/sağ
                // yürüyüşün karesel kötü durumu ortadan kalkar.
                let mut başlangıç = 0;
                while başlangıç < değerler.len() {
                    if değerler.get(başlangıç).is_some_and(Option::is_some) {
                        başlangıç += 1;
                        continue;
                    }
                    let mut bitiş = başlangıç + 1;
                    let mut açık_null_var = hizalama_maskesi
                        .get(başlangıç)
                        .is_some_and(|hizalama_eksiği| !hizalama_eksiği);
                    while değerler.get(bitiş).is_some_and(Option::is_none) {
                        açık_null_var |= hizalama_maskesi
                            .get(bitiş)
                            .is_some_and(|hizalama_eksiği| !hizalama_eksiği);
                        bitiş += 1;
                    }
                    if açık_null_var && let Some(koşu) = hizalama_maskesi.get_mut(başlangıç..bitiş)
                    {
                        koşu.fill(false);
                    }
                    başlangıç = bitiş;
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
    fn normal_hizalı_veri_tümü_false_join_maskesi_ayırmaz() -> Result<(), UplotHatası> {
        let veri = HizalıVeri::yeni(
            vec![0.0, 1.0, 2.0],
            vec![
                vec![Some(1.0), None, Some(3.0)],
                vec![Some(4.0), Some(5.0), Some(6.0)],
            ],
        )?;
        assert!(veri.iç.hizalama_eksikleri.is_none());
        assert!(!veri.hizalama_eksiği_mi(0, 1));
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

    #[test]
    fn join_null_kipleri_ve_sıfır_anahtarı_korunur() -> Result<(), UplotHatası> {
        let kaynak =
            HizalıVeri::yeni(vec![-0.0, 2.0, 4.0], vec![vec![Some(1.0), None, Some(3.0)]])?;
        let referans = HizalıVeri::yeni(vec![0.0, 1.0, 3.0, 4.0], vec![])?;

        let kaldır = hizalı_verileri_birleştir(
            &[kaynak.clone(), referans.clone()],
            Some(&[vec![BoşlukKipi::Kaldır], vec![]]),
        )?;
        let koru = hizalı_verileri_birleştir(
            &[kaynak.clone(), referans.clone()],
            Some(&[vec![BoşlukKipi::Koru], vec![]]),
        )?;
        let genişlet = hizalı_verileri_birleştir(
            &[kaynak, referans],
            Some(&[vec![BoşlukKipi::Genişlet], vec![]]),
        )?;

        assert_eq!(genişlet.x(), &[-0.0, 1.0, 2.0, 3.0, 4.0]);
        assert!(kaldır.hizalama_eksiği_mi(0, 2));
        assert!(!koru.hizalama_eksiği_mi(0, 2));
        assert!(koru.hizalama_eksiği_mi(0, 1));
        assert!(!genişlet.hizalama_eksiği_mi(0, 1));
        assert!(!genişlet.hizalama_eksiği_mi(0, 2));
        assert!(!genişlet.hizalama_eksiği_mi(0, 3));
        Ok(())
    }
}
