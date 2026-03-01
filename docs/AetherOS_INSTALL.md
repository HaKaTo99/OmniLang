# Panduan Instalasi OmniLang pada AetherOS

Selamat datang di ekosistem AetherOS! Menginstal OmniLang pada AetherOS dirancang agar semudah menginstal Microsoft Office pada Windows 11 atau Inkscape pada Ubuntu.

Berikut adalah tahapan instalasi OmniLang melalui tiga metode utama:

## 1. Melalui Aether Store (Metode GUI)
*Cocok bagi pengguna yang menyukai antarmuka visual (seperti Microsoft Store).*

1.  **Buka Aether Store**: Klik ikon galaksi pada sidebar atau ketuk dua kali pada desktop.
2.  **Cari OmniLang**: Gunakan bilah pencarian di bagian atas dan ketik "OmniLang".
3.  **Klik Instal**: Pilih hasil pencarian "OmniLang Development Suite" dan klik tombol **Instal**.
4.  **Autentikasi**: Sistem akan meminta konfirmasi. Gunakan **Neural Link (BCI)** atau tempelkan **Quantum Key** Anda pada sensor.
5.  **Selesai**: OmniLang akan otomatis terkonfigurasi di sistem Anda.

## 2. Melalui Aether Terminal (Metode CLI)
*Cocok bagi pengembang (seperti menggunakan `apt` di Ubuntu atau `brew` di macOS).*

1.  **Buka Aether Terminal**: Tekan `Ctrl + Alt + T`.
2.  **Jalankan Perintah**: Ketik perintah berikut:
    ```bash
    apm install omnilang
    ```
3.  **Proses**: Aether Package Manager (APM) akan mengunduh paket `.apkg` dan mengekstraknya ke direktori `/system/bin`.
4.  **Verifikasi**: Setelah selesai, cek instalasi dengan:
    ```bash
    omc --version
    ```

## 3. Melalui AI Intent (Metode Natural Interface)
*Fitur native AetherOS menggunakan NUI (Natural User Interface).*

1.  **Panggil Aether AI**: Katakan "Hey Aether" atau fokuskan pikiran Anda melalui Neural Link.
2.  **Berikan Perintah**: Cukup katakan atau pikirkan:
    > "Install OmniLang development tools."
3.  **Konfirmasi**: Aether AI akan memberikan ringkasan instalasi. Katakan "Confirm" atau "Lakukan".
4.  **Otomatisasi**: Sistem akan melakukan pengunduhan dan pengaturan environment secara otomatis di latar belakang.

---

## Memulai Penggunaan
Setelah terinstal, Anda bisa langsung membuat file game pertama Anda:
1. Buka editor teks bawaan AetherOS.
2. Buat file `hello.omni`.
3. Jalankan melalui terminal: `omc hello.omni --visual`.

🚀 **AetherOS + OmniLang: The Future of Distributed Intelligence is Secured.**
