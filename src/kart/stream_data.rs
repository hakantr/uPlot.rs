use std::sync::Arc;

use super::ortak_kart_etkileşimleri;
use crate::{
    Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

#[path = "veri/stream_data.rs"]
pub(crate) mod kaynak_veri;

use kaynak_veri::{SATIR_SAYISI, StreamKaynakVerisi, stream_kaynak_verisi};

pub const STREAM_DATA_KART_TANIM_ÖRNEĞİ: &str = r##"let mut grup = StreamDataGrubu::yeni()?;
let grafikler = grup.kartları()?
    .into_iter()
    .map(|(örnek, seçenekler, veri)| Grafik::yeni(seçenekler, veri))
    .collect::<Result<Vec<_>, _>>()?;
if grup.ilerlet() {
    for ((_, _, yeni_veri), mut grafik) in grup.kartları()?.into_iter().zip(grafikler) {
        grafik.canlı_veriyi_ayarla(yeni_veri)?;
    }
}
// Üç kaynak yüzeyi aynı 100 ms tikinde ilerler ve tek decode edilmiş kaynağı paylaşır.
"##;

pub const STREAM_DATA_ARALIK_MS: u64 = 100;
pub const STREAM_DATA_ADIMI: usize = 10;
pub const STREAM_DATA_PENCERESİ: usize = 3_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamDataÖrneği {
    SabitUzunluk,
    ArtanUzunluk,
    SabitXArtanUzunluk,
}

impl StreamDataÖrneği {
    pub const TÜMÜ: [Self; 3] = [
        Self::SabitUzunluk,
        Self::ArtanUzunluk,
        Self::SabitXArtanUzunluk,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::SabitUzunluk => "stream-data-fixed-sliding",
            Self::ArtanUzunluk => "stream-data-increasing",
            Self::SabitXArtanUzunluk => "stream-data-increasing-static-x",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::SabitUzunluk => "Fixed length / sliding data slices",
            Self::ArtanUzunluk => "Increasing length data",
            Self::SabitXArtanUzunluk => "Increasing length data, static x scale",
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

pub struct StreamDataAkışı {
    örnek: StreamDataÖrneği,
    kaynak: Arc<StreamKaynakVerisi>,
    başlangıç: usize,
    uzunluk: usize,
}

impl StreamDataAkışı {
    pub fn yeni(örnek: StreamDataÖrneği) -> Result<Self, UplotHatası> {
        let uzunluk = if örnek == StreamDataÖrneği::SabitXArtanUzunluk {
            STREAM_DATA_ADIMI
        } else {
            STREAM_DATA_PENCERESİ
        };
        Ok(Self {
            örnek,
            kaynak: stream_kaynak_verisi()?,
            başlangıç: 0,
            uzunluk,
        })
    }

    pub const fn örnek(&self) -> StreamDataÖrneği {
        self.örnek
    }

    pub const fn başlangıç(&self) -> usize {
        self.başlangıç
    }

    pub const fn uzunluk(&self) -> usize {
        self.uzunluk
    }

    pub fn ilerlet(&mut self) -> bool {
        match self.örnek {
            StreamDataÖrneği::SabitUzunluk => {
                let azami_başlangıç = SATIR_SAYISI.saturating_sub(self.uzunluk);
                let yeni = self
                    .başlangıç
                    .saturating_add(STREAM_DATA_ADIMI)
                    .min(azami_başlangıç);
                let değişti = yeni != self.başlangıç;
                self.başlangıç = yeni;
                değişti
            }
            StreamDataÖrneği::ArtanUzunluk | StreamDataÖrneği::SabitXArtanUzunluk => {
                let yeni = self
                    .uzunluk
                    .saturating_add(STREAM_DATA_ADIMI)
                    .min(SATIR_SAYISI);
                let değişti = yeni != self.uzunluk;
                self.uzunluk = yeni;
                değişti
            }
        }
    }

    pub fn kartı(&self) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
        let bitiş = self
            .başlangıç
            .checked_add(self.uzunluk)
            .map(|bitiş| bitiş.min(SATIR_SAYISI))
            .ok_or(UplotHatası::GeçersizKaynakVeri {
                varlık: "demos/stream-data.html",
                açıklama: "akış dilimi bitişi taştı".to_string(),
            })?;
        let x = dilim(&self.kaynak.x, self.başlangıç, bitiş)?;
        let cpu = dilim(&self.kaynak.cpu, self.başlangıç, bitiş)?;
        let ram = dilim(&self.kaynak.ram, self.başlangıç, bitiş)?;
        let tcp_out = dilim(&self.kaynak.tcp_out, self.başlangıç, bitiş)?;
        let veri = HizalıVeri::yeni(x, vec![cpu, ram, tcp_out])?;
        let etkileşimler = ortak_kart_etkileşimleri().seçim_yakınlaştır(false);
        let mut seçenekler = GrafikSeçenekleri::yeni(1_600, 600)?
            .başlık(self.örnek.başlık())
            .birincil_y_ölçeği("%")
            .y_ölçeği(
                YÖlçekSeçenekleri::yeni("%")
                    .birim("%")
                    .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
            )
            .y_ölçeği(
                YÖlçekSeçenekleri::yeni("mb")
                    .sağda(true)
                    .eksen(true)
                    .ızgara(false)
                    .birim(" MB")
                    .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
            )
            .etkileşimler(etkileşimler)
            .seri(SeriSeçenekleri::yeni("CPU").ölçek("%").renk("red"))
            .seri(SeriSeçenekleri::yeni("RAM").ölçek("%").renk("blue"))
            .seri(SeriSeçenekleri::yeni("TCP Out").ölçek("mb").renk("green"));
        if self.örnek == StreamDataÖrneği::SabitXArtanUzunluk {
            seçenekler = seçenekler
                .x_aralığı(Aralık::yeni(1_566_453_600.0, 1_566_813_540.0)?)
                .y_ölçeği(
                    YÖlçekSeçenekleri::yeni("%")
                        .aralık(Aralık::yeni(0.0, 100.0)?)
                        .birim("%")
                        .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
                )
                .y_ölçeği(
                    YÖlçekSeçenekleri::yeni("mb")
                        .aralık(Aralık::yeni(0.0, 70.0)?)
                        .sağda(true)
                        .eksen(true)
                        .ızgara(false)
                        .birim(" MB")
                        .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
                );
        }
        Ok((seçenekler, veri))
    }
}

pub struct StreamDataGrubu {
    akışlar: Vec<StreamDataAkışı>,
}

impl StreamDataGrubu {
    pub fn yeni() -> Result<Self, UplotHatası> {
        let kaynak = stream_kaynak_verisi()?;
        let akışlar = StreamDataÖrneği::TÜMÜ
            .into_iter()
            .map(|örnek| {
                let uzunluk = if örnek == StreamDataÖrneği::SabitXArtanUzunluk {
                    STREAM_DATA_ADIMI
                } else {
                    STREAM_DATA_PENCERESİ
                };
                StreamDataAkışı {
                    örnek,
                    kaynak: Arc::clone(&kaynak),
                    başlangıç: 0,
                    uzunluk,
                }
            })
            .collect();
        Ok(Self { akışlar })
    }

    pub fn kartları(
        &self,
    ) -> Result<Vec<(StreamDataÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
        self.akışlar
            .iter()
            .map(|akış| {
                akış
                    .kartı()
                    .map(|(seçenekler, veri)| (akış.örnek(), seçenekler, veri))
            })
            .collect()
    }

    pub fn ilerlet(&mut self) -> bool {
        let mut değişti = false;
        for akış in &mut self.akışlar {
            değişti = akış.ilerlet() || değişti;
        }
        değişti
    }

    pub fn akış(&self, örnek: StreamDataÖrneği) -> Option<&StreamDataAkışı> {
        self.akışlar.iter().find(|akış| akış.örnek() == örnek)
    }
}

pub fn stream_data_kartları()
-> Result<Vec<(StreamDataÖrneği, GrafikSeçenekleri, HizalıVeri)>, UplotHatası> {
    StreamDataGrubu::yeni()?.kartları()
}

pub fn stream_data_kartı(
    örnek: StreamDataÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    StreamDataAkışı::yeni(örnek)?.kartı()
}

fn dilim<T: Clone>(
    değerler: &[T], başlangıç: usize, bitiş: usize
) -> Result<Vec<T>, UplotHatası> {
    değerler
        .get(başlangıç..bitiş)
        .map(<[T]>::to_vec)
        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
            varlık: "bench/data.json",
            açıklama: format!("geçersiz akış dilimi: {başlangıç}..{bitiş}"),
        })
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::Grafik;

    #[test]
    fn üç_kaynak_yüzeyi_başlangıç_dilimlerini_korur() -> Result<(), UplotHatası> {
        let kartlar = stream_data_kartları()?;
        assert_eq!(kartlar.len(), 3);
        for (örnek, seçenekler, veri) in kartlar {
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), (1_600, 600));
            assert_eq!(veri.seriler().len(), 3);
            let beklenen = if örnek == StreamDataÖrneği::SabitXArtanUzunluk {
                10
            } else {
                3_000
            };
            assert_eq!(veri.uzunluk(), beklenen);
            assert_eq!(veri.x().first().copied(), Some(1_566_453_600.0));
            assert_eq!(
                veri.x().last().copied(),
                Some(if örnek == StreamDataÖrneği::SabitXArtanUzunluk {
                    1_566_454_140.0
                } else {
                    1_566_633_540.0
                })
            );
            assert!(seçenekler.etkileşimler.tekerlek_etkileşimi);
            assert!(!seçenekler.etkileşimler.seçim_yakınlaştır);
            let grafik = Grafik::yeni(seçenekler, veri)?;
            let (son_x, son_değerler) =
                grafik
                    .son_değerler()
                    .ok_or(UplotHatası::GeçersizKaynakVeri {
                        varlık: "demos/stream-data.html",
                        açıklama: "son lejant satırı üretilemedi".to_string(),
                    })?;
            assert!(son_x.is_finite());
            assert_eq!(son_değerler.len(), 3);
            assert!(grafik.çiz().test_svg().contains(örnek.başlık()));
        }
        Ok(())
    }

    #[test]
    fn sabit_pencere_onar_satır_kayarken_uzunluğu_korur() -> Result<(), UplotHatası> {
        let mut akış = StreamDataAkışı::yeni(StreamDataÖrneği::SabitUzunluk)?;
        assert!(akış.ilerlet());
        assert_eq!(akış.başlangıç(), 10);
        assert_eq!(akış.uzunluk(), 3_000);
        let (_, veri) = akış.kartı()?;
        assert_eq!(veri.uzunluk(), 3_000);
        assert_eq!(veri.x().first().copied(), Some(1_566_454_200.0));
        Ok(())
    }

    #[test]
    fn artan_yüzeyler_onar_satır_büyür_ve_sabit_ölçek_korunur() -> Result<(), UplotHatası> {
        let mut artan = StreamDataAkışı::yeni(StreamDataÖrneği::ArtanUzunluk)?;
        assert!(artan.ilerlet());
        assert_eq!(artan.uzunluk(), 3_010);

        let mut sabit = StreamDataAkışı::yeni(StreamDataÖrneği::SabitXArtanUzunluk)?;
        assert!(sabit.ilerlet());
        let (seçenekler, veri) = sabit.kartı()?;
        assert_eq!(veri.uzunluk(), 20);
        assert_eq!(
            seçenekler.x_aralığı,
            Some(Aralık::yeni(1_566_453_600.0, 1_566_813_540.0)?)
        );
        assert_eq!(
            seçenekler
                .y_ölçekleri
                .iter()
                .find(|ölçek| ölçek.anahtar == "%")
                .and_then(|ölçek| ölçek.aralık),
            Some(Aralık::yeni(0.0, 100.0)?)
        );
        assert_eq!(
            seçenekler
                .y_ölçekleri
                .iter()
                .find(|ölçek| ölçek.anahtar == "mb")
                .and_then(|ölçek| ölçek.aralık),
            Some(Aralık::yeni(0.0, 70.0)?)
        );
        Ok(())
    }

    #[test]
    fn grup_üç_yüzeyi_aynı_tikte_ilerletir() -> Result<(), UplotHatası> {
        let mut grup = StreamDataGrubu::yeni()?;
        assert!(
            grup.akışlar
                .windows(2)
                .all(|çift| matches!(çift, [bir, iki] if Arc::ptr_eq(&bir.kaynak, &iki.kaynak)))
        );
        assert!(grup.ilerlet());
        assert_eq!(
            grup.akış(StreamDataÖrneği::SabitUzunluk)
                .map(StreamDataAkışı::başlangıç),
            Some(10)
        );
        assert_eq!(
            grup.akış(StreamDataÖrneği::ArtanUzunluk)
                .map(StreamDataAkışı::uzunluk),
            Some(3_010)
        );
        assert_eq!(
            grup.akış(StreamDataÖrneği::SabitXArtanUzunluk)
                .map(StreamDataAkışı::uzunluk),
            Some(20)
        );
        Ok(())
    }

    #[test]
    fn canlı_set_data_seçenek_ağacını_yeniden_kurmaz() -> Result<(), UplotHatası> {
        let mut akış = StreamDataAkışı::yeni(StreamDataÖrneği::SabitUzunluk)?;
        let (seçenekler, veri) = akış.kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        let seri_seçenekleri = grafik.seri_seçenekleri().as_ptr();
        assert!(akış.ilerlet());
        let (_, yeni_veri) = akış.kartı()?;
        grafik.canlı_veriyi_ayarla(yeni_veri)?;
        assert_eq!(grafik.seri_seçenekleri().as_ptr(), seri_seçenekleri);
        assert_eq!(grafik.son_değerler().map(|(x, _)| x), Some(1_566_634_140.0));
        Ok(())
    }

    #[test]
    fn canlı_set_data_kullanıcının_tekerlek_görünümünü_korur() -> Result<(), UplotHatası> {
        let mut akış = StreamDataAkışı::yeni(StreamDataÖrneği::SabitUzunluk)?;
        let (seçenekler, veri) = akış.kartı()?;
        let mut grafik = Grafik::yeni(seçenekler, veri)?;
        assert!(grafik.tekerlek_eksende(0.5, 0.5, 100.0, true, crate::TekerlekEkseni::X,)?);
        let yakın = grafik.görünür_x_aralığı();
        assert!(akış.ilerlet());
        let (_, yeni_veri) = akış.kartı()?;
        grafik.canlı_veriyi_ayarla(yeni_veri)?;
        assert_eq!(grafik.görünür_x_aralığı(), yakın);
        assert!(grafik.yakınlaştırılmış());
        Ok(())
    }

    #[test]
    fn kaynak_sonu_üç_akışı_sınırlı_biçimde_durdurur() -> Result<(), UplotHatası> {
        let mut grup = StreamDataGrubu::yeni()?;
        let mut tik = 0;
        while grup.ilerlet() {
            tik += 1;
            assert!(tik <= 5_554);
        }
        assert_eq!(tik, 5_554);
        assert!(!grup.ilerlet());
        assert_eq!(
            grup.akış(StreamDataÖrneği::SabitUzunluk)
                .map(StreamDataAkışı::başlangıç),
            Some(52_550)
        );
        assert_eq!(
            grup.akış(StreamDataÖrneği::ArtanUzunluk)
                .map(StreamDataAkışı::uzunluk),
            Some(55_550)
        );
        assert_eq!(
            grup.akış(StreamDataÖrneği::SabitXArtanUzunluk)
                .map(StreamDataAkışı::uzunluk),
            Some(55_550)
        );
        Ok(())
    }
}
