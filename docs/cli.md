# OmniLang CLI Cheatsheet 🚀

## 📦 Perintah Dasar (Core)

| Perintah | Deskripsi | Contoh |
|----------|-----------|--------|
| `omnilang exec <file>` | Jalankan skrip OmniLang | `omnilang exec examples/loop_demo.omni` |
| `omnilang test <file>` | Jalankan skrip dengan konteks JSON | `omnilang test policy.omni --context data.json` |
| `omnilang lint <file>` | Periksa sintaksis tanpa eksekusi | `omnilang lint script.omni` |
| `omnilang metrics <file>` | Analisis performa dan latensi | `omnilang metrics ai_model.omni --format prometheus` |

## 🌐 Perintah Jaringan (Mesh)

| Perintah | Deskripsi | Contoh |
|----------|-----------|--------|
| `omnilang serve <file> --port <PORT>` | Jalankan sebagai worker daemon | `omnilang serve worker.omni --port 8081` |
| `omnilang serve <file> --token <TOKEN>` | Worker dengan keamanan token | `omnilang serve worker.omni --port 8081 --token "dummy-token"` |

## 🎨 Perintah Visual (TUI)

| Perintah | Deskripsi | Contoh |
|----------|-----------|--------|
| `omnilang exec <file> --visual` | Jalankan dengan antarmuka terminal visual | `omnilang exec match_demo.omni --visual` |

## 🔐 Keamanan Tambahan dalam Skrip (X-Capability)

| Fitur | Deskripsi | Contoh |
|-------|-----------|--------|
| `const X_CAPABILITY_TOKEN` | Definisikan token rahasia dalam skrip (Sisi Client) | `const X_CAPABILITY_TOKEN = "dummy-token";` |
| `@mesh(target: "ip:port")` | Anotasi dalam skrip fungsional untuk RPC | `@mesh(target: "127.0.0.1:8081") fn remote_call()` |

## 📊 Exit Codes (Sistem Deterministik)

| Kode | Arti |
|------|------|
| `0` | Sukses (Eksekusi bersih atau evaluasi kebijakan berhasil) |
| `1` | Error sintaksis/runtime (Kompilasi gagal atau parameter *shape* ONNX tidak cocok) |

*(Status kode lanjutan akan diformalisasikan pada versi berikutnya).*

## 🛠️ Tips Penggunaan

- **Penyatuan Output Mesh**: Daemon pekerja `serve` memancarkan log khusus `[MESH]` saat menerima atau menyalurkan balasan *tensor*.
- **Pendeteksi Token Pintar**: Skrip klien tidak perlu menyuntikkan token manual ke tiap pemanggilan fungsi `mesh`. Cukup deklarasikan variabel konstan `X_CAPABILITY_TOKEN` secara global, dan *Runtime Evaluator* secara implisit akan menyematkannya.
