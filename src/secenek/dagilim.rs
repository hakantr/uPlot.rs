#[derive(Debug, Clone, PartialEq)]
pub struct DağılımNoktası {
    pub x: f64,
    pub y: f64,
    pub boyut: f32,
    pub değer: Option<f64>,
    pub etiket: Option<String>,
}

impl DağılımNoktası {
    pub fn yeni(x: f64, y: f64, boyut: f32) -> Self {
        Self {
            x,
            y,
            boyut,
            değer: None,
            etiket: None,
        }
    }

    pub fn değer(mut self, değer: f64) -> Self {
        self.değer = değer.is_finite().then_some(değer);
        self
    }

    pub fn etiket(mut self, etiket: impl Into<String>) -> Self {
        self.etiket = Some(etiket.into());
        self
    }

    pub(crate) fn geçerli_mi(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.boyut.is_finite() && self.boyut > 0.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DağılımSerisi {
    pub etiket: String,
    pub ölçek: String,
    pub renk: String,
    pub dolgu: String,
    pub noktalar: Vec<DağılımNoktası>,
}

impl DağılımSerisi {
    pub fn yeni(etiket: impl Into<String>, renk: impl Into<String>) -> Self {
        let renk = renk.into();
        Self {
            etiket: etiket.into(),
            ölçek: "y".to_string(),
            dolgu: renk.clone(),
            renk,
            noktalar: Vec::new(),
        }
    }

    pub fn ölçek(mut self, ölçek: impl Into<String>) -> Self {
        self.ölçek = ölçek.into();
        self
    }

    pub fn dolgu(mut self, dolgu: impl Into<String>) -> Self {
        self.dolgu = dolgu.into();
        self
    }

    pub fn noktalar(mut self, noktalar: Vec<DağılımNoktası>) -> Self {
        self.noktalar = noktalar;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DağılımDüzeni {
    pub seriler: Vec<DağılımSerisi>,
    pub vuruş_etkin: bool,
}

impl DağılımDüzeni {
    pub fn seri(mut self, seri: DağılımSerisi) -> Self {
        self.seriler.push(seri);
        self
    }

    pub fn vuruş_etkin(mut self, etkin: bool) -> Self {
        self.vuruş_etkin = etkin;
        self
    }

    pub(crate) fn geçerli_mi(&self) -> bool {
        !self.seriler.is_empty()
            && self.seriler.iter().all(|seri| {
                !seri.noktalar.is_empty() && seri.noktalar.iter().all(|n| n.geçerli_mi())
            })
    }
}
