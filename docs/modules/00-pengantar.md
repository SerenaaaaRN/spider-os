## 0. Pengantar

### 0.1 Hasil akhir yang dituju
- Satu file ISO hybrid (`spideros-vX.Y.iso`) yang bisa boot di BIOS/Legacy dan UEFI, dari flashdisk.
- Tampilan bertema Spider-Man: wallpaper, tema GTK, ikon, login screen, boot menu, animasi boot (Plymouth), terminal.
- Branding sistem: `neofetch`, prompt bash, `/etc/os-release`.
- Aplikasi dasar yang ringan dan berguna.
- **Hiburan offline**: pembaca komik (mcomix) — cover/thumbnail, bookmark, dan posisi halaman terakhir (komik diisi pengguna sendiri), plus game bawaan dari repo Debian.
- **Komponen inti koding**: tool Spider-Sense (**Rust**) yang membaca `/proc`, mem-parsing data kernel secara manual, dan merender antarmuka terminal dengan ANSI.
- Dokumentasi lengkap yang langsung jadi bahan BAB III, IV, dan V.

> **Basis sistem.** Modul ini menggunakan **Debian 13 (trixie)**. Seluruh contoh `lb config`, paket, dan hook mengacu pada trixie. Jika menemukan referensi Debian 12 pada materi lama, ganti dengan `trixie` dan jalankan pemeriksaan konsistensi di Phase 2.3.

### 0.2 Konvensi di modul ini

| Penanda | Arti |
|---|---|
| `[BUILD-VM]` | Dijalankan di VM Debian 13 tempat kamu membangun ISO (default kalau tidak ditulis) |
| `[HOST]` | Dijalankan di Windows (komputer fisik) |
| `[TEST-VM]` | Dijalankan di VM baru yang boot dari ISO hasil build |
| `$` | Perintah sebagai user biasa (pakai `sudo` kalau perlu) |
| 📸 | Ambil screenshot untuk BAB IV |
| ✅ Selesai kalau | Kriteria lulus phase, jangan lanjut sebelum terpenuhi |

Folder kerja: `~/spideros`. Semua perintah dijalankan dari situ kecuali disebut lain.

### 0.3 Peta Phase

| Phase | Isi | Fase RUP | Estimasi effort |
|---|---|---|---|
| 1 | Persiapan lingkungan, repo, dokumentasi, pembagian tugas | Inception | 2–3 jam |
| 2 | Build ISO baseline (tanpa tema) | Elaboration | 3–4 jam (banyak menunggu build) |
| 3 | Uji baseline di VM test (BIOS dan UEFI) | Elaboration | 1–2 jam |
| 4 | Desain aset visual | Construction | 4–8 jam (paralel, oleh desainer) |
| 5 | Paket, user live, konfigurasi sistem, keamanan dasar | Construction | 3–4 jam |
| 6 | Tema desktop (wallpaper, GTK, ikon, login screen) | Construction | 3–5 jam |
| 7 | Branding boot (boot menu, Plymouth) | Construction | 3–4 jam |
| 8 | Branding terminal dan sistem | Construction | 2–3 jam |
| 9 | Spider-Sense (Rust, komponen inti koding) | Construction | 8–12 jam |
| 9.5 | Kolektor komik offline (mcomix) | Construction | 1–2 jam |
| 9.6 | Game offline dari repo Debian | Construction | 1–2 jam |
| 10 | Build ISO v1 dan uji menyeluruh di VM | Construction | 2–3 jam |
| 11 | Flash USB dan uji hardware (UEFI dan Legacy) | Transition | 3–5 jam |
| 12 | Perbaikan bug, ISO v2, freeze | Transition | 2–4 jam |
| 13 | Laporan, panduan pengguna, video demo, presentasi | Transition | 6–10 jam |

Phase 4 bisa jalan paralel dengan Phase 2 dan 3 (desainer tidak perlu menunggu build).

### 0.4 Pembagian peran (4 orang)

| Peran | Tanggung jawab utama | Phase utama |
|---|---|---|
| Builder | Pipeline live-build, hook, konfigurasi, build ISO, integrasi kompilasi Rust di chroot | 1, 2, 5, 6, 7, 8, 9, 10, 12 |
| Desainer | Semua aset visual | 4 (lalu bantu 6, 7) |
| Tester | VM test, USB, matriks hardware, catat boot sequence, verifikasi binary Spider-Sense | 3, 9, 10, 11 |
| Dokumentator | Catatan, screenshot, laporan, panduan, demo, dokumentasi arsitektur dan pengujian kode Rust | 1, 9, 13 |

Phase 9 dikerjakan secara kolaboratif: Builder menyiapkan hook kompilasi, satu anggota menjadi penanggung jawab utama kode Rust, Tester memverifikasi binary di ISO, dan Dokumentator mencatat keputusan desain serta hasil pengujian. Semua anggota tetap mencoba menjalankan ISO minimal sekali supaya bisa menjawab pertanyaan dosen.

### 0.5 Aturan emas
- Snapshot VM sebelum setiap phase besar.
- Commit Git setiap satu perubahan yang berhasil.
- Satu perubahan per build. Jangan mengubah 10 hal lalu build. Kalau gagal, tidak tahu penyebabnya.
- Screenshot dan catat command saat mengerjakan, bukan nanti.
- Baseline harus boot dulu sebelum menambah tema.
- Setelah ISO final di-freeze, tidak ada fitur baru.

---

