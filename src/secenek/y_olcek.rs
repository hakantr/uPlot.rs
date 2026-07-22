use crate::Aralık;

/// uPlot'un adlandırılmış Y ölçekleri ve bunlara bağlı eksenlerinin çekirdek
/// karşılığıdır. `y` anahtarı birincil sol eksendir; diğer ölçekler sağda
/// gösterilebilir.
#[derive(Debug, Clone, PartialEq)]
pub struct YÖlçekSeçenekleri {
    pub anahtar: String,
    pub aralık: Option<Aralık>,
    pub sağda: bool,
    pub ızgara: bool,
    pub birim: String,
}

impl YÖlçekSeçenekleri {
    pub fn yeni(anahtar: impl Into<String>) -> Self {
        Self {
            anahtar: anahtar.into(),
            aralık: None,
            sağda: false,
            ızgara: true,
            birim: String::new(),
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

    pub fn birim(mut self, birim: impl Into<String>) -> Self {
        self.birim = birim.into();
        self
    }
}
