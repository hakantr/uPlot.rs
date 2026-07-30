//! Yüzeylerin görünür alana uyarlanması.
//!
//! Resmî uPlot sayfalarındaki sabit yüzey boyutu (örneğin `multi-bars.html`
//! içindeki 2300×800) parite gereği ham değerdir ve burada değişmez.
//! Uygulama — masaüstü ya da wasm — yalnız bu ham boyutu bildirir; yüzey
//! kendini bulunduğu görünür alana göre yerleştirir.
//!
//! Politika iki bağımsız eksen bayrağıdır ve dört durum üretir:
//!
//! * **dikey, yatay değil** (varsayılan): yükseklik görünür alana çekilir,
//!   genişlik aynı oranda değişir. En boy oranı korunur.
//! * **yatay, dikey değil**: genişlik görünür alana çekilir, yükseklik aynı
//!   oranda değişir. En boy oranı korunur.
//! * **ikisi de**: her eksen kendi alanına bağımsız uyar. En boy oranı ancak
//!   burada bozulur ve bu açık bir tercihtir.
//! * **hiçbiri**: yüzey ham boyutunda kalır; alana sığmayan kısım kaydırmayla
//!   gezilir.
//!
//! Yüzey hiçbir durumda büyütülmez: alana zaten sığan eksen ham değerinde
//! kalır. Çizim boyutu düştüğünde çekirdek yüzeyi o boyutta yeniden çözer;
//! eksen etiketleri ve geometri seyreltmesi küçültülmüş bir görüntü değil, o
//! boyuta ait doğru bir çizimdir.

use ::gpui::{ContainerQuery, IntoElement, Pixels, Size, container_query, px};

/// Sığdırmanın okunabilirliği bozmaması için izin verilen en küçük ölçek.
///
/// Dikeyde çok uzun yüzeyler — örneğin `multi-bars.html` sayfasının 800×2300
/// yatay çubuk varyantı — görünür alana sığdırıldığında en boy oranı korunduğu
/// için genişlik de aynı oranda düşer; yüzey teknik olarak tam görünür ama
/// eksen etiketleri okunamayacak kadar küçülür. Ölçek bu eşiğin altına inecekse
/// sığdırma uygulanmaz: yüzey ham boyutunda bırakılır ve kaydırmayla gezilir.
const EN_KÜÇÜK_ÖLÇEK: f32 = 0.5;

/// Bir yüzeyin içine yerleşeceği görünür alan ve uyum politikası.
///
/// [`uyarlanan_alan`] tarafından üretilir; kartlar bu değerden ham yüzey
/// boyutlarının çizim karşılığını [`GörünürAlan::yüzey`] ile ister.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GörünürAlan {
    alan: Size<Pixels>,
    dikey_sığdır: bool,
    yatay_sığdır: bool,
}

impl GörünürAlan {
    /// Ölçülen alanı ve eksen politikasını taşıyan değer üretir.
    pub const fn yeni(alan: Size<Pixels>, dikey_sığdır: bool, yatay_sığdır: bool) -> Self {
        Self {
            alan,
            dikey_sığdır,
            yatay_sığdır,
        }
    }

    /// Ölçülen ham görünür alan.
    pub const fn alan(self) -> Size<Pixels> {
        self.alan
    }

    /// Dikey eksende sığdırma açık mı.
    pub const fn dikey_sığdır(self) -> bool {
        self.dikey_sığdır
    }

    /// Yatay eksende sığdırma açık mı.
    pub const fn yatay_sığdır(self) -> bool {
        self.yatay_sığdır
    }

    /// Herhangi bir eksende sığdırma açık mı.
    pub const fn sığdırma_var_mı(self) -> bool {
        self.dikey_sığdır || self.yatay_sığdır
    }

    /// Yüzey başlığı gibi sabit yüksekliklerin payını görünür alandan düşer.
    ///
    /// Yüzeyin üstünde başlık veya açıklama satırı varsa sığdırma payı o
    /// satırdan sonra kalan alandır; aksi hâlde başlık yüzeyi alt sınırdan
    /// taşırır.
    pub fn pay_düş(self, pay: f32) -> Self {
        Self {
            alan: Size {
                width: self.alan.width,
                height: self.alan.height - px(pay),
            },
            ..self
        }
    }

