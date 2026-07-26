/// Tarayıcı referansı ile Rust portuna aynı rastgele akışı vermek için kullanılan
/// küçük, platformdan bağımsız kanıt üreteci. Demo algoritması değişmez; yalnız
/// `Math.random` girdisi açık bir tohuma bağlanır.
pub(super) struct KanıtRastgele {
    durum: u32,
}

impl KanıtRastgele {
    pub(super) fn yeni(tohum: u32) -> Self {
        Self { durum: tohum }
    }

    /// Mulberry32'nin JavaScript'te de bit düzeyinde yeniden üretilebilen adımı.
    pub(super) fn sonraki(&mut self) -> f64 {
        self.durum = self.durum.wrapping_add(0x6D2B_79F5);
        let mut değer = self.durum;
        değer = (değer ^ (değer >> 15)).wrapping_mul(değer | 1);
        değer ^= değer.wrapping_add((değer ^ (değer >> 7)).wrapping_mul(değer | 61));
        f64::from(değer ^ (değer >> 14)) / 4_294_967_296.0
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn aynı_tohum_aynı_akışı_üretir() {
        let mut sol = KanıtRastgele::yeni(0xC0DE_1234);
        let mut sağ = KanıtRastgele::yeni(0xC0DE_1234);
        for _ in 0..100 {
            assert_eq!(sol.sonraki(), sağ.sonraki());
        }
    }
}
