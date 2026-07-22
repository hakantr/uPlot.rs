use std::fmt::Write as _;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Nokta {
    pub x: f32,
    pub y: f32,
}

impl Nokta {
    pub fn yeni(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetinHizası {
    Başlangıç,
    Orta,
    Bitiş,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Komut {
    ArkaPlan {
        renk: String,
    },
    Çizgi {
        başlangıç: Nokta,
        bitiş: Nokta,
        renk: String,
        kalınlık: f32,
    },
    KesikliÇizgi {
        başlangıç: Nokta,
        bitiş: Nokta,
        renk: String,
        kalınlık: f32,
        kesik: f32,
    },
    Yol {
        parçalar: Vec<Vec<Nokta>>,
        renk: String,
        kalınlık: f32,
    },
    Daire {
        merkez: Nokta,
        yarıçap: f32,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
    },
    Dikdörtgen {
        konum: Nokta,
        genişlik: f32,
        yükseklik: f32,
        dolgu: String,
        çizgi: String,
        kalınlık: f32,
    },
    Metin {
        konum: Nokta,
        içerik: String,
        renk: String,
        boyut: f32,
        hiza: MetinHizası,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sahne {
    genişlik: u32,
    yükseklik: u32,
    komutlar: Vec<Komut>,
}

impl Sahne {
    pub fn yeni(genişlik: u32, yükseklik: u32) -> Self {
        Self {
            genişlik,
            yükseklik,
            komutlar: Vec::new(),
        }
    }

    pub fn ekle(&mut self, komut: Komut) {
        self.komutlar.push(komut);
    }

    pub fn komutlar(&self) -> &[Komut] {
        &self.komutlar
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.genişlik, self.yükseklik)
    }

    /// Sahneyi bağımlılıksız ve belirlenimci bir SVG belgesine dönüştürür.
    pub fn svg(&self) -> String {
        let mut çıktı = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\n",
            self.genişlik, self.yükseklik, self.genişlik, self.yükseklik
        );
        for komut in &self.komutlar {
            match komut {
                Komut::ArkaPlan { renk } => {
                    let _ = writeln!(
                        çıktı,
                        "  <rect width=\"100%\" height=\"100%\" fill=\"{}\"/>",
                        kaçış(renk)
                    );
                }
                Komut::Çizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(başlangıç.x),
                        sayı(başlangıç.y),
                        sayı(bitiş.x),
                        sayı(bitiş.y),
                        kaçış(renk),
                        sayı(*kalınlık)
                    );
                }
                Komut::KesikliÇizgi {
                    başlangıç,
                    bitiş,
                    renk,
                    kalınlık,
                    kesik,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" stroke-dasharray=\"{} {}\"/>",
                        sayı(başlangıç.x),
                        sayı(başlangıç.y),
                        sayı(bitiş.x),
                        sayı(bitiş.y),
                        kaçış(renk),
                        sayı(*kalınlık),
                        sayı(*kesik),
                        sayı(*kesik)
                    );
                }
                Komut::Yol {
                    parçalar,
                    renk,
                    kalınlık,
                } => {
                    let mut d = String::new();
                    for parça in parçalar {
                        for (indeks, nokta) in parça.iter().enumerate() {
                            let işlem = if indeks == 0 { 'M' } else { 'L' };
                            let _ = write!(d, "{işlem}{} {} ", sayı(nokta.x), sayı(nokta.y));
                        }
                    }
                    let _ = writeln!(
                        çıktı,
                        "  <path d=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\" stroke-linejoin=\"round\"/>",
                        d.trim_end(),
                        kaçış(renk),
                        sayı(*kalınlık)
                    );
                }
                Komut::Daire {
                    merkez,
                    yarıçap,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(merkez.x),
                        sayı(merkez.y),
                        sayı(*yarıçap),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık)
                    );
                }
                Komut::Dikdörtgen {
                    konum,
                    genişlik,
                    yükseklik,
                    dolgu,
                    çizgi,
                    kalınlık,
                } => {
                    let _ = writeln!(
                        çıktı,
                        "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>",
                        sayı(konum.x),
                        sayı(konum.y),
                        sayı(*genişlik),
                        sayı(*yükseklik),
                        kaçış(dolgu),
                        kaçış(çizgi),
                        sayı(*kalınlık)
                    );
                }
                Komut::Metin {
                    konum,
                    içerik,
                    renk,
                    boyut,
                    hiza,
                } => {
                    let çapa = match hiza {
                        MetinHizası::Başlangıç => "start",
                        MetinHizası::Orta => "middle",
                        MetinHizası::Bitiş => "end",
                    };
                    let _ = writeln!(
                        çıktı,
                        "  <text x=\"{}\" y=\"{}\" fill=\"{}\" font-family=\"Arial, sans-serif\" font-size=\"{}\" text-anchor=\"{}\">{}</text>",
                        sayı(konum.x),
                        sayı(konum.y),
                        kaçış(renk),
                        sayı(*boyut),
                        çapa,
                        kaçış(içerik)
                    );
                }
            }
        }
        çıktı.push_str("</svg>\n");
        çıktı
    }

    /// Golden testlerde kullanılacak okunabilir komut dökümü.
    pub fn döküm(&self) -> String {
        self.komutlar
            .iter()
            .map(|komut| format!("{komut:?}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn sayı(değer: f32) -> String {
    let yuvarlanmış = (değer * 100.0).round() / 100.0;
    let yuvarlanmış = if yuvarlanmış == 0.0 {
        0.0
    } else {
        yuvarlanmış
    };
    format!("{yuvarlanmış:.2}")
}

fn kaçış(metin: &str) -> String {
    metin
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
