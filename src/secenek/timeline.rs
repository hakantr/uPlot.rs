/// Timeline eklentisinin tek bir anlamsal süre veya periyodik örnek hücresi.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineHücresi {
    pub seri_indeksi: usize,
    pub veri_indeksi: usize,
    pub başlangıç: f64,
    pub bitiş: f64,
    pub değer: String,
    pub dolgu: String,
    pub çizgi: String,
    pub çizgi_kalınlığı: f32,
    pub azami_genişlik: Option<f32>,
}

impl TimelineHücresi {
    pub fn yeni(
        seri_indeksi: usize,
        veri_indeksi: usize,
        başlangıç: f64,
        bitiş: f64,
        değer: impl Into<String>,
        dolgu: impl Into<String>,
        çizgi: impl Into<String>,
    ) -> Self {
        Self {
            seri_indeksi,
            veri_indeksi,
            başlangıç,
            bitiş,
            değer: değer.into(),
            dolgu: dolgu.into(),
            çizgi: çizgi.into(),
            çizgi_kalınlığı: 4.0,
            azami_genişlik: None,
        }
    }

    pub fn çizgi_kalınlığı(mut self, kalınlık: f32) -> Self {
        if kalınlık.is_finite() && kalınlık >= 0.0 {
            self.çizgi_kalınlığı = kalınlık;
        }
        self
    }

    pub fn azami_genişlik(mut self, genişlik: f32) -> Self {
        if genişlik.is_finite() && genişlik > 0.0 {
            self.azami_genişlik = Some(genişlik);
        }
        self
    }
}

/// `timelinePlugin()` tarafından üretilen şerit yerleşimi ve hücre katmanı.
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineDüzeni {
    pub seri_etiketleri: Vec<String>,
    pub hücreler: Vec<TimelineHücresi>,
    pub şerit_oranı: f32,
}

impl TimelineDüzeni {
    pub fn yeni<I, S>(seri_etiketleri: I, hücreler: Vec<TimelineHücresi>) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            seri_etiketleri: seri_etiketleri.into_iter().map(Into::into).collect(),
            hücreler,
            şerit_oranı: 0.9,
        }
    }

    pub(crate) fn geçerli_mi(&self, seri_sayısı: usize) -> bool {
        !self.seri_etiketleri.is_empty()
            && self.seri_etiketleri.len() == seri_sayısı
            && self.şerit_oranı.is_finite()
            && (0.0..=1.0).contains(&self.şerit_oranı)
            && self.hücreler.iter().all(|hücre| {
                hücre.seri_indeksi < seri_sayısı
                    && hücre.başlangıç.is_finite()
                    && hücre.bitiş.is_finite()
                    && hücre.bitiş > hücre.başlangıç
                    && hücre.çizgi_kalınlığı.is_finite()
                    && hücre.çizgi_kalınlığı >= 0.0
                    && hücre
                        .azami_genişlik
                        .is_none_or(|genişlik| genişlik.is_finite() && genişlik > 0.0)
                    && !hücre.dolgu.is_empty()
                    && !hücre.çizgi.is_empty()
            })
    }
}
