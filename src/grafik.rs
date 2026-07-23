mod bant;
mod isi_haritasi;
mod seri_geometrisi;

use seri_geometrisi::seri_yol_noktaları;

use crate::cizim::kirpma::{
    nokta_dikdörtgende, yolu_dikdörtgene_kırp, çokgeni_dikdörtgene_kırp
};
use crate::cizim::{DoğrusalGradyan, GradyanRenkDurağı, Komut, MetinHizası, Nokta, Sahne};
use crate::etkilesim::EtkileşimDenetleyicisi;
use crate::{
    Aralık, GradyanEkseni, GradyanKonumu, GrafikSeçenekleri, HizalıVeri, UplotHatası,
    XÖlçekDağılımı, YÖlçekDağılımı, YÖlçekEtiketBiçimi, ÖlçekGradyanı,
};

/// Bir işaretçi seçiminin çekirdekte çözümlenen sonucu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeçimEylemi {
    /// Kart ayarları seçimi devre dışı bıraktı veya görünüm değişmedi.
    Değişmedi,
    /// Seçilen X aralığı görünür aralık olarak uygulandı.
    Yakınlaştırıldı,
    /// `cursor-bind` bağı yakınlaştırmayı durdurup açıklama UI'si istedi.
    Açıklamaİstendi,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DağılımVuruşu {
    pub seri: usize,
    pub indeks: usize,
    pub merkez: Nokta,
    pub boyut: f32,
    pub x: f64,
    pub y: f64,
    pub değer: Option<f64>,
    pub etiket: Option<String>,
}

/// Doğrulanmış seçenek ve veriyi taşıyan çizelge örneği.
pub struct Grafik {
    seçenekler: GrafikSeçenekleri,
    veri: HizalıVeri,
    etkileşim: EtkileşimDenetleyicisi,
    odak_serisi: Option<usize>,
}

