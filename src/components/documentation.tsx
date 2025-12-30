import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"

const CodeBlock = ({ children }: { children: React.ReactNode }) => (
  <pre className="mt-2 rounded-md bg-secondary p-4 overflow-x-auto">
    <code className="font-code text-sm text-secondary-foreground">{children}</code>
  </pre>
);

export function Documentation() {
  return (
    <div className="h-full">
      <h2 className="text-2xl font-bold font-headline mb-4">OmniLang Docs</h2>
      <Accordion type="single" collapsible defaultValue="item-0" className="w-full">
        <AccordionItem value="item-0">
          <AccordionTrigger>Cara Menggunakan Studio</AccordionTrigger>
          <AccordionContent>
            <p className="font-semibold">Selamat datang di OmniLang Studio!</p>
            <ul className="list-disc pl-5 mt-2 space-y-2">
              <li>
                <strong>Editor Kode:</strong> Di sebelah kiri adalah tempat Anda menulis kode OmniLang. Mulailah mengetik, dan AI akan memberikan saran pelengkapan kode secara otomatis.
              </li>
              <li>
                <strong>Panel Bawah:</strong> Di bagian bawah editor, Anda akan menemukan dua tab:
                <ul className="list-disc pl-5 mt-1">
                  <li><strong>AI Suggestions:</strong> Saran kode dari AI akan muncul di sini saat Anda mengetik.</li>
                  <li><strong>Errors:</strong> Jika ada kesalahan sintaks, panel ini akan menunjukkannya.</li>
                </ul>
              </li>
              <li>
                <strong>Dokumentasi:</strong> Di panel ini, Anda dapat menemukan panduan singkat tentang sintaksis dan fitur OmniLang.
              </li>
              <li>
                <strong>Bagikan Kode:</strong> Gunakan tombol &quot;Share&quot; di bagian atas untuk mendapatkan tautan unik dari kode Anda dan bagikan dengan orang lain.
              </li>
            </ul>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-00">
          <AccordionTrigger>Mode & Kapabilitas</AccordionTrigger>
          <AccordionContent>
            <div className="rounded border border-amber-200 bg-amber-50 px-3 py-2 text-amber-900 text-sm">
              Mode: Validator-only (parse + runtime eval). Compiler/stdlib/runtime produksi belum tersedia.
            </div>
            <ul className="list-disc pl-5 mt-2 space-y-1 text-sm">
              <li>Engine utama: Rust evaluator, fallback Python bila cargo/linker tidak tersedia.</li>
              <li>Output API menyertakan <code>mode</code>, <code>capabilities</code>, <code>compiler</code> untuk transparansi.</li>
            </ul>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-5">
          <AccordionTrigger>Status Proyek & Kesiapan</AccordionTrigger>
          <AccordionContent>
            <p className="mb-2">Saat ini, OmniLang belum siap untuk digunakan membuat aplikasi sungguhan. Yang kita miliki adalah <strong>OmniLang Studio</strong>, sebuah IDE untuk mendesain dan mensimulasikan bahasa OmniLang.</p>
            <h4 className="font-semibold mt-4">Yang Sudah Dicapai:</h4>
            <ul className="list-disc pl-5 mt-2 space-y-1">
              <li><strong>Desain Bahasa yang Matang:</strong> Spesifikasi yang jelas untuk sintaks, fitur, dan filosofi bahasa.</li>
              <li><strong>Lingkungan Pengembangan (IDE):</strong> OmniLang Studio berfungsi sebagai *playground* yang hebat dengan deteksi *error* secara *real-time*.</li>
              <li><strong>Bukti Konsep Keamanan:</strong> Prototipe *Type Checker* & *Borrow Checker* di Rust (`checker.rs`) telah membuktikan bahwa konsep keamanan OmniLang secara teoretis solid.</li>
            </ul>
            <h4 className="font-semibold mt-4">Yang Masih Dibutuhkan:</h4>
            <ul className="list-disc pl-5 mt-2 space-y-1">
              <li><strong>Compiler Penuh:</strong> Program yang bisa mengubah kode OmniLang menjadi aplikasi yang dapat dijalankan.</li>
              <li><strong>Runtime System:</strong> Sistem untuk mengelola eksekusi program, *garbage collection*, dan konkurensi.</li>
              <li><strong>Standard Library yang Lengkap:</strong> Pustaka standar untuk operasi file, jaringan, web, dll.</li>
            </ul>
            <p className="mt-4 text-sm text-muted-foreground italic">
              <strong>Analogi:</strong> Kita telah berhasil menciptakan sebuah simulator mobil balap yang realistis (OmniLang Studio), tetapi kita belum membangun mesin mobilnya (Compiler & Runtime).
            </p>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-1">
          <AccordionTrigger>Getting Started</AccordionTrigger>
          <AccordionContent>
            <p>Welcome to OmniLang! Here&apos;s a simple &quot;Hello, World!&quot; program:</p>
            <CodeBlock>{`// This is a comment\nprint("Hello, World!")`}</CodeBlock>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-2">
          <AccordionTrigger>Variables</AccordionTrigger>
          <AccordionContent>
            <p>Declare variables using the &apos;let&apos; keyword. OmniLang is dynamically typed.</p>
            <CodeBlock>{`let message = "This is a string"\nlet count = 100\n\nprint(message)\nprint(count)`}</CodeBlock>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-3">
          <AccordionTrigger>Functions</AccordionTrigger>
          <AccordionContent>
            <p>Define functions with &apos;fn&apos;. They can take arguments and return values.</p>
            <CodeBlock>{`fn greet() {\n  print("Hello from a function!")\n}\n\ngreet()`}</CodeBlock>
            <p className="mt-4">Functions with arguments:</p>
            <CodeBlock>{`fn add(a, b) {\n  return a + b\n}\n\nlet sum = add(5, 10)\nprint(sum) // Outputs: 15`}</CodeBlock>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="item-4">
          <AccordionTrigger>Control Flow</AccordionTrigger>
          <AccordionContent>
            <p>Use &apos;if&apos;, &apos;else if&apos;, and &apos;else&apos; for conditional logic.</p>
            <CodeBlock>{`let number = 10\n\nif number > 15 {\n  print("Number is large")\n} else if number > 5 {\n  print("Number is medium")\n} else {\n  print("Number is small")\n}`}</CodeBlock>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  )
}
