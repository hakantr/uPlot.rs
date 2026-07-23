import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import vm from "node:vm";

const kök = resolve(import.meta.dirname, "../..");
const kaynakKök = resolve(kök, "../uPlot");
const demo = readFileSync(resolve(kaynakKök, "demos/points.html"), "utf8");
const randomWalkKaynağı = readFileSync(
  resolve(kaynakKök, "demos/lib/randomWalk.js"),
  "utf8",
);

let durum = 0x504f494e;
function rastgele() {
  durum = (durum + 0x6d2b79f5) >>> 0;
  let değer = durum;
  değer = Math.imul(değer ^ (değer >>> 15), değer | 1);
  değer ^= değer + Math.imul(değer ^ (değer >>> 7), değer | 61);
  return ((değer ^ (değer >>> 14)) >>> 0) / 4294967296;
}

const kanıtMath = Object.create(Math);
kanıtMath.random = rastgele;
const bağlam = vm.createContext({ Math: kanıtMath });
vm.runInContext(randomWalkKaynağı, bağlam, {
  filename: "demos/lib/randomWalk.js",
});

const ilk = [
  vm.runInContext("randomWalk(50, 200, 0, 100)", bağlam),
  vm.runInContext("randomWalk(50, 200, 0, 100)", bağlam),
  vm.runInContext("randomWalk(50, 200, 0, 100)", bağlam),
  vm.runInContext("randomWalk(75, 200, 0, 100)", bağlam),
];
for (const indeks of [1, 100, 102, 130, 133, 198]) {
  ilk[3][indeks] = null;
}
const yoğun180 = vm.runInContext("randomWalk(50, 180, 0, 100)", bağlam);

const seyrekEşleşme = demo.match(/let vals1 = '([,0-9]+)'\s*\.split/);
if (!seyrekEşleşme) {
  throw new Error("points.html içindeki seyrek veri dizisi bulunamadı");
}
const seyrek = seyrekEşleşme[1]
  .split(",")
  .map((değer) => (değer === "" ? null : Number(değer)));

const çıktı = {
  kanıt_tohumu: "0x504f494e",
  ilk,
  yoğun180,
  seyrek,
};
writeFileSync(
  resolve(kök, "src/kart/veri/points.json"),
  `${JSON.stringify(çıktı)}\n`,
);
