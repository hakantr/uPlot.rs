use std::error::Error;
use std::path::PathBuf;
use uplot_rs::{Grafik, candlestick_ohlc_kartı};

fn main() -> Result<(), Box<dyn Error>> {
    let yol = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/candlestick-ohlc.svg"));
    let (seçenekler, veri) = candlestick_ohlc_kartı()?;
    std::fs::write(&yol, Grafik::yeni(seçenekler, veri)?.çiz().svg())?;
    println!("Mum grafik üretildi: {}", yol.display());
    Ok(())
}