impl Grafik {
    pub fn yeni(seçenekler: GrafikSeçenekleri, veri: HizalıVeri) -> Result<Self, UplotHatası> {
        if seçenekler.seriler.len() != veri.seriler().len() {
            return Err(UplotHatası::SeriSeçeneğiEksik {
                beklenen: veri.seriler().len(),
                bulunan: seçenekler.seriler.len(),
            });
        }
        for (seri, ayarlar) in seçenekler.seriler.iter().enumerate() {
            if ayarlar.ölçek != seçenekler.birincil_y_ölçeği
                && !seçenekler
                    .y_ölçekleri
                    .iter()
                    .any(|ölçek| ölçek.anahtar == ayarlar.ölçek)
            {
                return Err(UplotHatası::BilinmeyenÖlçek {
                    seri,
                    anahtar: ayarlar.ölçek.clone(),
                });
            }
            if let Some(üst_seri) = ayarlar.yüzen_çubuk_üst_serisi
                && üst_seri >= veri.seriler().len()
            {
                return Err(UplotHatası::GeçersizSeriİndeksi {
                    indeks: üst_seri,
                    seri_sayısı: veri.seriler().len(),
                    ekleme: false,
                });
            }
        }
        if seçenekler
            .ısı_haritası_düzeni
            .as_ref()
            .is_some_and(|düzen| !düzen.geçerli_mi())
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "IsıHaritasıDüzeni",
                açıklama: "hücre konumu, boyutu veya rengi geçersiz".to_string(),
            });
        }
        if seçenekler
            .dağılım_düzeni
            .as_ref()
            .is_some_and(|düzen| !düzen.geçerli_mi())
        {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "DağılımDüzeni",
                açıklama: "seri, nokta koordinatı veya nokta boyutu geçersiz".to_string(),
            });
        }
        let mut tam = seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&veri).ok())
            .unwrap_or(Aralık {
                en_az: 0.0,
                en_çok: 1.0,
            });
        if (seçenekler.çubuk_düzeni.is_some()
            || seçenekler.kutu_bıyık_düzeni.is_some()
            || seçenekler.mum_düzeni.is_some())
            && veri.uzunluk() > 1
        {
            tam = Aralık::yeni(tam.en_az - 0.5, tam.en_çok + 0.5)?;
        }
        let birincil_ölçek = seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == seçenekler.birincil_y_ölçeği);
        let mut tam_y = birincil_ölçek
            .and_then(|ölçek| ölçek.aralık)
            .or(seçenekler.y_aralığı)
            .unwrap_or_else(|| {
                let değerler = || {
                    veri.seriler()
                        .iter()
                        .zip(seçenekler.seriler.iter())
                        .filter(|(_, ayarlar)| ayarlar.ölçek == seçenekler.birincil_y_ölçeği)
                        .flat_map(|(seri, _)| seri.iter())
                };
                match birincil_ölçek.map(|ölçek| ölçek.dağılım) {
                    Some(YÖlçekDağılımı::Logaritmik { taban }) => logaritmik_otomatik_aralık(
                        değerler(),
                        taban,
                        birincil_ölçek.is_none_or(|ölçek| ölçek.log_tam_büyüklükler),
                    )
                    .unwrap_or_else(|| Aralık::otomatik(değerler())),
                    _ => birincil_ölçek
                        .and_then(|ölçek| ölçek.sayısal_aralık)
                        .and_then(|ayarlar| {
                            sonlu_sınırlar(değerler().flatten().copied()).and_then(
                                |(en_az, en_çok)| {
                                    Aralık::uplot_yapılandırılmış(en_az, en_çok, ayarlar).ok()
                                },
                            )
                        })
                        .unwrap_or_else(|| Aralık::otomatik(değerler())),
                }
            });
        if let Some(düzen) = birincil_ölçek.and_then(|ölçek| ölçek.güzel_ölçek) {
            let ham_aralık = sonlu_aralık(
                veri.seriler()
                    .iter()
                    .zip(seçenekler.seriler.iter())
                    .filter(|(_, ayarlar)| ayarlar.ölçek == seçenekler.birincil_y_ölçeği)
                    .flat_map(|(seri, _)| seri.iter().flatten().copied()),
            );
            let çizim_yüksekliği = seçenekler.yükseklik.saturating_sub(96).max(1) as f32;
            if let Some((aralık, _)) = ham_aralık.and_then(|aralık| {
                güzel_ölçek(aralık, çizim_yüksekliği, düzen.en_az_etiket_boşluğu)
            }) {
                tam_y = aralık;
            }
        }
        if let Some(düzen) = &seçenekler.kutu_bıyık_düzeni {
            let mut değerler = veri
                .seriler()
                .iter()
                .flat_map(|seri| seri.iter().copied())
                .collect::<Vec<_>>();
            değerler.extend(
                düzen
                    .ayrık_değerler
                    .iter()
                    .flat_map(|ayrıklar| ayrıklar.iter().copied().map(Some)),
            );
            tam_y = Aralık::otomatik(değerler.iter());
        }
        let etkileşim = EtkileşimDenetleyicisi::yeni(tam, tam_y, seçenekler.etkileşimler);
        Ok(Self {
            seçenekler,
            veri,
            etkileşim,
            odak_serisi: None,
        })
    }

    pub fn çiz(&self) -> Sahne {
        self.çiz_boyutta_aralıklarla(
            self.seçenekler.genişlik,
            self.seçenekler.yükseklik,
            self.etkileşim
                .yakınlaştırılmış()
                .then(|| self.etkileşim.görünür_x()),
            self.etkileşim.görünür_y(),
        )
    }

    pub fn görünür_x_aralığı(&self) -> Aralık {
        self.etkileşim.görünür_x()
    }

    pub fn boyut(&self) -> (u32, u32) {
        (self.seçenekler.genişlik, self.seçenekler.yükseklik)
    }

    pub fn dağılım_vuruşu_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<DağılımVuruşu> {
        let düzen = self.seçenekler.dağılım_düzeni.as_ref()?;
        if !düzen.vuruş_etkin || !x.is_finite() || !y.is_finite() {
            return None;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if !(sol..=sağ).contains(&x) || !(üst..=alt).contains(&y) {
            return None;
        }
        let x_aralığı = self.görünür_x_aralığı();
        let görünür_y = self.etkileşim.görünür_y();
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let mut sonuç = None::<(f32, f32, DağılımVuruşu)>;
        for (seri, seri_düzeni) in düzen.seriler.iter().enumerate() {
            let y_aralığı =
                self.görünür_ölçek_aralığı(&seri_düzeni.ölçek, x_aralığı, görünür_y);
            for (indeks, nokta) in seri_düzeni.noktalar.iter().enumerate() {
                let merkez = Nokta::yeni(
                    self.x_konumu(x_aralığı, nokta.x, sol, genişlik),
                    alt - self.y_konumu(&seri_düzeni.ölçek, y_aralığı, nokta.y, 0.0, yükseklik),
                );
                let yarıçap = nokta.boyut / 2.0;
                let dx = merkez.x - x;
                let dy = merkez.y - y;
                let uzaklık_kare = dx * dx + dy * dy;
                if uzaklık_kare > yarıçap * yarıçap {
                    continue;
                }
                let alan = nokta.boyut * nokta.boyut;
                let aday = DağılımVuruşu {
                    seri,
                    indeks,
                    merkez,
                    boyut: nokta.boyut,
                    x: nokta.x,
                    y: nokta.y,
                    değer: nokta.değer,
                    etiket: nokta.etiket.clone(),
                };
                if sonuç
                    .as_ref()
                    .is_none_or(|(önceki_alan, önceki_uzaklık, _)| {
                        alan < *önceki_alan
                            || ((alan - *önceki_alan).abs() <= f32::EPSILON
                                && uzaklık_kare <= *önceki_uzaklık)
                    })
                {
                    sonuç = Some((alan, uzaklık_kare, aday));
                }
            }
        }
        sonuç.map(|(_, _, vuruş)| vuruş)
    }

    /// uPlot `setData(data)` karşılığı olarak hizalı veriyi doğrular, uygular
    /// ve otomatik ölçeklerle etkileşim görünümünü tam aralığa sıfırlar.
    pub fn veriyi_ayarla(&mut self, veri: HizalıVeri) -> Result<(), UplotHatası> {
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        let yeni = Self::yeni(seçenekler, veri)?;
        *self = yeni;
        Ok(())
    }

    /// Y-serisi seçeneğini ve hizalı değerlerini tek, doğrulanmış işlemle ekler.
    /// İndeks yalnız Y serilerini sayar; uPlot'un X dahil `seriesIdx = 2`
    /// değeri burada `indeks = 1` olur.
    pub fn seri_ekle(
        &mut self,
        indeks: usize,
        seçenek: crate::SeriSeçenekleri,
        değerler: Vec<Option<f64>>,
    ) -> Result<(), UplotHatası> {
        let seri_sayısı = self.veri.seriler().len();
        if indeks > seri_sayısı {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: true,
            });
        }
        let veri = self.veri.seri_ekle(indeks, değerler)?;
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        seçenekler.seriler.insert(indeks, seçenek);
        let yeni = Self::yeni(seçenekler, veri)?;
        *self = yeni;
        Ok(())
    }

    /// Y-serisi seçeneğini ve hizalı değerlerini aynı anda siler.
    pub fn seri_sil(&mut self, indeks: usize) -> Result<(), UplotHatası> {
        let seri_sayısı = self.veri.seriler().len();
        if indeks >= seri_sayısı {
            return Err(UplotHatası::GeçersizSeriİndeksi {
                indeks,
                seri_sayısı,
                ekleme: false,
            });
        }
        let veri = self.veri.seri_sil(indeks)?;
        let mut seçenekler = self.seçenekler.clone();
        seçenekler.etkileşimler = self.etkileşim.ayarlar();
        seçenekler.seriler.remove(indeks);
        let yeni = Self::yeni(seçenekler, veri)?;
        *self = yeni;
        Ok(())
    }

    /// Bütün Y serilerinin uPlot `spanGaps` değerini birlikte değiştirir.
    pub fn boşlukları_birleştir_ayarla(&mut self, birleştir: bool) -> bool {
        let mut değişti = false;
        for seri in &mut self.seçenekler.seriler {
            if seri.boşlukları_birleştir != birleştir {
                seri.boşlukları_birleştir = birleştir;
                değişti = true;
            }
        }
        değişti
    }

    /// Başlık ve eksen payları çıkarıldıktan sonraki gerçek çizim alanını
    /// `(sol, sağ, üst, alt)` olarak döndürür. Yüzey adaptörleri sabit sayı
    /// çoğaltmak yerine bu çekirdek geometrisini kullanır.
    pub fn çizim_alanı_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
    ) -> (f32, f32, f32, f32) {
        let genişlik_px = if self.seçenekler.kompakt_yüzey {
            genişlik_px.max(2)
        } else {
            genişlik_px.max(160)
        } as f32;
        let yükseklik_px = if self.seçenekler.kompakt_yüzey {
            yükseklik_px.max(2)
        } else {
            yükseklik_px.max(120)
        } as f32;
        let gizli_eksen_payı = if self.seçenekler.kompakt_yüzey {
            0.0
        } else {
            8.0
        };
        if let Some(düzen) = self.seçenekler.çubuk_düzeni {
            return match düzen.yön {
                crate::ÇubukYönü::Dikey => (64.0, genişlik_px - 24.0, 48.0, yükseklik_px - 72.0),
                crate::ÇubukYönü::Yatay => {
                    (150.0, genişlik_px - 32.0, 48.0, yükseklik_px - 48.0)
                }
            };
        }
        if self.seçenekler.kutu_bıyık_düzeni.is_some() {
            return (64.0, genişlik_px - 24.0, 48.0, yükseklik_px - 130.0);
        }
        if self.seçenekler.mum_düzeni.is_some() {
            return (72.0, genişlik_px - 72.0, 48.0, yükseklik_px - 48.0);
        }
        if self.seçenekler.x_dikey {
            let sol_pay = if !self.seçenekler.x_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.x_eksen_karşıda {
                24.0
            } else {
                64.0
            };
            let sağ_pay = if !self.seçenekler.x_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.x_eksen_karşıda {
                64.0
            } else {
                24.0
            };
            let üst_pay = if !self.seçenekler.birincil_y_eksen_görünür {
                if self.seçenekler.başlık.is_empty() {
                    gizli_eksen_payı
                } else {
                    48.0
                }
            } else if self.seçenekler.birincil_y_karşıda {
                48.0
            } else {
                68.0
            };
            let alt_pay = if !self.seçenekler.birincil_y_eksen_görünür {
                gizli_eksen_payı
            } else if self.seçenekler.birincil_y_karşıda {
                48.0
            } else {
                24.0
            };
            return (
                sol_pay,
                genişlik_px - sağ_pay,
                üst_pay,
                yükseklik_px - alt_pay,
            );
        }
        let sağ_eksen_sayısı = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .filter(|ölçek| ölçek.anahtar != self.seçenekler.birincil_y_ölçeği && ölçek.sağda)
            .count();
        let sol_eksen_sayısı = self
            .seçenekler
            .y_ölçekleri
            .iter()
            .filter(|ölçek| {
                ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                    && ölçek.eksen_görünür
                    && !ölçek.sağda
            })
            .count();
        let mut sağ_pay: f32 = if !self.seçenekler.birincil_y_eksen_görünür && sağ_eksen_sayısı == 0
        {
            gizli_eksen_payı
        } else if self.seçenekler.birincil_y_sağda {
            72.0 + sağ_eksen_sayısı as f32 * 56.0
        } else if sağ_eksen_sayısı > 0 {
            24.0 + sağ_eksen_sayısı as f32 * 56.0
        } else {
            24.0
        };
        if self.seçenekler.otomatik_x_sağ_pay {
            let x_artımı = uygun_artım(self.görünür_x_aralığı(), genişlik_px, 50.0);
            let son_etiket = eksen_değerini_yaz(
                self.görünür_x_aralığı().en_çok * self.seçenekler.x_eksen_değer_çarpanı,
                x_artımı * self.seçenekler.x_eksen_değer_çarpanı,
            );
            sağ_pay = sağ_pay.max(8.0 + son_etiket.chars().count() as f32 * 4.0);
        }
        let mut sol_pay: f32 = if !self.seçenekler.birincil_y_eksen_görünür && sol_eksen_sayısı == 0
        {
            gizli_eksen_payı
        } else if self.seçenekler.birincil_y_sağda {
            24.0 + sol_eksen_sayısı as f32 * 56.0
        } else {
            64.0 + sol_eksen_sayısı as f32 * 56.0
        };
        if self.seçenekler.otomatik_y_eksen_genişliği && !self.seçenekler.birincil_y_sağda {
            let aralık = self.görünür_y_aralığı();
            let artım = uygun_artım(aralık, yükseklik_px, 30.0);
            let ölçek = self.ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği);
            let birim = ölçek.map_or("", |ölçek| ölçek.birim.as_str());
            let dağılım = ölçek.map(|ölçek| ölçek.dağılım);
            let biçim = ölçek.map_or(YÖlçekEtiketBiçimi::Otomatik, |ölçek| ölçek.etiket_biçimi);
            let çarpan = ölçek.map_or(1.0, |ölçek| ölçek.eksen_değer_çarpanı);
            let en_uzun = self
                .y_eksen_bölmeleri(&self.seçenekler.birincil_y_ölçeği, aralık, yükseklik_px)
                .into_iter()
                .map(|değer| {
                    ölçek_eksen_değerini_yaz(değer * çarpan, artım, birim, dağılım, biçim)
                        .chars()
                        .count()
                })
                .max()
                .unwrap_or(1);
            sol_pay = sol_pay.max(24.0 + en_uzun as f32 * 7.0);
        }
        let alt_pay = if !self.seçenekler.x_eksen_görünür {
            gizli_eksen_payı
        } else if self.seçenekler.x_eksen_karşıda {
            24.0
        } else if self.seçenekler.x_eksen_etiketi.is_empty() {
            48.0
        } else {
            68.0
        };
        let üst_pay = if !self.seçenekler.x_eksen_görünür {
            if self.seçenekler.başlık.is_empty() {
                gizli_eksen_payı
            } else {
                48.0
            }
        } else if self.seçenekler.x_eksen_karşıda {
            if self.seçenekler.x_eksen_etiketi.is_empty() {
                68.0
            } else {
                88.0
            }
        } else {
            48.0
        };
        (
            sol_pay,
            genişlik_px - sağ_pay,
            üst_pay,
            yükseklik_px - alt_pay,
        )
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

    pub fn seri_seçenekleri(&self) -> &[crate::SeriSeçenekleri] {
        &self.seçenekler.seriler
    }

    pub fn eksen_göstergeleri_etkin(&self) -> bool {
        self.seçenekler.eksen_göstergeleri
    }

    pub fn çubuk_grafiği(&self) -> bool {
        self.seçenekler.çubuk_düzeni.is_some()
    }

    pub fn kutu_bıyık_grafiği(&self) -> bool {
        self.seçenekler.kutu_bıyık_düzeni.is_some()
    }

    pub fn mum_grafiği(&self) -> bool {
        self.seçenekler.mum_düzeni.is_some()
    }

    pub fn kutu_bıyık_vuruşu(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<(usize, Nokta, f32, f32, [f64; 5])> {
        if (!self.kutu_bıyık_grafiği() && !self.mum_grafiği()) || !x.is_finite() || !y.is_finite()
        {
            return None;
        }
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        if x < sol || x > sağ || y < üst || y > alt {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let açıklık = aralık.en_çok - aralık.en_az;
        if açıklık <= f64::EPSILON {
            return None;
        }
        let sütun_genişliği = (sağ - sol) / açıklık as f32;
        let hedef = aralık.en_az + f64::from((x - sol) / (sağ - sol)) * açıklık;
        let (indeks, x_değeri) = self
            .veri
            .x()
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, değer)| (*değer - hedef).abs() <= 0.5)
            .min_by(|(_, sol), (_, sağ)| (*sol - hedef).abs().total_cmp(&(*sağ - hedef).abs()))?;
        let değer = |seri: usize| {
            self.veri
                .seriler()
                .get(seri)
                .and_then(|değerler| değerler.get(indeks))
                .copied()
                .flatten()
        };
        let değerler = [değer(0)?, değer(1)?, değer(2)?, değer(3)?, değer(4)?];
        let merkez = sol + ((x_değeri - aralık.en_az) / açıklık) as f32 * (sağ - sol);
        let sütun_sol = (merkez - sütun_genişliği / 2.0).clamp(sol, sağ);
        let sütun_sağ = (merkez + sütun_genişliği / 2.0).clamp(sol, sağ);
        Some((
            indeks,
            Nokta::yeni(sütun_sol, üst),
            sütun_sağ - sütun_sol,
            alt - üst,
            değerler,
        ))
    }

    /// Çizim koordinatındaki noktayı kaynak çubuk dikdörtgenlerinden biriyle
    /// eşleştirir. Yüzey adaptörleri yerleşim veya hit-test kodu tekrarlamaz.
    pub fn çubuk_vuruşu(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<(usize, usize, Nokta, f32, f32, f64)> {
        if !self.çubuk_grafiği() || !x.is_finite() || !y.is_finite() {
            return None;
        }
        if self.veri.seriler().is_empty() {
            return None;
        }
        let görünür_x = self.görünür_x_aralığı();
        let çizilenler = self
            .veri
            .x()
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x_değeri)| {
                *x_değeri + 0.45 >= görünür_x.en_az && *x_değeri - 0.45 <= görünür_x.en_çok
            })
            .flat_map(|(indeks, _)| {
                self.veri
                    .seriler()
                    .iter()
                    .enumerate()
                    .filter_map(move |(seri, değerler)| {
                        değerler
                            .get(indeks)
                            .copied()
                            .flatten()
                            .map(|değer| (seri, indeks, değer))
                    })
            })
            .collect::<Vec<_>>();
        let sahne = self.çiz_görünür_boyutta(genişlik_px, yükseklik_px);
        let mut sıra = 0_usize;
        for komut in sahne.komutlar() {
            let Komut::Dikdörtgen {
                konum,
                genişlik,
                yükseklik,
                dolgu,
                kalınlık,
                ..
            } = komut
            else {
                continue;
            };
            if *kalınlık != 0.0
                || !self.seçenekler.seriler.iter().any(|seri| {
                    seri.dolgu.as_deref() == Some(dolgu.as_str()) || seri.renk == *dolgu
                })
            {
                continue;
            }
            let (seri, indeks, değer) = çizilenler.get(sıra).copied()?;
            sıra = sıra.saturating_add(1);
            if x >= konum.x
                && x <= konum.x + *genişlik
                && y >= konum.y
                && y <= konum.y + *yükseklik
            {
                return Some((seri, indeks, *konum, *genişlik, *yükseklik, değer));
            }
        }
        None
    }

    /// Yüzey imlecinin normalize edilmiş konumunu kartın çekirdek ayarına göre
    /// uyarlar. Izgara kapalıysa oranlar kesintisiz biçimde geri döner.
    pub fn imleç_oranlarını_uyarla(
        &self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_genişliği: f64,
        çizim_yüksekliği: f64,
    ) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite()
            || !dikey_oran.is_finite()
            || !çizim_genişliği.is_finite()
            || !çizim_yüksekliği.is_finite()
            || çizim_genişliği <= 0.0
            || çizim_yüksekliği <= 0.0
        {
            return None;
        }
        let yatay = yatay_oran.clamp(0.0, 1.0);
        let dikey = dikey_oran.clamp(0.0, 1.0);
        let Some(adım) = self.seçenekler.imleç_ızgara_adımı.map(f64::from) else {
            return Some((yatay, dikey));
        };
        let x = ((yatay * çizim_genişliği / adım).round() * adım) / çizim_genişliği;
        let y = ((dikey * çizim_yüksekliği / adım).round() * adım) / çizim_yüksekliği;
        Some((x.clamp(0.0, 1.0), y.clamp(0.0, 1.0)))
    }

    pub fn tekerlek_etkileşimi_ayarla(&mut self, etkin: bool) {
        self.etkileşim.tekerlek_etkileşimi_ayarla(etkin);
    }

    /// ArcSinh ölçeğinin doğrusal merkez eşiğini çalışma anında değiştirir.
    /// Platform yüzeyleri yalnız bu çekirdek API'sini çağırır.
    pub fn y_arcsinh_eşiği_ayarla(&mut self, anahtar: &str, eşik: f64) -> bool {
        if !eşik.is_finite() || eşik <= 0.0 {
            return false;
        }
        let Some(ölçek) = self
            .seçenekler
            .y_ölçekleri
            .iter_mut()
            .find(|ölçek| ölçek.anahtar == anahtar)
        else {
            return false;
        };
        if !matches!(ölçek.dağılım, YÖlçekDağılımı::ArcSinh { .. }) {
            return false;
        }
        ölçek.dağılım = YÖlçekDağılımı::ArcSinh { eşik };
        true
    }

    pub fn tekerlek(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        delta: f64,
        hassas: bool,
    ) -> Result<bool, UplotHatası> {
        let görünür_y = self.görünür_y_aralığı();
        let (x_oranı, y_oranı) =
            self.fiziksel_oranları_mantıksala(yatay_odak_oranı, dikey_odak_oranı);
        self.etkileşim
            .tekerlek(x_oranı, y_oranı, görünür_y, delta, hassas)
    }

    pub fn seçim_yakınlaştır(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        let (başlangıç_oranı, bitiş_oranı) = if self.seçenekler.x_ters_yön {
            (1.0 - başlangıç_oranı, 1.0 - bitiş_oranı)
        } else {
            (başlangıç_oranı, bitiş_oranı)
        };
        self.etkileşim
            .seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
    }

    /// Seçim bırakma davranışını kart ayarlarına göre çekirdekte çözümler.
    ///
    /// `açıklama_tuşu` açıkken `ctrl_açıklama` etkin bir kart normal seçim
    /// yakınlaştırmasını uygulamaz; yüzeyin metin istemesi için ayrı sonuç döner.
    pub fn seçimi_bitir(
        &mut self,
        başlangıç_oranı: f64,
        bitiş_oranı: f64,
        açıklama_tuşu: bool,
    ) -> Result<SeçimEylemi, UplotHatası> {
        let ayarlar = self.etkileşim.ayarlar();
        if !ayarlar.seçim_yakınlaştır {
            return Ok(SeçimEylemi::Değişmedi);
        }
        if açıklama_tuşu && ayarlar.ctrl_açıklama {
            if !başlangıç_oranı.is_finite() || !bitiş_oranı.is_finite() {
                return Err(UplotHatası::GeçersizAralık {
                    en_az: başlangıç_oranı,
                    en_çok: bitiş_oranı,
                });
            }
            return Ok(SeçimEylemi::Açıklamaİstendi);
        }
        self.seçim_yakınlaştır(başlangıç_oranı, bitiş_oranı)
            .map(|değişti| {
                if değişti {
                    SeçimEylemi::Yakınlaştırıldı
                } else {
                    SeçimEylemi::Değişmedi
                }
            })
    }

    pub fn tam_görünüm(&mut self) -> bool {
        self.etkileşim.tam_görünüm()
    }

    pub fn önceki_görünüm(&mut self) -> bool {
        self.etkileşim.geri()
    }

    pub fn taşımayı_başlat(&mut self) -> bool {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.taşımayı_başlat(görünür_y)
    }

    pub fn taşı(
        &mut self,
        yatay_fark_oranı: f64,
        dikey_fark_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        let (x_farkı, y_farkı) =
            self.fiziksel_farkları_mantıksala(yatay_fark_oranı, dikey_fark_oranı);
        self.etkileşim.taşı(x_farkı, y_farkı)
    }

    pub fn taşımayı_bitir(&mut self) {
        self.etkileşim.taşımayı_bitir();
    }

    pub fn dokunmayı_başlat(&mut self) -> bool {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.dokunmayı_başlat(görünür_y)
    }

    pub fn dokunma_yakınlaştır(
        &mut self,
        yatay_odak_oranı: f64,
        dikey_odak_oranı: f64,
        çarpan: f64,
    ) -> Result<bool, UplotHatası> {
        let (x_oranı, y_oranı) =
            self.fiziksel_oranları_mantıksala(yatay_odak_oranı, dikey_odak_oranı);
        self.etkileşim.dokunma_yakınlaştır(x_oranı, y_oranı, çarpan)
    }

    /// Yüzeydeki fiziksel oranları uPlot ölçek yönü ve yönelimine göre
    /// çekirdeğin mantıksal X/Y oranlarına dönüştürür.
    pub fn fiziksel_oranları_mantıksala(&self, yatay: f64, dikey: f64) -> (f64, f64) {
        let yatay = yatay.clamp(0.0, 1.0);
        let dikey = dikey.clamp(0.0, 1.0);
        let y_ters = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .is_some_and(|ölçek| ölçek.ters_yön);
        if self.seçenekler.x_dikey {
            let x = if self.seçenekler.x_ters_yön {
                dikey
            } else {
                1.0 - dikey
            };
            let y = if y_ters { yatay } else { 1.0 - yatay };
            (x, y)
        } else {
            let x = if self.seçenekler.x_ters_yön {
                1.0 - yatay
            } else {
                yatay
            };
            let y = if y_ters { 1.0 - dikey } else { dikey };
            (x, y)
        }
    }

    fn fiziksel_farkları_mantıksala(&self, yatay: f64, dikey: f64) -> (f64, f64) {
        let y_ters = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .is_some_and(|ölçek| ölçek.ters_yön);
        if self.seçenekler.x_dikey {
            let x = if self.seçenekler.x_ters_yön {
                -dikey
            } else {
                dikey
            };
            let y = if y_ters { -yatay } else { yatay };
            (x, y)
        } else {
            let x = if self.seçenekler.x_ters_yön {
                -yatay
            } else {
                yatay
            };
            let y = if y_ters { -dikey } else { dikey };
            (x, y)
        }
    }

    pub fn x_dikey_mi(&self) -> bool {
        self.seçenekler.x_dikey
    }

    pub fn dokunmayı_bitir(&mut self) {
        self.etkileşim.dokunmayı_bitir();
    }

    /// Geçerli X görünümündeki veriden hesaplanan Y aralığını döndürür.
    pub fn görünür_y_aralığı(&self) -> Aralık {
        self.etkileşim.görünür_y().unwrap_or_else(|| {
            if self.kutu_bıyık_grafiği() {
                self.kutu_bıyık_y_aralığı()
            } else {
                self.y_aralığı(self.görünür_x_aralığı())
            }
        })
    }

    pub fn seri_görünür_y_aralığı(&self, seri_indeksi: usize) -> Option<Aralık> {
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        Some(self.görünür_ölçek_aralığı(
            &seri.ölçek,
            self.görünür_x_aralığı(),
            self.etkileşim.görünür_y(),
        ))
    }

    pub fn seri_y_konum_oranı(&self, seri_indeksi: usize, değer: f64) -> Option<f64> {
        if !değer.is_finite() {
            return None;
        }
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        let aralık = self.seri_görünür_y_aralığı(seri_indeksi)?;
        Some(f64::from(self.y_konumu(
            &seri.ölçek,
            aralık,
            değer,
            0.0,
            1.0,
        )))
    }

    pub fn x_konum_oranı(&self, değer: f64) -> Option<f64> {
        if !değer.is_finite() {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        Some(f64::from(self.x_konumu(aralık, değer, 0.0, 1.0)))
    }

    /// Geçerli görünümde, normalize edilmiş yatay konuma en yakın seri noktasını bulur.
    pub fn en_yakın_nokta(&self, yatay_oran: f64, seri_indeksi: usize) -> Option<(f64, f64)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let seri = self.veri.seriler().get(seri_indeksi)?;
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        self.veri
            .x()
            .iter()
            .copied()
            .zip(seri.iter().copied())
            .filter_map(|(x, y)| y.map(|y| (x, y)))
            .filter(|(x, _)| *x >= aralık.en_az && *x <= aralık.en_çok)
            .min_by(|(x_a, _), (x_b, _)| (x_a - hedef).abs().total_cmp(&(x_b - hedef).abs()))
    }

    /// En yakın ortak X indeksini ve o indeksteki tüm seri değerlerini döndürür.
    /// uPlot'un hizalı cursor/legend davranışının çekirdek karşılığıdır.
    pub fn en_yakın_noktalar(&self, yatay_oran: f64) -> Option<(f64, Vec<Option<f64>>)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let hedef = self.x_değeri_orandan(aralık, yatay_oran.clamp(0.0, 1.0));
        let (indeks, x) = self
            .veri
            .x()
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, x)| *x >= aralık.en_az && *x <= aralık.en_çok)
            .min_by(|(_, x_a), (_, x_b)| (x_a - hedef).abs().total_cmp(&(x_b - hedef).abs()))?;
        let değerler = self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .map(|(seri, seçenek)| {
                seri.get(indeks)
                    .copied()
                    .flatten()
                    .map(|değer| değer * seçenek.gösterim_değer_çarpanı)
            })
            .collect();
        Some((x, değerler))
    }

    /// İmleç noktasının rengini seri gradyanının geçerli ölçek duraklarına göre çözer.
    pub fn seri_imleç_rengi(
        &self,
        seri_indeksi: usize,
        x_değeri: f64,
        y_değeri: f64,
    ) -> Option<String> {
        let seri = self.seçenekler.seriler.get(seri_indeksi)?;
        let gradyan = seri.çizgi_gradyanı.as_ref()?;
        let x_aralığı = self.görünür_x_aralığı();
        let y_aralığı =
            self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, self.etkileşim.görünür_y());
        let duraklar =
            self.gradyan_değerlerini_çöz(gradyan, &seri.ölçek, x_aralığı, y_aralığı)?;
        let değer = match gradyan.eksen {
            GradyanEkseni::X => x_değeri,
            GradyanEkseni::Y => y_değeri,
        };
        duraklar
            .iter()
            .rev()
            .find(|(durak, _)| değer >= *durak)
            .or_else(|| duraklar.first())
            .map(|(_, renk)| renk.clone())
    }

    /// `cursor.focus` eşdeğeri: en yakın X örneğindeki Y mesafesine göre
    /// odaklanan seriyi çekirdekte seçer. `true`, sahnenin yeniden çizilmesi
    /// gerektiğini bildirir.
    pub fn imleç_odağını_güncelle(
        &mut self,
        yatay_oran: f64,
        dikey_oran: f64,
        çizim_boyutu: f64,
    ) -> bool {
        let Some(düzen) = self.seçenekler.odak else {
            return false;
        };
        if düzen.yakınlık < 0.0 || !çizim_boyutu.is_finite() || çizim_boyutu <= 0.0 {
            return self.odağı_ayarla(None);
        }
        let x_oranı = if self.seçenekler.x_dikey {
            1.0 - dikey_oran
        } else {
            yatay_oran
        };
        let Some((_, değerler)) = self.en_yakın_noktalar(x_oranı) else {
            return self.odağı_ayarla(None);
        };
        let x_aralığı = self.görünür_x_aralığı();
        let fare_y = if self.seçenekler.x_dikey {
            yatay_oran.clamp(0.0, 1.0) * çizim_boyutu
        } else {
            dikey_oran.clamp(0.0, 1.0) * çizim_boyutu
        };
        let görünür_y = self.görünür_y_aralığı();
        let fare_değeri =
            görünür_y.en_çok - dikey_oran.clamp(0.0, 1.0) * (görünür_y.en_çok - görünür_y.en_az);
        let mut en_yakın = None;
        let mut en_kısa = f64::INFINITY;
        for (indeks, değer) in değerler.into_iter().enumerate() {
            let Some(değer) = değer else { continue };
            let Some(seri) = self.seçenekler.seriler.get(indeks) else {
                continue;
            };
            let aralık = self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, None);
            if düzen.yön_eğilimi != 0 {
                let aynı_işaret = değer.is_sign_negative() == fare_değeri.is_sign_negative();
                let uygun = if fare_değeri.is_sign_negative() {
                    if düzen.yön_eğilimi == 1 {
                        değer <= fare_değeri
                    } else {
                        değer >= fare_değeri
                    }
                } else if düzen.yön_eğilimi == 1 {
                    değer >= fare_değeri
                } else {
                    değer <= fare_değeri
                };
                if !aynı_işaret || !uygun {
                    continue;
                }
            }
            let ölçek_konumu =
                f64::from(self.y_konumu(&seri.ölçek, aralık, değer, 0.0, çizim_boyutu as f32));
            let konum = if self.seçenekler.x_dikey {
                ölçek_konumu
            } else {
                çizim_boyutu - ölçek_konumu
            };
            let mesafe = (konum - fare_y).abs();
            if mesafe < en_kısa {
                en_kısa = mesafe;
                en_yakın = Some(indeks);
            }
        }
        self.odağı_ayarla(
            (en_kısa <= f64::from(düzen.yakınlık))
                .then_some(en_yakın)
                .flatten(),
        )
    }

    pub fn imleç_odağını_temizle(&mut self) -> bool {
        self.odağı_ayarla(None)
    }

    fn odağı_ayarla(&mut self, seri: Option<usize>) -> bool {
        if self.odak_serisi == seri {
            return false;
        }
        self.odak_serisi = seri;
        true
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
        self.çiz_boyutta_aralıklarla(
            genişlik_px,
            yükseklik_px,
            görünür,
            self.etkileşim.görünür_y(),
        )
    }

    /// Resize demosundaki gibi hedef yüzey boyutuna göre yeniden yerleşim yapar.
    pub fn çiz_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
    ) -> Sahne {
        self.çiz_boyutta_aralıklarla(genişlik_px, yükseklik_px, görünür_x, None)
    }

    fn çiz_boyutta_aralıklarla(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) -> Sahne {
        let genişlik_px = if self.seçenekler.kompakt_yüzey {
            genişlik_px.max(2)
        } else {
            genişlik_px.max(160)
        };
        let yükseklik_px = if self.seçenekler.kompakt_yüzey {
            yükseklik_px.max(2)
        } else {
            yükseklik_px.max(120)
        };
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        sahne.ekle(Komut::ArkaPlan {
            renk: self.seçenekler.arka_plan_rengi.clone(),
        });

        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;

        if let Some((üst_renk, alt_renk)) = self
            .seçenekler
            .çizim_kancaları
            .as_ref()
            .and_then(|düzen| düzen.gradyan_durakları.as_ref())
        {
            const ŞERİT_SAYISI: usize = 32;
            let şerit_yüksekliği = yükseklik / ŞERİT_SAYISI as f32;
            for şerit in 0..ŞERİT_SAYISI {
                let oran = şerit as f32 / ŞERİT_SAYISI.saturating_sub(1) as f32;
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(sol, üst + şerit as f32 * şerit_yüksekliği),
                    genişlik,
                    yükseklik: şerit_yüksekliği + 1.0,
                    dolgu: renkler_arası(üst_renk, alt_renk, oran),
                    çizgi: "#00000000".to_string(),
                    kalınlık: 0.0,
                });
            }
        }

        if !self.seçenekler.başlık.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(genişlik_px as f32 / 2.0, 26.0),
                içerik: self.seçenekler.başlık.clone(),
                renk: self.seçenekler.başlık_rengi.clone(),
                boyut: 18.0,
                hiza: MetinHizası::Orta,
            });
        }

        // uPlot'un ölçek `range()` sonucu `[null, null]` olan boş veri
        // yüzeyi yalnız başlığı taşır. Özel X/Y aralığı tanımlanan boş
        // yüzeyler ise normal eksen ve ızgara çiziminden geçer.
        if self.veri.x().is_empty()
            && self.seçenekler.x_aralığı.is_none()
            && self.seçenekler.y_aralığı.is_none()
        {
            return sahne;
        }

        if let Some(düzen) = self.seçenekler.çubuk_düzeni {
            self.çubukları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }
        if let Some(düzen) = &self.seçenekler.kutu_bıyık_düzeni {
            self.kutu_bıyıkları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }
        if let Some(düzen) = &self.seçenekler.mum_düzeni {
            self.mumları_çiz(
                &mut sahne,
                genişlik_px,
                yükseklik_px,
                düzen,
                görünür_x,
                görünür_y,
            );
            return sahne;
        }

        let tam_x_aralığı = self
            .seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&self.veri).ok())
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
        let (y_aralığı, güzel_y_artımı) = görünür_y.map_or_else(
            || {
                self.güzel_ölçek_aralığı(
                    &self.seçenekler.birincil_y_ölçeği,
                    x_aralığı,
                    yükseklik,
                )
                .map_or_else(
                    || (self.y_aralığı(x_aralığı), None),
                    |(aralık, artım)| (aralık, Some(artım)),
                )
            },
            |aralık| (aralık, None),
        );
        let birincil_ölçek = self.ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği);
        let birincil_birim = birincil_ölçek.map_or("", |ölçek| ölçek.birim.as_str());
        let birincil_dağılım = birincil_ölçek.map(|ölçek| ölçek.dağılım);
        let birincil_biçim =
            birincil_ölçek.map_or(YÖlçekEtiketBiçimi::Otomatik, |ölçek| ölçek.etiket_biçimi);
        let birincil_çarpan = birincil_ölçek.map_or(1.0, |ölçek| ölçek.eksen_değer_çarpanı);

        let eksen_komutları_başlangıcı = sahne.komutlar().len();
        let y_boyutu = if self.seçenekler.x_dikey {
            genişlik
        } else {
            yükseklik
        };
        let y_artımı = güzel_y_artımı.unwrap_or_else(|| uygun_artım(y_aralığı, y_boyutu, 30.0));
        let y_bölmeleri = self
            .seçenekler
            .birincil_y_sabit_bölmeler
            .clone()
            .unwrap_or_else(|| {
                güzel_y_artımı.map_or_else(
                    || {
                        self.y_eksen_bölmeleri(
                            &self.seçenekler.birincil_y_ölçeği,
                            y_aralığı,
                            y_boyutu,
                        )
                    },
                    |artım| eksen_bölmeleri_artımla(y_aralığı, artım),
                )
            })
            .into_iter()
            .filter(|değer| *değer >= y_aralığı.en_az && *değer <= y_aralığı.en_çok)
            .collect::<Vec<_>>();
        for y_değeri in y_bölmeleri {
            if self.seçenekler.x_dikey {
                let x = piksele_hizala(
                    sol + self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        y_aralığı,
                        y_değeri,
                        0.0,
                        genişlik,
                    ),
                    self.seçenekler.piksel_hizası,
                );
                if self.seçenekler.birincil_y_ızgara_görünür {
                    self.birincil_y_ızgara_çizgisini_ekle(
                        &mut sahne,
                        Nokta::yeni(x, üst),
                        Nokta::yeni(x, alt),
                    );
                }
                if self.seçenekler.birincil_y_eksen_görünür
                    && log_etiketi_göster(
                        y_değeri,
                        y_aralığı,
                        genişlik,
                        birincil_dağılım,
                        birincil_biçim,
                    )
                {
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(
                            x,
                            if self.seçenekler.birincil_y_karşıda {
                                alt + 20.0
                            } else {
                                üst - 8.0
                            },
                        ),
                        içerik: ölçek_eksen_değerini_yaz(
                            y_değeri * birincil_çarpan,
                            y_artımı,
                            birincil_birim,
                            birincil_dağılım,
                            birincil_biçim,
                        ),
                        renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                continue;
            }
            let y = piksele_hizala(
                üst + yükseklik
                    - self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        y_aralığı,
                        y_değeri,
                        0.0,
                        yükseklik,
                    ),
                self.seçenekler.piksel_hizası,
            );
            if self.seçenekler.birincil_y_ızgara_görünür {
                self.birincil_y_ızgara_çizgisini_ekle(
                    &mut sahne,
                    Nokta::yeni(sol, y),
                    Nokta::yeni(sağ, y),
                );
            }
            if self.seçenekler.birincil_y_eksen_görünür
                && log_etiketi_göster(
                    y_değeri,
                    y_aralığı,
                    yükseklik,
                    birincil_dağılım,
                    birincil_biçim,
                )
            {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(
                        if self.seçenekler.birincil_y_karşıda {
                            sağ + 8.0
                        } else {
                            sol - 8.0
                        },
                        y + 4.0,
                    ),
                    içerik: ölçek_eksen_değerini_yaz(
                        y_değeri * birincil_çarpan,
                        y_artımı,
                        birincil_birim,
                        birincil_dağılım,
                        birincil_biçim,
                    ),
                    renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                    boyut: 11.0,
                    hiza: if self.seçenekler.birincil_y_karşıda {
                        MetinHizası::Başlangıç
                    } else {
                        MetinHizası::Bitiş
                    },
                });
            }
        }

        if !self.seçenekler.y_eksen_etiketi.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(
                    if self.seçenekler.x_dikey {
                        (sol + sağ) / 2.0
                    } else if self.seçenekler.birincil_y_karşıda {
                        sağ
                    } else {
                        sol
                    },
                    if self.seçenekler.x_dikey && self.seçenekler.birincil_y_karşıda {
                        alt + 40.0
                    } else {
                        üst - 12.0
                    },
                ),
                içerik: self.seçenekler.y_eksen_etiketi.clone(),
                renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                boyut: 12.0,
                hiza: if self.seçenekler.x_dikey {
                    MetinHizası::Orta
                } else if self.seçenekler.birincil_y_karşıda {
                    MetinHizası::Bitiş
                } else {
                    MetinHizası::Başlangıç
                },
            });
        }

        let mut sol_ikincil = 0_usize;
        let mut sağ_ikincil = 0_usize;
        for ölçek in self.seçenekler.y_ölçekleri.iter().filter(|ölçek| {
            ölçek.anahtar != self.seçenekler.birincil_y_ölçeği
                && (ölçek.sağda || ölçek.eksen_görünür)
        }) {
            let eksen_x = if ölçek.sağda {
                let x = sağ + 8.0 + sağ_ikincil as f32 * 56.0;
                sağ_ikincil += 1;
                x
            } else {
                let x = sol - 8.0 - sol_ikincil as f32 * 56.0;
                sol_ikincil += 1;
                x
            };
            let aralık = self.görünür_ölçek_aralığı(&ölçek.anahtar, x_aralığı, görünür_y);
            let artım = uygun_artım(aralık, yükseklik, 30.0);
            for değer in self.y_eksen_bölmeleri(&ölçek.anahtar, aralık, yükseklik) {
                let y = piksele_hizala(
                    alt - self.y_konumu(&ölçek.anahtar, aralık, değer, 0.0, yükseklik),
                    self.seçenekler.piksel_hizası,
                );
                if ölçek.ızgara {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                if log_etiketi_göster(
                    değer,
                    aralık,
                    yükseklik,
                    Some(ölçek.dağılım),
                    ölçek.etiket_biçimi,
                ) {
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(eksen_x, y + 4.0),
                        içerik: ölçek_eksen_değerini_yaz(
                            değer * ölçek.eksen_değer_çarpanı,
                            artım,
                            &ölçek.birim,
                            Some(ölçek.dağılım),
                            ölçek.etiket_biçimi,
                        ),
                        renk: ölçek.eksen_rengi.clone(),
                        boyut: 11.0,
                        hiza: if ölçek.sağda {
                            MetinHizası::Başlangıç
                        } else {
                            MetinHizası::Bitiş
                        },
                    });
                }
            }
            if !ölçek.eksen_etiketi.is_empty() {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(if ölçek.sağda { sağ } else { sol }, üst - 12.0),
                    içerik: ölçek.eksen_etiketi.clone(),
                    renk: ölçek.eksen_rengi.clone(),
                    boyut: 12.0,
                    hiza: if ölçek.sağda {
                        MetinHizası::Bitiş
                    } else {
                        MetinHizası::Başlangıç
                    },
                });
            }
        }

        let x_boyutu = if self.seçenekler.x_dikey {
            yükseklik
        } else {
            genişlik
        };
        let x_etiket_boşluğu = self.seçenekler.x_eksen_asgari_etiket_boşluğu;
        let (x_bölmeleri, x_artımı) = match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban } => (
                logaritmik_bölmeler(x_aralığı, taban),
                uygun_artım(x_aralığı, x_boyutu, x_etiket_boşluğu),
            ),
            XÖlçekDağılımı::Doğrusal if self.seçenekler.x_zaman => zaman_bölmeleri(
                x_aralığı,
                x_boyutu,
                x_etiket_boşluğu,
                self.seçenekler.x_zaman_milisaniye,
            ),
            XÖlçekDağılımı::Doğrusal => (
                eksen_bölmeleri(x_aralığı, x_boyutu, x_etiket_boşluğu),
                uygun_artım(x_aralığı, x_boyutu, x_etiket_boşluğu),
            ),
        };
        let mut önceki_x_yılı = None;
        for x_değeri in x_bölmeleri {
            let (etiket_konumu, etiket_hizası) = if self.seçenekler.x_dikey {
                let y = piksele_hizala(
                    alt - self.x_konumu(x_aralığı, x_değeri, 0.0, yükseklik),
                    self.seçenekler.piksel_hizası,
                );
                if self.seçenekler.x_ızgara_görünür {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                (
                    Nokta::yeni(
                        if self.seçenekler.x_eksen_karşıda {
                            sağ + 8.0
                        } else {
                            sol - 8.0
                        },
                        y + 4.0,
                    ),
                    if self.seçenekler.x_eksen_karşıda {
                        MetinHizası::Başlangıç
                    } else {
                        MetinHizası::Bitiş
                    },
                )
            } else {
                let x = piksele_hizala(
                    self.x_konumu(x_aralığı, x_değeri, sol, genişlik),
                    self.seçenekler.piksel_hizası,
                );
                if self.seçenekler.x_ızgara_görünür {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(x, üst),
                        bitiş: Nokta::yeni(x, alt),
                        renk: self.seçenekler.ızgara_rengi.clone(),
                        kalınlık: 1.0,
                    });
                }
                (
                    Nokta::yeni(
                        x,
                        if self.seçenekler.x_eksen_karşıda {
                            üst - 8.0
                        } else {
                            alt + 20.0
                        },
                    ),
                    MetinHizası::Orta,
                )
            };
            if self.seçenekler.x_eksen_görünür {
                sahne.ekle(Komut::Metin {
                    konum: etiket_konumu,
                    içerik: if self.seçenekler.x_zaman {
                        let birim = if self.seçenekler.x_zaman_milisaniye {
                            1_000.0
                        } else {
                            1.0
                        };
                        crate::zaman::yerel_eksen_etiketi(
                            x_değeri / birim,
                            x_artımı / birim,
                            &self.seçenekler.x_tarih_adları,
                            önceki_x_yılı,
                        )
                        .map_or_else(
                            || eksen_değerini_yaz(x_değeri, x_artımı),
                            |(etiket, yıl)| {
                                önceki_x_yılı = Some(yıl);
                                etiket
                            },
                        )
                    } else {
                        let değer = x_değeri * self.seçenekler.x_eksen_değer_çarpanı;
                        let artım = x_artımı * self.seçenekler.x_eksen_değer_çarpanı;
                        match self.seçenekler.x_eksen_etiket_biçimi {
                            YÖlçekEtiketBiçimi::Otomatik => eksen_değerini_yaz(değer, artım),
                            biçim => ölçek_eksen_değerini_yaz(değer, artım, "", None, biçim),
                        }
                    },
                    renk: self.seçenekler.x_eksen_rengi.clone(),
                    boyut: 11.0,
                    hiza: etiket_hizası,
                });
            }
        }

        if self.seçenekler.x_eksen_görünür && !self.seçenekler.x_eksen_etiketi.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: if self.seçenekler.x_dikey {
                    Nokta::yeni(
                        if self.seçenekler.x_eksen_karşıda {
                            sağ
                        } else {
                            sol
                        },
                        üst - 12.0,
                    )
                } else {
                    Nokta::yeni(
                        (sol + sağ) / 2.0,
                        if self.seçenekler.x_eksen_karşıda {
                            üst - 28.0
                        } else {
                            alt + 42.0
                        },
                    )
                },
                içerik: self.seçenekler.x_eksen_etiketi.clone(),
                renk: self.seçenekler.x_eksen_rengi.clone(),
                boyut: 12.0,
                hiza: if self.seçenekler.x_dikey && self.seçenekler.x_eksen_karşıda {
                    MetinHizası::Bitiş
                } else if self.seçenekler.x_dikey {
                    MetinHizası::Başlangıç
                } else {
                    MetinHizası::Orta
                },
            });
        }
        let eksen_komutları_bitişi = sahne.komutlar().len();

        if let Some(düzen) = &self.seçenekler.ısı_haritası_düzeni {
            self.ısı_haritasını_çiz(
                &mut sahne,
                düzen,
                x_aralığı,
                y_aralığı,
                sol,
                sağ,
                üst,
                alt,
            );
        }
        if let Some(düzen) = &self.seçenekler.dağılım_düzeni {
            for seri in &düzen.seriler {
                let seri_y_aralığı =
                    self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y);
                for nokta in &seri.noktalar {
                    if nokta.x < x_aralığı.en_az
                        || nokta.x > x_aralığı.en_çok
                        || nokta.y < seri_y_aralığı.en_az
                        || nokta.y > seri_y_aralığı.en_çok
                    {
                        continue;
                    }
                    let merkez = Nokta::yeni(
                        self.x_konumu(x_aralığı, nokta.x, sol, genişlik),
                        alt - self.y_konumu(&seri.ölçek, seri_y_aralığı, nokta.y, 0.0, yükseklik),
                    );
                    let yarıçap = nokta.boyut / 2.0;
                    if merkez.x - yarıçap >= sol
                        && merkez.x + yarıçap <= sağ
                        && merkez.y - yarıçap >= üst
                        && merkez.y + yarıçap <= alt
                    {
                        sahne.ekle(Komut::Daire {
                            merkez,
                            yarıçap,
                            dolgu: seri.dolgu.clone(),
                            çizgi: seri.renk.clone(),
                            kalınlık: 1.0,
                        });
                    } else {
                        let çokgen = daire_çokgeni(merkez, yarıçap, 32);
                        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                        if kırpılmış.len() >= 3 {
                            sahne.ekle(Komut::Alan {
                                çokgenler: vec![kırpılmış],
                                dolgu: seri.dolgu.clone(),
                            });
                        }
                    }
                }
            }
        }

        for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
            let Some(seri) = self.seçenekler.seriler.get(seri_indeksi) else {
                continue;
            };
            if !seri.göster {
                continue;
            }
            let (seri_rengi, seri_dolgusu, seri_kalınlığı) =
                odaklı_seri_stili(seri, self.seçenekler.odak, self.odak_serisi, seri_indeksi);
            let seri_y_aralığı =
                self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y);
            self.seri_bantlarını_çiz(
                &mut sahne,
                seri_indeksi,
                x_aralığı,
                seri_y_aralığı,
                sol,
                sağ,
                üst,
                alt,
            );
            let bant_dolgusu = self
                .seçenekler
                .bantlar
                .iter()
                .any(|bant| bant.üst_seri == seri_indeksi);
            if seri.çizim_türü == crate::SeriÇizimTürü::Çubuk {
                if !bant_dolgusu {
                    self.karma_çubuk_serisini_çiz(
                        &mut sahne,
                        seri,
                        değerler,
                        x_aralığı,
                        seri_y_aralığı,
                        sol,
                        sağ,
                        üst,
                        alt,
                    );
                }
                continue;
            }
            let mut ham_parçalar = Vec::<Vec<Nokta>>::new();
            let mut parça = Vec::<Nokta>::new();
            let mut görünür_noktalar = Vec::<(usize, Nokta, f64, f64)>::new();
            let mut önceki_x = None::<f64>;
            let piksel_hizası = seri.piksel_hizası.unwrap_or(self.seçenekler.piksel_hizası);
            let x_piksel_uzunluğu = if self.seçenekler.x_dikey {
                yükseklik
            } else {
                genişlik
            };
            let çizilecek_indeksler =
                çizilecek_indeksler(self.veri.x(), değerler, x_aralığı, x_piksel_uzunluğu);
            let ilk_görünür = self
                .veri
                .x()
                .partition_point(|değer| *değer < x_aralığı.en_az);
            let görünür_bitiş = self
                .veri
                .x()
                .partition_point(|değer| *değer <= x_aralığı.en_çok);
            let görünür_indeks_sayısı = görünür_bitiş.saturating_sub(ilk_görünür);
            let nokta_piksel_açıklığı = ilk_görünür
                .checked_add(görünür_indeks_sayısı.saturating_sub(1))
                .and_then(|son| self.veri.x().get(ilk_görünür).zip(self.veri.x().get(son)))
                .map_or(0.0, |(ilk, son)| {
                    (self.x_konumu(x_aralığı, *son, 0.0, x_piksel_uzunluğu)
                        - self.x_konumu(x_aralığı, *ilk, 0.0, x_piksel_uzunluğu))
                    .abs()
                });
            for indeks in çizilecek_indeksler {
                let Some(değer) = değerler.get(indeks) else {
                    continue;
                };
                let Some(x_değeri) = self.veri.x().get(indeks) else {
                    continue;
                };
                match değer {
                    Some(y_değeri) => {
                        if önceki_x.is_some_and(|önceki| {
                            seri.azami_x_boşluğu
                                .is_some_and(|azami| *x_değeri - önceki > azami)
                        }) && !parça.is_empty()
                        {
                            ham_parçalar.push(std::mem::take(&mut parça));
                        }
                        let (x, y) = if self.seçenekler.x_dikey {
                            (
                                piksele_hizala(
                                    sol + self.y_konumu(
                                        &seri.ölçek,
                                        seri_y_aralığı,
                                        *y_değeri,
                                        0.0,
                                        genişlik,
                                    ),
                                    piksel_hizası,
                                ),
                                piksele_hizala(
                                    alt - self.x_konumu(x_aralığı, *x_değeri, 0.0, yükseklik),
                                    piksel_hizası,
                                ),
                            )
                        } else {
                            (
                                piksele_hizala(
                                    self.x_konumu(x_aralığı, *x_değeri, sol, genişlik),
                                    piksel_hizası,
                                ),
                                piksele_hizala(
                                    alt - self.y_konumu(
                                        &seri.ölçek,
                                        seri_y_aralığı,
                                        *y_değeri,
                                        0.0,
                                        yükseklik,
                                    ),
                                    piksel_hizası,
                                ),
                            )
                        };
                        let nokta = Nokta::yeni(x, y);
                        parça.push(nokta);
                        önceki_x = Some(*x_değeri);
                        if nokta_dikdörtgende(nokta, sol, sağ, üst, alt) {
                            görünür_noktalar.push((indeks, nokta, *x_değeri, *y_değeri));
                        }
                    }
                    _ if self.veri.hizalama_eksiği_mi(seri_indeksi, indeks)
                        || seri.boşlukları_birleştir => {}
                    _ if !parça.is_empty() => {
                        ham_parçalar.push(std::mem::take(&mut parça));
                        önceki_x = None;
                    }
                    _ => önceki_x = None,
                }
            }
            if !parça.is_empty() {
                ham_parçalar.push(parça);
            }
            let ham_parçalar = ham_parçalar
                .into_iter()
                .map(|parça| seri_yol_noktaları(&parça, seri.çizim_türü))
                .collect::<Vec<_>>();
            let parçalar = yolu_dikdörtgene_kırp(&ham_parçalar, sol, sağ, üst, alt);
            if !bant_dolgusu
                && seri.çizim_türü != crate::SeriÇizimTürü::Noktalar
                && (seri_dolgusu.is_some() || seri.dolgu_gradyanı.is_some())
            {
                let taban = if self.seçenekler.x_dikey {
                    sol + self.y_konumu(
                        &seri.ölçek,
                        seri_y_aralığı,
                        seri.dolgu_tabanı,
                        0.0,
                        genişlik,
                    )
                } else {
                    alt - self.y_konumu(
                        &seri.ölçek,
                        seri_y_aralığı,
                        seri.dolgu_tabanı,
                        0.0,
                        yükseklik,
                    )
                };
                let taban = if self.seçenekler.x_dikey {
                    taban.clamp(sol, sağ)
                } else {
                    taban.clamp(üst, alt)
                };
                let çokgenler = ham_parçalar
                    .iter()
                    .filter_map(|parça| {
                        let ilk = parça.first()?;
                        let son = parça.last()?;
                        let mut çokgen = parça.clone();
                        if self.seçenekler.x_dikey {
                            çokgen.push(Nokta::yeni(taban, son.y));
                            çokgen.push(Nokta::yeni(taban, ilk.y));
                        } else {
                            çokgen.push(Nokta::yeni(son.x, taban));
                            çokgen.push(Nokta::yeni(ilk.x, taban));
                        }
                        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                        (kırpılmış.len() >= 3).then_some(kırpılmış)
                    })
                    .collect();
                if let Some(gradyan) = seri.dolgu_gradyanı.as_ref().and_then(|düzen| {
                    self.ölçek_gradyanını_çöz(
                        düzen,
                        &seri.ölçek,
                        x_aralığı,
                        seri_y_aralığı,
                        sol,
                        üst,
                        genişlik,
                        yükseklik,
                    )
                }) {
                    sahne.ekle(Komut::GradyanAlan {
                        çokgenler, gradyan
                    });
                } else if let Some(dolgu) = &seri_dolgusu {
                    sahne.ekle(Komut::Alan {
                        çokgenler,
                        dolgu: dolgu.clone(),
                    });
                }
            }
            if seri.çizim_türü == crate::SeriÇizimTürü::Noktalar {
                // Resmî null path: yalnız koşullu veri noktaları çizilir.
            } else if let Some(gradyan) = seri.çizgi_gradyanı.as_ref().and_then(|düzen| {
                self.ölçek_gradyanını_çöz(
                    düzen,
                    &seri.ölçek,
                    x_aralığı,
                    seri_y_aralığı,
                    sol,
                    üst,
                    genişlik,
                    yükseklik,
                )
            }) {
                sahne.ekle(Komut::GradyanYol {
                    parçalar,
                    gradyan,
                    kalınlık: seri_kalınlığı,
                });
            } else if let Some((çizgi, boşluk)) = seri.çizgi_kesik {
                sahne.ekle(Komut::KesikliYol {
                    parçalar,
                    renk: seri_rengi.clone(),
                    kalınlık: seri_kalınlığı,
                    çizgi,
                    boşluk,
                });
            } else {
                sahne.ekle(Komut::Yol {
                    parçalar,
                    renk: seri_rengi.clone(),
                    kalınlık: seri_kalınlığı,
                });
            }

            // uPlot'un varsayılanı, görünür indeks sayısını çizim genişliğinin
            // `points.space` kapasitesiyle karşılaştırır.
            let kanca = self.seçenekler.çizim_kancaları.as_ref();
            if let Some((uçlar, düzen)) =
                kanca.and_then(|düzen| düzen.yıldız_uçları.map(|uçlar| (uçlar, düzen)))
            {
                for (_, nokta, _, _) in &görünür_noktalar {
                    sahne.ekle(Komut::Alan {
                        çokgenler: vec![yıldız_çokgeni(
                            *nokta,
                            uçlar,
                            düzen.yıldız_dış_yarıçapı,
                            düzen.yıldız_iç_yarıçapı,
                        )],
                        dolgu: seri_rengi.clone(),
                    });
                }
            } else {
                let noktalar_görünür = seri.noktaları_göster.unwrap_or_else(|| {
                    seri.nokta_boşluğu <= 0.0
                        || görünür_indeks_sayısı.saturating_sub(1) as f32
                            <= nokta_piksel_açıklığı / seri.nokta_boşluğu.max(f32::EPSILON)
                });
                for (indeks, nokta, x_değeri, y_değeri) in &görünür_noktalar {
                    let filtreli_tekil = !noktalar_görünür
                        && seri.nokta_filtresi
                            == crate::NoktaFiltreKipi::BoşlukArasındakiTekiller
                        && tekil_değer_mi(değerler, *indeks);
                    if !noktalar_görünür && !filtreli_tekil {
                        continue;
                    }
                    let nokta_rengi = self
                        .seri_imleç_rengi(seri_indeksi, *x_değeri, *y_değeri)
                        .unwrap_or_else(|| seri_rengi.clone());
                    sahne.ekle(Komut::Daire {
                        merkez: *nokta,
                        yarıçap: ((seri.nokta_boyutu - seri.nokta_kalınlığı) / 2.0).max(0.0),
                        dolgu: seri
                            .nokta_dolgusu
                            .clone()
                            .unwrap_or_else(|| "#ffffff".to_string()),
                        çizgi: nokta_rengi,
                        kalınlık: seri.nokta_kalınlığı,
                    });
                }
            }

            if let Some(düzen) = kanca.filter(|düzen| düzen.seri_medyanları) {
                let mut sıralı = değerler.iter().copied().flatten().collect::<Vec<_>>();
                sıralı.sort_by(f64::total_cmp);
                if let Some(medyan) = medyan(&sıralı) {
                    let y =
                        alt - self.y_konumu(&seri.ölçek, seri_y_aralığı, medyan, 0.0, yükseklik);
                    let dış_kalınlık =
                        düzen.medyan_kalınlığı + düzen.medyan_bulanıklığı.max(0.0) * 2.0;
                    if düzen.medyan_bulanıklığı > 0.0 {
                        sahne.ekle(Komut::Çizgi {
                            başlangıç: Nokta::yeni(sol, y),
                            bitiş: Nokta::yeni(sağ, y),
                            renk: renk_alfa(&seri_rengi, 0x14),
                            kalınlık: dış_kalınlık,
                        });
                    }
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: renk_alfa(&seri_rengi, 0x33),
                        kalınlık: düzen.medyan_kalınlığı,
                    });
                }
            }
        }

        let birincil_aralık = self.görünür_ölçek_aralığı(
            &self.seçenekler.birincil_y_ölçeği,
            x_aralığı,
            görünür_y,
        );
        for katman in &self.seçenekler.nokta_katmanları {
            for (x_değeri, y_değeri) in katman.noktalar.iter().copied() {
                if x_değeri < x_aralığı.en_az || x_değeri > x_aralığı.en_çok {
                    continue;
                }
                let x = self.x_konumu(x_aralığı, x_değeri, sol, genişlik);
                let y = alt
                    - self.y_konumu(
                        &self.seçenekler.birincil_y_ölçeği,
                        birincil_aralık,
                        y_değeri,
                        0.0,
                        yükseklik,
                    );
                if nokta_dikdörtgende(Nokta::yeni(x, y), sol, sağ, üst, alt) {
                    sahne.ekle(Komut::Dikdörtgen {
                        konum: Nokta::yeni(x, y),
                        genişlik: katman.boyut,
                        yükseklik: katman.boyut,
                        dolgu: katman.renk.clone(),
                        çizgi: katman.renk.clone(),
                        kalınlık: 0.0,
                    });
                }
            }
        }

        if self
            .seçenekler
            .çizim_kancaları
            .as_ref()
            .is_some_and(|düzen| düzen.çizim_süresi_metni)
        {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol + 10.0, üst + 22.0),
                içerik: "Time to Draw: 0ms".to_string(),
                renk: "#ffffff".to_string(),
                boyut: 12.0,
                hiza: MetinHizası::Başlangıç,
            });
        }

        if self.seçenekler.çizim_sırası == crate::ÇizimSırası::SerilerEksenler {
            sahne.komut_aralığını_sona_taşı(
                eksen_komutları_başlangıcı,
                eksen_komutları_bitişi,
            );
        }

        sahne
    }

    fn birincil_y_ızgara_çizgisini_ekle(
        &self,
        sahne: &mut Sahne,
        başlangıç: Nokta,
        bitiş: Nokta,
    ) {
        if let Some(kesik) = self.seçenekler.birincil_y_ızgara_kesik {
            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç,
                bitiş,
                renk: self.seçenekler.ızgara_rengi.clone(),
                kalınlık: 1.0,
                kesik,
            });
        } else {
            sahne.ekle(Komut::Çizgi {
                başlangıç,
                bitiş,
                renk: self.seçenekler.ızgara_rengi.clone(),
                kalınlık: 1.0,
            });
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn karma_çubuk_serisini_çiz(
        &self,
        sahne: &mut Sahne,
        seri: &crate::SeriSeçenekleri,
        değerler: &[Option<f64>],
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        sağ: f32,
        üst: f32,
        alt: f32,
    ) {
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;
        let mut önceki_x = None;
        let mut en_küçük_fark = f64::INFINITY;
        for (x, değer) in self.veri.x().iter().zip(değerler.iter()) {
            if değer.is_none() || *x < x_aralığı.en_az || *x > x_aralığı.en_çok {
                continue;
            }
            if let Some(önceki) = önceki_x {
                let fark = *x - önceki;
                if fark > 0.0 {
                    en_küçük_fark = en_küçük_fark.min(fark);
                }
            }
            önceki_x = Some(*x);
        }
        let varsayılan_fark =
            (x_aralığı.en_çok - x_aralığı.en_az) / değerler.len().saturating_sub(1).max(1) as f64;
        let veri_farkı = if en_küçük_fark.is_finite() {
            en_küçük_fark
        } else {
            varsayılan_fark
        };
        let çubuk_genişliği = (veri_farkı / (x_aralığı.en_çok - x_aralığı.en_az)
            * f64::from(genişlik)
            * f64::from(seri.çubuk_genişlik_oranı)) as f32;
        let çubuk_genişliği = çubuk_genişliği.min(seri.azami_çubuk_genişliği);
        let üst_değerler = seri
            .yüzen_çubuk_üst_serisi
            .and_then(|indeks| self.veri.seriler().get(indeks));
        let varsayılan_dolgu = seri.dolgu.as_ref().unwrap_or(&seri.renk);
        let mut gradyan_çokgenleri = Vec::new();
        for (indeks, (x_değeri, değer)) in self.veri.x().iter().zip(değerler.iter()).enumerate()
        {
            let Some(alt_değer) = değer else {
                continue;
            };
            let üst_değer = if let Some(üst_değerler) = üst_değerler {
                let Some(üst_değer) = üst_değerler.get(indeks).copied().flatten() else {
                    continue;
                };
                üst_değer
            } else {
                *alt_değer
            };
            let taban_değer = if üst_değerler.is_some() {
                *alt_değer
            } else {
                0.0
            };
            if *x_değeri < x_aralığı.en_az || *x_değeri > x_aralığı.en_çok {
                continue;
            }
            let merkez = self.x_konumu(x_aralığı, *x_değeri, sol, genişlik);
            let y0 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, taban_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            let y1 = (alt - self.y_konumu(&seri.ölçek, y_aralığı, üst_değer, 0.0, yükseklik))
                .clamp(üst, alt);
            let (ham_x0, ham_x1) = match seri.çubuk_hizası {
                1 => (merkez, merkez + çubuk_genişliği),
                -1 => (merkez - çubuk_genişliği, merkez),
                _ => (
                    merkez - çubuk_genişliği / 2.0,
                    merkez + çubuk_genişliği / 2.0,
                ),
            };
            let x0 = ham_x0.clamp(sol, sağ);
            let x1 = ham_x1.clamp(sol, sağ);
            let çubuk_üst = y1.min(y0);
            let çubuk_alt = y1.max(y0);
            if x1 <= x0 || çubuk_alt <= çubuk_üst {
                continue;
            }
            if seri.dolgu_gradyanı.is_some() {
                gradyan_çokgenleri.push(vec![
                    Nokta::yeni(x0, çubuk_üst),
                    Nokta::yeni(x1, çubuk_üst),
                    Nokta::yeni(x1, çubuk_alt),
                    Nokta::yeni(x0, çubuk_alt),
                ]);
            } else {
                let dolgu = seri.çubuk_dolguları.get(indeks).unwrap_or(varsayılan_dolgu);
                sahne.ekle(Komut::Dikdörtgen {
                    konum: Nokta::yeni(x0, çubuk_üst),
                    genişlik: x1 - x0,
                    yükseklik: çubuk_alt - çubuk_üst,
                    dolgu: dolgu.clone(),
                    çizgi: seri.renk.clone(),
                    kalınlık: seri.çizgi_kalınlığı,
                });
            }
        }
        if !gradyan_çokgenleri.is_empty()
            && let Some(gradyan) = seri.dolgu_gradyanı.as_ref().and_then(|düzen| {
                self.ölçek_gradyanını_çöz(
                    düzen,
                    &seri.ölçek,
                    x_aralığı,
                    y_aralığı,
                    sol,
                    üst,
                    genişlik,
                    yükseklik,
                )
            })
        {
            sahne.ekle(Komut::GradyanAlan {
                çokgenler: gradyan_çokgenleri,
                gradyan,
            });
        }
    }

    fn çubukları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: crate::ÇubukDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let grup_sayısı = self.veri.uzunluk();
        let seri_sayısı = self.veri.seriler().len().max(1);
        if grup_sayısı == 0 {
            return;
        }
        let kategoriler = (0..grup_sayısı)
            .map(|indeks| {
                self.seçenekler
                    .kategoriler
                    .get(indeks)
                    .cloned()
                    .or_else(|| self.veri.x().get(indeks).map(|değer| format!("{değer:.0}")))
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>();
        let (en_az, en_çok) = (0..grup_sayısı).fold((0.0_f64, 0.0_f64), |sonuç, indeks| {
            let değerler = self
                .veri
                .seriler()
                .iter()
                .filter_map(|seri| seri.get(indeks).copied().flatten());
            let (alt, üst) = if düzen.yığılmış {
                değerler.fold((0.0_f64, 0.0_f64), |(alt, üst), değer| {
                    if değer < 0.0 {
                        (alt + değer, üst)
                    } else {
                        (alt, üst + değer)
                    }
                })
            } else {
                değerler.fold((0.0_f64, 0.0_f64), |(alt, üst), değer| {
                    (alt.min(değer), üst.max(değer))
                })
            };
            (sonuç.0.min(alt), sonuç.1.max(üst))
        });
        let ham_açıklık = (en_çok - en_az).max(1.0);
        let veri_aralığı = Aralık {
            en_az: if en_az < 0.0 {
                en_az - ham_açıklık * 0.05
            } else {
                0.0
            },
            en_çok: if en_çok > 0.0 {
                en_çok + ham_açıklık * 0.05
            } else {
                0.0
            },
        };
        let aralık = görünür_y.unwrap_or(veri_aralığı);
        let tam_x = tam_x_aralığı(&self.veri)
            .ok()
            .and_then(|aralık| {
                if grup_sayısı > 1 {
                    Aralık::yeni(aralık.en_az - 0.5, aralık.en_çok + 0.5).ok()
                } else {
                    Some(aralık)
                }
            })
            .unwrap_or(Aralık {
                en_az: -0.5,
                en_çok: 0.5,
            });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);

        match düzen.yön {
            crate::ÇubukYönü::Dikey => {
                let (sol, sağ, üst, alt) = (
                    64.0,
                    genişlik_px as f32 - 24.0,
                    48.0,
                    yükseklik_px as f32 - 72.0,
                );
                let çizim_g = sağ - sol;
                let çizim_y = alt - üst;
                for değer in eksen_bölmeleri(aralık, çizim_y, 30.0) {
                    let y = alt - aralık.konum(değer, 0.0, çizim_y);
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: "#e5e7eb".to_string(),
                        kalınlık: 1.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(sol - 8.0, y + 4.0),
                        içerik: eksen_değerini_yaz(değer, uygun_artım(aralık, çizim_y, 30.0)),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Bitiş,
                    });
                }
                let grup_adımı = çizim_g / x_açıklığı as f32;
                let grup_genişliği = grup_adımı * 0.9;
                let otomatik_yazı_boyutu = düzen.değer_etiketi_otomatik.then(|| {
                    let azami_metin_genişliği = self
                        .veri
                        .seriler()
                        .iter()
                        .flat_map(|seri| seri.iter().copied().flatten())
                        .map(|değer| kompakt_sayı(değer).chars().count() as f32 * 6.0)
                        .fold(1.0_f32, f32::max);
                    let kullanılabilir_yükseklik = self
                        .veri
                        .seriler()
                        .iter()
                        .flat_map(|seri| seri.iter().copied().flatten())
                        .map(|değer| {
                            let uç_y = alt - aralık.konum(değer, 0.0, çizim_y);
                            if değer < 0.0 {
                                alt - uç_y
                            } else {
                                uç_y - üst
                            }
                        })
                        .fold(f32::INFINITY, f32::min);
                    let ölçek = (grup_genişliği * 0.8 / azami_metin_genişliği)
                        .min(kullanılabilir_yükseklik / 10.0);
                    (ölçek * 10.0).min(25.0)
                });
                for indeks in 0..grup_sayısı {
                    let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                        continue;
                    };
                    let merkez =
                        sol + ((x_değeri - x_aralığı.en_az) / x_açıklığı) as f32 * çizim_g;
                    if merkez + grup_genişliği / 2.0 < sol || merkez - grup_genişliği / 2.0 > sağ
                    {
                        continue;
                    }
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(merkez, alt + 22.0),
                        içerik: kategoriler.get(indeks).cloned().unwrap_or_default(),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                    let mut birikim = 0.0_f64;
                    for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
                        let Some(değer) = değerler.get(indeks).copied().flatten() else {
                            continue;
                        };
                        let (x, genişlik) = if düzen.yığılmış {
                            (merkez - grup_genişliği / 2.0, grup_genişliği)
                        } else {
                            let genişlik = grup_genişliği / seri_sayısı as f32;
                            (
                                merkez - grup_genişliği / 2.0 + seri_indeksi as f32 * genişlik,
                                genişlik,
                            )
                        };
                        let taban = if düzen.yığılmış { birikim } else { 0.0 };
                        birikim += if düzen.yığılmış { değer } else { 0.0 };
                        let tepe = if düzen.yığılmış {
                            birikim
                        } else {
                            değer
                        };
                        let y0 = alt - aralık.konum(taban, 0.0, çizim_y);
                        let y1 = alt - aralık.konum(tepe, 0.0, çizim_y);
                        let çubuk_sol = x.clamp(sol, sağ);
                        let çubuk_sağ = (x + genişlik).clamp(sol, sağ);
                        let renk = self
                            .seçenekler
                            .seriler
                            .get(seri_indeksi)
                            .and_then(|seri| seri.dolgu.clone().or_else(|| Some(seri.renk.clone())))
                            .unwrap_or_else(|| "#6b7280".to_string());
                        let çubuk_üst = y1.min(y0).clamp(üst, alt);
                        let çubuk_alt = y1.max(y0).clamp(üst, alt);
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(çubuk_sol, çubuk_üst),
                            genişlik: (çubuk_sağ - çubuk_sol).max(0.0),
                            yükseklik: (çubuk_alt - çubuk_üst).max(0.0),
                            dolgu: renk.clone(),
                            çizgi: renk,
                            kalınlık: 0.0,
                        });
                        if let Some(yazı_boyutu) = otomatik_yazı_boyutu {
                            let (alan_y, alan_yüksekliği) = if değer < 0.0 {
                                (y1, alt - y1)
                            } else {
                                (üst, y1 - üst)
                            };
                            sahne.ekle(Komut::Dikdörtgen {
                                konum: Nokta::yeni(çubuk_sol, alan_y),
                                genişlik: (çubuk_sağ - çubuk_sol).max(0.0),
                                yükseklik: alan_yüksekliği.max(0.0),
                                dolgu: "#00ff0022".to_string(),
                                çizgi: "none".to_string(),
                                kalınlık: 0.0,
                            });
                            if yazı_boyutu >= 10.0 {
                                let etiket_y = if değer < 0.0 {
                                    y1 + yazı_boyutu
                                } else {
                                    y1 - yazı_boyutu * 0.4
                                };
                                sahne.ekle(Komut::Metin {
                                    konum: Nokta::yeni(x + genişlik / 2.0, etiket_y),
                                    içerik: kompakt_sayı(değer),
                                    renk: "#111111".to_string(),
                                    boyut: yazı_boyutu,
                                    hiza: MetinHizası::Orta,
                                });
                            }
                        } else {
                            sahne.ekle(Komut::Metin {
                                konum: Nokta::yeni(x + genişlik / 2.0, y1 - 4.0),
                                içerik: format!("{tepe}"),
                                renk: "#111111".to_string(),
                                boyut: 10.0,
                                hiza: MetinHizası::Orta,
                            });
                        }
                    }
                }
            }
            crate::ÇubukYönü::Yatay => {
                let (sol, sağ, üst, alt) = (
                    150.0,
                    genişlik_px as f32 - 32.0,
                    48.0,
                    yükseklik_px as f32 - 48.0,
                );
                let çizim_g = sağ - sol;
                let çizim_y = alt - üst;
                for değer in eksen_bölmeleri(aralık, çizim_g, 50.0) {
                    let x = aralık.konum(değer, sol, çizim_g);
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(x, üst),
                        bitiş: Nokta::yeni(x, alt),
                        renk: "#e5e7eb".to_string(),
                        kalınlık: 1.0,
                    });
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(x, alt + 20.0),
                        içerik: eksen_değerini_yaz(değer, uygun_artım(aralık, çizim_g, 50.0)),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Orta,
                    });
                }
                let grup_adımı = çizim_y / x_açıklığı as f32;
                let grup_yüksekliği = grup_adımı * 0.9;
                let otomatik_yazı_boyutu = düzen
                    .değer_etiketi_otomatik
                    .then_some((grup_yüksekliği * 0.8).min(25.0));
                for indeks in 0..grup_sayısı {
                    let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                        continue;
                    };
                    let oran = if düzen.ters {
                        (x_aralığı.en_çok - x_değeri) / x_açıklığı
                    } else {
                        (x_değeri - x_aralığı.en_az) / x_açıklığı
                    };
                    let merkez = üst + oran as f32 * çizim_y;
                    if merkez + grup_yüksekliği / 2.0 < üst || merkez - grup_yüksekliği / 2.0 > alt
                    {
                        continue;
                    }
                    sahne.ekle(Komut::Metin {
                        konum: Nokta::yeni(sol - 10.0, merkez + 4.0),
                        içerik: kategoriler.get(indeks).cloned().unwrap_or_default(),
                        renk: "#4b5563".to_string(),
                        boyut: 11.0,
                        hiza: MetinHizası::Bitiş,
                    });
                    let mut birikim = 0.0_f64;
                    for (seri_indeksi, değerler) in self.veri.seriler().iter().enumerate() {
                        let Some(değer) = değerler.get(indeks).copied().flatten() else {
                            continue;
                        };
                        let (y, yükseklik) = if düzen.yığılmış {
                            (merkez - grup_yüksekliği / 2.0, grup_yüksekliği)
                        } else {
                            let yükseklik = grup_yüksekliği / seri_sayısı as f32;
                            (
                                merkez - grup_yüksekliği / 2.0 + seri_indeksi as f32 * yükseklik,
                                yükseklik,
                            )
                        };
                        let taban = if düzen.yığılmış { birikim } else { 0.0 };
                        birikim += if düzen.yığılmış { değer } else { 0.0 };
                        let uç = if düzen.yığılmış {
                            birikim
                        } else {
                            değer
                        };
                        let x0 = aralık.konum(taban, sol, çizim_g);
                        let x1 = aralık.konum(uç, sol, çizim_g);
                        let çubuk_üst = y.clamp(üst, alt);
                        let çubuk_alt = (y + yükseklik).clamp(üst, alt);
                        let renk = self
                            .seçenekler
                            .seriler
                            .get(seri_indeksi)
                            .and_then(|seri| seri.dolgu.clone().or_else(|| Some(seri.renk.clone())))
                            .unwrap_or_else(|| "#6b7280".to_string());
                        let çubuk_sol = x0.min(x1).clamp(sol, sağ);
                        let çubuk_sağ = x0.max(x1).clamp(sol, sağ);
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(çubuk_sol, çubuk_üst),
                            genişlik: (çubuk_sağ - çubuk_sol).max(0.0),
                            yükseklik: (çubuk_alt - çubuk_üst).max(0.0),
                            dolgu: renk.clone(),
                            çizgi: renk,
                            kalınlık: 0.0,
                        });
                        if let Some(yazı_boyutu) = otomatik_yazı_boyutu {
                            let (alan_x, alan_genişliği) = if değer < 0.0 {
                                (sol, x1 - sol)
                            } else {
                                (x1, sağ - x1)
                            };
                            sahne.ekle(Komut::Dikdörtgen {
                                konum: Nokta::yeni(alan_x, çubuk_üst),
                                genişlik: alan_genişliği.max(0.0),
                                yükseklik: (çubuk_alt - çubuk_üst).max(0.0),
                                dolgu: "#00ff0022".to_string(),
                                çizgi: "none".to_string(),
                                kalınlık: 0.0,
                            });
                            if yazı_boyutu >= 10.0 {
                                let metin = kompakt_sayı(değer);
                                let yarım_metin = metin.chars().count() as f32 * yazı_boyutu * 0.3;
                                let etiket_x = if değer < 0.0 {
                                    x1 - yarım_metin - yazı_boyutu * 0.4
                                } else {
                                    x1 + yarım_metin + yazı_boyutu * 0.4
                                };
                                sahne.ekle(Komut::Metin {
                                    konum: Nokta::yeni(etiket_x, y + yükseklik / 2.0 + 4.0),
                                    içerik: metin,
                                    renk: "#111111".to_string(),
                                    boyut: yazı_boyutu,
                                    hiza: MetinHizası::Orta,
                                });
                            }
                        } else {
                            sahne.ekle(Komut::Metin {
                                konum: Nokta::yeni(x1 + 4.0, y + yükseklik / 2.0 + 4.0),
                                içerik: format!("{uç}"),
                                renk: "#111111".to_string(),
                                boyut: 10.0,
                                hiza: MetinHizası::Başlangıç,
                            });
                        }
                    }
                }
            }
        }
    }

    fn kutu_bıyık_y_aralığı(&self) -> Aralık {
        let mut değerler = self
            .veri
            .seriler()
            .iter()
            .flat_map(|seri| seri.iter().copied())
            .collect::<Vec<_>>();
        if let Some(düzen) = &self.seçenekler.kutu_bıyık_düzeni {
            değerler.extend(
                düzen
                    .ayrık_değerler
                    .iter()
                    .flat_map(|ayrıklar| ayrıklar.iter().copied().map(Some)),
            );
        }
        Aralık::otomatik(değerler.iter())
    }

    fn kutu_bıyıkları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: &crate::KutuBıyıkDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let çizim_g = sağ - sol;
        let çizim_y = alt - üst;
        let grup_sayısı = self.veri.uzunluk();
        if grup_sayısı == 0 || çizim_g <= 0.0 || çizim_y <= 0.0 {
            return;
        }
        let tam_x = tam_x_aralığı(&self.veri)
            .ok()
            .and_then(|aralık| {
                if grup_sayısı > 1 {
                    Aralık::yeni(aralık.en_az - 0.5, aralık.en_çok + 0.5).ok()
                } else {
                    Some(aralık)
                }
            })
            .unwrap_or(Aralık {
                en_az: -0.5,
                en_çok: 0.5,
            });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let y_aralığı = görünür_y.unwrap_or_else(|| self.kutu_bıyık_y_aralığı());
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let sütun_genişliği = çizim_g / x_açıklığı as f32;
        let gövde_genişliği = (düzen.gövde_genişlik_oranı * (sütun_genişliği - 2.0)).max(1.0);

        let artım = uygun_artım(y_aralığı, çizim_y, 30.0);
        for değer in eksen_bölmeleri(y_aralığı, çizim_y, 30.0) {
            let y = alt - y_aralığı.konum(değer, 0.0, çizim_y);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: eksen_değerini_yaz(değer, artım),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });
        }

        for indeks in 0..grup_sayısı {
            let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                continue;
            };
            let merkez = sol + ((x_değeri - x_aralığı.en_az) / x_açıklığı) as f32 * çizim_g;
            if merkez + gövde_genişliği / 2.0 < sol || merkez - gövde_genişliği / 2.0 > sağ {
                continue;
            }
            let değer = |seri: usize| {
                self.veri
                    .seriler()
                    .get(seri)
                    .and_then(|değerler| değerler.get(indeks))
                    .copied()
                    .flatten()
            };
            let (Some(medyan), Some(q1), Some(q3), Some(en_az), Some(en_çok)) =
                (değer(0), değer(1), değer(2), değer(3), değer(4))
            else {
                continue;
            };
            let y_konumu = |değer| alt - y_aralığı.konum(değer, 0.0, çizim_y);
            let medyan_y = y_konumu(medyan).clamp(üst, alt);
            let q1_y = y_konumu(q1).clamp(üst, alt);
            let q3_y = y_konumu(q3).clamp(üst, alt);
            let min_y = y_konumu(en_az).clamp(üst, alt);
            let max_y = y_konumu(en_çok).clamp(üst, alt);
            let gövde_sol = (merkez - gövde_genişliği / 2.0).clamp(sol, sağ);
            let gövde_sağ = (merkez + gövde_genişliği / 2.0).clamp(sol, sağ);
            let gövde_üst = q1_y.min(q3_y);
            let gövde_alt = q1_y.max(q3_y);

            sahne.ekle(Komut::KesikliÇizgi {
                başlangıç: Nokta::yeni(merkez.clamp(sol, sağ), max_y.min(min_y)),
                bitiş: Nokta::yeni(merkez.clamp(sol, sağ), max_y.max(min_y)),
                renk: "#000000".to_string(),
                kalınlık: 2.0,
                kesik: 4.0,
            });
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_sol, gövde_üst),
                genişlik: (gövde_sağ - gövde_sol).max(0.0),
                yükseklik: (gövde_alt - gövde_üst).max(0.0),
                dolgu: "#eeeeee".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(gövde_sol, medyan_y - 1.0),
                genişlik: (gövde_sağ - gövde_sol).max(0.0),
                yükseklik: 2.0,
                dolgu: "#000000".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 0.0,
            });
            for y in [min_y, max_y] {
                sahne.ekle(Komut::Çizgi {
                    başlangıç: Nokta::yeni(gövde_sol, y),
                    bitiş: Nokta::yeni(gövde_sağ, y),
                    renk: "#000000".to_string(),
                    kalınlık: 2.0,
                });
            }
            if let Some(ayrıklar) = düzen.ayrık_değerler.get(indeks) {
                for ayrık in ayrıklar {
                    let y = y_konumu(*ayrık);
                    if y >= üst && y <= alt {
                        sahne.ekle(Komut::Dikdörtgen {
                            konum: Nokta::yeni(merkez - 4.0, y - 4.0),
                            genişlik: 8.0,
                            yükseklik: 8.0,
                            dolgu: "#000000".to_string(),
                            çizgi: "#000000".to_string(),
                            kalınlık: 0.0,
                        });
                    }
                }
            }
            let etiket = self
                .seçenekler
                .kategoriler
                .get(indeks)
                .map_or_else(String::new, |değer| kısalt(değer, 18));
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(merkez, alt + 18.0 + (indeks % 3) as f32 * 12.0),
                içerik: etiket,
                renk: "#4b5563".to_string(),
                boyut: 8.0,
                hiza: MetinHizası::Orta,
            });
        }
    }

    fn mumları_çiz(
        &self,
        sahne: &mut Sahne,
        genişlik_px: u32,
        yükseklik_px: u32,
        düzen: &crate::MumDüzeni,
        görünür_x: Option<Aralık>,
        görünür_y: Option<Aralık>,
    ) {
        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let çizim_g = sağ - sol;
        let çizim_y = alt - üst;
        let tam_x = tam_x_aralığı(&self.veri)
            .ok()
            .and_then(|aralık| Aralık::yeni(aralık.en_az - 0.5, aralık.en_çok + 0.5).ok())
            .unwrap_or(Aralık {
                en_az: -0.5,
                en_çok: 0.5,
            });
        let x_aralığı = görünür_x.unwrap_or(tam_x);
        let y_aralığı = görünür_y.unwrap_or_else(|| self.y_aralığı(x_aralığı));
        let x_açıklığı = (x_aralığı.en_çok - x_aralığı.en_az).max(f64::EPSILON);
        let sütun_genişliği = çizim_g / x_açıklığı as f32;
        let gövde_genişliği = düzen
            .azami_gövde_genişliği
            .min((sütun_genişliği - 2.0).max(1.0));
        let artım = uygun_artım(y_aralığı, çizim_y, 30.0);
        for değer in eksen_bölmeleri(y_aralığı, çizim_y, 30.0) {
            let y = alt - y_aralığı.konum(değer, 0.0, çizim_y);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sol - 8.0, y + 4.0),
                içerik: format!("${}", eksen_değerini_yaz(değer, artım)),
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Bitiş,
            });
        }
        let değer = |seri: usize, indeks: usize| {
            self.veri
                .seriler()
                .get(seri)
                .and_then(|değerler| değerler.get(indeks))
                .copied()
                .flatten()
        };
        let etiket_adımı = ((60.0 / sütun_genişliği.max(1.0)).ceil() as usize).max(1);
        for indeks in 0..self.veri.uzunluk() {
            let Some(x_değeri) = self.veri.x().get(indeks).copied() else {
                continue;
            };
            let merkez = sol + ((x_değeri - x_aralığı.en_az) / x_açıklığı) as f32 * çizim_g;
            if merkez + gövde_genişliği / 2.0 < sol || merkez - gövde_genişliği / 2.0 > sağ {
                continue;
            }
            let (Some(açılış), Some(yüksek), Some(düşük), Some(kapanış), Some(hacim)) = (
                değer(0, indeks),
                değer(1, indeks),
                değer(2, indeks),
                değer(3, indeks),
                değer(4, indeks),
            ) else {
                continue;
            };
            let y_konumu = |değer| alt - y_aralığı.konum(değer, 0.0, çizim_y);
            let yüksek_y = y_konumu(yüksek).clamp(üst, alt);
            let düşük_y = y_konumu(düşük).clamp(üst, alt);
            let açılış_y = y_konumu(açılış).clamp(üst, alt);
            let kapanış_y = y_konumu(kapanış).clamp(üst, alt);
            let renk = if açılış > kapanış {
                &düzen.düşüş_rengi
            } else {
                &düzen.yükseliş_rengi
            };
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(merkez - 1.0, yüksek_y.min(düşük_y)),
                genişlik: 2.0,
                yükseklik: (düşük_y - yüksek_y).abs(),
                dolgu: "#000000".to_string(),
                çizgi: "#000000".to_string(),
                kalınlık: 0.0,
            });
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(merkez - gövde_genişliği / 2.0, açılış_y.min(kapanış_y)),
                genişlik: gövde_genişliği,
                yükseklik: (kapanış_y - açılış_y).abs().max(1.0),
                dolgu: renk.clone(),
                çizgi: "#000000".to_string(),
                kalınlık: 1.0,
            });
            let hacim_y = alt - (hacim / 2_000.0).clamp(0.0, 1.0) as f32 * çizim_y;
            sahne.ekle(Komut::Dikdörtgen {
                konum: Nokta::yeni(merkez - gövde_genişliği / 2.0, hacim_y),
                genişlik: gövde_genişliği,
                yükseklik: alt - hacim_y,
                dolgu: renk.clone(),
                çizgi: "none".to_string(),
                kalınlık: 0.0,
            });
            if indeks.is_multiple_of(etiket_adımı)
                && let Some(zaman) = düzen.zamanlar.get(indeks)
                && let Some((yıl, ay, gün, ..)) = crate::zaman::utc_alanları(*zaman)
            {
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(merkez, alt + 18.0),
                    içerik: format!("{yıl:04}-{ay:02}-{gün:02}"),
                    renk: "#4b5563".to_string(),
                    boyut: 9.0,
                    hiza: MetinHizası::Orta,
                });
            }
        }
        for değer in [0.0, 500.0, 1_000.0, 1_500.0, 2_000.0] {
            let y = alt - (değer / 2_000.0) as f32 * çizim_y;
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(sağ + 8.0, y + 4.0),
                içerik: format!("{değer:.0}"),
                renk: "#4b5563".to_string(),
                boyut: 10.0,
                hiza: MetinHizası::Başlangıç,
            });
        }
    }

    fn güzel_ölçek_aralığı(
        &self,
        anahtar: &str,
        x_aralığı: Aralık,
        çizim_yüksekliği: f32,
    ) -> Option<(Aralık, f64)> {
        let ölçek = self.ölçek_seçeneği(anahtar)?;
        let düzen = ölçek.güzel_ölçek?;
        if ölçek.aralık.is_some()
            || (anahtar == self.seçenekler.birincil_y_ölçeği && self.seçenekler.y_aralığı.is_some())
        {
            return None;
        }
        let ham_aralık = sonlu_aralık(
            self.veri
                .x()
                .iter()
                .enumerate()
                .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                .flat_map(|(indeks, _)| {
                    self.veri
                        .seriler()
                        .iter()
                        .zip(self.seçenekler.seriler.iter())
                        .filter(move |(_, ayarlar)| ayarlar.ölçek == anahtar)
                        .filter_map(move |(seri, _)| seri.get(indeks).copied().flatten())
                }),
        )?;
        güzel_ölçek(ham_aralık, çizim_yüksekliği, düzen.en_az_etiket_boşluğu)
    }

    fn y_aralığı(&self, x_aralığı: Aralık) -> Aralık {
        self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, x_aralığı)
    }

    fn y_aralığı_ölçek(&self, anahtar: &str, x_aralığı: Aralık) -> Aralık {
        if let Some(ölçek) = self.ölçek_seçeneği(anahtar)
            && let Some(kaynak) = ölçek.kaynak.as_deref()
            && kaynak != anahtar
        {
            let kaynak_aralığı = self.ham_y_aralığı_ölçek(kaynak, x_aralığı);
            let dönüştür = |değer: f64| değer * ölçek.dönüşüm_çarpanı + ölçek.dönüşüm_kaydırması;
            let ilk = dönüştür(kaynak_aralığı.en_az);
            let son = dönüştür(kaynak_aralığı.en_çok);
            if let Ok(aralık) = Aralık::yeni(ilk.min(son), ilk.max(son)) {
                return aralık;
            }
        }
        self.ham_y_aralığı_ölçek(anahtar, x_aralığı)
    }

    fn ham_y_aralığı_ölçek(&self, anahtar: &str, x_aralığı: Aralık) -> Aralık {
        self.ölçek_seçeneği(anahtar)
            .and_then(|ölçek| ölçek.aralık)
            .or_else(|| {
                (anahtar == self.seçenekler.birincil_y_ölçeği)
                    .then_some(self.seçenekler.y_aralığı)
                    .flatten()
            })
            .unwrap_or_else(|| {
                let nominal_yükseklik = self.seçenekler.yükseklik.saturating_sub(96).max(1) as f32;
                if let Some((aralık, _)) =
                    self.güzel_ölçek_aralığı(anahtar, x_aralığı, nominal_yükseklik)
                {
                    return aralık;
                }
                let görünür = || {
                    self.veri
                        .x()
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| **x >= x_aralığı.en_az && **x <= x_aralığı.en_çok)
                        .flat_map(|(indeks, _)| {
                            self.veri
                                .seriler()
                                .iter()
                                .zip(self.seçenekler.seriler.iter())
                                .filter(move |(_, ayarlar)| ayarlar.ölçek == anahtar)
                                .filter_map(move |(seri, _)| seri.get(indeks))
                        })
                };
                match self.ölçek_seçeneği(anahtar).map(|ölçek| ölçek.dağılım) {
                    Some(YÖlçekDağılımı::Logaritmik { taban }) => {
                        let tam = self
                            .ölçek_seçeneği(anahtar)
                            .is_none_or(|ölçek| ölçek.log_tam_büyüklükler);
                        logaritmik_otomatik_aralık(görünür(), taban, tam)
                            .unwrap_or_else(|| Aralık::otomatik(görünür()))
                    }
                    _ => self
                        .ölçek_seçeneği(anahtar)
                        .and_then(|ölçek| ölçek.sayısal_aralık)
                        .and_then(|ayarlar| {
                            sonlu_sınırlar(görünür().flatten().copied()).and_then(
                                |(en_az, en_çok)| {
                                    Aralık::uplot_yapılandırılmış(en_az, en_çok, ayarlar).ok()
                                },
                            )
                        })
                        .unwrap_or_else(|| Aralık::otomatik(görünür())),
                }
            })
    }

    fn görünür_ölçek_aralığı(
        &self,
        anahtar: &str,
        x_aralığı: Aralık,
        görünür_birincil: Option<Aralık>,
    ) -> Aralık {
        if anahtar == self.seçenekler.birincil_y_ölçeği {
            return görünür_birincil.unwrap_or_else(|| {
                self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, x_aralığı)
            });
        }
        let Some(görünür_birincil) = görünür_birincil else {
            return self.y_aralığı_ölçek(anahtar, x_aralığı);
        };
        let Some(tam_x) = self.tam_x_aralığı() else {
            return self.y_aralığı_ölçek(anahtar, x_aralığı);
        };
        let tam_birincil = self.y_aralığı_ölçek(&self.seçenekler.birincil_y_ölçeği, tam_x);
        let tam_ikincil = self.y_aralığı_ölçek(anahtar, tam_x);
        let birincil_uzunluk = tam_birincil.en_çok - tam_birincil.en_az;
        if birincil_uzunluk <= f64::EPSILON {
            return tam_ikincil;
        }
        let ikincil_uzunluk = tam_ikincil.en_çok - tam_ikincil.en_az;
        let en_az_oran = (görünür_birincil.en_az - tam_birincil.en_az) / birincil_uzunluk;
        let en_çok_oran = (görünür_birincil.en_çok - tam_birincil.en_az) / birincil_uzunluk;
        Aralık::yeni(
            tam_ikincil.en_az + en_az_oran * ikincil_uzunluk,
            tam_ikincil.en_az + en_çok_oran * ikincil_uzunluk,
        )
        .unwrap_or(tam_ikincil)
    }

    fn gradyan_değerlerini_çöz(
        &self,
        gradyan: &ÖlçekGradyanı,
        ölçek: &str,
        x_aralığı: Aralık,
        ölçek_aralığı: Aralık,
    ) -> Option<Vec<(f64, String)>> {
        let göreli = gradyan
            .duraklar
            .iter()
            .any(|durak| matches!(durak.konum, GradyanKonumu::GörünürVeriOranı(_)));
        let (veri_en_az, veri_en_çok) = if göreli {
            self.görünür_veri_aralığı(ölçek, x_aralığı)
                .filter(|(en_az, en_çok)| en_çok - en_az > f64::EPSILON)
                .unwrap_or((ölçek_aralığı.en_az, ölçek_aralığı.en_çok))
        } else {
            (ölçek_aralığı.en_az, ölçek_aralığı.en_çok)
        };
        let veri_aralığı = veri_en_çok - veri_en_az;
        gradyan
            .duraklar
            .iter()
            .map(|durak| {
                let değer = match durak.konum {
                    GradyanKonumu::Değer(değer) => değer,
                    GradyanKonumu::NegatifSonsuz => f64::NEG_INFINITY,
                    GradyanKonumu::PozitifSonsuz => f64::INFINITY,
                    GradyanKonumu::GörünürVeriOranı(oran) => veri_en_az + veri_aralığı * oran,
                };
                (!değer.is_nan()).then(|| (değer, durak.renk.clone()))
            })
            .collect()
    }

    fn görünür_veri_aralığı(
        &self, ölçek: &str, x_aralığı: Aralık
    ) -> Option<(f64, f64)> {
        let mut en_az = f64::INFINITY;
        let mut en_çok = f64::NEG_INFINITY;
        for (seri, _) in self
            .veri
            .seriler()
            .iter()
            .zip(self.seçenekler.seriler.iter())
            .filter(|(_, seçenek)| seçenek.göster && seçenek.ölçek == ölçek)
        {
            for (x, değer) in self.veri.x().iter().zip(seri.iter()) {
                if *x < x_aralığı.en_az || *x > x_aralığı.en_çok {
                    continue;
                }
                let Some(değer) = değer else { continue };
                en_az = en_az.min(*değer);
                en_çok = en_çok.max(*değer);
            }
        }
        (en_az.is_finite() && en_çok.is_finite()).then_some((en_az, en_çok))
    }

    #[allow(clippy::too_many_arguments)]
    fn ölçek_gradyanını_çöz(
        &self,
        gradyan: &ÖlçekGradyanı,
        ölçek: &str,
        x_aralığı: Aralık,
        y_aralığı: Aralık,
        sol: f32,
        üst: f32,
        genişlik: f32,
        yükseklik: f32,
    ) -> Option<DoğrusalGradyan> {
        let ölçek_aralığı = match gradyan.eksen {
            GradyanEkseni::X => x_aralığı,
            GradyanEkseni::Y => y_aralığı,
        };
        let değerler = self.gradyan_değerlerini_çöz(gradyan, ölçek, x_aralığı, ölçek_aralığı)?;
        let mut en_az_indeks = None;
        let mut en_çok_indeks = None;
        for (indeks, (değer, _)) in değerler.iter().enumerate() {
            if *değer <= ölçek_aralığı.en_az || en_az_indeks.is_none() {
                en_az_indeks = Some(indeks);
            }
            en_çok_indeks = Some(indeks);
            if *değer >= ölçek_aralığı.en_çok {
                break;
            }
        }
        let (en_az_indeks, en_çok_indeks) = (en_az_indeks?, en_çok_indeks?);
        let en_az_durak = değerler.get(en_az_indeks)?;
        let en_çok_durak = değerler.get(en_çok_indeks)?;
        let en_az_değer = if en_az_durak.0.is_infinite() {
            ölçek_aralığı.en_az
        } else {
            en_az_durak.0
        };
        let en_çok_değer = if en_çok_durak.0.is_infinite() {
            ölçek_aralığı.en_çok
        } else {
            en_çok_durak.0
        };
        let alt = üst + yükseklik;
        let konum = |değer: f64| match gradyan.eksen {
            GradyanEkseni::X => self.x_konumu(x_aralığı, değer, sol, genişlik),
            GradyanEkseni::Y => alt - self.y_konumu(ölçek, y_aralığı, değer, 0.0, yükseklik),
        };
        let en_az_konum = konum(en_az_değer);
        let en_çok_konum = konum(en_çok_değer);
        let başlangıç = match gradyan.eksen {
            GradyanEkseni::X => Nokta::yeni(en_az_konum, üst),
            GradyanEkseni::Y => Nokta::yeni(sol, en_az_konum),
        };
        let bitiş = match gradyan.eksen {
            GradyanEkseni::X => Nokta::yeni(en_çok_konum, üst),
            GradyanEkseni::Y => Nokta::yeni(sol, en_çok_konum),
        };
        if en_az_indeks == en_çok_indeks || (en_az_konum - en_çok_konum).abs() <= f32::EPSILON {
            return Some(DoğrusalGradyan {
                başlangıç,
                bitiş,
                duraklar: vec![
                    GradyanRenkDurağı {
                        oran: 0.0,
                        renk: en_az_durak.1.clone(),
                    },
                    GradyanRenkDurağı {
                        oran: 1.0,
                        renk: en_az_durak.1.clone(),
                    },
                ],
            });
        }
        let seçilenler = değerler.get(en_az_indeks..=en_çok_indeks)?;
        let fark = en_az_konum - en_çok_konum;
        let mut duraklar = Vec::new();
        let mut önceki_renk = None::<String>;
        for (yerel_indeks, (değer, renk)) in seçilenler.iter().enumerate() {
            let durak_konumu = if yerel_indeks == 0 {
                en_az_konum
            } else if yerel_indeks + 1 == seçilenler.len() {
                en_çok_konum
            } else {
                konum(*değer)
            };
            let oran = ((en_az_konum - durak_konumu) / fark).clamp(0.0, 1.0);
            if gradyan.ayrık
                && yerel_indeks > 0
                && let Some(önceki_renk) = önceki_renk.as_ref()
            {
                duraklar.push(GradyanRenkDurağı {
                    oran,
                    renk: önceki_renk.clone(),
                });
            }
            duraklar.push(GradyanRenkDurağı {
                oran,
                renk: renk.clone(),
            });
            önceki_renk = Some(renk.clone());
        }
        Some(DoğrusalGradyan {
            başlangıç,
            bitiş,
            duraklar,
        })
    }

    fn tam_x_aralığı(&self) -> Option<Aralık> {
        self.seçenekler
            .x_aralığı
            .or_else(|| tam_x_aralığı(&self.veri).ok())
    }

    fn ölçek_seçeneği(&self, anahtar: &str) -> Option<&crate::YÖlçekSeçenekleri> {
        self.seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == anahtar)
    }

    fn y_konumu(
        &self,
        anahtar: &str,
        aralık: Aralık,
        değer: f64,
        başlangıç: f32,
        uzunluk: f32,
    ) -> f32 {
        let ölçek = self.ölçek_seçeneği(anahtar);
        let konum = match ölçek.map(|ölçek| ölçek.dağılım) {
            Some(YÖlçekDağılımı::Logaritmik { taban })
                if taban.is_finite() && taban > 1.0 && aralık.en_az > 0.0 =>
            {
                let dönüştür = |sayı: f64| sayı.log(taban);
                let değer = if değer > 0.0 {
                    değer
                } else {
                    aralık.en_az / taban
                };
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, dönüştür)
            }
            Some(YÖlçekDağılımı::Weibull)
                if aralık.en_az > 0.0 && aralık.en_çok < 1.0 && değer > 0.0 && değer < 1.0 =>
            {
                let dönüştür = |sayı: f64| (-(-sayı).ln_1p()).ln();
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, dönüştür)
            }
            Some(YÖlçekDağılımı::ArcSinh { eşik }) if eşik.is_finite() && eşik > 0.0 => {
                let dönüştür = |sayı: f64| (sayı / eşik).asinh();
                let en_az = dönüştür(aralık.en_az);
                let en_çok = dönüştür(aralık.en_çok);
                let değer = dönüştür(değer);
                let oran = (değer - en_az) / (en_çok - en_az);
                başlangıç + oran as f32 * uzunluk
            }
            _ => aralık.konum(değer, başlangıç, uzunluk),
        };
        if ölçek.is_some_and(|ölçek| ölçek.ters_yön) {
            başlangıç + uzunluk - (konum - başlangıç)
        } else {
            konum
        }
    }

    fn y_eksen_bölmeleri(&self, anahtar: &str, aralık: Aralık, boyut: f32) -> Vec<f64> {
        match self.ölçek_seçeneği(anahtar).map(|ölçek| ölçek.dağılım) {
            Some(YÖlçekDağılımı::ArcSinh { eşik }) if eşik.is_finite() && eşik > 0.0 => {
                let en_az = (aralık.en_az / eşik).asinh();
                let en_çok = (aralık.en_çok / eşik).asinh();
                dönüşmüş_bölmeler(en_az, en_çok, boyut, |değer| eşik * değer.sinh())
            }
            Some(YÖlçekDağılımı::Logaritmik { taban }) if aralık.en_az > 0.0 => {
                logaritmik_bölmeler(aralık, taban)
            }
            Some(YÖlçekDağılımı::Weibull) => [
                0.00001, 0.0001, 0.001, 0.01, 0.1, 0.2, 0.3, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99,
                0.999, 0.9999, 0.99999, 0.999999,
            ]
            .into_iter()
            .filter(|değer| (*değer >= aralık.en_az) && (*değer <= aralık.en_çok))
            .collect(),
            _ => eksen_bölmeleri(aralık, boyut, 30.0),
        }
    }

    fn x_konumu(&self, aralık: Aralık, değer: f64, başlangıç: f32, uzunluk: f32) -> f32 {
        let konum = match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban }
                if aralık.en_az > 0.0 && değer > 0.0 && taban > 1.0 =>
            {
                dönüştürülmüş_konum(aralık, değer, başlangıç, uzunluk, |sayı| {
                    sayı.log(taban)
                })
            }
            _ => aralık.konum(değer, başlangıç, uzunluk),
        };
        if self.seçenekler.x_ters_yön {
            başlangıç + uzunluk - (konum - başlangıç)
        } else {
            konum
        }
    }

    fn x_değeri_orandan(&self, aralık: Aralık, oran: f64) -> f64 {
        let oran = if self.seçenekler.x_ters_yön {
            1.0 - oran
        } else {
            oran
        };
        match self.seçenekler.x_dağılımı {
            XÖlçekDağılımı::Logaritmik { taban } if aralık.en_az > 0.0 && taban > 1.0 => {
                let en_az = aralık.en_az.log(taban);
                let en_çok = aralık.en_çok.log(taban);
                taban.powf(en_az + oran * (en_çok - en_az))
            }
            _ => aralık.en_az + oran * (aralık.en_çok - aralık.en_az),
        }
    }
}

