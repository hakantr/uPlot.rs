use super::{ortak_kart_etkileşimleri, stream_data::kaynak_veri::stream_kaynak_verisi};
use crate::{
    GrafikSeçenekleri, HizalıVeri, OdakDüzeni, SeriSeçenekleri, UplotHatası, YÖlçekEtiketBiçimi,
    YÖlçekSeçenekleri,
};

pub const SYNC_CURSOR_KART_TANIM_ÖRNEĞİ: &str = r##"let mut grup = SyncCursorGrubu::yeni();
let yüzeyler = SyncCursorÖrneği::TÜMÜ.map(sync_cursor_kartı);
let hedefler = grup.imleç_hedefleri(SyncCursorÖrneği::Cpu);
// Fare hareketi, seri odağı ve cursor kilidi eşlemesi çekirdektedir.
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

    const fn grup(self) -> u8 {
        match self {
            Self::Cpu | Self::Ram | Self::Tcp => 0,
            Self::UyumsuzKırmızıMavi | Self::UyumsuzYeşilKırmızı => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncCursorGrubu {
    senkron: bool,
    fare_basma_bırakma_senkron: bool,
    kilitli: [bool; 5],
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
            kilitli: [false; 5],
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

    pub fn imleç_hedefleri(&self, kaynak: SyncCursorÖrneği) -> Vec<SyncCursorÖrneği> {
        SyncCursorÖrneği::TÜMÜ
            .into_iter()
            .filter(|hedef| {
                *hedef != kaynak
                    && hedef.grup() == kaynak.grup()
                    && (kaynak.grup() != 0 || self.senkron)
            })
            .collect()
    }

    pub fn seri_hedefi(
        &self,
        kaynak: SyncCursorÖrneği,
        hedef: SyncCursorÖrneği,
        kaynak_seri_indeksi: usize,
    ) -> Option<usize> {
        if kaynak.grup() != hedef.grup()
            || (kaynak.grup() == 0 && !self.senkron)
            || kaynak_seri_indeksi >= 3
        {
            return None;
        }
        if kaynak.grup() == 0 {
            return Some(kaynak_seri_indeksi);
        }
        let kaynak_etiketleri = seri_etiketleri(kaynak);
        let kaynak_etiketi = kaynak_etiketleri.get(kaynak_seri_indeksi)?;
        seri_etiketleri(hedef)
            .iter()
            .position(|etiket| etiket == kaynak_etiketi)
    }

    /// Kaynak uPlot davranışındaki scale-key eşleşmesini korur. CPU ve RAM
    /// aynı `y` ölçeğini paylaşırken TCP'nin `mb` ölçeği yalnızca yatay
    /// imleci senkronlar.
    pub const fn dikey_imleç_senkron_mu(
        &self,
        kaynak: SyncCursorÖrneği,
        hedef: SyncCursorÖrneği,
    ) -> bool {
        if kaynak.grup() != hedef.grup() {
            return false;
        }
        if kaynak.grup() == 1 {
            return true;
        }
        matches!(kaynak, SyncCursorÖrneği::Tcp) == matches!(hedef, SyncCursorÖrneği::Tcp)
    }

    pub fn fare_bırak(&mut self, kaynak: SyncCursorÖrneği) -> Vec<(SyncCursorÖrneği, bool)> {
        let kaynak_indeksi = örnek_indeksi(kaynak);
        if let Some(kilitli) = self.kilitli.get_mut(kaynak_indeksi) {
            *kilitli = !*kilitli;
        }
        let mut değişenler = vec![(
            kaynak,
            self.kilitli.get(kaynak_indeksi).copied().unwrap_or(false),
        )];
        if kaynak.grup() == 0 && (!self.senkron || !self.fare_basma_bırakma_senkron) {
            return değişenler;
        }
        for hedef in self.imleç_hedefleri(kaynak) {
            let indeks = örnek_indeksi(hedef);
            if let Some(kilitli) = self.kilitli.get_mut(indeks) {
                *kilitli = !*kilitli;
                değişenler.push((hedef, *kilitli));
            }
        }
        değişenler
    }

    pub fn kilitli(&self, örnek: SyncCursorÖrneği) -> bool {
        self.kilitli
            .get(örnek_indeksi(örnek))
            .copied()
            .unwrap_or(false)
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
        .odak(OdakDüzeni::yeni(0.3, 16.0))
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni(etiketler[0]).renk(etiketler[0]))
        .seri(SeriSeçenekleri::yeni(etiketler[1]).renk(etiketler[1]));
    let veri = HizalıVeri::yeni(
        vec![0.0, 1.0],
        vec![vec![Some(0.0), Some(5.0)], vec![Some(5.0), Some(0.0)]],
    )?;
    Ok((seçenekler, veri))
}

fn seri_etiketleri(örnek: SyncCursorÖrneği) -> [&'static str; 3] {
    match örnek {
        SyncCursorÖrneği::Cpu => ["CPU 1", "CPU 2", "CPU 3"],
        SyncCursorÖrneği::Ram => ["RAM 1", "RAM 2", "RAM 3"],
        SyncCursorÖrneği::Tcp => ["TCP 1", "TCP 2", "TCP 3"],
        SyncCursorÖrneği::UyumsuzKırmızıMavi => ["red", "blue", ""],
        SyncCursorÖrneği::UyumsuzYeşilKırmızı => ["green", "red", ""],
    }
}

const fn örnek_indeksi(örnek: SyncCursorÖrneği) -> usize {
    match örnek {
        SyncCursorÖrneği::Cpu => 0,
        SyncCursorÖrneği::Ram => 1,
        SyncCursorÖrneği::Tcp => 2,
        SyncCursorÖrneği::UyumsuzKırmızıMavi => 3,
        SyncCursorÖrneği::UyumsuzYeşilKırmızı => 4,
    }
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
    use crate::Grafik;

    #[test]
    fn beş_kaynak_yüzeyi_boyut_veri_ve_serileri_korur() -> Result<(), UplotHatası> {
        for örnek in SyncCursorÖrneği::TÜMÜ {
            let (seçenekler, veri) = sync_cursor_kartı(örnek)?;
            assert_eq!((seçenekler.genişlik, seçenekler.yükseklik), örnek.boyut());
            let beklenen = if örnek.grup() == 0 { 1_000 } else { 2 };
            assert_eq!(veri.uzunluk(), beklenen);
            assert!(
                Grafik::yeni(seçenekler, veri)?
                    .çiz()
                    .svg()
                    .contains(örnek.başlık())
            );
        }
        Ok(())
    }

    #[test]
    fn senkron_grupları_ve_etikete_göre_seri_eşleme_korunur() {
        let mut grup = SyncCursorGrubu::yeni();
        assert_eq!(
            grup.imleç_hedefleri(SyncCursorÖrneği::Cpu),
            vec![SyncCursorÖrneği::Ram, SyncCursorÖrneği::Tcp]
        );
        assert_eq!(
            grup.seri_hedefi(
                SyncCursorÖrneği::UyumsuzKırmızıMavi,
                SyncCursorÖrneği::UyumsuzYeşilKırmızı,
                0
            ),
            Some(1)
        );
        assert_eq!(
            grup.seri_hedefi(
                SyncCursorÖrneği::UyumsuzKırmızıMavi,
                SyncCursorÖrneği::UyumsuzYeşilKırmızı,
                1
            ),
            None
        );
        assert!(grup.dikey_imleç_senkron_mu(SyncCursorÖrneği::Cpu, SyncCursorÖrneği::Ram));
        assert!(!grup.dikey_imleç_senkron_mu(SyncCursorÖrneği::Cpu, SyncCursorÖrneği::Tcp));
        assert!(grup.senkronu_ayarla(false));
        assert!(grup.imleç_hedefleri(SyncCursorÖrneği::Cpu).is_empty());
        assert_eq!(
            grup.imleç_hedefleri(SyncCursorÖrneği::UyumsuzKırmızıMavi),
            vec![SyncCursorÖrneği::UyumsuzYeşilKırmızı]
        );
    }

    #[test]
    fn fare_bırakma_filtresi_cursor_kilidini_kaynak_gibi_ayırır() {
        let mut grup = SyncCursorGrubu::yeni();
        let değişenler = grup.fare_bırak(SyncCursorÖrneği::Cpu);
        assert_eq!(değişenler.len(), 3);
        assert!(
            SyncCursorÖrneği::TÜMÜ[..3]
                .iter()
                .all(|örnek| grup.kilitli(*örnek))
        );
        assert!(grup.fare_basma_bırakma_senkronunu_ayarla(false));
        let değişenler = grup.fare_bırak(SyncCursorÖrneği::Cpu);
        assert_eq!(değişenler, vec![(SyncCursorÖrneği::Cpu, false)]);
        assert!(grup.kilitli(SyncCursorÖrneği::Ram));
        let değişenler = grup.fare_bırak(SyncCursorÖrneği::UyumsuzKırmızıMavi);
        assert_eq!(değişenler.len(), 2);
    }
}
