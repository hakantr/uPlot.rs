use crate::hata::UplotHatası;

/// Sonlu ve artan sayısal ölçek aralığı.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aralık {
    pub en_az: f64,
    pub en_çok: f64,
}

impl Aralık {
    pub fn yeni(en_az: f64, en_çok: f64) -> Result<Self, UplotHatası> {
        if !en_az.is_finite() || !en_çok.is_finite() || en_az >= en_çok {
            return Err(UplotHatası::GeçersizAralık { en_az, en_çok });
        }
        Ok(Self { en_az, en_çok })
    }

    pub(crate) fn otomatik<'a>(değerler: impl Iterator<Item = &'a Option<f64>>) -> Self {
        let mut en_az = f64::INFINITY;
        let mut en_çok = f64::NEG_INFINITY;
        for değer in değerler.flatten() {
            en_az = en_az.min(*değer);
            en_çok = en_çok.max(*değer);
        }

        if !en_az.is_finite() || !en_çok.is_finite() {
            return Self {
                en_az: 0.0,
                en_çok: 1.0,
            };
        }
        if en_az == en_çok {
            let pay = en_az.abs().max(1.0) * 0.1;
            return Self {
                en_az: en_az - pay,
                en_çok: en_çok + pay,
            };
        }

        let pay = (en_çok - en_az) * 0.1;
        Self {
            en_az: en_az - pay,
            en_çok: en_çok + pay,
        }
    }

    pub(crate) fn konum(self, değer: f64, başlangıç: f32, uzunluk: f32) -> f32 {
        let oran = (değer - self.en_az) / (self.en_çok - self.en_az);
        başlangıç + (oran as f32 * uzunluk)
    }
}
