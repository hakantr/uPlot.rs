import { readFileSync, writeFileSync } from "node:fs";

const kaynakYolu = "../uPlot/demos/wind-direction.html";
const hedefYolu = "src/kart/veri/wind_direction.rs";
const kaynak = readFileSync(kaynakYolu, "utf8");
const veriBaşlangıcı = kaynak.indexOf("let data = ");
const diziBaşlangıcı = kaynak.indexOf("[", veriBaşlangıcı);

if (veriBaşlangıcı < 0 || diziBaşlangıcı < 0) {
  throw new Error("wind-direction veri dizisi bulunamadı");
}

let derinlik = 0;
let diziBitişi = -1;
for (let indeks = diziBaşlangıcı; indeks < kaynak.length; indeks += 1) {
  if (kaynak[indeks] === "[") {
    derinlik += 1;
  } else if (kaynak[indeks] === "]") {
    derinlik -= 1;
    if (derinlik === 0) {
      diziBitişi = indeks + 1;
      break;
    }
  }
}

if (diziBitişi < 0) {
  throw new Error("wind-direction veri dizisi kapanmıyor");
}

const veri = JSON.parse(kaynak.slice(diziBaşlangıcı, diziBitişi));
if (
  veri.length !== 4
  || veri.some((seri) => seri.length !== veri[0].length)
  || veri[0].some((değer, indeks) => indeks > 0 && değer - veri[0][indeks - 1] !== 3600)
) {
  throw new Error("wind-direction kaynak veri yapısı beklenen biçimde değil");
}

const rustSayısı = (değer) => Number.isInteger(değer) ? `${değer}.0` : `${değer}`;
const sayılar = (dizi) => dizi.map(rustSayısı).join(", ");
const seçenekler = (dizi) => dizi
  .map((değer) => değer === null ? "None" : `Some(${rustSayısı(değer)})`)
  .join(", ");
const uzunluk = veri[0].length;
const çıktı = `//! \`demos/wind-direction.html\` içindeki birebir saatlik kaynak veri.
#![allow(clippy::approx_constant)]

pub const NOKTA_SAYISI: usize = ${uzunluk};
pub const ZAMANLAR: [f64; NOKTA_SAYISI] = [${sayılar(veri[0])}];
pub const SICAKLIKLAR: [Option<f64>; NOKTA_SAYISI] = [${seçenekler(veri[1])}];
pub const RÜZGAR_HIZLARI: [Option<f64>; NOKTA_SAYISI] = [${seçenekler(veri[2])}];
pub const RÜZGAR_YÖNLERİ: [Option<f64>; NOKTA_SAYISI] = [${seçenekler(veri[3])}];
`;

writeFileSync(hedefYolu, çıktı);
console.log(`${hedefYolu}: ${uzunluk} nokta aktarıldı`);
