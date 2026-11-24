## PHASE 8: Branding Terminal dan Identitas Sistem

**Tujuan:** terminal, `neofetch`, dan identitas sistem terlihat sebagai SpiderOS.
**Output:** `neofetch` dengan ASCII laba-laba, prompt bash custom, `os-release` dan banner login.

### 8.1 Identitas sistem (`os-release`, `issue`, `motd`)

Penting: ubah hanya `NAME`, `PRETTY_NAME`, dan URL. Jangan ubah `ID` dan `VERSION_CODENAME` karena beberapa tool bergantung padanya.

```bash
cat > config/hooks/live/0300-branding.hook.chroot <<'EOF'
#!/bin/bash
set -e
# os-release (di Debian 13 /etc/os-release adalah symlink ke /usr/lib/os-release)
OSR=/usr/lib/os-release
sed -i 's/^NAME=.*/NAME="SpiderOS"/'                                        $OSR
sed -i 's/^PRETTY_NAME=.*/PRETTY_NAME="SpiderOS 1.0 (berbasis Debian 13)"/'  $OSR
sed -i 's/^HOME_URL=.*/HOME_URL="https:\/\/www.debian.org\/"/'               $OSR

# Banner login teks
cat > /etc/issue <<'BANNER'
SpiderOS 1.0 (Debian 13 Trixie) \n \l
BANNER
cp /etc/issue /etc/issue.net

cat > /etc/motd <<'BANNER'
  Selamat datang di SpiderOS.
  Dengan kekuatan besar datang tanggung jawab besar.
  Ketik `sense` untuk memeriksa kondisi sistem.
BANNER
EOF
chmod +x config/hooks/live/0300-branding.hook.chroot
```

### 8.2 `neofetch` dengan ASCII laba-laba

```bash
INC=config/includes.chroot_after_packages
NF=$INC/etc/skel/.config/neofetch
mkdir -p $NF
```

ASCII sederhana (`${c1}` maroon, `${c2}` krem — neofetch hanya mendukung 8 warna, jadi ini pendekatan terdekat dari palet 256-color Spider-Sense):

```bash
cat > $NF/spider.txt <<'EOF'
${c1}      \  \     /  /
       \  \   /  /
  ______\  \_/  /______
  \      \ (${c2}O${c1}) /      /
   `-.___/ / \ \___.-'
      _.-'  |  '-._
     /     / \     \
    /  .-'/   \'-.  \
   '__/  /     \  \__'
${c2}       SpiderOS
EOF
```

Alternatif: ubah logo kamu otomatis jadi ASCII lalu simpan ke `spider.txt`:

```bash
jp2a --width=40 --colors assets-final/logo.png > $NF/spider.txt
```

Konfigurasi neofetch:

```bash
cat > $NF/config.conf <<'EOF'
print_info() {
    info title
    info underline
    info "OS" distro
    info "Kernel" kernel
    info "Uptime" uptime
    info "Packages" packages
    info "Shell" shell
    info "DE" de
    info "CPU" cpu
    info "Memory" memory
}
image_backend="ascii"
image_source="/etc/skel/.config/neofetch/spider.txt"
ascii_colors=(1 3)
title_fqdn="off"
memory_percent="on"
EOF
```

### 8.3 Prompt bash dan alias (`.bash_aliases`)

Debian memuat `~/.bash_aliases` otomatis dari `.bashrc`, jadi kita tidak perlu menimpa `.bashrc`.

```bash
cat > $INC/etc/skel/.bash_aliases <<'EOF'
# Prompt SpiderOS: [user@host dir] dengan palet hangat Spider-Sense
# 88 maroon, 180 sand, 223 cream — sama seperti render/ansi.rs
PS1='\[\e[38;5;88m\][\u\[\e[38;5;180m\]@\h \[\e[38;5;223m\]\W\[\e[38;5;88m\]]\[\e[0m\]\$ '

# Alias
alias ll='ls -lah --color=auto'
alias update='sudo apt update && sudo apt full-upgrade'
alias sense='spider-sense'

# Tampilkan neofetch sekali di terminal interaktif pertama
if [ -t 1 ] && command -v neofetch >/dev/null 2>&1 && [ -z "$SPIDEROS_GREETED" ]; then
    export SPIDEROS_GREETED=1
    neofetch
fi
EOF
```

### 8.4 Warna terminal XFCE

```bash
mkdir -p $INC/etc/skel/.config/xfce4/terminal
cat > $INC/etc/skel/.config/xfce4/terminal/terminalrc <<'EOF'
[Configuration]
FontName=Fira Code 11
MiscAlwaysShowTabs=FALSE
MiscMenubarDefault=FALSE
ColorForeground=#ffd7af
ColorBackground=#17100f
ColorCursor=#d78787
ColorPalette=#17100f;#870000;#87af87;#d7af5f;#af8787;#d78787;#ffd7af;#f3e2cf;#4a3a35;#ad1414;#a7cfa7;#e8c775;#d7af87;#e8a0a0;#ffe8c8;#fff5e6
EOF
```

Pemetaan 16 warna: 0 kanvas, 1 maroon, 2 sage, 3 amber, 4 dusk, 5 rose, 6 cream, 7 cream muda, 8 panel, 9 maroon terang, 10 sage terang, 11 amber terang, 12 sand, 13 rose terang, 14 cream terang, 15 putih hangat. Ini membuat warna `spider-sense` (88/174/223/180/138/108/179) tampil konsisten di terminal.

### 8.5 Build dan uji

```bash
./scripts/build.sh v0.5
```

Cek di `[TEST-VM]`:

```bash
cat /etc/os-release | head -3
neofetch
echo $PS1
```

- [ ] `neofetch` menampilkan ASCII laba-laba dan nama "SpiderOS"
- [ ] Prompt berwarna maroon-krem
- [ ] Warna terminal gelap dengan kursor rose

📸 `P08-neofetch-01.png`, `P08-terminal-01.png`.

### 8.6 Troubleshooting Phase 8

| Gejala | Solusi |
|---|---|
| `neofetch` tidak memakai ASCII kita | Cek `image_source` path, cek `ls -l /etc/skel/.config/neofetch/spider.txt` |
| Prompt tidak berubah | Pastikan `.bash_aliases` ada di `~` (`ls -a ~`), buka terminal baru |
| Nama OS tetap "Debian" | Cek hook 0300 di `build.log`, cek `cat /usr/lib/os-release` |
| Warna ASCII aneh | Sesuaikan `ascii_colors=(1 3)` (1 merah, 3 kuning — pendekatan 8 warna untuk maroon/krem) |

### 8.7 Commit dan snapshot

```bash
git add -A
git commit -m "feat: branding terminal, neofetch, os-release"
```

Snapshot: `P8-terminal-branding`.

### 8.8 Checklist Phase 8

- [ ] `os-release`, banner, neofetch, prompt, warna terminal sesuai
- [ ] Sistem masih normal (apt jalan, tidak ada error aneh)
- [ ] Screenshot, commit, snapshot

✅ **Selesai kalau:** membuka terminal langsung terasa "SpiderOS".

---

