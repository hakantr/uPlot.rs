import { readFileSync, writeFileSync } from "node:fs";
import vm from "node:vm";

const kök = new URL("../../../uPlot/demos/", import.meta.url);
const ham = JSON.parse(readFileSync(new URL("data/taxi-trips.json", kök), "utf8"));
const bağlam = { data: ham.slice() };
vm.createContext(bağlam);
vm.runInContext(readFileSync(new URL("lib/sgg.js", kök), "utf8") + "; this.sggOut = sgg(data, 1, {windowSize: 101});", bağlam);
vm.runInContext(readFileSync(new URL("lib/ASAP-optimized.js", kök), "utf8") + "; this.asapOut = smooth(data.slice(), 150);", bağlam);
vm.runInContext(`this.movingOut = (() => { let out=Array(data.length).fill(null),sum=0,count=0; for(let i=0;i<data.length;i++){sum+=data[i];count++;if(i>299){sum-=data[i-300];count--;}out[i]=sum/count;} return out; })();`, bağlam);

const ondalık = (değer) => Number.isInteger(değer) ? `${değer}.0` : `${değer}`;
const dizi = (ad, değerler, nitelik = "const", test = false) => `${test ? "#[cfg(test)]\n" : ""}pub ${nitelik} ${ad}: [f64; ${değerler.length}] = [\n${değerler.map(v => `    ${ondalık(v)},`).join("\n")}\n];\n`;
const indeksler = [0, 1, 50, 100, 900, 1800, 2700, 3599];
const çiftler = (ad, değerler) => `#[cfg(test)]\npub const ${ad}: [(usize, f64); ${indeksler.length}] = [\n${indeksler.map(i => `    (${i}, ${ondalık(değerler[i])}),`).join("\n")}\n];\n`;
const çıktı = "// Resmî taxi-trips.json ve JS algoritmalarından mekanik üretilir.\n" +
  dizi("TAXI_TRIPS", ham, "static") + dizi("ASAP_REFERENCE", bağlam.asapOut, "const", true) +
  çiftler("SGG_REFERENCE", Array.from(bağlam.sggOut)) + çiftler("MOVING_REFERENCE", bağlam.movingOut) +
  `#[cfg(test)]\npub const SGG_REFERENCE_SUM: f64 = ${Array.from(bağlam.sggOut).reduce((a,b)=>a+b,0)};\n` +
  `#[cfg(test)]\npub const MOVING_REFERENCE_SUM: f64 = ${bağlam.movingOut.reduce((a,b)=>a+b,0)};\n`;
writeFileSync(new URL("../../src/kart/veri/data_smoothing_kaynak.rs", import.meta.url), çıktı);
