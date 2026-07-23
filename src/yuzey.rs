use crate::Nokta;

/// Grafik yüzeyinin pencere/istemci koordinatlarındaki güncel dikdörtgeni.
///
/// Kaydırma, yeniden boyutlandırma veya yerleşim değişiminden sonra adaptör bu
/// değeri yeniler. Böylece işaretçi koordinatları platformdan bağımsız olarak
/// aynı çekirdek dönüşümüyle sahneye eşlenir.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct YüzeyDikdörtgeni {
    pub sol: f64,
    pub üst: f64,
    pub genişlik: f64,
    pub yükseklik: f64,
}

impl YüzeyDikdörtgeni {
    pub fn yeni(sol: f64, üst: f64, genişlik: f64, yükseklik: f64) -> Option<Self> {
        (sol.is_finite()
            && üst.is_finite()
            && genişlik.is_finite()
            && yükseklik.is_finite()
            && genişlik > 0.0
            && yükseklik > 0.0)
            .then_some(Self {
                sol,
                üst,
                genişlik,
                yükseklik,
            })
    }

    /// İstemci koordinatını, en-boy oranını koruyan ortalanmış sahneye taşır.
    pub fn sahne_konumu(
        self,
        istemci_x: f64,
        istemci_y: f64,
        sahne_genişliği: u32,
        sahne_yüksekliği: u32,
    ) -> Option<Nokta> {
        if !istemci_x.is_finite()
            || !istemci_y.is_finite()
            || sahne_genişliği == 0
            || sahne_yüksekliği == 0
        {
            return None;
        }
        let sahne_genişliği = f64::from(sahne_genişliği);
        let sahne_yüksekliği = f64::from(sahne_yüksekliği);
        let ölçek = (self.genişlik / sahne_genişliği).min(self.yükseklik / sahne_yüksekliği);
        if !ölçek.is_finite() || ölçek <= 0.0 {
            return None;
        }
        let köken_x = self.sol + (self.genişlik - sahne_genişliği * ölçek) / 2.0;
        let köken_y = self.üst + (self.yükseklik - sahne_yüksekliği * ölçek) / 2.0;
        Some(Nokta::yeni(
            ((istemci_x - köken_x) / ölçek) as f32,
            ((istemci_y - köken_y) / ölçek) as f32,
        ))
    }
}

#[cfg(test)]
mod testler {
    use super::*;

    #[test]
    fn kaydırılan_yüzey_yeni_kökene_göre_eşlenir() {
        let önce = YüzeyDikdörtgeni::yeni(100.0, 300.0, 400.0, 200.0);
        let sonra = YüzeyDikdörtgeni::yeni(100.0, 120.0, 400.0, 200.0);
        assert_eq!(
            önce.and_then(|yüzey| yüzey.sahne_konumu(300.0, 400.0, 400, 200)),
            Some(Nokta::yeni(200.0, 100.0))
        );
        assert_eq!(
            sonra.and_then(|yüzey| yüzey.sahne_konumu(300.0, 220.0, 400, 200)),
            Some(Nokta::yeni(200.0, 100.0))
        );
    }

    #[test]
    fn geçersiz_yüzey_ve_koordinat_reddedilir() {
        assert!(YüzeyDikdörtgeni::yeni(0.0, 0.0, 0.0, 200.0).is_none());
        let yüzey = YüzeyDikdörtgeni::yeni(0.0, 0.0, 400.0, 200.0);
        assert!(
            yüzey
                .and_then(|değer| değer.sahne_konumu(f64::NAN, 0.0, 400, 200))
                .is_none()
        );
    }
}
