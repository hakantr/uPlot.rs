#[path = "veri/data_smoothing_kaynak.rs"]
mod kaynak;

use super::ortak_kart_etkileşimleri;
use crate::{Aralık, GrafikSeçenekleri, HizalıVeri, SeriSeçenekleri, UplotHatası};
use kaynak::TAXI_TRIPS;

pub const DATA_SMOOTHING_KART_TANIM_ÖRNEĞİ: &str = r##"let (seçenekler, veri) = data_smoothing_kartı(SmoothingÖrneği::Asap)?;
let grafik = Grafik::yeni(seçenekler, veri)?;"##;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmoothingÖrneği {
    Ham,
    SavitzkyGolay,
    Asap,
    HareketliOrtalama,
}

impl SmoothingÖrneği {
    pub const TÜMÜ: [Self; 4] = [
        Self::Ham,
        Self::SavitzkyGolay,
        Self::Asap,
        Self::HareketliOrtalama,
    ];
    pub fn kimlik(self) -> &'static str {
        match self {
            Self::Ham => "data-smoothing-raw",
            Self::SavitzkyGolay => "data-smoothing-sgg",
            Self::Asap => "data-smoothing-asap",
            Self::HareketliOrtalama => "data-smoothing-moving-average",
        }
    }
    pub fn başlık(self) -> &'static str {
        match self {
            Self::Ham => "Taxi Trips (raw)",
            Self::SavitzkyGolay => "Savitzky-Golay",
            Self::Asap => "Taxi Trips (ASAP FFT)",
            Self::HareketliOrtalama => "Taxi Trips (Moving Avg 300)",
        }
    }
    fn aralık(self) -> Result<Aralık, UplotHatası> {
        match self {
            Self::Ham => Aralık::yeni(0.0, 40_000.0),
            Self::SavitzkyGolay => Aralık::yeni(0.0, 30_000.0),
            Self::Asap => Aralık::yeni(12_000.0, 20_000.0),
            Self::HareketliOrtalama => Aralık::yeni(10_000.0, 20_000.0),
        }
    }
}

fn algoritma_hatası(açıklama: impl Into<String>) -> UplotHatası {
    UplotHatası::GeçersizKaynakVeri {
        varlık: "data-smoothing",
        açıklama: açıklama.into(),
    }
}

pub fn hareketli_ortalama(veri: &[f64], pencere: usize) -> Result<Vec<f64>, UplotHatası> {
    if pencere == 0 {
        return Err(algoritma_hatası(
            "hareketli ortalama penceresi sıfır olamaz",
        ));
    }
    let mut sonuç = Vec::with_capacity(veri.len());
    let mut toplam = 0.0;
    let mut sayı = 0usize;
    for (indeks, değer) in veri.iter().copied().enumerate() {
        if !değer.is_finite() {
            return Err(algoritma_hatası(format!("sonlu olmayan değer: {indeks}")));
        }
        toplam += değer;
        sayı = sayı
            .checked_add(1)
            .ok_or_else(|| algoritma_hatası("hareketli ortalama sayacı taştı"))?;
        if indeks >= pencere
            && let Some(eski) = veri.get(indeks - pencere)
        {
            toplam -= eski;
            sayı = sayı.saturating_sub(1);
        }
        if sayı == 0 {
            return Err(algoritma_hatası("hareketli ortalama penceresi boş kaldı"));
        }
        sonuç.push(toplam / sayı as f64);
    }
    Ok(sonuç)
}