fn dönüştürülmüş_konum(
    aralık: Aralık,
    değer: f64,
    başlangıç: f32,
    uzunluk: f32,
    dönüştür: impl Fn(f64) -> f64,
) -> f32 {
    let en_az = dönüştür(aralık.en_az);
    let en_çok = dönüştür(aralık.en_çok);
    let değer = dönüştür(değer);
    başlangıç + ((değer - en_az) / (en_çok - en_az)) as f32 * uzunluk
}

fn dönüşmüş_bölmeler(
    en_az: f64,
    en_çok: f64,
    boyut: f32,
    geri: impl Fn(f64) -> f64,
) -> Vec<f64> {
    let adım_sayısı = (boyut / 55.0).round().clamp(3.0, 12.0) as u32;
    (0..=adım_sayısı)
        .map(|indeks| {
            let oran = f64::from(indeks) / f64::from(adım_sayısı);
            geri(en_az + (en_çok - en_az) * oran)
        })
        .collect()
}

fn zaman_bölmeleri(
    aralık: Aralık,
    boyut: f32,
    en_az_boşluk: f32,
    milisaniye: bool,
) -> (Vec<f64>, f64) {
    const SANİYE_ADIMLARI: [f64; 25] = [
        1.0,
        2.0,
        5.0,
        10.0,
        15.0,
        30.0,
        60.0,
        120.0,
        300.0,
        600.0,
        900.0,
        1_800.0,
        3_600.0,
        7_200.0,
        10_800.0,
        21_600.0,
        43_200.0,
        86_400.0,
        172_800.0,
        604_800.0,
        2_592_000.0,
        7_776_000.0,
        15_552_000.0,
        31_536_000.0,
        63_072_000.0,
    ];
    let birim = if milisaniye { 1_000.0 } else { 1.0 };
    let hedef =
        (aralık.en_çok - aralık.en_az) * f64::from(en_az_boşluk) / f64::from(boyut.max(1.0));
    let adım = SANİYE_ADIMLARI
        .into_iter()
        .map(|adım| adım * birim)
        .find(|adım| *adım >= hedef)
        .unwrap_or(63_072_000.0 * birim);
    let saniye_adımı = adım / birim;
    if saniye_adımı >= 2_592_000.0 {
        let ay_adımı = if saniye_adımı >= 31_536_000.0 {
            let yıl_adımı = (saniye_adımı / 31_536_000.0).round().max(1.0) as i64;
            yıl_adımı.saturating_mul(12)
        } else {
            (saniye_adımı / 2_592_000.0).round().max(1.0) as i64
        };
        return (takvim_ay_bölmeleri(aralık, birim, ay_adımı), adım);
    }
    let ilk = (aralık.en_az / adım).ceil() * adım;
    let mut sonuç = Vec::new();
    let mut değer = ilk;
    while değer <= aralık.en_çok && sonuç.len() < 10_000 {
        sonuç.push(değer);
        değer += adım;
    }
    (sonuç, adım)
}

