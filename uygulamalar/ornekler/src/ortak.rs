use crate::{EtkileşimSeçenekleri, TekerlekAyarları, TekerlekKipi};

/// Resize doğrulamasında olgunlaştırılan ve kaynak kart açıkça aksini
/// gerektirmedikçe bütün port kartlarının devraldığı ortak etkileşim profili.
pub fn ortak_kart_etkileşimleri() -> EtkileşimSeçenekleri {
    EtkileşimSeçenekleri::default()
        .tekerlek_etkileşimi(true)
        .tekerlek_ayarları(TekerlekAyarları::default().kip(TekerlekKipi::Otomatik))
        .seçim_yakınlaştır(true)
        .çift_tıkla_tam_görünüm(true)
        .görünüm_geçmişi(true)
        .dokunma_etkileşimi(true)
}
