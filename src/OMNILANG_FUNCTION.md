# 📘 OMNILANG GRAMMAR v1.0 (Semi-Formal Specification)

**Status:** Draft Stabil
**Tujuan:** Menjadikan OmniLang **konsisten, dapat dibaca manusia, dan dapat diproses mesin** tanpa kehilangan makna.

---

## 1. Prinsip Desain Grammar

OmniLang **bukan bahasa pemrograman**, maka grammarnya:

1. **Deklaratif**, bukan imperatif
2. **Berbasis makna (semantic-first)**, bukan sintaks
3. **Toleran terhadap variasi**, tapi **ketat pada struktur**
4. **Aman secara etika by design**

> Grammar ini sengaja **semi-formal**: cukup ketat untuk AI, cukup manusiawi untuk manusia.

---

## 2. Unit Dasar (Lexical Units)

### 2.1 Keyword Wajib (Reserved Words)

Keyword berikut **tidak boleh diganti sinonim**:

```
INTENT
ACTOR
CONTEXT
ASSUMPTION
RULE
CONSTRAINT
IMPACT
TRACE
REVIEW
```

✔ Case-insensitive
✔ Disarankan UPPERCASE untuk interoperabilitas

---

### 2.2 Separator & Format

* `:` → pemisah label dan isi
* `-` → item list
* `>` → threshold / relasi
* `< >` → placeholder (opsional)

Contoh sah:

```
INTENT: Menjaga keselamatan manusia
```

---

## 3. Struktur Dokumen (Canonical Grammar Order)

Grammar inti **harus mengikuti urutan berikut**:

```
INTENT
ACTOR
CONTEXT
[ASSUMPTION]
RULE
CONSTRAINT
IMPACT
TRACE
[REVIEW]
```

Aturan:

* `[ ]` = opsional
* Jika bagian dihilangkan → **tidak ditulis sama sekali**
* **Urutan tidak boleh dilanggar**

---

## 4. Grammar per Komponen (Formal-ish)

### 4.1 INTENT

**Fungsi:** Tujuan bernilai manusia
**Batasan:**

* Tidak boleh mengandung metode teknis
* Tidak boleh lebih dari 2 kalimat

```
INTENT := "INTENT:" <Human-Centric Objective>
```

Contoh valid:

```
INTENT: Melindungi keselamatan dan martabat pekerja
```

---

### 4.2 ACTOR

```
ACTOR :=
"ACTOR:"
  "- Primary:" <Role>
  ["- Secondary:" <Role>]
```

Aturan:

* Role → fungsi, bukan nama individu
* Minimal 1 Primary

---

### 4.3 CONTEXT

```
CONTEXT :=
"CONTEXT:"
  "- Domain:" <Domain>
  "- Lokasi:" <Location>
  "- Fase:" <Phase>
```

Boleh ditambah atribut lain, **tidak boleh menghapus tiga inti**.

---

### 4.4 ASSUMPTION (Opsional)

```
ASSUMPTION :=
"ASSUMPTION:"
  "-" <Assumption Statement>
```

Aturan:

* Harus eksplisit
* Tidak boleh normatif

---

### 4.5 RULE

**Ini inti logika OmniLang.**

```
RULE :=
"RULE:"
  "- IF" <Condition>
  "- THEN" <Action>
```

Aturan keras:

* Wajib IF–THEN
* Tidak boleh nested
* Tidak boleh ambigu waktu

---

### 4.6 CONSTRAINT

```
CONSTRAINT :=
"CONSTRAINT:"
  "- Legal:" <Law / Regulation>
  "- Ethical:" <Ethical Boundary>
  "- Technical:" <Technical Limit>
```

Minimal **1 dari 3 wajib ada**.

---

### 4.7 IMPACT

```
IMPACT :=
"IMPACT:"
  "- Benefit:" <Positive Outcome>
  "- Risk:" <Negative Outcome>
  "- Trade-off:" <Balance>
```

Tujuan: **anti-utopia**, anti klaim sempurna.

---

### 4.8 TRACE

```
TRACE :=
"TRACE:"
  "- Moral:" <Value>
  "- Regulation:" <Reference>
  "- Evidence:" <Data / Experience>
```

TRACE = **anti black-box**.

---

### 4.9 REVIEW (Opsional)

```
REVIEW :=
"REVIEW:"
  "- Interval:" <Timeframe>
  "- Criteria:" <Evaluation Metric>
```

---

## 5. Validity Rules (Validation Layer)

Sebuah dokumen OmniLang **TIDAK VALID** jika:

* INTENT bertentangan dengan CONSTRAINT
* RULE tidak dapat ditelusuri ke TRACE
* ACTOR tidak memiliki otoritas kontekstual
* IMPACT hanya berisi benefit tanpa risk

---

## 6. Machine Readiness (AI-Friendly)

Grammar ini **langsung bisa dipetakan** ke:

* JSON
* YAML
* Prompt AI
* Policy Engine
* Audit Log

Contoh mapping singkat:

```
INTENT → system_goal
RULE → decision_logic
CONSTRAINT → guardrail
TRACE → explainability
```

---

## 7. Status Grammar v1.0

✔ Konsisten
✔ Dapat diaudit
✔ Dapat diajarkan
✔ Dapat diterjemahkan ke mesin
✔ Tidak bergantung vendor