pub fn savitzky_golay(
    veri: &[f64],
    pencere: usize,
    polinom: usize,
) -> Result<Vec<f64>, UplotHatası> {
    if pencere < 5
        || pencere.is_multiple_of(2)
        || pencere > veri.len()
        || polinom == 0
        || polinom > 32
        || polinom > pencere / 2
    {
        return Err(algoritma_hatası(
            "Savitzky-Golay pencere/polinom ayarı geçersiz",
        ));
    }
    if veri.iter().any(|değer| !değer.is_finite()) {
        return Err(algoritma_hatası("Savitzky-Golay verisi sonlu olmalı"));
    }
    let yarı = pencere / 2;
    let mut ağırlıklar = Vec::with_capacity(pencere);
    for t in -(yarı as isize)..=(yarı as isize) {
        let mut satır = Vec::with_capacity(pencere);
        for j in -(yarı as isize)..=(yarı as isize) {
            satır.push(sg_ağırlığı(j, t, yarı, polinom));
        }
        ağırlıklar.push(satır);
    }
    let mut sonuç = vec![0.0; veri.len()];
    for i in 0..yarı {
        let sol = ağırlıklar
            .get(yarı - i - 1)
            .ok_or_else(|| algoritma_hatası("SGG sol ağırlığı yok"))?;
        let sağ = ağırlıklar
            .get(yarı + i + 1)
            .ok_or_else(|| algoritma_hatası("SGG sağ ağırlığı yok"))?;
        let mut d1 = 0.0;
        let mut d2 = 0.0;
        for l in 0..pencere {
            let y1 = veri
                .get(l)
                .copied()
                .ok_or_else(|| algoritma_hatası("SGG sol veri yok"))?;
            let y2 = veri
                .get(veri.len() - pencere + l)
                .copied()
                .ok_or_else(|| algoritma_hatası("SGG sağ veri yok"))?;
            d1 += sol.get(l).copied().unwrap_or(0.0) * y1;
            d2 += sağ.get(l).copied().unwrap_or(0.0) * y2;
        }
        let sol_indeks = yarı - i - 1;
        let sağ_indeks = veri.len() - yarı + i;
        *sonuç
            .get_mut(sol_indeks)
            .ok_or_else(|| algoritma_hatası("SGG sol sonuç indeksi geçersiz"))? = d1;
        *sonuç
            .get_mut(sağ_indeks)
            .ok_or_else(|| algoritma_hatası("SGG sağ sonuç indeksi geçersiz"))? = d2;
    }
    let merkez = ağırlıklar
        .get(yarı)
        .ok_or_else(|| algoritma_hatası("SGG merkez ağırlığı yok"))?;
    for i in pencere..=veri.len() {
        let mut d = 0.0;
        for l in 0..pencere {
            d += merkez.get(l).copied().unwrap_or(0.0)
                * veri.get(l + i - pencere).copied().unwrap_or(0.0);
        }
        *sonuç
            .get_mut(i - yarı - 1)
            .ok_or_else(|| algoritma_hatası("SGG merkez sonuç indeksi geçersiz"))? = d;
    }
    Ok(sonuç)
}

fn sg_ağırlığı(i: isize, t: isize, m: usize, n: usize) -> f64 {
    (0..=n)
        .map(|k| {
            (2 * k + 1) as f64 * gen_fact(2 * m, k) / gen_fact(2 * m + k + 1, k + 1)
                * gram(i, m, k, 0)
                * gram(t, m, k, 0)
        })
        .sum()
}
fn gen_fact(a: usize, b: usize) -> f64 {
    if a < b {
        1.0
    } else {
        (a - b + 1..=a).map(|v| v as f64).product()
    }
}
fn gram(i: isize, m: usize, k: usize, s: usize) -> f64 {
    if k == 0 {
        return if s == 0 { 1.0 } else { 0.0 };
    }
    let kf = k as f64;
    let mf = m as f64;
    ((4.0 * kf - 2.0) / (kf * (2.0 * mf - kf + 1.0)))
        * (i as f64 * gram(i, m, k - 1, s) + s as f64 * gram(i, m, k - 1, s.saturating_sub(1)))
        - if k > 1 {
            ((k - 1) as f64 * (2.0 * mf + kf)) / (kf * (2.0 * mf - kf + 1.0)) * gram(i, m, k - 2, s)
        } else {
            0.0
        }
}

