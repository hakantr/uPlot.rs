use crate::Aralık;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum YÖlçekDağılımı {
    Doğrusal,
    ArcSinh { eşik: f64 },
}

/// uPlot'un adlandırılmış Y ölçekleri ve bunlara bağlı eksenlerinin çekirdek
/// karşılığıdır. `y` anahtarı birincil sol eksendir; diğer ölçekler sağda
/// gösterilebilir.
#[derive(Debug, Clone, PartialEq)]
pub struct YÖlçekSeçenekleri {
    pub anahtar: String,
    pub aralık: Option<Aralık>,
    pub sağda: bool,
    pub ızgara: bool,
    pub eksen_görünür: bool,
    pub eksen_rengi: String,
    pub birim: String,
    pub kaynak: Option<String>,
    pub dönüşüm_çarpanı: f64,
    pub dönüşüm_kaydırması: f64,
    pub dağılım: YÖlçekDağılımı,
}

impl YÖlçekSeçenekleri {
    pub fn yeni(anahtar: impl Into<String>) -> Self {
        Self {
            anahtar: anahtar.into(),
            aralık: None,
            sağda: false,
            ızgara: true,
            eksen_görünür: false,
            eksen_rengi: "#4b5563".to_string(),
            birim: String::new(),
            kaynak: None,
            dönüşüm_çarpanı: 1.0,
            dönüşüm_kaydırması: 0.0,
            dağılım: YÖlçekDağılımı::Doğrusal,
        }
    }

    pub fn aralık(mut self, aralık: Aralık) -> Self {
        self.aralık = Some(aralık);
        self
    }

    pub fn sağda(mut self, sağda: bool) -> Self {
        self.sağda = sağda;
        self
    }

    pub fn ızgara(mut self, görünür: bool) -> Self {
        self.ızgara = görünür;
        self
    }

    pub fn eksen(mut self, görünür: bool) -> Self {
        self.eksen_görünür = görünür;
        self
    }

    pub fn eksen_rengi(mut self, renk: impl Into<String>) -> Self {
        self.eksen_rengi = renk.into();
        self
    }

    pub fn birim(mut self, birim: impl Into<String>) -> Self {
        self.birim = birim.into();
        self
    }

    /// uPlot `scale.from` ve `scale.range` birleşiminin doğrusal karşılığıdır.
    /// `çıktı = kaynak * çarpan + kaydırma` dönüşümünü uygular.
    pub fn kaynak_dönüşümü(
        mut self,
        kaynak: impl Into<String>,
        çarpan: f64,
        kaydırma: f64,
    ) -> Self {
        if çarpan.is_finite() && çarpan.abs() > f64::EPSILON && kaydırma.is_finite() {
            self.kaynak = Some(kaynak.into());
            self.dönüşüm_çarpanı = çarpan;
            self.dönüşüm_kaydırması = kaydırma;
        }
        self
    }

    /// uPlot `distr: 4` ve `asinh` doğrusal eşik ayarını etkinleştirir.
    pub fn arcsinh(mut self, eşik: f64) -> Self {
        if eşik.is_finite() && eşik > 0.0 {
            self.dağılım = YÖlçekDağılımı::ArcSinh { eşik };
        }
        self
    }
}
