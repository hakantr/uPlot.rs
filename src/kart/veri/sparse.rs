use crate::UplotHatası;

const VARLIK: &str = "demos/data/sparse.json";
const KODLU: &str = include_str!("sparse.b64");

pub fn sparse_verisi() -> Result<(Vec<f64>, Vec<Option<f64>>), UplotHatası> {
    let baytlar = base64_çöz(KODLU)?;
    let mut okuyucu = Okuyucu::yeni(&baytlar);
    let uzunluk = usize::from(okuyucu.u16()?);
    if uzunluk == 0 {
        return Err(hata("kaynak veri boş"));
    }
    let mut x = Vec::with_capacity(uzunluk);
    let mut güncel_x = okuyucu.u32()?;
    x.push(f64::from(güncel_x));
    for _ in 1..uzunluk {
        güncel_x = güncel_x
            .checked_add(u32::from(okuyucu.u8()?))
            .ok_or_else(|| hata("X delta toplamı u32 sınırını aştı"))?;
        x.push(f64::from(güncel_x));
    }

    let mut y = vec![None; uzunluk];
    let koşu_sayısı = usize::from(okuyucu.u16()?);
    for _ in 0..koşu_sayısı {
        let başlangıç = usize::from(okuyucu.u16()?);
        let koşu_uzunluğu = usize::from(okuyucu.u16()?);
        let bitiş = başlangıç
            .checked_add(koşu_uzunluğu)
            .ok_or_else(|| hata("Y koşu aralığı taştı"))?;
        if bitiş > uzunluk {
            return Err(hata("Y koşusu kaynak uzunluğunun dışında"));
        }
        for indeks in başlangıç..bitiş {
            let değer = f64::from(okuyucu.u16()?);
            let Some(hedef) = y.get_mut(indeks) else {
                return Err(hata("Y koşu indeksi bulunamadı"));
            };
            *hedef = Some(değer);
        }
    }
    if okuyucu.kalan() != 0 {
        return Err(hata("kodlu kaynağın sonunda beklenmeyen baytlar var"));
    }
    Ok((x, y))
}

fn base64_çöz(girdi: &str) -> Result<Vec<u8>, UplotHatası> {
    let mut çıktı = Vec::with_capacity(girdi.len().saturating_mul(3) / 4);
    let mut birikim = 0_u32;
    let mut bit = 0_u8;
    for bayt in girdi.bytes().filter(|bayt| !bayt.is_ascii_whitespace()) {
        if bayt == b'=' {
            break;
        }
        let değer = match bayt {
            b'A'..=b'Z' => bayt - b'A',
            b'a'..=b'z' => bayt - b'a' + 26,
            b'0'..=b'9' => bayt - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => return Err(hata("geçersiz base64 karakteri")),
        };
        birikim = (birikim << 6) | u32::from(değer);
        bit = bit.saturating_add(6);
        if bit >= 8 {
            bit -= 8;
            çıktı.push(((birikim >> bit) & 0xff) as u8);
        }
    }
    Ok(çıktı)
}

struct Okuyucu<'a> {
    baytlar: &'a [u8],
    konum: usize,
}

impl<'a> Okuyucu<'a> {
    const fn yeni(baytlar: &'a [u8]) -> Self {
        Self { baytlar, konum: 0 }
    }

    fn u8(&mut self) -> Result<u8, UplotHatası> {
        let Some(değer) = self.baytlar.get(self.konum).copied() else {
            return Err(hata("kodlu kaynak beklenenden erken bitti"));
        };
        self.konum = self.konum.saturating_add(1);
        Ok(değer)
    }

    fn u16(&mut self) -> Result<u16, UplotHatası> {
        let düşük = u16::from(self.u8()?);
        let yüksek = u16::from(self.u8()?);
        Ok(düşük | (yüksek << 8))
    }

    fn u32(&mut self) -> Result<u32, UplotHatası> {
        let düşük = u32::from(self.u16()?);
        let yüksek = u32::from(self.u16()?);
        Ok(düşük | (yüksek << 16))
    }

    fn kalan(&self) -> usize {
        self.baytlar.len().saturating_sub(self.konum)
    }
}

fn hata(açıklama: impl Into<String>) -> UplotHatası {
    UplotHatası::GeçersizKaynakVeri {
        varlık: VARLIK,
        açıklama: açıklama.into(),
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn sıkıştırılmış_kaynak_sınırları_ve_sıklığı_korur() -> Result<(), UplotHatası> {
        let (x, y) = sparse_verisi()?;
        assert_eq!(x.len(), 13_608);
        assert_eq!(y.len(), 13_608);
        assert_eq!(x.first().copied(), Some(172.0));
        assert_eq!(x.last().copied(), Some(368_313.0));
        assert_eq!(y.iter().flatten().count(), 4_608);
        assert_eq!(
            y.iter().flatten().copied().min_by(f64::total_cmp),
            Some(68.0)
        );
        assert_eq!(
            y.iter().flatten().copied().max_by(f64::total_cmp),
            Some(511.0)
        );
        Ok(())
    }
}
