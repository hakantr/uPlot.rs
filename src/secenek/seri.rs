#[derive(Debug, Clone, PartialEq)]
pub struct SeriSeçenekleri {
    pub etiket: String,
    pub renk: String,
    pub çizgi_kalınlığı: f32,
    pub dolgu: Option<String>,
    pub dolgu_tabanı: f64,
    pub göster: bool,
}

impl SeriSeçenekleri {
    pub fn yeni(etiket: impl Into<String>) -> Self {
        Self {
            etiket: etiket.into(),
            renk: "#000000".to_string(),
            çizgi_kalınlığı: 1.0,
            dolgu: None,
            dolgu_tabanı: 0.0,
            göster: true,
        }
    }

    pub fn renk(mut self, renk: impl Into<String>) -> Self {
        self.renk = renk.into();
        self
    }

    pub fn çizgi_kalınlığı(mut self, kalınlık: f32) -> Self {
        if kalınlık.is_finite() {
            self.çizgi_kalınlığı = kalınlık.max(0.0);
        }
        self
    }

    /// uPlot `Series.fill` karşılığıdır. Doğrusal yol varsayılan olarak
    /// `fillTo = 0` tabanına kapatılır.
    pub fn dolgu(mut self, renk: impl Into<String>) -> Self {
        self.dolgu = Some(renk.into());
        self
    }

    pub fn dolgu_tabanı(mut self, değer: f64) -> Self {
        if değer.is_finite() {
            self.dolgu_tabanı = değer;
        }
        self
    }
}
