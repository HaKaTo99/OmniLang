
# Latar Belakang Konseptual Terbentuknya OmniLang

Secara konseptual, OmniLang tidak "terbentuk" secara kebetulan; ia terbentuk dari serangkaian krisis dan ketidakpuasan kolektif terhadap kompromi yang dipaksakan oleh bahasa pemrograman mainstream.

Berikut adalah narasi konseptual tentang bagaimana OmniLang "terbentuk," fokus pada masalah yang ingin dipecahkannya:

### 📜 1. Krisis Fragmentasi (The Fragmentation Crisis)

**Masalah:**
Dalam proyek modern full-stack, pengembang dipaksa menggunakan tiga hingga empat bahasa yang berbeda:
- **JavaScript/TypeScript** (Frontend)
- **Go/Java/Python** (Backend)
- **C++/Rust** (Untuk *performance-critical core*)
- **SQL** (Basis data)

**Solusi OmniLang: A Unified Synthesis Platform**
OmniLang lahir dari ide bahwa satu bahasa seharusnya dapat menangani **semua domain** dengan optimal.
- **Frontend (Wasm):** OmniLang menargetkan WebAssembly (Wasm), menghilangkan ketergantungan pada *runtime* JavaScript yang seringkali lebih lambat.
- **Backend (Native):** Kompilasi ke *native binary* (melalui LLVM) memberikan kinerja superior untuk layanan mikro.

**Hasil:** Tidak perlu lagi *serialization/deserialization* yang kompleks antar-bahasa. *Struct* data bisa dibagikan secara *type-safe* antara *backend* dan *frontend*.

### 🛡️ 2. Krisis Keamanan Memori vs. Kinerja (The Safety vs. Speed Crisis)

**Masalah:**
Anda harus memilih salah satu:
- **C++:** Kinerja *Zero-Overhead* (cepat), tetapi penuh risiko **Kebocoran Memori** dan **Data Race** (tidak aman).
- **Java/Python:** Keamanan Memori (relatif aman) melalui *Garbage Collector* (GC), tetapi kinerja terhambat oleh *GC Pause* yang tak terduga (bisa menjadi lambat).
- **Rust:** Sangat aman, tetapi terkenal memiliki kurva pembelajaran yang sangat curam.

**Solusi OmniLang: The Hybrid Memory Model**
OmniLang terbentuk dengan mengadopsi janji keamanan Rust, tetapi mengintegrasikannya dalam sistem yang lebih fleksibel.
- **Mode `@ownership`:** Mengadopsi *Borrow Checker* untuk menjamin keamanan memori *compile-time* tanpa GC, memberikan kinerja setara C++.
- **Mode `@gc` (Default):** Menyediakan *Garbage Collector* modern yang efisien untuk produktivitas cepat, mirip dengan Go atau C#.

**Hasil:** Pengembang dapat memilih model memori yang paling sesuai untuk setiap bagian dari aplikasi, memaksimalkan keamanan di bagian kritis dan produktivitas di bagian lain.

### ⚡ 3. Krisis Konkurensi vs. Produktivitas (The Concurrency Crisis)

**Masalah:**
Dalam pengembangan backend modern (API, *microservices*):
- **Go:** Produktif dalam konkurensi (*Goroutine* & *Channels*), tetapi sistem tipenya sering dianggap kurang kaya dan keamanan memorinya kurang ketat dibanding Rust.
- **Python:** Terhalang oleh *Global Interpreter Lock* (GIL), yang menghambat paralelisme CPU sejati.

**Solusi OmniLang: Structured Concurrency dengan Keamanan**
OmniLang terbentuk dengan mengambil model konkurensi yang ringan dari Go, lalu membungkusnya dengan lapisan keamanan yang ketat.
- **`OmniRoutine` & `Channels`:** Model *thread* ringan yang didorong oleh *scheduler* canggih, ideal untuk I/O asinkron.
- **Jaminan *Borrow Checker*:** Data yang dikirim antar `OmniRoutine` melalui `Channels` tetap divalidasi oleh *Borrow Checker* dalam mode `@ownership`, memastikan tidak ada *Data Race* yang terjadi.

---

### 🎯 Kesimpulan: Tujuan Akhir

OmniLang tidak lahir sebagai ide baru yang radikal, tetapi sebagai **respons sintetis** terhadap ketidakmampuan bahasa-bahasa *existing* untuk memenuhi tiga tuntutan zaman ini secara simultan: **Kinerja Maksimal, Keamanan Absolut, dan Universalitas Platform.**

OmniLang terbentuk dari kebutuhan untuk menciptakan bahasa yang tidak memaksa pengembang untuk bernegosiasi dengan *trade-off* fundamental tersebut, melainkan menyediakan alat terbaik untuk setiap pekerjaan dalam satu paket yang kohesif.