fn sma(veri: &[f64], aralık: usize, kayma: usize) -> Result<Vec<f64>, UplotHatası> {
    if aralık == 0 || kayma == 0 {
        return Err(algoritma_hatası("SMA aralık/kayma sıfır olamaz"));
    }
    let mut başlangıç = 0usize;
    let mut toplam = 0.0;
    let mut sayı = 0usize;
    let mut sonuç = Vec::new();
    for (i, değer) in veri.iter().copied().enumerate() {
        if i.saturating_sub(başlangıç) >= aralık {
            if sayı == 0 {
                return Err(algoritma_hatası("SMA penceresi boş kaldı"));
            }
            sonuç.push(toplam / sayı as f64);
            let eski = başlangıç;
            while başlangıç < veri.len() && başlangıç - eski < kayma {
                toplam -= veri.get(başlangıç).copied().unwrap_or(0.0);
                sayı = sayı.saturating_sub(1);
                başlangıç = başlangıç
                    .checked_add(1)
                    .ok_or_else(|| algoritma_hatası("SMA başlangıç indeksi taştı"))?;
            }
        }
        toplam += değer;
        sayı = sayı
            .checked_add(1)
            .ok_or_else(|| algoritma_hatası("SMA sayacı taştı"))?;
    }
    if sayı == aralık {
        sonuç.push(toplam / sayı as f64);
    }
    Ok(sonuç)
}

pub fn asap_yumuşat(veri: &[f64], çözünürlük: usize) -> Result<Vec<f64>, UplotHatası> {
    if çözünürlük == 0 || veri.is_empty() || veri.iter().any(|v| !v.is_finite()) {
        return Err(algoritma_hatası("ASAP girdisi geçersiz"));
    }
    let veri = if veri.len() >= çözünürlük.saturating_mul(2) {
        let oran = veri.len() / çözünürlük;
        sma(veri, oran, oran)?
    } else {
        veri.to_vec()
    };
    let mut acf = Acf::yeni(&veri, (veri.len() as f64 / 10.0).round() as usize)?;
    let tepeler = acf.tepeleri_bul();
    let özgün = Metrics::yeni(&veri);
    let özgün_kurt = özgün.kurtosis();
    let mut min_nesne = özgün.roughness();
    let mut pencere = 1usize;
    let mut alt = 1usize;
    let mut en_büyük = None;
    let mut kuyruk = veri.len() / 10;
    for (i, w) in tepeler.iter().copied().enumerate().rev() {
        if w < alt || w == 1 {
            break;
        }
        let cw = acf.correlations.get(w).copied().unwrap_or(0.0);
        let cp = acf.correlations.get(pencere).copied().unwrap_or(0.0);
        if (1.0 - cw).sqrt() * pencere as f64 > (1.0 - cp).sqrt() * w as f64 {
            continue;
        }
        let y = sma(&veri, w, 1)?;
        let m = Metrics::yeni(&y);
        let pürüz = m.roughness();
        if m.kurtosis() >= özgün_kurt {
            if pürüz < min_nesne {
                min_nesne = pürüz;
                pencere = w;
            }
            let oran = ((acf.max_acf - 1.0) / (cw - 1.0)).sqrt();
            alt = ((w as f64 * oran).max(alt as f64)).round() as usize;
            if en_büyük.is_none() {
                en_büyük = Some(i);
            }
        }
    }
    if let Some(i) = en_büyük.filter(|i| *i > 0) {
        if i < tepeler.len().saturating_sub(2) {
            kuyruk = tepeler.get(i + 1).copied().unwrap_or(kuyruk);
        }
        alt = alt.max(tepeler.get(i).copied().unwrap_or(0).saturating_add(1));
    }
    pencere = asap_ikili_ara(alt, kuyruk, &veri, min_nesne, özgün_kurt, pencere)?;
    sma(&veri, pencere, 1)
}

fn asap_ikili_ara(
    mut baş: usize,
    mut son: usize,
    veri: &[f64],
    mut min: f64,
    kurt: f64,
    mut pencere: usize,
) -> Result<usize, UplotHatası> {
    while baş <= son {
        let w = baş + (son - baş) / 2;
        let y = sma(veri, w, 1)?;
        let m = Metrics::yeni(&y);
        if m.kurtosis() >= kurt {
            let r = m.roughness();
            if r < min {
                pencere = w;
                min = r;
            }
            baş = w.saturating_add(1);
        } else {
            if w == 0 {
                break;
            }
            son = w - 1;
        }
    }
    Ok(pencere)
}

