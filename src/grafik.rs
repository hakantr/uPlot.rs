use crate::cizim::kirpma::{
    nokta_dikdörtgende, yolu_dikdörtgene_kırp, çokgeni_dikdörtgene_kırp
};
use crate::cizim::{Komut, MetinHizası, Nokta, Sahne};
use crate::etkilesim::EtkileşimDenetleyicisi;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, UplotHatası, YÖlçekDağılımı};

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
        }
        let mut tam = tam_x_aralığı(&veri)?;
        if (seçenekler.çubuk_düzeni.is_some() || seçenekler.kutu_bıyık_düzeni.is_some())
            && veri.uzunluk() > 1
        {
            tam = Aralık::yeni(tam.en_az - 0.5, tam.en_çok + 0.5)?;
        }
        let mut tam_y = seçenekler
            .y_ölçekleri
            .iter()
            .find(|ölçek| ölçek.anahtar == seçenekler.birincil_y_ölçeği)
            .and_then(|ölçek| ölçek.aralık)
            .or(seçenekler.y_aralığı)
            .unwrap_or_else(|| {
                Aralık::otomatik(
                    veri.seriler()
                        .iter()
                        .zip(seçenekler.seriler.iter())
                        .filter(|(_, ayarlar)| ayarlar.ölçek == seçenekler.birincil_y_ölçeği)
                        .flat_map(|(seri, _)| seri.iter()),
                )
            });
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

    /// Başlık ve eksen payları çıkarıldıktan sonraki gerçek çizim alanını
    /// `(sol, sağ, üst, alt)` olarak döndürür. Yüzey adaptörleri sabit sayı
    /// çoğaltmak yerine bu çekirdek geometrisini kullanır.
    pub fn çizim_alanı_boyutta(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
    ) -> (f32, f32, f32, f32) {
        let genişlik_px = genişlik_px.max(160) as f32;
        let yükseklik_px = yükseklik_px.max(120) as f32;
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
        let mut sağ_pay: f32 = if self.seçenekler.birincil_y_sağda {
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
        let mut sol_pay: f32 = if self.seçenekler.birincil_y_sağda {
            24.0 + sol_eksen_sayısı as f32 * 56.0
        } else {
            64.0 + sol_eksen_sayısı as f32 * 56.0
        };
        if self.seçenekler.otomatik_y_eksen_genişliği && !self.seçenekler.birincil_y_sağda {
            let aralık = self.görünür_y_aralığı();
            let artım = uygun_artım(aralık, yükseklik_px, 30.0);
            let birim = self
                .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
                .map_or("", |ölçek| ölçek.birim.as_str());
            let en_uzun = self
                .y_eksen_bölmeleri(&self.seçenekler.birincil_y_ölçeği, aralık, yükseklik_px)
                .into_iter()
                .map(|değer| {
                    eksen_değerini_birimle_yaz(değer, artım, birim)
                        .chars()
                        .count()
                })
                .max()
                .unwrap_or(1);
            sol_pay = sol_pay.max(24.0 + en_uzun as f32 * 7.0);
        }
        let alt_pay = if self.seçenekler.x_eksen_etiketi.is_empty() {
            48.0
        } else {
            68.0
        };
        (sol_pay, genişlik_px - sağ_pay, 48.0, yükseklik_px - alt_pay)
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

    pub fn kutu_bıyık_vuruşu(
        &self,
        genişlik_px: u32,
        yükseklik_px: u32,
        x: f32,
        y: f32,
    ) -> Option<(usize, Nokta, f32, f32, [f64; 5])> {
        if !self.kutu_bıyık_grafiği() || !x.is_finite() || !y.is_finite() {
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
        self.etkileşim
            .tekerlek(yatay_odak_oranı, dikey_odak_oranı, görünür_y, delta, hassas)
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

    pub fn taşımayı_başlat(&mut self) -> bool {
        let görünür_y = self.görünür_y_aralığı();
        self.etkileşim.taşımayı_başlat(görünür_y)
    }

    pub fn taşı(
        &mut self,
        yatay_fark_oranı: f64,
        dikey_fark_oranı: f64,
    ) -> Result<bool, UplotHatası> {
        self.etkileşim.taşı(yatay_fark_oranı, dikey_fark_oranı)
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
        self.etkileşim
            .dokunma_yakınlaştır(yatay_odak_oranı, dikey_odak_oranı, çarpan)
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

    /// En yakın ortak X indeksini ve o indeksteki tüm seri değerlerini döndürür.
    /// uPlot'un hizalı cursor/legend davranışının çekirdek karşılığıdır.
    pub fn en_yakın_noktalar(&self, yatay_oran: f64) -> Option<(f64, Vec<Option<f64>>)> {
        if !yatay_oran.is_finite() {
            return None;
        }
        let aralık = self.görünür_x_aralığı();
        let hedef = aralık.en_az + yatay_oran.clamp(0.0, 1.0) * (aralık.en_çok - aralık.en_az);
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
            .map(|seri| seri.get(indeks).copied().flatten())
            .collect();
        Some((x, değerler))
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
        let genişlik_px = genişlik_px.max(160);
        let yükseklik_px = yükseklik_px.max(120);
        let mut sahne = Sahne::yeni(genişlik_px, yükseklik_px);
        sahne.ekle(Komut::ArkaPlan {
            renk: "#ffffff".to_string(),
        });

        let (sol, sağ, üst, alt) = self.çizim_alanı_boyutta(genişlik_px, yükseklik_px);
        let genişlik = sağ - sol;
        let yükseklik = alt - üst;

        sahne.ekle(Komut::Metin {
            konum: Nokta::yeni(genişlik_px as f32 / 2.0, 26.0),
            içerik: self.seçenekler.başlık.clone(),
            renk: "#111111".to_string(),
            boyut: 18.0,
            hiza: MetinHizası::Orta,
        });

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

        let tam_x_aralığı = tam_x_aralığı(&self.veri).unwrap_or(Aralık {
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
        let y_aralığı = görünür_y.unwrap_or_else(|| self.y_aralığı(x_aralığı));
        let birincil_birim = self
            .ölçek_seçeneği(&self.seçenekler.birincil_y_ölçeği)
            .map_or("", |ölçek| ölçek.birim.as_str());

        let y_artımı = uygun_artım(y_aralığı, yükseklik, 30.0);
        for y_değeri in
            self.y_eksen_bölmeleri(&self.seçenekler.birincil_y_ölçeği, y_aralığı, yükseklik)
        {
            let y = üst + yükseklik
                - self.y_konumu(
                    &self.seçenekler.birincil_y_ölçeği,
                    y_aralığı,
                    y_değeri,
                    0.0,
                    yükseklik,
                );
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(sol, y),
                bitiş: Nokta::yeni(sağ, y),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(
                    if self.seçenekler.birincil_y_sağda {
                        sağ + 8.0
                    } else {
                        sol - 8.0
                    },
                    y + 4.0,
                ),
                içerik: eksen_değerini_birimle_yaz(y_değeri, y_artımı, birincil_birim),
                renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                boyut: 11.0,
                hiza: if self.seçenekler.birincil_y_sağda {
                    MetinHizası::Başlangıç
                } else {
                    MetinHizası::Bitiş
                },
            });
        }

        if !self.seçenekler.y_eksen_etiketi.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(
                    if self.seçenekler.birincil_y_sağda {
                        sağ
                    } else {
                        sol
                    },
                    üst - 12.0,
                ),
                içerik: self.seçenekler.y_eksen_etiketi.clone(),
                renk: self.seçenekler.birincil_y_eksen_rengi.clone(),
                boyut: 12.0,
                hiza: if self.seçenekler.birincil_y_sağda {
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
                let y = alt - self.y_konumu(&ölçek.anahtar, aralık, değer, 0.0, yükseklik);
                if ölçek.ızgara {
                    sahne.ekle(Komut::Çizgi {
                        başlangıç: Nokta::yeni(sol, y),
                        bitiş: Nokta::yeni(sağ, y),
                        renk: "#e5e7eb".to_string(),
                        kalınlık: 1.0,
                    });
                }
                sahne.ekle(Komut::Metin {
                    konum: Nokta::yeni(eksen_x, y + 4.0),
                    içerik: eksen_değerini_birimle_yaz(değer, artım, &ölçek.birim),
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

        let x_artımı = uygun_artım(x_aralığı, genişlik, 50.0);
        for x_değeri in eksen_bölmeleri(x_aralığı, genişlik, 50.0) {
            let x = x_aralığı.konum(x_değeri, sol, genişlik);
            sahne.ekle(Komut::Çizgi {
                başlangıç: Nokta::yeni(x, üst),
                bitiş: Nokta::yeni(x, alt),
                renk: "#e5e7eb".to_string(),
                kalınlık: 1.0,
            });
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni(x, alt + 20.0),
                içerik: if self.seçenekler.x_zaman {
                    crate::zaman::eksen_etiketi(x_değeri, x_artımı)
                        .unwrap_or_else(|| eksen_değerini_yaz(x_değeri, x_artımı))
                } else {
                    eksen_değerini_yaz(
                        x_değeri * self.seçenekler.x_eksen_değer_çarpanı,
                        x_artımı * self.seçenekler.x_eksen_değer_çarpanı,
                    )
                },
                renk: "#4b5563".to_string(),
                boyut: 11.0,
                hiza: MetinHizası::Orta,
            });
        }

        if !self.seçenekler.x_eksen_etiketi.is_empty() {
            sahne.ekle(Komut::Metin {
                konum: Nokta::yeni((sol + sağ) / 2.0, alt + 42.0),
                içerik: self.seçenekler.x_eksen_etiketi.clone(),
                renk: "#4b5563".to_string(),
                boyut: 12.0,
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
            let seri_y_aralığı =
                self.görünür_ölçek_aralığı(&seri.ölçek, x_aralığı, görünür_y);
            let mut ham_parçalar = Vec::<Vec<Nokta>>::new();
            let mut parça = Vec::<Nokta>::new();
            let mut görünür_noktalar = Vec::<Nokta>::new();
            let mut önceki_x = None::<f64>;
            let çizilecek_indeksler =
                çizilecek_indeksler(self.veri.x(), değerler, x_aralığı, genişlik);
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
                        let x = x_aralığı.konum(*x_değeri, sol, genişlik);
                        let y = alt
                            - self.y_konumu(&seri.ölçek, seri_y_aralığı, *y_değeri, 0.0, yükseklik);
                        let nokta = Nokta::yeni(x, y);
                        parça.push(nokta);
                        önceki_x = Some(*x_değeri);
                        if nokta_dikdörtgende(nokta, sol, sağ, üst, alt) {
                            görünür_noktalar.push(nokta);
                        }
                    }
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
            let parçalar = yolu_dikdörtgene_kırp(&ham_parçalar, sol, sağ, üst, alt);
            if let Some(dolgu) = &seri.dolgu {
                let taban = alt
                    - self.y_konumu(
                        &seri.ölçek,
                        seri_y_aralığı,
                        seri.dolgu_tabanı,
                        0.0,
                        yükseklik,
                    );
                let taban = taban.clamp(üst, alt);
                let çokgenler = ham_parçalar
                    .iter()
                    .filter_map(|parça| {
                        let ilk = parça.first()?;
                        let son = parça.last()?;
                        let mut çokgen = parça.clone();
                        çokgen.push(Nokta::yeni(son.x, taban));
                        çokgen.push(Nokta::yeni(ilk.x, taban));
                        let kırpılmış = çokgeni_dikdörtgene_kırp(&çokgen, sol, sağ, üst, alt);
                        (kırpılmış.len() >= 3).then_some(kırpılmış)
                    })
                    .collect();
                sahne.ekle(Komut::Alan {
                    çokgenler,
                    dolgu: dolgu.clone(),
                });
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
                            .zip(self.seçenekler.seriler.iter())
                            .filter(move |(_, ayarlar)| ayarlar.ölçek == anahtar)
                            .filter_map(move |(seri, _)| seri.get(indeks))
                    });
                Aralık::otomatik(görünür)
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

    fn tam_x_aralığı(&self) -> Option<Aralık> {
        self.veri
            .x()
            .first()
            .zip(self.veri.x().last())
            .and_then(|(ilk, son)| Aralık::yeni(*ilk, *son).ok())
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
        match self.ölçek_seçeneği(anahtar).map(|ölçek| ölçek.dağılım) {
            Some(YÖlçekDağılımı::ArcSinh { eşik }) if eşik.is_finite() && eşik > 0.0 => {
                let dönüştür = |sayı: f64| (sayı / eşik).asinh();
                let en_az = dönüştür(aralık.en_az);
                let en_çok = dönüştür(aralık.en_çok);
                let değer = dönüştür(değer);
                let oran = (değer - en_az) / (en_çok - en_az);
                başlangıç + oran as f32 * uzunluk
            }
            _ => aralık.konum(değer, başlangıç, uzunluk),
        }
    }

    fn y_eksen_bölmeleri(&self, anahtar: &str, aralık: Aralık, boyut: f32) -> Vec<f64> {
        let Some(YÖlçekDağılımı::ArcSinh { eşik }) =
            self.ölçek_seçeneği(anahtar).map(|ölçek| ölçek.dağılım)
        else {
            return eksen_bölmeleri(aralık, boyut, 30.0);
        };
        if !eşik.is_finite() || eşik <= 0.0 {
            return eksen_bölmeleri(aralık, boyut, 30.0);
        }
        let en_az = (aralık.en_az / eşik).asinh();
        let en_çok = (aralık.en_çok / eşik).asinh();
        let adım_sayısı = (boyut / 55.0).round().clamp(3.0, 12.0) as u32;
        (0..=adım_sayısı)
            .map(|indeks| {
                let oran = f64::from(indeks) / f64::from(adım_sayısı);
                eşik * (en_az + (en_çok - en_az) * oran).sinh()
            })
            .collect()
    }
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

fn eksen_bölmeleri(aralık: Aralık, boyut: f32, en_az_boşluk: f32) -> Vec<f64> {
    let artım = uygun_artım(aralık, boyut, en_az_boşluk);
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

fn eksen_değerini_birimle_yaz(değer: f64, artım: f64, birim: &str) -> String {
    let sayı = eksen_değerini_yaz(değer, artım);
    if birim.is_empty() {
        sayı
    } else {
        format!("{sayı} {birim}")
    }
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
    fn kompakt_değerler_üç_anlamlı_basamağa_sığar() {
        assert_eq!(kompakt_sayı(99_949.0), "99.9K");
        assert_eq!(kompakt_sayı(-1_250.0), "-1.25K");
        assert_eq!(kompakt_sayı(42.0), "42");
    }
}