fn takvim_ay_bölmeleri(aralık: Aralık, birim: f64, ay_adımı: i64) -> Vec<f64> {
    if !birim.is_finite() || birim <= 0.0 || ay_adımı <= 0 {
        return Vec::new();
    }
    let en_az = aralık.en_az / birim;
    let en_çok = aralık.en_çok / birim;
    let Some((yıl, ay, _, _, _, _)) = crate::zaman::utc_alanları(en_az) else {
        return Vec::new();
    };
    let mut ay_indeksi = yıl
        .saturating_mul(12)
        .saturating_add(i64::from(ay).saturating_sub(1));
    ay_indeksi = ay_indeksi.div_euclid(ay_adımı).saturating_mul(ay_adımı);
    let mut sonuç = Vec::new();
    for _ in 0..10_000 {
        let bölme_yılı = ay_indeksi.div_euclid(12);
        let ay_sıfır = ay_indeksi.rem_euclid(12);
        let Ok(bölme_ayı) = u32::try_from(ay_sıfır.saturating_add(1)) else {
            break;
        };
        let Some(zaman) = crate::zaman::utc_zaman_damgası(bölme_yılı, bölme_ayı, 1) else {
            break;
        };
        if zaman > en_çok {
            break;
        }
        if zaman >= en_az {
            sonuç.push(zaman * birim);
        }
        ay_indeksi = ay_indeksi.saturating_add(ay_adımı);
    }
    sonuç
}

