/// Isı haritası hücresinin bir eksendeki boyut birimi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsıHücresiBoyutu {
    /// Yakınlaştırmayla birlikte ölçeklenen veri birimi.
    Veri(f64),
    /// Kaynak çizim kancasındaki gibi ekranda sabit kalan piksel birimi.
    Piksel(f32),
}

/// Yüzeyden bağımsız tek bir ısı haritası hücresi.
#[derive(Debug, Clone, PartialEq)]
pub struct IsıHücresi {
    pub x: f64,
    pub y: f64,
    pub genişlik: IsıHücresiBoyutu,
    pub yükseklik: IsıHücresiBoyutu,
    /// Hücre merkezinin ölçeklenmiş veri noktasına göre CSS piksel ofseti.
    pub piksel_ofseti: [f32; 2],
    pub renk: String,
}

impl IsıHücresi {
    pub fn yeni(
        x: f64,
        y: f64,
        genişlik: IsıHücresiBoyutu,
        yükseklik: IsıHücresiBoyutu,
        renk: impl Into<String>,
    ) -> Self {
        Self {
            x,
            y,
            genişlik,
            yükseklik,
            piksel_ofseti: [0.0, 0.0],
            renk: renk.into(),
        }
    }

    pub fn piksel_ofseti(mut self, x: f32, y: f32) -> Self {
        if x.is_finite() && y.is_finite() {
            self.piksel_ofseti = [x, y];
        }
        self
    }
}

/// uPlot çizim kancaları ve mode-2 yollarıyla üretilen hücre katmanı.
#[derive(Debug, Clone, PartialEq)]
pub struct IsıHaritasıDüzeni {
    pub hücreler: Vec<IsıHücresi>,
}

impl IsıHaritasıDüzeni {
    pub fn yeni(hücreler: Vec<IsıHücresi>) -> Self {
        Self { hücreler }
    }

    pub(crate) fn geçerli_mi(&self) -> bool {
        self.hücreler.iter().all(|hücre| {
            hücre.x.is_finite()
                && hücre.y.is_finite()
                && hücre.piksel_ofseti.iter().all(|değer| değer.is_finite())
                && boyut_geçerli(hücre.genişlik)
                && boyut_geçerli(hücre.yükseklik)
                && !hücre.renk.is_empty()
        })
    }
}

fn boyut_geçerli(boyut: IsıHücresiBoyutu) -> bool {
    match boyut {
        IsıHücresiBoyutu::Veri(değer) => değer.is_finite() && değer > 0.0,
        IsıHücresiBoyutu::Piksel(değer) => değer.is_finite() && değer > 0.0,
    }
}