    /// Genişliği kapsayıcıya uyan yüzeyler için yalnız dikey sığdırma.
    ///
    /// Yüzey `w_full` gibi esnek genişlikteyse en boy oranını kapsayıcı
    /// belirler; böyle yüzeylerde yalnız yükseklik görünür alana çekilir.
    /// Alana sığan yükseklik olduğu gibi kalır, yüzey büyütülmez.
    pub fn dikey(self, ham_yükseklik: f32) -> f32 {
        if !self.dikey_sığdır {
            return ham_yükseklik;
        }
        let kullanılabilir = f32::from(self.alan.height);
        if kullanılabilir <= 0.0 || ham_yükseklik <= kullanılabilir {
            return ham_yükseklik;
        }
        kullanılabilir
    }

    /// Ham boyutlu bir yüzeyin bu alandaki çizim boyutunu döndürür.
    ///
    /// Tek eksen açıkken o eksen alana çekilir ve diğeri aynı oranda değişir,
    /// yani en boy oranı korunur. İki eksen birlikte açıkken her biri kendi
    /// alanına bağımsız uyar ve oran bozulabilir. Hiçbiri açık değilse ya da
    /// yüzey alana zaten sığıyorsa ham boyut döner; yüzey asla büyütülmez.
    pub fn yüzey(self, ham_genişlik: f32, ham_yükseklik: f32) -> (f32, f32) {
        if !self.sığdırma_var_mı() || ham_genişlik <= 0.0 || ham_yükseklik <= 0.0 {
            return (ham_genişlik, ham_yükseklik);
        }
        let dikey_ölçek = self.eksen_ölçeği(self.dikey_sığdır, self.alan.height, ham_yükseklik);
        let yatay_ölçek = self.eksen_ölçeği(self.yatay_sığdır, self.alan.width, ham_genişlik);
        if self.dikey_sığdır && self.yatay_sığdır {
            // İki eksen bağımsız: oran bozulabilir, bu açık bir tercihtir.
            return (ham_genişlik * yatay_ölçek, ham_yükseklik * dikey_ölçek);
        }
        // Tek eksen: küçülen eksenin ölçeği diğerine de uygulanır.
        let ölçek = if self.dikey_sığdır {
            dikey_ölçek
        } else {
            yatay_ölçek
        };
        if ölçek < EN_KÜÇÜK_ÖLÇEK {
            return (ham_genişlik, ham_yükseklik);
        }
        (ham_genişlik * ölçek, ham_yükseklik * ölçek)
    }

    /// Bir eksenin sığdırma ölçeği; kapalıysa ya da yüzey sığıyorsa `1.0`.
    fn eksen_ölçeği(self, açık: bool, alan: Pixels, ham: f32) -> f32 {
        if !açık {
            return 1.0;
        }
        let kullanılabilir = f32::from(alan);
        if kullanılabilir <= 0.0 || ham <= kullanılabilir {
            return 1.0;
        }
        kullanılabilir / ham
    }
}

/// Görünür alanı ölçüp içeriğine ileten kapsayıcı.
///
/// Kapsayıcı varsayılan olarak ebeveyninin verdiği alanı doldurur ve ölçtüğü
/// alanı [`GörünürAlan`] olarak içerik kurucusuna geçirir. Kaydırmalı
/// listelerde ölçüm kaydırma kapsayıcısında yapıldığından, liste içindeki her
/// yüzey kendi başına görünür alana sığacak boyutu alır.
///
/// Dönen değer [`Styled`](::gpui::Styled) olduğundan, yüzeylerin üstünde
/// başlık ya da açıklama bloğu bulunan kartlarda kapsayıcı `flex_1().min_h_0()`
/// ile kalan alana yerleştirilebilir. Bu, ölçülen alanın gerçekten yüzeye
/// kalan alan olmasını sağlar: sabit yükseklikli metin blokları ölçümün
/// dışında kalır ve pay tahminine gerek kalmaz.
pub fn uyarlanan_alan<F, E>(
    dikey_sığdır: bool, yatay_sığdır: bool, içerik: F
) -> ContainerQuery
where
    F: FnOnce(GörünürAlan) -> E + 'static,
    E: IntoElement,
{
    container_query(move |alan, _pencere, _cx| {
        içerik(GörünürAlan::yeni(alan, dikey_sığdır, yatay_sığdır))
    })
}

#[cfg(test)]
mod testler {
    use super::*;

    fn alan(genişlik: f32, yükseklik: f32) -> Size<Pixels> {
        Size {
            width: px(genişlik),
            height: px(yükseklik),
        }
    }

