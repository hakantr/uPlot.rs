use crate::cizim::{Komut, MetinHizası, Nokta, Sahne};
use crate::etkilesim::EtkileşimDenetleyicisi;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, UplotHatası};

/// Doğrulanmış seçenek ve veriyi taşıyan çizelge örneği.
pub struct Grafik {
    seçenekler: GrafikSeçenekleri,
    veri: HizalıVeri,
    etkileşim: EtkileşimDenetleyicisi,
}

impl Grafik {
    pub fn yeni(seçenekler: GrafikSeçenekleri, veri: HizalıVeri) -> Result<Self, UplotHatası> {
        if seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: seçenekler.seriler.len(),
            });
        }
        let tam = veri
            .x()
            .first()
            .zip(veri.x().last())
            .and_then(|(ilk, son)| Aralık::yeni(*ilk, *son).ok())
            .ok_or(UplotHatası::YetersizVeri {
                uzunluk: veri.x().len(),
            })?;
        let etkileşim = EtkileşimDenetleyicisi::yeni(tam, seçenekler.etkileşimler);
        Ok(Self {
            seçenekler,
            veri,
            etkileşim,
        })
    }

    pub fn çiz(&self) -> Sahne {
        self.çiz_aralıkta(
            self.etkileşim
                .yakınlaştırılmış()
                .then(|| self.etkileşim.görünür()),
        )
    }

    pub fn görünür_x_aralığı(&self) -> Aralık {
        self.etkileşim.görünür()
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.seçenekler.genişlik, self.seçenekler.yükseklik)
    }

    pub fn yakınlaştırılmış(&self) -> bool {
        self.etkileşim.yakınlaştırılmış()
    }

    pub fn geri_var(&self) -> bool {
        self.etkileşim.geri_var()
    }

    pub fn etkileşim_seçenekleri(&self) -> crate::EtkileşimSeçenekleri {
        self.etkileşim.ayarlar()
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.etkileşim.tekerlek_etkileşimi_ayarla(etkin);
    }

    pub fn tekerlek(
        &mut self,
        odak_oranı: f64,
        delta: f64,
        hassas: bool,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim.tekerlek(odak_oranı, delta, hassas)
    }

    pub fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim
            .seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
    }

    pub fn tam_görünüm(&mut self) -> bool {
        self.etkileşim.tam_görünüm()
    }

    pub fn önceki_görünüm(&mut self) -> bool {
        self.etkileşim.geri()
    }

    /// Geçerli X görünümündeki veriden hesaplanan Y aralığını döndürür.
    pub fn görünür_y_aralığı(&self) -> Aralık {
        self.y_aralığı(self.görünür_x_aralığı())
    }

    /// Geçerli görünümde, normalize edilmiş yatay konuma en yakın seri noktasını bulur.
    pub fn en_yakın_nokta(&self, yatay_oran: f64, seri_indeksi: usize) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let seri = self.veri.seriler().get(seri_indeksi)?;
        let aralık = self.görünür_x_aralığı();
        let hedef = aralık.en_az + yatay_oran.clamp(0.0, 1.0) * (aralık.en_çok - aralık.en_az);
        self.veri
            .x()
            .iter()
            .copied()
            .zip(seri.iter().copied())
            .filter_map(|(x, y)| y.map(|y| (x, y)))
            .filter(|(x, _)| *x >= aralık.en_az && *x <= aralık.en_çok)
            .min_by(|(x_a, _), (x_b, _)| (x_a - hedef).abs().total_cmp(&(x_b - hedef).abs()))
    }

    /// Grafiği belirli bir görünür X aralığında çizer.
    pub fn çiz_aralıkta(&self, görünür_x: Option<Aralık>) -> Sahne {
        self.çiz_boyutta(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            görünür_x,
        )
    }

    /// Etkileşim denetleyicisindeki güncel görünümü hedef yüzey boyutunda çizer.
    pub fn çiz_görünür_boyutta(&self, genişlik_px: u32, yükseklik_px: u32) -> Sahne {
        let görünür = self.yakınlaştırılmış().then(|| self.görünür_x_aralığı());
        self.çiz_boyutta(genişlik_px, yükseklik_px, görünür)
    }

    /// Resize demosundaki gibi hedef yüzey boyutuna göre yeniden yerleşim yapar.
    pub fn çiz_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
    ) -> Sahne {
        let genişlik_px = genişlik_px.max(160);
        let yükseklik_px = yükseklik_px.max(120);
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        sahne.ekle(Komut::ArkaPlan {
            renk: "#ffffff".to_string(),
        });

        let sol = 64.0_f32;
        let sağ = 24.0_f32;
        let üst = 48.0_f32;
        let alt = 48.0_f32;
        let genişlik = genişlik_px as f32 - sol - sağ;
        let yükseklik = yükseklik_px as f32 - üst - alt;

        sahne.ekle(Komut::Metin {
            konum: Nokta::yeni(genişlik_px as f32 / 2.0, 26.0),
            içerik: self.seçenekler.başlık.clone(),
            renk: "#111111".to_string(),
            boyut: 18.0,
            hiza: MetinHizası::Orta,
        });

        let tam_x_aralığı = self
            .veri
            .x()
            .first()
            .zip(self.veri.x().last())
            .and_then(|(ilk, son)| Aralık::yeni(*ilk, *son).ok())
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        let x_aralığı = görünür_x
            .and_then(|aralık| {
                Aralık::yeni(
                    aralık.en_az.max(tam_x_aralığı.en_az),
                    aralık.en_çok.min(tam_x_aralığı.en_çok),
                )
                .ok()
            })
            .unwrap_or(tam_x_aralığı);
        let y_aralığı = self.y_aralığı(x_aralığı);

        let bölme = 4_u32;
        for sıra in 0..=bölme {
            let oran = sıra as f32 / bölme as f32;
            let y = üst + oran * yükseklik;
            let x = sol + oran * genişlik;
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sol + genişlik, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(x, üst),
                bitiş: Nokta::yeni(x, üst + yükseklik),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });

            let y_değeri = y_aralığı.en_çok
                - f64::from(sıra) / f64::from(bölme) * (y_aralığı.en_çok - y_aralığı.en_az);
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: format!("{y_değeri:.2}"),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });

            let x_değeri = x_aralığı.en_az
                + f64::from(sıra) / f64::from(bölme) * (x_aralığı.en_çok - x_aralığı.en_az);
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(x, üst + yükseklik + 20.0),
                içerik: format!("{x_değeri:.2}"),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Orta,
            });
        }

        for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
            let Some(seri) = self.seçenekler.seriler.get(seri_indeksi) else {
                continue;
            };
            if !seri.göster {
                continue;
            }
            let mut parçalar = Vec::<Vec<Nokta>>::new();
            let mut parça = Vec::<Nokta>::new();
            let mut görünür_noktalar = Vec::<Nokta>::new();
            for (indeks, değer) in değerler.iter().enumerate() {
                let Some(x_değeri) = self.veri.x().get(indeks) else {
                    continue;
                };
                match değer {
                    Some(y_değeri)
                        if *x_değeri >= x_aralığı.en_az && *x_değeri <= x_aralığı.en_çok =>
                    {
                        let x = x_aralığı.konum(*x_değeri, sol, genişlik);
                        let y = üst + yükseklik - (y_aralığı.konum(*y_değeri, 0.0, yükseklik));
                        let nokta = Nokta::yeni(x, y);
                        parça.push(nokta);
                        görünür_noktalar.push(nokta);
                    }
                    _ if !parça.is_empty() => {
                        parçalar.push(std::mem::take(&mut parça));
                    }
                    _ => {}
                }
            }
            if !parça.is_empty() {
                parçalar.push(parça);
            }
            sahne.ekle(Komut::Yol {
                parçalar,
                renk: seri.renk.clone(),
                kalınlık: seri.çizgi_kalınlığı,
            });

            // uPlot'un varsayılanı: noktalar ancak ortalama yatay boşluk,
            // nokta çapının iki katını karşılayabildiğinde görünür.
            let ortalama_boşluk = genişlik / görünür_noktalar.len().saturating_sub(1).max(1) as f32;
            if ortalama_boşluk >= 10.0 {
                for nokta in görünür_noktalar {
                    sahne.ekle(Komut::Daire {
                        merkez: nokta,
                        yarıçap: 2.5,
                        dolgu: "#ffffff".to_string(),
                        çizgi: seri.renk.clone(),
                        kalınlık: 1.0,
                    });
                }
            }
        }

        sahne
    }

    fn y_aralığı(&self, x_aralığı: Aralık) -> Aralık {
        self.seçenekler.y_aralığı.unwrap_or_else(|| {
            let görünür = self
                .veri
                .x()
                .iter()
                .enumerate()
                .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                .flat_map(|(indeks, _)| {
                    self.veri
                        .seriler()
                        .iter()
                        .filter_map(move |seri| seri.get(indeks))
                });
            Aralık::otomatik(görünür)
        })
    }
}
