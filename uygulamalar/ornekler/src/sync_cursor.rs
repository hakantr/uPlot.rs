use super::{ortak_kart_etkileşimleri, stream_data::kaynak_veri::stream_kaynak_verisi};
use crate::{
    GrafikSeçenekleri, HizalıVeri, OdakDüzeni, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const SYNC_CURSOR_KART_TANIM_ÖRNEĞİ: &str = r##"let grup = cx.new(|_| {
    GpuiGrafikGrubu::yeni(GpuiGrafikGrupAyarları::default())
});
for (kimlik, grafik) in cpu_ram_tcp_yüzeyleri {
    grup.update(cx, |grup, cx| {
        grup.grafik_ekle(kimlik, grafik, cx);
    });
}
// Cursor, wheel, seçim, pan, eksen zoomu, tam görünüm ve setSeries
// farklı yüzey boyutlarında normalize oranlarla çekirdekte paylaşılır.
"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncCursorÖrneği {
    Cpu,
    Ram,
    Tcp,
    UyumsuzKırmızıMavi,
    UyumsuzYeşilKırmızı,
}

impl SyncCursorÖrneği {
    pub const TÜMÜ: [Self; 5] = [
        Self::Cpu,
        Self::Ram,
        Self::Tcp,
        Self::UyumsuzKırmızıMavi,
        Self::UyumsuzYeşilKırmızı,
    ];

    pub const fn kimlik(self) -> &'static str {
        match self {
            Self::Cpu => "sync-cursor-cpu",
            Self::Ram => "sync-cursor-ram",
            Self::Tcp => "sync-cursor-tcp",
            Self::UyumsuzKırmızıMavi => "sync-cursor-mismatch-red-blue",
            Self::UyumsuzYeşilKırmızı => "sync-cursor-mismatch-green-red",
        }
    }

    pub const fn başlık(self) -> &'static str {
        match self {
            Self::Cpu => "CPU",
            Self::Ram => "RAM",
            Self::Tcp => "TCP",
            Self::UyumsuzKırmızıMavi | Self::UyumsuzYeşilKırmızı => {
                "Mis-matched series order"
            }
        }
    }

    pub const fn boyut(self) -> (u32, u32) {
        match self {
            Self::Cpu => (1_920, 400),
            Self::Ram | Self::Tcp => (940, 400),
            Self::UyumsuzKırmızıMavi | Self::UyumsuzYeşilKırmızı => (600, 400),
        }
    }

    pub fn kimlikten(kimlik: &str) -> Option<Self> {
        Self::TÜMÜ
            .into_iter()
            .find(|örnek| örnek.kimlik() == kimlik)
    }
}

#[derive(Debug, Clone)]
pub struct SyncCursorGrubu {
    senkron: bool,
    fare_basma_bırakma_senkron: bool,
}

impl Default for SyncCursorGrubu {
    fn default() -> Self {
        Self::yeni()
    }
}

impl SyncCursorGrubu {
    pub const fn yeni() -> Self {
        Self {
            senkron: true,
            fare_basma_bırakma_senkron: true,
        }
    }

    pub const fn senkron(&self) -> bool {
        self.senkron
    }

    pub const fn fare_basma_bırakma_senkron(&self) -> bool {
        self.fare_basma_bırakma_senkron
    }

    pub fn senkronu_ayarla(&mut self, etkin: bool) -> bool {
        let değişti = self.senkron != etkin;
        self.senkron = etkin;
        değişti
    }

    pub fn fare_basma_bırakma_senkronunu_ayarla(&mut self, etkin: bool) -> bool {
        let değişti = self.fare_basma_bırakma_senkron != etkin;
        self.fare_basma_bırakma_senkron = etkin;
        değişti
    }
}

pub fn sync_cursor_kartı(
    örnek: SyncCursorÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    match örnek {
        SyncCursorÖrneği::Cpu | SyncCursorÖrneği::Ram | SyncCursorÖrneği::Tcp => {
            kaynak_kartı(örnek)
        }
        SyncCursorÖrneği::UyumsuzKırmızıMavi => uyumsuz_kartı(örnek, ["red", "blue"]),
        SyncCursorÖrneği::UyumsuzYeşilKırmızı => uyumsuz_kartı(örnek, ["green", "red"]),
    }
}