fn logaritmik_bölmeler(aralık: Aralık, taban: f64) -> Vec<f64> {
    if !taban.is_finite() || taban <= 1.0 || aralık.en_az <= 0.0 {
        return Vec::new();
    }
    let ilk = aralık.en_az.log(taban).floor() as i32;
    let son = aralık.en_çok.log(taban).ceil() as i32;
    let çarpanlar: &[f64] = if (taban - 10.0).abs() <= f64::EPSILON {
        &[1.0, 2.0, 5.0]
    } else {
        &[1.0]
    };
    let mut sonuç = Vec::new();
    for üs in ilk..=son {
        let kuvvet = taban.powi(üs);
        for çarpan in çarpanlar {
            let değer = kuvvet * çarpan;
            if değer >= aralık.en_az && değer <= aralık.en_çok {
                sonuç.push(değer);
            }
        }
    }
    sonuç
}

fn logaritmik_otomatik_aralık<'a>(
    değerler: impl Iterator<Item = &'a Option<f64>>,
    taban: f64,
    tam_büyüklükler: bool,
) -> Option<Aralık> {
    if !taban.is_finite() || taban <= 1.0 {
        return None;
    }
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for değer in değerler.flatten().filter(|değer| **değer > 0.0) {
        en_az = en_az.min(*değer);
        en_çok = en_çok.max(*değer);
    }
    if !en_az.is_finite() || !en_çok.is_finite() {
        return None;
    }
    if en_az == en_çok {
        en_az /= taban;
        en_çok *= taban;
    }
    let (alt, üst) = if tam_büyüklükler {
        let alt_üs = en_az.log(taban).floor() as i32;
        let üst_üs = en_çok.log(taban).ceil() as i32;
        (taban.powi(alt_üs), taban.powi(üst_üs))
    } else {
        let alt_adım = taban.powi(en_az.log(taban).floor() as i32);
        let üst_adım = taban.powi(en_çok.log(taban).floor() as i32);
        (
            (en_az / alt_adım).floor() * alt_adım,
            (en_çok / üst_adım).ceil() * üst_adım,
        )
    };
    Aralık::yeni(alt, üst).ok()
}

