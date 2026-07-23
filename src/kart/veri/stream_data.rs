use base64::{Engine as _, engine::general_purpose::STANDARD};
use flate2::read::ZlibDecoder;
use std::io::Read;

use crate::UplotHatası;

const VARLIK: &str = "bench/data.json";
const SIKIŞTIRILMIŞ: &str = include_str!("stream_data.b64");
pub(super) const SATIR_SAYISI: usize = 55_550;
pub(super) const İLK_EPOCH_DAKİKA: u32 = 26_107_560;

#[derive(Clone)]
pub(super) struct StreamKaynakVerisi {
    pub x: Vec<f64>,
    pub cpu: Vec<Option<f64>>,
    pub ram: Vec<Option<f64>>,
    pub tcp_out: Vec<Option<f64>>,
}

pub(super) fn stream_kaynak_verisi() -> Result<StreamKaynakVerisi, UplotHatası> {
    let sıkıştırılmış = STANDARD
        .decode(SIKIŞTIRILMIŞ.split_whitespace().collect::<String>())
        .map_err(|hata| kaynak_hatası(format!("base64 çözülemedi: {hata}")))?;
    let mut çözücü = ZlibDecoder::new(sıkıştırılmış.as_slice());
    let mut baytlar = Vec::with_capacity(SATIR_SAYISI * 6);
    çözücü
        .read_to_end(&mut baytlar)
        .map_err(|hata| kaynak_hatası(format!("zlib çözülemedi: {hata}")))?;
    let beklenen = SATIR_SAYISI
        .checked_mul(6)
        .ok_or_else(|| kaynak_hatası("beklenen veri uzunluğu taştı"))?;
    if baytlar.len() != beklenen {
        return Err(kaynak_hatası(format!(
            "çözülmüş uzunluk uyuşmuyor: beklenen {beklenen}, bulunan {}",
            baytlar.len()
        )));
    }
    let mut x = Vec::with_capacity(SATIR_SAYISI);
    let mut cpu = Vec::with_capacity(SATIR_SAYISI);
    let mut ram = Vec::with_capacity(SATIR_SAYISI);
    let mut tcp_out = Vec::with_capacity(SATIR_SAYISI);
    for satır in 0..SATIR_SAYISI {
        let ofset = satır
            .checked_mul(6)
            .ok_or_else(|| kaynak_hatası("satır ofseti taştı"))?;
        let cpu_ham = u16_oku(&baytlar, ofset)?;
        let ram_ham = u16_oku(&baytlar, ofset.saturating_add(2))?;
        let tcp_ham = u16_oku(&baytlar, ofset.saturating_add(4))?;
        let dakika = u32::try_from(satır)
            .ok()
            .and_then(|satır| İLK_EPOCH_DAKİKA.checked_add(satır))
            .ok_or_else(|| kaynak_hatası("epoch dakika değeri taştı"))?;
        x.push(f64::from(dakika) * 60.0);
        cpu.push(Some(f64::from(cpu_ham) / 100.0));
        ram.push(Some(f64::from(ram_ham) / 100.0));
        tcp_out.push(Some(f64::from(tcp_ham) / 100.0));
    }
    Ok(StreamKaynakVerisi {
        x,
        cpu,
        ram,
        tcp_out,
    })
}

fn u16_oku(baytlar: &[u8], ofset: usize) -> Result<u16, UplotHatası> {
    let düşük = baytlar
        .get(ofset)
        .copied()
        .ok_or_else(|| kaynak_hatası(format!("u16 düşük baytı eksik: {ofset}")))?;
    let yüksek_ofset = ofset
        .checked_add(1)
        .ok_or_else(|| kaynak_hatası("u16 ofseti taştı"))?;
    let yüksek = baytlar
        .get(yüksek_ofset)
        .copied()
        .ok_or_else(|| kaynak_hatası(format!("u16 yüksek baytı eksik: {yüksek_ofset}")))?;
    Ok(u16::from_le_bytes([düşük, yüksek]))
}

fn kaynak_hatası(açıklama: impl Into<String>) -> UplotHatası {
    UplotHatası::GeçersizKaynakVeri {
        varlık: VARLIK,
        açıklama: açıklama.into(),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kaynak_elli_beş_bin_beş_yüz_elli_satırı_aynen_korur() -> Result<(), UplotHatası> {
        let veri = stream_kaynak_verisi()?;
        assert_eq!(veri.x.len(), SATIR_SAYISI);
        assert_eq!(veri.cpu.len(), SATIR_SAYISI);
        assert_eq!(veri.ram.len(), SATIR_SAYISI);
        assert_eq!(veri.tcp_out.len(), SATIR_SAYISI);
        assert_eq!(veri.x.first().copied(), Some(1_566_453_600.0));
        assert_eq!(veri.x.last().copied(), Some(1_569_786_540.0));
        assert_eq!(veri.cpu.first().copied().flatten(), Some(0.54));
        assert_eq!(veri.ram.first().copied().flatten(), Some(14.02));
        assert_eq!(veri.tcp_out.first().copied().flatten(), Some(0.0));
        assert_eq!(veri.cpu.last().copied().flatten(), Some(0.15));
        assert_eq!(veri.ram.last().copied().flatten(), Some(15.75));
        assert_eq!(veri.tcp_out.last().copied().flatten(), Some(0.0));
        Ok(())
    }
}
