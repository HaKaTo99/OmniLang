document.addEventListener('DOMContentLoaded', () => {
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
});