struct Acf {
    correlations: Vec<f64>,
    max_acf: f64,
}
impl Acf {
    fn yeni(veri: &[f64], azami: usize) -> Result<Self, UplotHatası> {
        let ort = Metrics::ortalama(veri);
        let uzunluk = if veri.len().is_power_of_two() {
            veri.len().checked_mul(2)
        } else {
            veri.len().checked_next_power_of_two()
        }
        .filter(|uzunluk| *uzunluk >= 2)
        .ok_or_else(|| algoritma_hatası("FFT tampon uzunluğu taşması"))?;
        let mut gerçek = vec![0.0; uzunluk];
        let mut sanal = vec![0.0; uzunluk];
        for (i, v) in veri.iter().copied().enumerate() {
            if let Some(h) = gerçek.get_mut(i) {
                *h = v - ort;
            }
        }
        fft(&mut gerçek, &mut sanal)?;
        for (g, s) in gerçek.iter_mut().zip(sanal.iter_mut()) {
            *g = *g * *g + *s * *s;
            *s = 0.0;
        }
        ters_fft(&mut gerçek, &mut sanal)?;
        let taban = gerçek.first().copied().unwrap_or(1.0);
        let mut c = vec![0.0; azami];
        for i in 1..azami {
            if let Some(h) = c.get_mut(i) {
                *h = gerçek.get(i).copied().unwrap_or(0.0) / taban;
            }
        }
        Ok(Self {
            correlations: c,
            max_acf: 0.0,
        })
    }
    fn tepeleri_bul(&mut self) -> Vec<usize> {
        let mut sonuç = Vec::new();
        if self.correlations.len() > 1 {
            let mut pozitif = false;
            let mut max = 1usize;
            for i in 2..self.correlations.len() {
                let Some((&güncel, &önceki, &azami)) = self
                    .correlations
                    .get(i)
                    .zip(self.correlations.get(i - 1))
                    .zip(self.correlations.get(max))
                    .map(|((güncel, önceki), azami)| (güncel, önceki, azami))
                else {
                    continue;
                };
                if !pozitif && güncel > önceki {
                    max = i;
                    pozitif = true;
                } else if pozitif && güncel > azami {
                    max = i;
                } else if pozitif && güncel < önceki {
                    if max > 1 && azami > 0.2 {
                        sonuç.push(max);
                        self.max_acf = self.max_acf.max(azami);
                    }
                    pozitif = false;
                }
            }
        }
        if sonuç.len() <= 1 {
            sonuç = (2..self.correlations.len()).collect();
        }
        sonuç
    }
}

struct Metrics<'a> {
    v: &'a [f64],
    ortalama: f64,
}
impl<'a> Metrics<'a> {
    fn yeni(v: &'a [f64]) -> Self {
        Self {
            v,
            ortalama: Self::ortalama(v),
        }
    }
    fn ortalama(v: &[f64]) -> f64 {
        v.iter().sum::<f64>() / v.len().max(1) as f64
    }
    fn std(v: &[f64]) -> f64 {
        let m = Self::ortalama(v);
        (v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len().max(1) as f64).sqrt()
    }
    fn kurtosis(&self) -> f64 {
        let (u4, var) = self.v.iter().fold((0.0, 0.0), |(u, var), x| {
            let d = *x - self.ortalama;
            (u + d.powi(4), var + d.powi(2))
        });
        self.v.len() as f64 * u4 / var.powi(2)
    }
    fn roughness(&self) -> f64 {
        let d = self
            .v
            .windows(2)
            .filter_map(|w| w.first().zip(w.last()).map(|(ilk, son)| son - ilk))
            .collect::<Vec<_>>();
        Self::std(&d)
    }
}