fn tam_x_aralığı(veri: &HizalıVeri) -> Result<Aralık, UplotHatası> {
    let Some(ilk) = veri.x().first().copied() else {
        return Err(UplotHatası::YetersizVeri { uzunluk: 0 });
    };
    let son = veri.x().last().copied().unwrap_or(ilk);
    if ilk == son {
        Aralık::yeni(ilk - 0.5, son + 0.5)
    } else {
        Aralık::yeni(ilk, son)
    }
}

fn çizilecek_indeksler(
    x: &[f64],
    y: &[Option<f64>],
    aralık: Aralık,
    piksel_genişliği: f32,
) -> Vec<usize> {
    let eşik = (piksel_genişliği.max(1.0) as usize).saturating_mul(4);
    if y.len() <= eşik || y.iter().any(Option::is_none) {
        return (0..y.len()).collect();
    }

    let mut sonuç = Vec::with_capacity(eşik);
    let mut kova = None::<(usize, usize, usize, usize, usize, f64, f64)>;
    for (indeks, (x_değeri, y_değeri)) in x.iter().zip(y.iter()).enumerate() {
        if *x_değeri < aralık.en_az || *x_değeri > aralık.en_çok {
            continue;
        }
        let Some(y_değeri) = y_değeri else {
            continue;
        };
        let oran = (*x_değeri - aralık.en_az) / (aralık.en_çok - aralık.en_az);
        let yeni_kova = (oran * f64::from(piksel_genişliği)).floor().max(0.0) as usize;
        match kova.as_mut() {
            Some((kimlik, _ilk, son, en_az_i, en_çok_i, en_az, en_çok)) if *kimlik == yeni_kova =>
            {
                *son = indeks;
                if *y_değeri < *en_az {
                    *en_az = *y_değeri;
                    *en_az_i = indeks;
                }
                if *y_değeri > *en_çok {
                    *en_çok = *y_değeri;
                    *en_çok_i = indeks;
                }
            }
            _ => {
                if let Some((_, ilk, son, en_az_i, en_çok_i, _, _)) = kova.take() {
                    kova_indekslerini_ekle(&mut sonuç, ilk, en_az_i, en_çok_i, son);
                }
                kova = Some((
                    yeni_kova, indeks, indeks, indeks, indeks, *y_değeri, *y_değeri,
                ));
            }
        }
    }
    if let Some((_, ilk, son, en_az_i, en_çok_i, _, _)) = kova {
        kova_indekslerini_ekle(&mut sonuç, ilk, en_az_i, en_çok_i, son);
    }
    sonuç
}

