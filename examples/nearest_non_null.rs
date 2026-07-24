use uplot_rs::{
    Grafik, NearestNonNullÖrneği, NullAtlamaYönü, UplotHatası, nearest_non_null_kartı
};

fn main() -> Result<(), UplotHatası> {
    let (seçenekler, veri) = nearest_non_null_kartı(NearestNonNullÖrneği::ÖncekiNullOlmayan)?;
    let grafik = Grafik::yeni(seçenekler, veri)?;
    println!(
        "seçilen indeks: {:?}",
        grafik.en_yakın_null_olmayan_indeks(0.6, 0, NullAtlamaYönü::Önceki)
    );
    Ok(())
}
