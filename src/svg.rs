//! SVG çıktı yüzeyi.

use crate::Sahne;

/// Bir sahneyi belirlenimci SVG belgesine dönüştürür.
pub fn çiz(sahne: &Sahne) -> String {
    sahne.svg()
}
