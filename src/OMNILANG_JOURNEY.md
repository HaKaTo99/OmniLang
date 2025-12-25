# 🚀 Perjalanan Menciptakan OmniLang: Sebuah Catatan Pengembangan

Dokumen ini merangkum seluruh diskusi dan proses pengembangan yang telah kita lalui bersama dalam membangun fondasi bahasa pemrograman **OmniLang** dan **OmniLang Studio**.

---

### Babak 1: Konseptualisasi & Fondasi Awal

Perjalanan kita dimulai dengan sebuah ide ambisius: menciptakan **OmniLang**, sebuah bahasa pemrograman universal. Untuk mewujudkan ini, kita tidak langsung membuat *compiler*, melainkan sebuah "rumah" untuk bahasa ini: **OmniLang Studio**.

Struktur awal proyek ini dibangun menggunakan teknologi modern:
- **Next.js & React:** Untuk membangun antarmuka web yang interaktif.
- **TypeScript:** Untuk memastikan kode yang kita tulis aman dan andal.
- **Tailwind CSS & ShadCN:** Untuk desain visual yang modern dan konsisten.
- **Genkit:** Sebagai fondasi untuk fitur-fitur berbasis AI.
- **Rust (`ast.rs`, `checker.rs`):** Sebagai prototipe *backend* untuk analisis kode OmniLang, membuktikan bahwa konsep keamanan dan *type system*-nya solid.

### Babak 2: Menghadapi Tantangan Teknis (Error 404)

Setelah fondasi awal terbentuk, kita menghadapi tantangan terbesar pertama: **Error 404 - Halaman Tidak Ditemukan**. Aplikasi kita, meskipun memiliki file `src/app/page.tsx`, tidak dapat dirender oleh Next.js.

Ini adalah fase *debugging* yang panjang dan penuh pelajaran:
1.  **Analisis Awal:** Kita mencurigai adanya masalah pada konfigurasi Next.js, struktur file, atau duplikasi komponen.
2.  **Percobaan & Kesalahan:** Kita mencoba berbagai perbaikan, mulai dari membersihkan file duplikat, memperbaiki `tailwind.config.ts`, hingga memastikan dependensi seperti `eslint` terpasang. Setiap langkah membawa kita lebih dekat ke akar masalah.
3.  **Penemuan Kunci:** Akhirnya, kita menyadari bahwa masalahnya bukan pada satu file, melainkan pada **ketidakkonsistenan** antara struktur file, konfigurasi Tailwind, dan definisi tema global. Ini adalah pelajaran penting: dalam pengembangan modern, semua bagian harus selaras.
4.  **Solusi Final (Reset Menyeluruh):** Kita mengambil langkah berani untuk melakukan "reset total" pada file-file inti (`page.tsx`, `layout.tsx`, `globals.css`) ke bentuknya yang paling sederhana dan stabil. Ini berhasil menyelesaikan error 404 secara tuntas.

### Babak 3: Penyempurnaan Fungsional & Visual

Setelah aplikasi berhasil berjalan, kita fokus pada penyempurnaan:

1.  **Klarifikasi Tujuan Proyek:** Menjawab pertanyaan krusial "apakah ini sudah bisa digunakan?", kita menyadari pentingnya mengelola ekspektasi. Kita menambahkan bagian **"Status Proyek & Kesiapan"** pada dokumentasi untuk menjelaskan bahwa OmniLang Studio saat ini adalah sebuah *playground* atau simulator, bukan *compiler* yang siap produksi.

2.  **Membangun Identitas Visual:** Kita mengubah skema warna aplikasi secara menyeluruh, dari tema gelap standar menjadi tema yang lebih bersih dan modern sesuai visi desain OmniLang:
    - Latar Belakang: Abu-abu terang (`#F0F0F0`)
    - Warna Primer: Indigo (`#4B0082`)
    - Warna Aksen: Magenta (`#FF00FF`)
    Perubahan ini langsung memberikan aplikasi ini identitas visual yang unik dan profesional.

3.  **Menambahkan Fitur Esensial:** Berdasarkan kebutuhan praktis, kita menambahkan fungsionalitas **"Download"**, yang memungkinkan pengguna menyimpan kode mereka ke komputer lokal. Ini adalah langkah kecil namun penting untuk membuat studio ini lebih berguna.

### Kesimpulan & Langkah Berikutnya

Dari awal yang konseptual, melalui badai *debugging* yang menantang, hingga penyempurnaan fungsional dan visual, kita telah berhasil membangun sebuah fondasi yang kokoh untuk **OmniLang Studio**.

Perjalanan ini adalah bukti nyata dari proses pengembangan perangkat lunak modern: sebuah siklus berkelanjutan antara **ide, implementasi, masalah, solusi, dan penyempurnaan**.

Terima kasih atas kolaborasi dan kesabaran Anda, Herman. Fondasi ini siap untuk dikembangkan lebih lanjut.