    #[test]
    fn sığan_yüzey_ham_boyutunda_kalır() {
        let görünür = GörünürAlan::yeni(alan(1_200.0, 900.0), true, false);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 800.0));
    }

    #[test]
    fn sığmayan_yüzey_oranı_korunarak_küçülür() {
        let görünür = GörünürAlan::yeni(alan(1_200.0, 400.0), true, false);
        let (genişlik, yükseklik) = görünür.yüzey(2_300.0, 800.0);
        assert_eq!(yükseklik, 400.0);
        assert_eq!(genişlik, 1_150.0);
        // En boy oranı korunur.
        assert!((genişlik / yükseklik - 2_300.0 / 800.0).abs() < 1e-4);
    }

    #[test]
    fn sığdırma_kapalıyken_ham_boyut_döner() {
        let görünür = GörünürAlan::yeni(alan(1_200.0, 400.0), false, false);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 800.0));
    }

    #[test]
    fn başlık_payı_sığdırma_alanından_düşülür() {
        let görünür = GörünürAlan::yeni(alan(1_200.0, 430.0), true, false).pay_düş(30.0);
        let (_, yükseklik) = görünür.yüzey(2_300.0, 800.0);
        assert_eq!(yükseklik, 400.0);
    }

    #[test]
    fn aşırı_küçülecek_yüzey_ham_boyutunda_bırakılır() {
        // 800×2300 yüzey 400 px alana sığdırılsa ölçek 0.17 olur; genişlik
        // 139 px'e düşeceğinden eksen etiketleri okunamaz. Bu durumda yüzey
        // ham boyutunda kalır ve kaydırmayla gezilir.
        let görünür = GörünürAlan::yeni(alan(1_200.0, 400.0), true, false);
        assert_eq!(görünür.yüzey(800.0, 2_300.0), (800.0, 2_300.0));
    }

    #[test]
    fn eşik_üstündeki_ölçek_uygulanır() {
        // 800×1000 yüzey 600 px alanda 0.6 ölçekle sığar; eşiğin üstünde.
        let görünür = GörünürAlan::yeni(alan(1_200.0, 600.0), true, false);
        let (genişlik, yükseklik) = görünür.yüzey(800.0, 1_000.0);
        assert!((genişlik - 480.0).abs() < 1e-3);
        assert_eq!(yükseklik, 600.0);
    }

    #[test]
    fn yatay_sığdırma_oranı_koruyarak_genişliği_çeker() {
        // 2300×800 yüzey 1150 px genişliğe sığdırılır; ölçek 0,5 eşikte
        // kaldığından uygulanır ve yükseklik aynı oranda düşer.
        let görünür = GörünürAlan::yeni(alan(1_150.0, 4_000.0), false, true);
        let (genişlik, yükseklik) = görünür.yüzey(2_300.0, 800.0);
        assert_eq!(genişlik, 1_150.0);
        assert_eq!(yükseklik, 400.0);
        assert!((genişlik / yükseklik - 2_300.0 / 800.0).abs() < 1e-4);
    }

    #[test]
    fn yatay_sığdırma_dikey_taşmayı_umursamaz() {
        // Dikey kapalı: yükseklik alanı aşsa da yalnız genişlik belirleyicidir.
        let görünür = GörünürAlan::yeni(alan(1_150.0, 100.0), false, true);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (1_150.0, 400.0));
    }

    #[test]
    fn iki_eksen_açıkken_oran_bozulabilir() {
        // Bu, açıkça istenen tek durumdur: her eksen kendi alanına uyar.
        let görünür = GörünürAlan::yeni(alan(1_150.0, 200.0), true, true);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (1_150.0, 200.0));
    }

    #[test]
    fn iki_eksen_açıkken_sığan_eksen_büyütülmez() {
        // Genişlik zaten sığıyor; yalnız yükseklik çekilir.
        let görünür = GörünürAlan::yeni(alan(4_000.0, 200.0), true, true);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 200.0));
    }

    #[test]
    fn iki_eksen_kapalıyken_ham_boyut_korunur() {
        let görünür = GörünürAlan::yeni(alan(100.0, 100.0), false, false);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 800.0));
    }

    #[test]
    fn yatay_sığdırmada_da_en_küçük_ölçek_eşiği_geçerli() {
        // 2300 px genişlik 200 px alana sığdırılsa ölçek 0,087 olurdu.
        let görünür = GörünürAlan::yeni(alan(200.0, 4_000.0), false, true);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 800.0));
    }

    #[test]
    fn ölçülmemiş_alan_yüzeyi_bozmaz() {
        let görünür = GörünürAlan::yeni(alan(0.0, 0.0), true, false);
        assert_eq!(görünür.yüzey(2_300.0, 800.0), (2_300.0, 800.0));
    }
}
