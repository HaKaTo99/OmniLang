# 📜 Spesifikasi OmniLang v2.1: Distributed Fabric Era
*Status: Stable / Production-Ready*

Dokumen ini mendefinisikan ekstensi fungsional untuk OmniLang v1.x, dimulai dengan Pattern Matching.

## 1. Pattern Matching (`MATCH`)

Ekspresi `match` memungkinkan pengambilan keputusan berdasarkan pola data, menggantikan rantai `if-else` yang kompleks.

### Sintaksis
```ebnf
MatchExpr ::= "match" Expression "{" (MatchArm ",")* MatchArm? "}"
MatchArm  ::= Pattern "=>" Expression
Pattern   ::= Literal | Identifier | "_"
```

### Contoh
```omni
let status = 200;
let message = match status {
    200 => "OK",
    404 => "Not Found",
    500 => "Server Error",
    _   => "Unknown"
};
```

### Semantik
1.  Evaluator mengevaluasi `Expression` utama (scrutinee).
2.  Evaluator mencocokkan hasil dengan setiap `Pattern` dari atas ke bawah.
3.  Jika pola cocok:
    - **Literal**: Nilai sama persis.
    - **Identifier**: Nilai diikat ke variabel baru (binding).
    - **Wildcard (`_`)**: Selalu cocok (catch-all).
4.  Ekspresi di sebelah kanan `=>` dieksekusi.
5.  Jika tidak ada yang cocok, terjadi runtime error (non-exhaustive match), kecuali ada `_`.

## 2. Roadmap Berikutnya (Harmonious+)
- Lambda Expressions `|x| x + 1`
- Higher Order Functions (`map`, `filter`)
- **BCI Stream Processing**: Sintaksis native untuk decoding sinyal EEG.
- **MMUI Orchestration**: Event handling gabungan untuk antarmuka multimodal.
