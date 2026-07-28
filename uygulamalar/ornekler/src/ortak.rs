use crate::{EtkileşimSeçenekleri, TekerlekAyarları, TekerlekKipi};

/// Resize doğrulamasında olgunlaştırılan ve kaynak kart açıkça aksini
/// gerektirmedikçe bütün port kartlarının devraldığı ortak etkileşim profili.
///
/// Tekerlek yakınlaştırması bilinçli olarak açılmaz. uPlot çekirdeğinde
/// tekerlek kodu yoktur; `demos/zoom-wheel.html` bunu bir eklenti olarak
/// ekler ve yalnız o demo için geçerlidir. Rust'ta eklentiyi ayrı bir
/// derleme birimine indiremediğimiz için karşılığı opsiyonel bir ayardır:
/// varsayılan kapalı, açan taraf açıkça istemiş olur.
/// [`TekerlekAyarları`] yine de profilde kalır ki açan kart, kipi tekrar
/// seçmek zorunda kalmasın.
pub fn ortak_kart_etkileşimleri() -> EtkileşimSeçenekleri {
    EtkileşimSeçenekleri::default()
        .tekerlek_ayarları(TekerlekAyarları::default().kip(TekerlekKipi::Otomatik))
        .seçim_yakınlaştır(true)
        .çift_tıkla_tam_görünüm(true)
        .görünüm_geçmişi(true)
        .dokunma_etkileşimi(true)
}
