import init, { run_omnilang } from '../../pkg/omnilang_core.js';

document.addEventListener('DOMContentLoaded', async () => {
    // --- 1. Navigation & Smooth Scroll ---
    const navLinks = document.querySelectorAll('.nav-links a, .hero-actions a');
    navLinks.forEach(link => {
        link.addEventListener('click', (e) => {
            const href = link.getAttribute('href');
            if (href.startsWith('#')) {
                e.preventDefault();
                const targetId = href.substring(1);
                const targetElement = document.getElementById(targetId);
                if (targetElement) {
                    window.scrollTo({
                        top: targetElement.offsetTop - 80,
                        behavior: 'smooth'
                    });
                }
            }
        });
    });

    // --- 2. Showcase Filter Logic ---
    const filterButtons = document.querySelectorAll('.filter-btn');
    const showcaseCards = document.querySelectorAll('.showcase-card');

    filterButtons.forEach(btn => {
        btn.addEventListener('click', () => {
            // Update button states
            filterButtons.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            const filter = btn.getAttribute('data-filter');

            showcaseCards.forEach(card => {
                if (filter === 'all' || card.getAttribute('data-category') === filter) {
                    card.style.display = 'flex';
                    setTimeout(() => {
                        card.classList.remove('filtered-out');
                    }, 10);
                } else {
                    card.classList.add('filtered-out');
                    setTimeout(() => {
                        card.style.display = 'none';
                    }, 400);
                }
            });
        });
    });

    // --- 3. Playground Logic & Sample Loading ---
    const sampleCodes = {
        'digital_twin_3d_pro': `@oracle(model="spatial_solver")\nfn calculate_gap(p1: Vector3, p2: Vector3) -> f64;\n\nconst main: i32 = {\n    print("--- 3D FACTORY ORCHESTRATION ---");\n    let drone = Vector3 { x: 10.0, y: 15.0, z: 5.0 };\n    let agv = Vector3 { x: 25.0, y: 0.0, z: 12.0 };\n    \n    let gap = calculate_gap(drone, agv);\n    print("Spatial Distance: " + gap);\n    0\n};`,
        'rocket_launch_3d': `struct Rocket { stage: i32, fuel: f64 }\n\nconst main: i32 = {\n    print("🚀 Ignition...");\n    let saturn_v = Rocket { stage: 1, fuel: 100.0 };\n    \n    if saturn_v.fuel > 0 {\n        print("Lift-off Success.");\n    }\n    0\n};`,
        'robotics_arm_control': `fn solve_ik(x: f64, y: f64) -> [f64];\n\nconst main: i32 = {\n    print("🤖 Moving Robotic Arm...");\n    let angles = solve_ik(500.2, -120.5);\n    print("Joint Angles: " + angles);\n    0\n};`,
        'satellite_orbital_recon': `const main: i32 = {\n    print("🛰️ SAT-07: Passing over Jakarta...");\n    print("Ground Track: -6.17, 106.82");\n    print("Mission: Imaging active.");\n    0\n};`,
        'autonomous_vehicle_3d': `@oracle(model="yolo_v8")\nfn detect_objects(frame: [f64]) -> [String];\n\nconst main: i32 = {\n    print("🚗 AutoDrive: Scanning road...");\n    let detected = detect_objects([]);\n    print("Obstacles: " + detected);\n    0\n};`,
        'unix_kernel_sim': `const main: i32 = {\n    print("🐧 [xAetherOS] UNIX-Compatibility Layer");\n    print("PID: 1 | COMMAND: systemd");\n    print("Stat: /etc/shadow -> 600");\n    0\n};`,
        'retro_win3x_sim': `const main: i32 = {\n    print("📟 Windows 3.11 Startup...");\n    print("Memory: 640KB Baseline");\n    print("ProgMan: Running.");\n    0\n};`,
        'iron_dome_defense_3d': `const main: i32 = {\n    print("🛡️ Sentinel Jakarta: Active");\n    print("Threat: SCUD-B Detected (North)");\n    print("Action: Intercepting...");\n    0\n};`,
        // OODA Demo Snippets
        'demo_mesh': `@oracle(model: "models/mobilenet.onnx")\nfn classify_image(pixels: [f64]) -> [f64];\n\nfn main() {\n    println("Menjalankan Daemon AI Mesh...");\n    // $ omnilang serve worker.omni --port 8081\n}`,
        'demo_ai': `@mesh(target: "18.232.1.9:8081")\nfn classify_image(pixels: [f64]) -> [f64];\n\nfn main() {\n    let pixels = capture_camera();\n    let ai_result = classify_image(pixels);\n    println("Hasil AI Terdistribusi: " + ai_result);\n}`,
        'demo_hardware': `@hardware(port: "COM3", baud_rate: "115200")\nfn trigger_alarm(severity: i32) -> bool;\n\n@mesh(target: "127.0.0.1:8082")\nfn main() {\n    if detect_anomaly() > 99.0 {\n        trigger_alarm(1);\n    }\n}`
    };

    const runBtn = document.getElementById('run-wasm-btn');
    const editor = document.getElementById('omnilang-editor');
    const outputConsole = document.getElementById('omnilang-output');
    const clearEditorBtn = document.getElementById('clear-editor-btn');
    const clearConsoleBtn = document.getElementById('clear-console-btn');
    const loadSampleBtns = document.querySelectorAll('.load-sample-btn');

    // Sample Loader
    loadSampleBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const sampleId = btn.getAttribute('data-sample');
            if (sampleCodes[sampleId]) {
                editor.value = sampleCodes[sampleId];
                // Smooth scroll to playground
                document.getElementById('playground').scrollIntoView({ behavior: 'smooth' });
                // Flash effect on editor
                editor.style.background = 'rgba(0, 240, 255, 0.1)';
                setTimeout(() => { editor.style.background = 'transparent'; }, 500);

                // Auto-run after a short delay for scroll finish (v2.3.0 Feature)
                if (runBtn && !runBtn.disabled) {
                    setTimeout(() => {
                        runBtn.click();
                    }, 800);
                }
            }
        });
    });

    if (clearEditorBtn) {
        clearEditorBtn.addEventListener('click', () => { editor.value = ''; editor.focus(); });
    }

    if (clearConsoleBtn) {
        clearConsoleBtn.addEventListener('click', () => { outputConsole.textContent = '> Console cleared.\n'; });
    }

    // WASM Init
    if (runBtn && editor && outputConsole) {
        try {
            outputConsole.textContent = "// Memuat Modul WebAssembly...";
            await init();
            outputConsole.textContent = "// Modul WebAssembly siap.\n";

            runBtn.addEventListener('click', () => {
                const sourceCode = editor.value;
                if (!sourceCode.trim()) return;

                outputConsole.textContent += "\n⚙️ Mengeksekusi...\n";
                runBtn.disabled = true;
                const originalText = runBtn.textContent;
                runBtn.textContent = "...";

                setTimeout(() => {
                    try {
                        const t0 = performance.now();
                        const result = run_omnilang(sourceCode);
                        const t1 = performance.now();

                        outputConsole.textContent += `✅ Selesai dalam ${(t1 - t0).toFixed(2)}ms\n----------------\n${result}\n`;
                        outputConsole.scrollTop = outputConsole.scrollHeight;
                    } catch (err) {
                        outputConsole.textContent += `❌ Kesalahan: ${err}\n`;
                    } finally {
                        runBtn.disabled = false;
                        runBtn.textContent = originalText;
                    }
                }, 50);
            });
        } catch (e) {
            outputConsole.textContent = `❌ WASM Error: ${e.message}`;
        }
    }

    // --- 4. Demo Tabs ---
    const tabs = document.querySelectorAll('.tab');
    const snippets = document.querySelectorAll('.snippet');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            tabs.forEach(t => t.classList.remove('active'));
            snippets.forEach(s => s.classList.remove('active'));
            tab.classList.add('active');
            const targetId = tab.getAttribute('data-target');
            document.getElementById(`snippet-${targetId}`).classList.add('active');
        });
    });
});
