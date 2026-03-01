# 10 Minutes to ONNX: OmniLang AI Integration

OmniLang dirancang dari awal untuk membuat AI terintegrasi sealami pemanggilan fungsi biasa. Visi "Satu Bahasa untuk Memerintah Semuanya" diwujudkan melalui **Native ONNX Proxy** — kemampuan untuk menempelkan model kecerdasan buatan kelas industri langsung ke dalam _toolchain_ perangkat lunak.

Tutorial singkat ini memandu Anda membuat skrip **Klasifikasi Gambar (Image Classification)** dengan arsitektur MobileNet.

---

## 🏗️ 1. Persiapan Model ONNX
Pertama, kita membutuhkan model `.onnx`. Untuk contoh ini (karena keterbatasan ukuran repositori), kita akan mensimulasikan model ringan **MobileNet** yang telah diekspor ke ekstensio ONNX (`dummy_mobilenet.onnx`).

Model ini menerima data 1 dimensi berukuran `10` elemen (sebagai representasi _features extraction_ visual) dan akan mengeluarkan probabilitas untuk `3` buah kelas (Cat, Dog, Bird).

Letakkan model Anda pada folder bebas, contohnya: `examples/dummy_mobilenet.onnx`.

---

## ✍️ 2. Kode OmniLang Anda
Mari kita tulis skrip logika klasifikasi pada file baru (`examples/onnx_mobilenet.omni`).

### A. Mendeklarasikan Fungsi AI
Gunakan dekorator `@oracle` tepat sebelum deklarasi fungsi tanpa tubuh `fn ...;`. Dekorator ini berfungsi untuk mengalihkan eksekusi (*intercept*) ke ONNX Runtime internal OmniLang.

```omnilang
module onnx_mobilenet_test {
    // Input: shape "1,10", output "1,3" -> list 2 dimensi
    @oracle(format: "onnx", model: "examples/dummy_mobilenet.onnx", shape: "1,10")
    fn classify_image(features: [f64]) -> [[f64]];
```
> [!TIP]
> Parameter *shape* (seperti `"1,10"`) memberitahukan mesin inferensi ukuran dimensi array statis secara deterministik, sehingga mesin bisa mem-bypass kesalahan run-time dengan proteksi kuat.

### B. Memanggil Inferensi (Inference Call)
Sihir OmniLang terletak pada langkah ini: Anda tidak perlu menggunakan library FFI atau SDK Machine Learning rumit. Panggil saja fungsi tersebut layaknya fungsi regular!

```omnilang
    const main: i32 = {
        print("--- 10 Minutes to ONNX: OmniLang Image Classification ---");
        
        // 1. Ekstraksi fitur kamera (disimulasikan sebagai list 10 angka)
        print("[SENSOR] Capturing image features...");
        let image_features = [0.12, 0.44, 0.23, 0.88, 0.55, 0.91, 0.32, 0.61, 0.77, 0.05];
        
        // 2. Inference Real-Time (Eksekusi Engine)
        print("[AI] Running MobileNet Inference via ONNX Proxy...");
        let logits_batch = classify_image(image_features);
```

### C. Mengolah Hasil / Keputusan Tindakan (ArgMax)
Keluaran dari ONNX selalu mempertahankan hirarkinya. Shape `[1, 3]` dikonversi menjadi *List(List(Number))*. Kita hanya perlu mengakses elemen pertamanya.

```omnilang
        let class_probs = logits_batch[0];
        print("Class Probabilities (Cat, Dog, Bird):");
        print(class_probs);
        
        let cat_prob = class_probs[0];
        let dog_prob = class_probs[1];
        let bird_prob = class_probs[2];
        
        // Menentukan label teratas (ArgMax manual sederhana)
        if dog_prob > cat_prob {
            if dog_prob > bird_prob {
                print(">> PREDICTION: DOG Detected! (Confidence High)");
            } else {
                print(">> PREDICTION: BIRD Detected!");
            }
        } else {
            if cat_prob > bird_prob {
                 print(">> PREDICTION: CAT Detected!");
            } else {
                 print(">> PREDICTION: BIRD Detected!");
            }
        }
        
        0
    };
}
```

---

## ⚡ 3. Menjalankan Kode
Eksekusi di CLI menggunakan eksekutor terintegrasi:

```bash
$ omnilang test examples/onnx_mobilenet.omni
```

**Output Terminal:**
```console
--- 10 Minutes to ONNX: OmniLang Image Classification ---
[SENSOR] Capturing image features...
[AI] Running MobileNet Inference via ONNX Proxy...
[ORACLE TIMER] Inference ran in 6.01ms
Class Probabilities (Cat, Dog, Bird):
List([Number(0.05), Number(0.85), Number(0.1)])
>> PREDICTION: DOG Detected! (Confidence High)
--- Classification Test PASSED ---
Test PASSED: examples\onnx_mobilenet.omni
```

## 🎉 Selesai!
Hanya dengan beberapa baris notasi `@oracle` deklaratif, Anda telah mengintegrasikan komputasi AI kelas berat langsung ke dalam ekosistem sistem operasi xAetherOS yang super gesit. 

**Langkah Lanjut:** 
Ingin melakukan ini secara jarak jauh? Simak juga **[Tutorial Aplikasi Terdistribusi dan Mesh Fabric](terdistribusi.md)** untuk meneruskan _bounding boxes_ dari Node AI pekerja ke Node Sensor IoT Anda di tepi (_edge_).