fn kova_indekslerini_ekle(
    sonuç: &mut Vec<usize>,
    ilk: usize,
    en_az: usize,
    en_çok: usize,
    son: usize,
) {
    let mut adaylar = vec![ilk, en_az, en_çok, son];
    adaylar.sort_unstable();
    adaylar.dedup();
    sonuç.extend(adaylar);
}

/// uPlot'un sayısal eksen yaklaşımı gibi görünür aralık ve piksel yoğunluğuna
/// göre 1/2/2.5/5 × 10ⁿ ailesinden uygun artımı seçer.
fn uygun_artım(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> f64 {
    let uzunluk = aralık.en_çok - aralık.en_az;
    if !uzunluk.is_finite() || uzunluk <= 0.0 || !boyut.is_finite() || boyut <= 0.0 {
        return 1.0;
    }
    let hedef = uzunluk * f64::from(en_az_boşluk.max(1.0)) / f64::from(boyut);
    if !hedef.is_finite() || hedef <= 0.0 {
        return 1.0;
    }
    let taban = 10_f64.powf(hedef.log10().floor());
    for çarpan in [1.0_f64, 2.0, 2.5, 5.0, 10.0] {
        let aday = taban * çarpan;
        if aday >= hedef && aday.is_finite() {
            return aday;
        }
    }
    hedef
}

fn sonlu_aralık(değerler: impl Iterator<Item = f64>) -> Option<Aralık> {
    let (en_az, en_çok) = sonlu_sınırlar(değerler)?;
    Aralık::yeni(en_az, en_çok).ok()
}

fn sonlu_sınırlar(değerler: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let mut en_az = f64::INFINITY;
    let mut en_çok = f64::NEG_INFINITY;
    for değer in değerler.filter(|değer| değer.is_finite()) {
        en_az = en_az.min(değer);
        en_çok = en_çok.max(değer);
    }
    (en_az.is_finite() && en_çok.is_finite()).then_some((en_az, en_çok))
}

fn güzel_sayı(fark: f64, yuvarla: bool) -> Option<f64> {
    if !fark.is_finite() || fark <= 0.0 {
        return None;
    }
    let üs = fark.log10().floor();
    let kuvvet = 10_f64.powf(üs);
    if !kuvvet.is_finite() || kuvvet <= 0.0 {
        return None;
    }
    let kesir = fark / kuvvet;
    let güzel_kesir = if yuvarla {
        if kesir < 1.5 {
            1.0
        } else if kesir < 3.0 {
            if kesir > 2.25 { 2.5 } else { 2.0 }
        } else if kesir < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if kesir <= 1.0 {
        1.0
    } else if kesir <= 2.0 {
        2.0
    } else if kesir <= 5.0 {
        5.0
    } else {
        10.0
    };
    let sonuç = güzel_kesir * kuvvet;
    (sonuç.is_finite() && sonuç > 0.0).then_some(sonuç)
}

fn güzel_ölçek(
    veri_aralığı: Aralık, boyut: f32, en_az_boşluk: f32
) -> Option<(Aralık, f64)> {
    if !boyut.is_finite() || boyut <= 0.0 || !en_az_boşluk.is_finite() || en_az_boşluk <= 0.0 {
        return None;
    }
    let en_az = veri_aralığı.en_az
        * if veri_aralığı.en_az < 0.0 {
            1.02
        } else if veri_aralığı.en_az > 0.0 {
            0.98
        } else {
            1.0
        };
    let en_çok = veri_aralığı.en_çok
        * if veri_aralığı.en_çok < 0.0 {
            0.98
        } else if veri_aralığı.en_çok > 0.0 {
            1.02
        } else {
            1.0
        };
    let en_fazla_etiket = (boyut / en_az_boşluk).floor().clamp(2.0, 10_000.0) as u32;
    let güzel_aralık = güzel_sayı(en_çok - en_az, false)?;
    let artım = güzel_sayı(güzel_aralık / f64::from(en_fazla_etiket - 1), true)?;
    let alt = artıma_yuvarla((en_az / artım).floor() * artım, artım);
    let üst = artıma_yuvarla((en_çok / artım).ceil() * artım, artım);
    Some((Aralık::yeni(alt, üst).ok()?, artım))
}

fn eksen_bölmeleri(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> Vec<f64> {
    let artım = uygun_artım(aralık, boyut, en_az_boşluk);
    eksen_bölmeleri_artımla(aralık, artım)
}

fn eksen_bölmeleri_artımla(aralık: Aralık, artım: f64) -> Vec<f64> {
    if !artım.is_finite() || artım <= 0.0 {
        return Vec::new();
    }
    let tolerans = artım.abs() * 1e-9;
    let mut değer = ((aralık.en_az - tolerans) / artım).ceil() * artım;
    let mut bölmeler = Vec::new();
    for _ in 0..1_000 {
        if değer > aralık.en_çok + tolerans {
            break;
        }
        let yuvarlanmış = artıma_yuvarla(değer, artım);
        bölmeler.push(if yuvarlanmış.abs() <= tolerans {
            0.0
        } else {
            yuvarlanmış
        });
        değer += artım;
    }
    bölmeler
}

fn artıma_yuvarla(değer: f64, artım: f64) -> f64 {
    let basamak = ondalık_basamak(artım);
    let kuvvet = 10_f64.powf(f64::from(basamak));
    (değer * kuvvet).round() / kuvvet
}

fn ondalık_basamak(artım: f64) -> u32 {
    let mut ölçekli = artım.abs();
    for basamak in 0..=12 {
        if (ölçekli - ölçekli.round()).abs() <= 1e-9 {
            return basamak;
        }
        ölçekli *= 10.0;
    }
    12
}

fn eksen_değerini_yaz(değer: f64, artım: f64) -> String {
    let basamak = usize::try_from(ondalık_basamak(artım).max(2)).unwrap_or(12);
    format!("{değer:.basamak$}")
}

fn eksen_değerini_artıma_göre_yaz(değer: f64, artım: f64) -> String {
    let basamak = usize::try_from(ondalık_basamak(artım)).unwrap_or(12);
    let mut sayı = format!("{değer:.basamak$}");
    if sayı.contains('.') {
        while sayı.ends_with('0') {
            sayı.pop();
        }
        if sayı.ends_with('.') {
            sayı.pop();
        }
    }
    sayı
}

fn kompakt_sayı(değer: f64) -> String {
    if !değer.is_finite() {
        return "—".to_string();
    }
    let mut ölçekli = değer;
    let mut sonek = "";
    for (eşik, aday) in [(1e9, "B"), (1e6, "M"), (1e3, "K")] {
        if değer.abs() >= eşik {
            ölçekli = değer / eşik;
            sonek = aday;
            break;
        }
    }
    let basamak = if sonek.is_empty() || ölçekli.abs() >= 100.0 {
        0
    } else if ölçekli.abs() >= 10.0 {
        1
    } else {
        2
    };
    let mut sayı = format!("{ölçekli:.basamak$}");
    if sayı.contains('.') {
        while sayı.ends_with('0') {
            sayı.pop();
        }
        if sayı.ends_with('.') {
            sayı.pop();
        }
    }
    format!("{sayı}{sonek}")
}

fn kısalt(metin: &str, azami_karakter: usize) -> String {
    let mut karakterler = metin.chars();
    let kısa = karakterler
        .by_ref()
        .take(azami_karakter)
        .collect::<String>();
    if karakterler.next().is_some() {
        format!("{kısa}…")
    } else {
        kısa
    }
}

fn ölçek_eksen_değerini_yaz(
    değer: f64,
    artım: f64,
    birim: &str,
    dağılım: Option<YÖlçekDağılımı>,
    biçim: YÖlçekEtiketBiçimi,
) -> String {
    let sayı = match biçim {
        YÖlçekEtiketBiçimi::ArtımaGöre => eksen_değerini_artıma_göre_yaz(değer, artım),
        YÖlçekEtiketBiçimi::Bilimsel => format!("{değer:e}"),
        YÖlçekEtiketBiçimi::İkiliÜs => ikili_üs_etiketi(değer),
        YÖlçekEtiketBiçimi::İkiliŞapka => ikili_şapka_etiketi(değer),
        YÖlçekEtiketBiçimi::Kompakt => kompakt_sayı(değer),
        YÖlçekEtiketBiçimi::Otomatik => match dağılım {
            Some(YÖlçekDağılımı::Logaritmik { taban }) if (taban - 2.0).abs() <= f64::EPSILON => {
                ikili_üs_etiketi(değer)
            }
            Some(YÖlçekDağılımı::Logaritmik { .. }) if değer.abs() >= 1.0 => {
                format!("{değer:.0}")
            }
            Some(YÖlçekDağılımı::Logaritmik { .. } | YÖlçekDağılımı::Weibull) => {
                format!("{değer:e}")
            }
            _ => eksen_değerini_yaz(değer, artım),
        },
    };
    if birim.is_empty() {
        sayı
    } else {
        format!("{sayı} {birim}")
    }
}

/// Resmî geniş log demolarında 1/2/5 ızgarası korunurken bilimsel eksen
/// metinleri yalnız tam taban kuvvetlerine yazılır.
fn log_etiketi_göster(
    değer: f64,
    aralık: Aralık,
    boyut: f32,
    dağılım: Option<YÖlçekDağılımı>,
    biçim: YÖlçekEtiketBiçimi,
) -> bool {
    let Some(YÖlçekDağılımı::Logaritmik { taban }) = dağılım else {
        return true;
    };
    if !matches!(
        biçim,
        YÖlçekEtiketBiçimi::Bilimsel
            | YÖlçekEtiketBiçimi::İkiliÜs
            | YÖlçekEtiketBiçimi::İkiliŞapka
    ) || değer <= 0.0
        || aralık.en_az <= 0.0
        || !boyut.is_finite()
        || boyut <= 0.0
    {
        return true;
    }
    let üs = değer.log(taban);
    if !üs.is_finite() || (üs - üs.round()).abs() > 1e-9 {
        return false;
    }
    let en_az_üs = aralık.en_az.log(taban).floor() as i32;
    let en_çok_üs = aralık.en_çok.log(taban).ceil() as i32;
    let üs = üs.round() as i32;
    let açıklık = en_çok_üs.saturating_sub(en_az_üs).max(1);
    let adım = (f64::from(açıklık) * 22.0 / f64::from(boyut))
        .ceil()
        .max(1.0) as i32;
    en_çok_üs.saturating_sub(üs).rem_euclid(adım) == 0
}

fn ikili_üs_etiketi(değer: f64) -> String {
    if !değer.is_finite() || değer <= 0.0 {
        return "—".to_string();
    }
    let üs = değer.log2().round() as i32;
    let mut sonuç = String::from("2");
    if üs < 0 {
        sonuç.push('⁻');
    }
    for rakam in üs.unsigned_abs().to_string().bytes() {
        sonuç.push(match rakam {
            b'0' => '⁰',
            b'1' => '¹',
            b'2' => '²',
            b'3' => '³',
            b'4' => '⁴',
            b'5' => '⁵',
            b'6' => '⁶',
            b'7' => '⁷',
            b'8' => '⁸',
            b'9' => '⁹',
            _ => '�',
        });
    }
    sonuç
}

fn ikili_şapka_etiketi(değer: f64) -> String {
    if !değer.is_finite() || değer <= 0.0 {
        return "—".to_string();
    }
    format!("2^{}", değer.log2().round() as i32)
}

fn renk_rgb(renk: &str) -> Option<(u8, u8, u8)> {
    let ham = renk.strip_prefix('#')?;
    if ham.len() != 6 {
        return None;
    }
    let kırmızı = u8::from_str_radix(ham.get(0..2)?, 16).ok()?;
    let yeşil = u8::from_str_radix(ham.get(2..4)?, 16).ok()?;
    let mavi = u8::from_str_radix(ham.get(4..6)?, 16).ok()?;
    Some((kırmızı, yeşil, mavi))
}

fn renkler_arası(üst: &str, alt: &str, oran: f32) -> String {
    let Some((üst_r, üst_g, üst_b)) = renk_rgb(üst) else {
        return üst.to_string();
    };
    let Some((alt_r, alt_g, alt_b)) = renk_rgb(alt) else {
        return üst.to_string();
    };
    let oran = oran.clamp(0.0, 1.0);
    let karıştır = |başlangıç: u8, bitiş: u8| {
        (f32::from(başlangıç) + (f32::from(bitiş) - f32::from(başlangıç)) * oran)
            .round()
            .clamp(0.0, 255.0) as u8
    };
    format!(
        "#{:02x}{:02x}{:02x}",
        karıştır(üst_r, alt_r),
        karıştır(üst_g, alt_g),
        karıştır(üst_b, alt_b)
    )
}

fn renk_alfa(renk: &str, alfa: u8) -> String {
    renk_rgb(renk).map_or_else(
        || renk.to_string(),
        |(r, g, b)| format!("#{r:02x}{g:02x}{b:02x}{alfa:02x}"),
    )
}

fn renk_opaklığı(renk: &str, opaklık: f32) -> String {
    let Some(ham) = renk.strip_prefix('#') else {
        return renk.to_string();
    };
    let temel = match ham.len() {
        6 => ham,
        8 => ham.get(0..6).unwrap_or(ham),
        _ => return renk.to_string(),
    };
    let mevcut = if ham.len() == 8 {
        ham.get(6..8)
            .and_then(|değer| u8::from_str_radix(değer, 16).ok())
            .unwrap_or(255)
    } else {
        255
    };
    let alfa = (f32::from(mevcut) * opaklık.clamp(0.0, 1.0)).round() as u8;
    format!("#{temel}{alfa:02x}")
}

fn odaklı_seri_stili(
    seri: &crate::SeriSeçenekleri,
    düzen: Option<crate::OdakDüzeni>,
    odak: Option<usize>,
    seri_indeksi: usize,
) -> (String, Option<String>, f32) {
    let (Some(düzen), Some(odak)) = (düzen, odak) else {
        return (seri.renk.clone(), seri.dolgu.clone(), seri.çizgi_kalınlığı);
    };
    let odaklı = odak == seri_indeksi;
    let kalınlık = if odaklı {
        düzen.odak_kalınlığı.unwrap_or(seri.çizgi_kalınlığı)
    } else {
        seri.çizgi_kalınlığı
    };
    match düzen.stil {
        crate::OdakStili::Opaklık if !odaklı => (
            renk_opaklığı(&seri.renk, düzen.alfa),
            seri.dolgu
                .as_ref()
                .map(|renk| renk_opaklığı(renk, düzen.alfa)),
            kalınlık,
        ),
        crate::OdakStili::OdakDışıSiyah if !odaklı => (
            "#000000".to_string(),
            seri.dolgu.as_ref().map(|_| "#0000001a".to_string()),
            kalınlık,
        ),
        crate::OdakStili::OdaklıMacenta if odaklı => {
            ("#ff00ff".to_string(), seri.dolgu.clone(), kalınlık)
        }
        _ => (seri.renk.clone(), seri.dolgu.clone(), kalınlık),
    }
}

fn medyan(sıralı: &[f64]) -> Option<f64> {
    let sağ = sıralı.get(sıralı.len() / 2).copied()?;
    let sol = sıralı.get(sıralı.len().saturating_sub(1) / 2).copied()?;
    Some((sol + sağ) / 2.0)
}

fn yıldız_çokgeni(merkez: Nokta, uçlar: usize, dış: f32, iç: f32) -> Vec<Nokta> {
    let nokta_sayısı = uçlar.saturating_mul(2);
    let mut noktalar = Vec::with_capacity(nokta_sayısı);
    for indeks in 0..nokta_sayısı {
        let açı = -std::f32::consts::FRAC_PI_2
            + indeks as f32 * std::f32::consts::PI / uçlar.max(1) as f32;
        let yarıçap = if indeks.is_multiple_of(2) { dış } else { iç };
        noktalar.push(Nokta::yeni(
            merkez.x + açı.cos() * yarıçap,
            merkez.y + açı.sin() * yarıçap,
        ));
    }
    noktalar
}

fn daire_çokgeni(merkez: Nokta, yarıçap: f32, parça: usize) -> Vec<Nokta> {
    let parça = parça.max(8);
    (0..parça)
        .map(|indeks| {
            let açı = std::f32::consts::TAU * indeks as f32 / parça as f32;
            Nokta::yeni(
                merkez.x + yarıçap * açı.cos(),
                merkez.y + yarıçap * açı.sin(),
            )
        })
        .collect()
}

fn piksele_hizala(değer: f32, adım: f32) -> f32 {
    if adım > 0.0 && adım.is_finite() && değer.is_finite() {
        (değer / adım).round() * adım
    } else {
        değer
    }
}

fn tekil_değer_mi(değerler: &[Option<f64>], indeks: usize) -> bool {
    değerler.get(indeks).is_some_and(Option::is_some)
        && indeks
            .checked_sub(1)
            .and_then(|önceki| değerler.get(önceki))
            .is_none_or(Option::is_none)
        && değerler
            .get(indeks.saturating_add(1))
            .is_none_or(Option::is_none)
}

#[cfg(test)]
mod eksen_testleri {
    use super::*;

    #[test]
    fn bölmeler_sıfıra_hizalanır_ve_yakınlaştıkça_ondalık_detayı_artırır() {
        let tam = Aralık {
            en_az: -1.2,
            en_çok: 1.2,
        };
        let yakın = Aralık {
            en_az: -0.011,
            en_çok: 0.013,
        };
        let tam_artım = uygun_artım(tam, 304.0, 30.0);
        let yakın_artım = uygun_artım(yakın, 304.0, 30.0);
        let yakın_bölmeler = eksen_bölmeleri(yakın, 304.0, 30.0);
        let hizalı_tam_bölmeler = eksen_bölmeleri(tam, 593.0, 30.0);

        assert!(yakın_artım < tam_artım);
        assert!(yakın_bölmeler.contains(&0.0));
        assert!(hizalı_tam_bölmeler.contains(&-1.2));
        assert!(hizalı_tam_bölmeler.contains(&1.2));
        assert_eq!(eksen_değerini_yaz(0.0, yakın_artım), "0.0000");
        assert!(yakın_bölmeler.windows(2).all(|çift| {
            çift
                .first()
                .zip(çift.get(1))
                .is_some_and(|(sol, sağ)| sol < sağ)
        }));
    }

    #[test]
    fn güzel_sayı_kaynak_eşiklerini_korur() {
        assert_eq!(güzel_sayı(225.0, true), Some(200.0));
        assert_eq!(güzel_sayı(226.0, true), Some(250.0));
        assert_eq!(güzel_sayı(300.0, true), Some(500.0));
        assert_eq!(güzel_sayı(700.0, true), Some(1_000.0));
        assert_eq!(güzel_sayı(200.0, false), Some(200.0));
        assert_eq!(güzel_sayı(201.0, false), Some(500.0));
        assert_eq!(güzel_sayı(0.0, false), None);
    }

    #[test]
    fn kompakt_değerler_üç_anlamlı_basamağa_sığar() {
        assert_eq!(kompakt_sayı(99_949.0), "99.9K");
        assert_eq!(kompakt_sayı(-1_250.0), "-1.25K");
        assert_eq!(kompakt_sayı(42.0), "42");
    }

    #[test]
    fn log10_bölmeleri_kaynak_bir_iki_beş_düzenini_korur() {
        let aralık = Aralık::yeni(1.0, 1_000.0);
        let Ok(aralık) = aralık else { return };
        assert_eq!(
            logaritmik_bölmeler(aralık, 10.0),
            [
                1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1_000.0
            ]
        );
        assert_eq!(
            ölçek_eksen_değerini_yaz(
                50_000.0,
                1.0,
                "",
                Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
                YÖlçekEtiketBiçimi::Otomatik,
            ),
            "50000"
        );
        assert_eq!(
            ölçek_eksen_değerini_yaz(
                2_f64.powi(-10),
                1.0,
                "",
                Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
                YÖlçekEtiketBiçimi::İkiliÜs,
            ),
            "2⁻¹⁰"
        );
        assert!(log_etiketi_göster(
            1e-6,
            Aralık {
                en_az: 1e-6,
                en_çok: 1e8,
            },
            600.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
            YÖlçekEtiketBiçimi::Bilimsel,
        ));
        assert!(!log_etiketi_göster(
            2e-6,
            Aralık {
                en_az: 1e-6,
                en_çok: 1e8,
            },
            600.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 10.0 }),
            YÖlçekEtiketBiçimi::Bilimsel,
        ));
        assert!(log_etiketi_göster(
            2_f64.powi(20),
            Aralık {
                en_az: 2_f64.powi(-10),
                en_çok: 2_f64.powi(20),
            },
            204.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
            YÖlçekEtiketBiçimi::İkiliŞapka,
        ));
        assert!(!log_etiketi_göster(
            2_f64.powi(18),
            Aralık {
                en_az: 2_f64.powi(-10),
                en_çok: 2_f64.powi(20),
            },
            204.0,
            Some(YÖlçekDağılımı::Logaritmik { taban: 2.0 }),
            YÖlçekEtiketBiçimi::İkiliŞapka,
        ));
    }

    #[test]
    fn zaman_bölmeleri_saat_sınırlarına_hizalanır() {
        let aralık = Aralık::yeni(1_594_953_046.0, 1_595_039_415.0);
        let Ok(aralık) = aralık else { return };
        let (bölmeler, adım) = zaman_bölmeleri(aralık, 1_400.0, 50.0, false);
        assert_eq!(adım, 3_600.0);
        assert!(
            bölmeler
                .iter()
                .all(|değer| (*değer % 3_600.0).abs() <= f64::EPSILON)
        );
    }

    #[test]
    fn aylık_zaman_bölmeleri_gerçek_takvim_sınırlarına_hizalanır() {
        let aralık = Aralık::yeni(1_483_228_800.0, 1_575_158_400.0);
        let Ok(aralık) = aralık else { return };
        let (bölmeler, adım) = zaman_bölmeleri(aralık, 1_850.0, 50.0, false);
        assert_eq!(adım, 2_592_000.0);
        assert!(!bölmeler.is_empty());
        assert!(bölmeler.iter().all(|değer| {
            crate::zaman::utc_alanları(*değer).is_some_and(|(_, _, gün, saat, dakika, saniye)| {
                gün == 1 && saat == 0 && dakika == 0 && saniye == 0
            })
        }));
    }
}
