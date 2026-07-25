use crate::Nokta;

/// `placement(..., "right", "start", { bound })` ile eşdeğer yatay taraf.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BilgiKutusuTarafı {
    Sağ,
    Sol,
}

/// Hafif imleç bilgi kutusunun sınırlandırılmış yüzey yerleşimi.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BilgiKutusuYerleşimi {
    pub sol: f64,
    pub üst: f64,
    pub taraf: BilgiKutusuTarafı,
    pub azami_genişlik: f64,
    pub azami_yükseklik: f64,
}

/// Sağ/start yerleşimini dener; sağda yer yoksa daha geniş yatay tarafa
/// çevirir ve kutuyu çizim yüzeyi sınırlarında tutar.
pub fn bilgi_kutusunu_yerleştir(
    sınır: YüzeyDikdörtgeni,
    dayanak_x: f64,
    dayanak_y: f64,
    kutu_genişliği: f64,
    kutu_yüksekliği: f64,
    boşluk: f64,
) -> Option<BilgiKutusuYerleşimi> {
    if ![
        dayanak_x,
        dayanak_y,
        kutu_genişliği,
        kutu_yüksekliği,
        boşluk,
    ]
    .into_iter()
    .all(f64::is_finite)
        || kutu_genişliği < 0.0
        || kutu_yüksekliği < 0.0
        || boşluk < 0.0
    {
        return None;
    }
    let sağ_sınır = sınır.sol + sınır.genişlik;
    let alt_sınır = sınır.üst + sınır.yükseklik;
    let sol_pay = dayanak_x - sınır.sol - boşluk;
    let sağ_pay = sağ_sınır - dayanak_x - boşluk;
    let taraf = if kutu_genişliği <= sağ_pay || sağ_pay >= sol_pay {
        BilgiKutusuTarafı::Sağ
    } else {
        BilgiKutusuTarafı::Sol
    };
    let hedef_sol = match taraf {
        BilgiKutusuTarafı::Sağ => dayanak_x + boşluk,
        BilgiKutusuTarafı::Sol => dayanak_x - kutu_genişliği - boşluk,
    };
    let en_sağ_sol = (sağ_sınır - kutu_genişliği).max(sınır.sol);
    let en_alt_üst = (alt_sınır - kutu_yüksekliği).max(sınır.üst);
    Some(BilgiKutusuYerleşimi {
        sol: hedef_sol.clamp(sınır.sol, en_sağ_sol),
        üst: dayanak_y.clamp(sınır.üst, en_alt_üst),
        taraf,
        azami_genişlik: (sınır.genişlik - boşluk * 2.0).max(0.0),
        azami_yükseklik: (sınır.yükseklik - boşluk * 2.0).max(0.0),
    })
}

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

    #[test]
    fn oranı_farklı_css_kutusunda_ortalanmış_svg_sahnesini_korur() {
        let yüzey = YüzeyDikdörtgeni::yeni(10.0, 20.0, 500.0, 500.0);
        assert_eq!(
            yüzey.and_then(|değer| değer.sahne_konumu(260.0, 270.0, 400, 200)),
            Some(Nokta::yeni(200.0, 100.0))
        );
        assert_eq!(
            yüzey.and_then(|değer| değer.sahne_konumu(60.0, 145.0, 400, 200)),
            Some(Nokta::yeni(40.0, 0.0))
        );
    }

    #[test]
    fn bilgi_kutusu_sağdan_sola_döner_ve_plotta_kalır() {
        let Some(sınır) = YüzeyDikdörtgeni::yeni(50.0, 30.0, 500.0, 300.0) else {
            return;
        };
        let sağ = bilgi_kutusunu_yerleştir(sınır, 200.0, 100.0, 120.0, 40.0, 12.0);
        assert_eq!(
            sağ,
            Some(BilgiKutusuYerleşimi {
                sol: 212.0,
                üst: 100.0,
                taraf: BilgiKutusuTarafı::Sağ,
                azami_genişlik: 476.0,
                azami_yükseklik: 276.0,
            })
        );
        let sol = bilgi_kutusunu_yerleştir(sınır, 540.0, 325.0, 120.0, 40.0, 12.0);
        assert_eq!(
            sol,
            Some(BilgiKutusuYerleşimi {
                sol: 408.0,
                üst: 290.0,
                taraf: BilgiKutusuTarafı::Sol,
                azami_genişlik: 476.0,
                azami_yükseklik: 276.0,
            })
        );
    }
}
