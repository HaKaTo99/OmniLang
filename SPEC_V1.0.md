# 📜 Spesifikasi OmniLang v1.0: Universal Intent Language

Dokumen ini mendefinisikan standar bahasa OmniLang v1.0, yang berevolusi dari bahasa pemrograman umum menjadi **Universal Intent Language**. Fokus utama versi ini adalah standarisasi cara manusia dan sistem mendefinisikan niat (*intent*), kebijakan (*policy*), dan aturan (*rule*) secara formal, aman, dan dapat diverifikasi.

## 1. Filosofi Desain
OmniLang dirancang untuk menjadi jembatan antara bahasa alami manusia yang ambigu dan kode mesin yang kaku. 
- **Verifiable**: Setiap pernyataan harus dapat diperiksa validitasnya.
- **Safety**: Menggunakan prinsip *Ownership & Borrowing* untuk menjamin keamanan memori dan konkurensi.
- **Ordered**: Struktur dokumen mengikuti urutan logis (*Canonical Order*) untuk memudahkan parsing dan pemahaman.

## 2. Struktur Dasar (Canonical Order)
Setiap dokumen OmniLang (`.omni`) harus mengikuti urutan seksi berikut:

1.  **INTENT**: Deskripsi tujuan utama kebijakan.
2.  **ACTOR**: Definisi entitas yang terlibat (Primary/Secondary).
3.  **CONTEXT**: Lingkup operasional (Domain, Lokasi, Fase).
4.  **ASSUMPTION** (Opsional): Kondisi yang dianggap benar.
5. - **RULE**: Definisi aturan logika tindakan. Mendukung konstruksi standar dan perulangan:
    - `IF <Kondisi> THEN <Aksi>`
    - `FOR <Iterator> IN <Koleksi> { <Aturan_Bersarang> }`
    - `WHILE <Kondisi> { <Aturan_Bersarang> }`
6. - **CONSTRAINT**: Batasan sistem (Legal, Ethical, Technical).
7.  **IMPACT**: Estimasi manfaat, risiko, atau *trade-off*.
8.  **TRACE**: Referensi ke regulasi, moral, atau bukti pendukung.
9.  **REVIEW** (Opsional): Catatan untuk tinjauan masa depan.

## 3. Tata Bahasa (Grammar)

### 3.1 Kata Kunci Utama
Setiap seksi dimulai dengan nama seksi dalam huruf kapital diikuti tanda titik dua (`:`).

```ebnf
Section = Header ":" Content ;
Header  = "INTENT" | "ACTOR" | "CONTEXT" | "ASSUMPTION" | "IMPACT" | "TRACE" | "REVIEW" ;
RuleSection      ::= "RULE:" (StandardRule | ForLoop | WhileLoop)+
StandardRule     ::= "- IF" Text "THEN" Text
ForLoop          ::= "- FOR" Identifier "IN" Identifier "{" (StandardRule | ForLoop | WhileLoop)+ "}"
WhileLoop        ::= "- WHILE" Text "{" (StandardRule | ForLoop | WhileLoop)+ "}"
ConstraintSection ::= "CONSTRAINT:" (ConstraintItem)+
```

### 3.2 List Item
Konten di bawah seksi biasanya berupa daftar poin yang dimulai dengan tanda hubung (`-`).

```omni
ACTOR:
- Primary: RobotArm
- Secondary: SafetySupervisor
```

### 3.3 Logika Aturan (Rule Logic)
Aturan didefinisikan dengan pola IF-THEN.

```ebnf
RuleEntry = "- IF" Condition "THEN" Action ;
```

## 4. Sistem Tipe & Keamanan
Meskipun v1.0 berfokus pada "Intent", sistem tipe di backend (Rust) tetap melakukan validasi ketat:
- **Int, Float, Bool, String**: Tipe data dasar untuk kondisi.
- **Ownership**: Objek yang didefinisikan memiliki pemilik tunggal, mencegah ambiguitas dalam eksekusi kebijakan.
- **Borrowing**: Referensi (`&`) digunakan untuk membaca data status tanpa mengubahnya.

## 5. Contoh Implementasi v1.0

```omni
INTENT: Memastikan keamanan operasional drone di area publik
ACTOR:
- Primary: DronePilot
- Secondary: AirTrafficControl
CONTEXT:
- Domain: Logistics
- Lokasi: UrbanArea
- Fase: Flight
RULE:
- IF BatteryLevel < 15% THEN ReturnToHome
- IF ObstacleDistance < 2m THEN EmergencyBrake
CONSTRAINT:
- Legal: FAA Regulation Part 107
IMPACT:
- Benefit: Menghindari kecelakaan
- Risk: Keterlambatan pengiriman
TRACE:
- Regulation: https://www.faa.gov/uas
```

---
*Dokumen ini merupakan standar resmi untuk OmniLang v1.0. Versi sebelumnya (v0.1) sudah dinyatakan usang (deprecated).*
