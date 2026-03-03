import init, { run_omnilang } from '../../pkg/omnilang_core.js';

document.addEventListener('DOMContentLoaded', async () => {
    // Interaktivitas Tab Kode Demo
    const tabs = document.querySelectorAll('.tab');
    const snippets = document.querySelectorAll('.snippet');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            // Hapus kelas aktif dari semua tab
            tabs.forEach(t => t.classList.remove('active'));
            // Hapus kelas aktif dari semua kontainer kode
            snippets.forEach(s => s.classList.remove('active'));

            // Aktifkan tab yang diklik
            tab.classList.add('active');

            // Tampilkan kode konten terkait
            const targetId = tab.getAttribute('data-target');
            document.getElementById(`snippet-${targetId}`).classList.add('active');
        });
    });

    // Penganimasian Halus Kemunculan Hero Image saat load pertama
    const heroVisual = document.querySelector('.hero-visual');
    if (heroVisual) {
        heroVisual.style.opacity = '0';
        heroVisual.style.transform = 'translateY(20px)';

        setTimeout(() => {
            heroVisual.style.transition = 'all 1s cubic-bezier(0.2, 0.8, 0.2, 1)';
            heroVisual.style.opacity = '1';
            heroVisual.style.transform = 'translateY(0)';
        }, 300);
    }

    // Inisialisasi WASM Playground
    const runBtn = document.getElementById('run-wasm-btn');
    const editor = document.getElementById('omnilang-editor');
    const outputConsole = document.getElementById('omnilang-output');

    if (runBtn && editor && outputConsole) {
        try {
            outputConsole.textContent = "// Memuat Modul WebAssembly...";
            await init();
            outputConsole.textContent = "// Modul WebAssembly siap. Silakan eksekusi kode Anda.\n";

            runBtn.addEventListener('click', () => {
                const sourceCode = editor.value;
                if (!sourceCode.trim()) return;

                outputConsole.textContent = "⚙️ Mengeksekusi...\n";
                runBtn.disabled = true;
                runBtn.textContent = "Running...";

                setTimeout(() => {
                    try {
                        const t0 = performance.now();
                        const result = run_omnilang(sourceCode);
                        const t1 = performance.now();

                        outputConsole.textContent = `✅ Selesai dalam ${(t1 - t0).toFixed(2)}ms\n\n---------------- OUTPUT ----------------\n${result}`;
                    } catch (err) {
                        outputConsole.textContent = `❌ Terjadi Kesalahan:\n${err}`;
                    } finally {
                        runBtn.disabled = false;
                        runBtn.textContent = "Run Code";
                    }
                }, 50);
            });
        } catch (e) {
            outputConsole.textContent = `❌ Gagal memuat WASM: ${e.message}`;
        }
    }
});