fn kaynak_kartı(
    örnek: SyncCursorÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let kaynak = stream_kaynak_verisi()?;
    let x = dilim(&kaynak.x, 0, 1_000)?;
    let ham = match örnek {
        SyncCursorÖrneği::Cpu => &kaynak.cpu,
        SyncCursorÖrneği::Ram => &kaynak.ram,
        SyncCursorÖrneği::Tcp => &kaynak.tcp_out,
        SyncCursorÖrneği::UyumsuzKırmızıMavi | SyncCursorÖrneği::UyumsuzYeşilKırmızı => {
            return Err(UplotHatası::GeçersizKaynakVeri {
                varlık: "demos/sync-cursor.html",
                açıklama: "uyumsuz seri yüzeyi kaynak veri dilimine gönderildi".to_string(),
            });
        }
    };
    let seriler = vec![
        dilim(ham, 0, 1_000)?,
        dilim(ham, 1_000, 2_000)?,
        dilim(ham, 2_000, 3_000)?,
    ];
    let birim = if örnek == SyncCursorÖrneği::Tcp {
        " MB"
    } else {
        "%"
    };
    let ölçek = if örnek == SyncCursorÖrneği::Tcp {
        "mb"
    } else {
        "y"
    };
    let (genişlik, yükseklik) = örnek.boyut();
    let renkler = ["red", "green", "blue"];
    let mut seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .odak(OdakDüzeni::yeni(0.3, 16.0))
        .etkileşimler(ortak_kart_etkileşimleri())
        .y_ölçeği(
            YÖlçekSeçenekleri::yeni(ölçek)
                .birim(birim)
                .etiket_biçimi(YÖlçekEtiketBiçimi::ArtımaGöre),
        );
    for (indeks, renk) in renkler.into_iter().enumerate() {
        seçenekler = seçenekler.seri(
            SeriSeçenekleri::yeni(format!("{} {}", örnek.başlık(), indeks + 1))
                .ölçek(ölçek)
                .renk(renk),
        );
    }
    Ok((seçenekler, HizalıVeri::yeni(x, seriler)?))
}

fn uyumsuz_kartı(
    örnek: SyncCursorÖrneği,
    etiketler: [&str; 2],
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let (genişlik, yükseklik) = örnek.boyut();
    let seçenekler = GrafikSeçenekleri::yeni(genişlik, yükseklik)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni(etiketler[0]).renk(etiketler[0]))
        .seri(SeriSeçenekleri::yeni(etiketler[1]).renk(etiketler[1]));
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0],
        vec![vec![Some(0.0), Some(5.0)], vec![Some(5.0), Some(0.0)]],
    )?;
    Ok((seçenekler, veri))
}

fn dilim<T: Clone>(
    değerler: &[T], başlangıç: usize, bitiş: usize
) -> Result<Vec<T>, UplotHatası> {
    değerler
        .get(başlangıç..bitiş)
        .map(<[T]>::to_vec)
        .ok_or_else(|| UplotHatası::GeçersizKaynakVeri {
            varlık: "bench/data.json",
            açıklama: format!("geçersiz Sync Cursor dilimi: {başlangıç}..{bitiş}"),
        })
}

#[cfg(test)]
mod testler {
    use super::*;
    use crate::{Grafik, Komut};

