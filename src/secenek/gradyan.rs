use crate::UplotHatası;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GradyanEkseni {
    X,
    Y,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradyanKonumu {
    Değer(f64),
    NegatifSonsuz,
    PozitifSonsuz,
    GörünürVeriOranı(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GradyanDurağı {
    pub konum: GradyanKonumu,
    pub renk: String,
}

impl GradyanDurağı {
    pub fn değer(değer: f64, renk: impl Into<String>) -> Result<Self, UplotHatası> {
        if !değer.is_finite() {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: format!("gradyan durağı sonlu olmalı; bulunan: {değer}"),
            });
        }
        Ok(Self {
            konum: GradyanKonumu::Değer(değer),
            renk: renk.into(),
        })
    }

    pub fn negatif_sonsuz(renk: impl Into<String>) -> Self {
        Self {
            konum: GradyanKonumu::NegatifSonsuz,
            renk: renk.into(),
        }
    }

    pub fn pozitif_sonsuz(renk: impl Into<String>) -> Self {
        Self {
            konum: GradyanKonumu::PozitifSonsuz,
            renk: renk.into(),
        }
    }

    pub fn görünür_veri_oranı(
        oran: f64,
        renk: impl Into<String>,
    ) -> Result<Self, UplotHatası> {
        if !oran.is_finite() || !(0.0..=1.0).contains(&oran) {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: format!("görünür veri oranı 0..=1 olmalı; bulunan: {oran}"),
            });
        }
        Ok(Self {
            konum: GradyanKonumu::GörünürVeriOranı(oran),
            renk: renk.into(),
        })
    }
}

/// uPlot örneklerindeki `scaleGradient` yardımcısının yüzeyden bağımsız karşılığı.
#[derive(Debug, Clone, PartialEq)]
pub struct ÖlçekGradyanı {
    pub eksen: GradyanEkseni,
    pub duraklar: Vec<GradyanDurağı>,
    pub ayrık: bool,
}

impl ÖlçekGradyanı {
    pub fn yeni(
        eksen: GradyanEkseni, duraklar: Vec<GradyanDurağı>
    ) -> Result<Self, UplotHatası> {
        if duraklar.is_empty() {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: "en az bir renk durağı gerekli".to_string(),
            });
        }
        if duraklar.iter().any(|durak| durak.renk.trim().is_empty()) {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: "gradyan durağının rengi boş olamaz".to_string(),
            });
        }
        let göreli = duraklar
            .iter()
            .any(|durak| matches!(durak.konum, GradyanKonumu::GörünürVeriOranı(_)));
        if göreli
            && duraklar
                .iter()
                .any(|durak| !matches!(durak.konum, GradyanKonumu::GörünürVeriOranı(_)))
        {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: "sabit ve görünür-veriye göreli duraklar karıştırılamaz".to_string(),
            });
        }
        let sıralı = duraklar.windows(2).all(|çift| {
            let sol = çift.first().map(|durak| durak.konum);
            let sağ = çift.get(1).map(|durak| durak.konum);
            konum_sırası(sol) <= konum_sırası(sağ)
        });
        if !sıralı {
            return Err(UplotHatası::GeçersizGradyan {
                açıklama: "gradyan durakları küçükten büyüğe sıralanmalı".to_string(),
            });
        }
        Ok(Self {
            eksen,
            duraklar,
            ayrık: false,
        })
    }

    pub fn ayrık(mut self, ayrık: bool) -> Self {
        self.ayrık = ayrık;
        self
    }
}

fn konum_sırası(konum: Option<GradyanKonumu>) -> f64 {
    match konum {
        Some(GradyanKonumu::NegatifSonsuz) => f64::NEG_INFINITY,
        Some(GradyanKonumu::Değer(değer)) => değer,
        Some(GradyanKonumu::GörünürVeriOranı(oran)) => oran,
        Some(GradyanKonumu::PozitifSonsuz) => f64::INFINITY,
        None => f64::NAN,
    }
}
