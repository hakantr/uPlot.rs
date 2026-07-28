// GPUI Web'in iş parçacığı havuzu SharedArrayBuffer ister; tarayıcı bunu
// yalnız sayfa `crossOriginIsolated` iken verir, o da COOP/COEP başlıklarını
// gerektirir. GitHub Pages özel yanıt başlığı gönderemediğinden bu service
// worker aradaki yanıtlara başlıkları kendisi ekler.
//
// Yerel `trunk serve` başlıkları zaten `Trunk.toml` üzerinden gönderir; bu
// dosya orada devreye girmez, çünkü sayfa hâlihazırda izole olur.

if (typeof window === "undefined") {
  // Service worker bağlamı.
  self.addEventListener("install", () => self.skipWaiting());
  self.addEventListener("activate", (olay) => olay.waitUntil(self.clients.claim()));

  self.addEventListener("fetch", (olay) => {
    // `only-if-cached` + `no-cors` birleşimi respondWith içinde geçersizdir.
    if (olay.request.cache === "only-if-cached" && olay.request.mode !== "same-origin") {
      return;
    }
    olay.respondWith(
      fetch(olay.request)
        .then((yanıt) => {
          // Opak yanıtın gövdesi ve başlıkları okunamaz; olduğu gibi geçer.
          if (yanıt.status === 0) {
            return yanıt;
          }
          const başlıklar = new Headers(yanıt.headers);
          başlıklar.set("Cross-Origin-Embedder-Policy", "require-corp");
          başlıklar.set("Cross-Origin-Opener-Policy", "same-origin");
          return new Response(yanıt.body, {
            status: yanıt.status,
            statusText: yanıt.statusText,
            headers: başlıklar,
          });
        })
        .catch((hata) => {
          console.error("COOP/COEP service worker getirme hatası:", hata);
          throw hata;
        }),
    );
  });
} else {
  // Sayfa bağlamı: gerekiyorsa worker'ı kaydet ve bir kez yeniden yükle.
  (async () => {
    if (window.crossOriginIsolated !== false) {
      // Sunucu başlıkları zaten göndermiş; worker'a gerek yok.
      return;
    }
    if (!window.isSecureContext || !("serviceWorker" in navigator)) {
      console.warn(
        "COOP/COEP service worker kaydedilemiyor; GPUI Web tek iş parçacığına düşer.",
      );
      return;
    }
    const kaynak = document.currentScript && document.currentScript.src;
    if (!kaynak) {
      return;
    }
    try {
      const kayıt = await navigator.serviceWorker.register(kaynak);
      // İlk yüklemede worker henüz denetimi almamış olur; başlıkların
      // uygulanması için tek bir yeniden yükleme gerekir.
      if (kayıt.active && !navigator.serviceWorker.controller) {
        window.location.reload();
      }
    } catch (hata) {
      console.error("COOP/COEP service worker kaydedilemedi:", hata);
    }
  })();
}
