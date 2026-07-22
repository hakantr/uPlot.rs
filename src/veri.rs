use crate::hata::UplotHatası;

/// uPlot'un sütunlu, ortak x eksenine hizalı veri biçimi.
#[derive(Debug, Clone, PartialEq)]
pub struct HizalıVeri {
    x: Vec<f64>,
    seriler: Vec<Vec<Option<f64>>>,
}

impl HizalıVeri {
    /// Veriyi doğrular. X değerleri sonlu, benzersiz ve kesin artan olmalıdır.
    pub fn yeni(x: Vec<f64>, seriler: Vec<Vec<Option<f64>>>) -> Result<Self, UplotHatası> {
        if x.len() < 2 {
            return Err(UplotHatası::YetersizVeri { uzunluk: x.len() });
        }

        for (indeks, değer) in x.iter().enumerate() {
            if !değer.is_finite() {
                return Err(UplotHatası::SonluOlmayanX { indeks });
            }
            if indeks > 0
                && x.get(indeks.saturating_sub(1))
                    .is_some_and(|önceki| önceki >= değer)
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

        Ok(Self { x, seriler })
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
}
