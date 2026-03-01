# OmniLang Core API Reference

Dokumentasi ini mencakup daftar dekorator dan integrasi natif khusus yang tertanam dalam _engine_ kompilator.

---

## Decorators

Dekorator di OmniLang diletakkan di baris sebelum deklarasi entitas (misalnya, `fn`). Anotasi ini berinteraksi langsung secara internal terhadap *Compiler Rules* tanpa memerlukan modifikasi pada blok kode tubuh secara manual.

### `@oracle`

Dekorator ini mendelegasikan eksekusi pemanggilan fungsi kepada *inference engine* (misal: ONNX Runtime). Semua tipe statis parameter masukan fungsi dikonversi ke format yang sesuai sebelum menjajal Tensor Engine.

**Sintaks Khusus:**
```omnilang
@oracle(format: "onnx", model: "path/to/model.onnx", shape: "1,10|1,5")
fn perform_inference(inputA: [f64], inputB: [f64]) -> [[f64]];
```

**Konfigurasi Parameter:**
* `format` *(wajib)*: Format mesin model (saat ini hanya mendukung `onnx`).
* `model` *(wajib)*: Arah file relatif terhadap titik eksekusi atau absolut untuk dimuat saat runtime.
* `shape` *(opsional)*: Jika tipe dasar `Value::List(Value::Number)` di OmniLang berbentuk 1D Array, proxy `.dll/.so` ONNX butuh *tensor dimensionality*. Argumen `shape` mengatur urutan pemetaan untuk *n*-argument yang disekat oleh `|`, sedangkan elemen dimensi dipisahkan oleh koma `,`.

---

## `@mesh`

Dekorator ini menghubungkan *function dispatch* lokal dengan pemanggilan fungsi jarak jauh via jaringan Fabric TCP RPC OmniLang.

**Sintaks Khusus:**
```omnilang
@mesh(target: "192.168.1.100:8081")
fn fetch_weather_data(coord: [f64]) -> [f64];
```

**Konfigurasi Parameter:**
* `target` *(wajib)*: IP dan Port Node Worker tujuan.

**Keamanan & Otorisasi:**
RPC `@mesh` diluncurkan dengan menyertakan *X-Capability-Token*.
1. **Sisi Worker**: Dinyalakan dengan flag `--token "secret-key"`. Worker akan menolak request tanpa token yang cocok.
2. **Sisi Client**: Menggunakan mekanisme *Shadowing Token*. Jika terdapat konstanta global `const X_CAPABILITY_TOKEN: String` di dalam skrip, `ProgramEvaluator` akan otomatis menyisipkan nilai tersebut ke dalam protokol transport.

**Catatan Integrasi:**
Pada fase awal (v2.1), objek argument dan balasan direduksi dan diserialisasi dengan `serde_json` di *Core Engine*. Sangat disarankan untuk mendefinisikan *Primitive Tying* yang jelas agar konversi kembali deterministik. Tipe data yang kompleks dengan Closure / fungsi didalam parameter *List* belum didukung untuk RPC.