    #[test]
    fn beş_kaynak_yüzeyi_boyut_veri_ve_serileri_korur() -> Result<(), UplotHatası> {
        for örnek in SyncCursorÖrneği::TÜMÜ {
            let (seçenekler, veri) = sync_cursor_kartı(örnek)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), örnek.boyut());
            let beklenen = if matches!(
                örnek,
                SyncCursorÖrneği::Cpu | SyncCursorÖrneği::Ram | SyncCursorÖrneği::Tcp
            ) {
                1_000
            } else {
                2
            };
            assert_eq!(veri.uzunluk(), beklenen);
            assert!(Grafik::yeni(seçenekler, veri)?.çiz().komutlar().iter().any(
                |komut| matches!(komut, Komut::Metin { içerik, .. } if içerik == örnek.başlık())
            ));
        }
        Ok(())
    }

    #[test]
    fn demo_senkron_kontrolleri_varsayılan_açık_ve_değiştirilebilirdir() {
        let mut grup = SyncCursorGrubu::yeni();
        assert!(grup.senkron());
        assert!(grup.fare_basma_bırakma_senkron());
        assert!(grup.senkronu_ayarla(false));
        assert!(grup.fare_basma_bırakma_senkronunu_ayarla(false));
        assert!(!grup.senkron());
        assert!(!grup.fare_basma_bırakma_senkron());
    }

    #[test]
    fn kaynak_verisinin_zaman_ve_değer_aralıkları_korunur() -> Result<(), UplotHatası> {
        let beklentiler = [
            (SyncCursorÖrneği::Cpu, (0.05, 31.99)),
            (SyncCursorÖrneği::Ram, (11.99, 22.44)),
            (SyncCursorÖrneği::Tcp, (0.0, 59.93)),
        ];
        for (örnek, (beklenen_en_az, beklenen_en_çok)) in beklentiler {
            let (_, veri) = sync_cursor_kartı(örnek)?;
            assert_eq!(veri.x().first(), Some(&1_566_453_600.0));
            assert_eq!(veri.x().last(), Some(&1_566_513_540.0));
            let değerler = veri
                .seriler()
                .iter()
                .flat_map(|seri| seri.iter().flatten().copied())
                .collect::<Vec<_>>();
            let en_az = değerler.iter().copied().fold(f64::INFINITY, f64::min);
            let en_çok = değerler.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            assert_eq!((en_az, en_çok), (beklenen_en_az, beklenen_en_çok));
        }
        Ok(())
    }

    #[test]
    fn seçim_penceresi_hedeflere_oransal_olarak_taşınır() -> Result<(), UplotHatası> {
        let (kaynak_seçenekleri, kaynak_verisi) = sync_cursor_kartı(SyncCursorÖrneği::Cpu)?;
        let (hedef_seçenekleri, hedef_verisi) = sync_cursor_kartı(SyncCursorÖrneği::Ram)?;
        let (tcp_seçenekleri, tcp_verisi) = sync_cursor_kartı(SyncCursorÖrneği::Tcp)?;
        let mut kaynak = Grafik::yeni(kaynak_seçenekleri, kaynak_verisi)?;
        let mut hedef = Grafik::yeni(hedef_seçenekleri, hedef_verisi)?;
        let mut tcp = Grafik::yeni(tcp_seçenekleri, tcp_verisi)?;
        assert_eq!(
            kaynak.seçimi_bitir(0.25, 0.75, false)?,
            crate::SeçimEylemi::Yakınlaştırıldı
        );
        let görünüm = kaynak.oransal_görünüm();
        assert!(hedef.oransal_görünümü_ayarla(görünüm, true)?);
        assert!(tcp.oransal_görünümü_ayarla(görünüm, true)?);
        assert_eq!(hedef.oransal_görünüm(), görünüm);
        assert_eq!(tcp.oransal_görünüm(), görünüm);
        Ok(())
    }

    #[test]
    fn ikinci_grup_x_seçimini_paylaşır_ama_ilk_grup_kapatılınca_bile_bağımsız_kalır()
    -> Result<(), UplotHatası> {
        let mut grup = SyncCursorGrubu::yeni();
        assert!(grup.senkronu_ayarla(false));

        let (a_seçenekleri, a_verisi) = sync_cursor_kartı(SyncCursorÖrneği::UyumsuzKırmızıMavi)?;
        let (b_seçenekleri, b_verisi) = sync_cursor_kartı(SyncCursorÖrneği::UyumsuzYeşilKırmızı)?;
        let mut a = Grafik::yeni(a_seçenekleri, a_verisi)?;
        let mut b = Grafik::yeni(b_seçenekleri, b_verisi)?;
        assert_eq!(
            a.seçimi_bitir(0.2, 0.8, false)?,
            crate::SeçimEylemi::Yakınlaştırıldı
        );
        assert!(b.oransal_görünümü_ayarla(a.oransal_görünüm(), true)?);
        assert_eq!(a.oransal_görünüm(), b.oransal_görünüm());
        Ok(())
    }
}