fn fft(g: &mut [f64], s: &mut [f64]) -> Result<(), UplotHatası> {
    let n = g.len();
    if n < 2 || n != s.len() || !n.is_power_of_two() {
        return Err(algoritma_hatası("FFT uzunluğu iki kuvveti olmalı"));
    }
    let bit = n.trailing_zeros();
    for i in 0..n {
        let j = i.reverse_bits() >> (usize::BITS - bit);
        if j > i {
            g.swap(i, j);
            s.swap(i, j);
        }
    }
    let mut boy = 2;
    while boy <= n {
        let yarı = boy / 2;
        for i in (0..n).step_by(boy) {
            for k in 0..yarı {
                let açı = 2.0 * std::f64::consts::PI * k as f64 / boy as f64;
                let cos = açı.cos();
                let sin = açı.sin();
                let j = i
                    .checked_add(k)
                    .ok_or_else(|| algoritma_hatası("FFT sol indeksi taştı"))?;
                let ö = j
                    .checked_add(yarı)
                    .ok_or_else(|| algoritma_hatası("FFT sağ indeksi taştı"))?;
                let (g_sol, g_sağ) = g.split_at_mut(ö);
                let (s_sol, s_sağ) = s.split_at_mut(ö);
                let g_j = g_sol
                    .get_mut(j)
                    .ok_or_else(|| algoritma_hatası("FFT gerçek sol indeksi geçersiz"))?;
                let g_ö = g_sağ
                    .first_mut()
                    .ok_or_else(|| algoritma_hatası("FFT gerçek sağ indeksi geçersiz"))?;
                let s_j = s_sol
                    .get_mut(j)
                    .ok_or_else(|| algoritma_hatası("FFT sanal sol indeksi geçersiz"))?;
                let s_ö = s_sağ
                    .first_mut()
                    .ok_or_else(|| algoritma_hatası("FFT sanal sağ indeksi geçersiz"))?;
                let tg = *g_ö * cos + *s_ö * sin;
                let ts = -*g_ö * sin + *s_ö * cos;
                *g_ö = *g_j - tg;
                *s_ö = *s_j - ts;
                *g_j += tg;
                *s_j += ts;
            }
        }
        let Some(yeni_boy) = boy.checked_mul(2) else {
            break;
        };
        boy = yeni_boy;
    }
    Ok(())
}
fn ters_fft(g: &mut [f64], s: &mut [f64]) -> Result<(), UplotHatası> {
    fft(s, g)
}

pub fn data_smoothing_kartı(
    örnek: SmoothingÖrneği,
) -> Result<(GrafikSeçenekleri, HizalıVeri), UplotHatası> {
    let değerler = match örnek {
        SmoothingÖrneği::Ham => TAXI_TRIPS.to_vec(),
        SmoothingÖrneği::SavitzkyGolay => savitzky_golay(&TAXI_TRIPS, 101, 3)?,
        SmoothingÖrneği::Asap => asap_yumuşat(&TAXI_TRIPS, 150)?,
        SmoothingÖrneği::HareketliOrtalama => hareketli_ortalama(&TAXI_TRIPS, 300)?,
    };
    let x = (0..değerler.len()).map(|i| i as f64).collect();
    let veri = HizalıVeri::yeni(x, vec![değerler.into_iter().map(Some).collect()])?;
    let seçenekler = GrafikSeçenekleri::yeni(1920, 300)?
        .başlık(örnek.başlık())
        .x_zaman(false)
        .y_aralığı(örnek.aralık()?)
        .etkileşimler(ortak_kart_etkileşimleri())
        .seri(SeriSeçenekleri::yeni("Trips").renk("#ff0000"));
    Ok((seçenekler, veri))
}

#[cfg(test)]
mod testler {
    use super::*;
    use kaynak::{
        ASAP_REFERENCE, MOVING_REFERENCE, MOVING_REFERENCE_SUM, SGG_REFERENCE, SGG_REFERENCE_SUM,
    };
    fn yakın(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-7 * a.abs().max(b.abs()).max(1.0)
    }
    #[test]
    fn üç_algoritma_kaynak_js_ile_sayısal_eştir() -> Result<(), UplotHatası> {
        let s = savitzky_golay(&TAXI_TRIPS, 101, 3)?;
        for (i, e) in SGG_REFERENCE {
            assert!(s.get(i).is_some_and(|v| yakın(*v, e)));
        }
        assert!(yakın(s.iter().sum(), SGG_REFERENCE_SUM));
        let m = hareketli_ortalama(&TAXI_TRIPS, 300)?;
        for (i, e) in MOVING_REFERENCE {
            assert!(m.get(i).is_some_and(|v| yakın(*v, e)));
        }
        assert!(yakın(m.iter().sum(), MOVING_REFERENCE_SUM));
        let a = asap_yumuşat(&TAXI_TRIPS, 150)?;
        assert_eq!(a.len(), ASAP_REFERENCE.len());
        assert!(a.iter().zip(ASAP_REFERENCE).all(|(v, e)| yakın(*v, e)));
        Ok(())
    }
}
