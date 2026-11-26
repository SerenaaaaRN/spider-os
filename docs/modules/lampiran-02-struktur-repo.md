### Lampiran C: Struktur akhir repo

```
spideros/
 ├── auto/
 │   ├── build
 │   ├── clean
 │   └── config
 ├── config/
 │   ├── package-lists/
 │   │   ├── spideros.list.chroot
 │   │   ├── content.list.chroot                 # mcomix, mpv, audacious
 │   │   ├── games.list.chroot                   # bastet, nethack-console, gnome-mines, gnome-sudoku
 │   │   ├── hardware.list.chroot                # firmware laptop, Bluetooth, storage, power, VA-API
 │   │   └── spider-sense-build.list.chroot      # cargo, rustc, build-essential (build-only)
 │   ├── hooks/live/
 │   │   ├── 0050-system-config.hook.chroot
 │   │   ├── 0100-theme.hook.chroot
 │   │   ├── 0200-plymouth.hook.chroot
 │   │   ├── 0300-branding.hook.chroot
 │   │   └── 0400-spider-sense-build.hook.chroot
 │   ├── bootloaders/                       # template boot menu yang diedit
 │   └── includes.chroot_after_packages/
 │       ├── etc/
 │       │   ├── lightdm/lightdm-gtk-greeter.conf
 │       │   ├── X11/xorg.conf.d/40-libinput-touchpad.conf   # tap-to-click (B01)
 │       │   ├── xdg/autostart/spideros-touchpad.desktop     # fallback tap-to-click
 │       │   └── skel/
 │       │       ├── .bash_aliases
 │       │       └── .config/
 │       │           ├── autostart/spideros-wallpaper.desktop
 │       │           └── xfce4/
 │       │               ├── terminal/terminalrc
 │       │               └── xfconf/xfce-perchannel-xml/{xsettings.xml,xfwm4.xml}
 │       ├── opt/spider-sense-src/           # staging generated (gitignored)
 │       └── usr/
 │           ├── local/bin/spideros-wallpaper        # skrip wallpaper
 │           ├── local/bin/spideros-touchpad-tap     # fallback tap-to-click (B01)
 │           └── local/bin/neofetch                # symlink dibuat hook 0300 (tidak di repo)
 │           ├── share/applications/
 │           │   ├── spider-sense.desktop
 │           │   ├── spideros-comics.desktop
 │           │   ├── spideros-bastet.desktop
 │           │   └── spideros-nethack.desktop
 │           ├── share/spideros/comics/README.txt     # folder koleksi komik (diisi pengguna)
 │           ├── share/backgrounds/spiderman/{wallpaper.png,login.png}
 │           ├── share/pixmaps/spideros-logo.png
 │           └── share/plymouth/themes/spiderman/{spiderman.plymouth,spiderman.script,logo.png}
 ├── assets-src/ , assets-final/
 ├── tools/spider-sense/                    # proyek Rust: Cargo.toml, Cargo.lock, src/main.rs
 ├── scripts/
 │   ├── build.sh
 │   └── stage-spider-sense.sh
 ├── dist/                                  # ISO hasil (tidak di-commit)
 └── docs/
     ├── catatan.md, desain.md
     ├── release_notes.md, panduan_pengguna.md, laporan.md
     └── screenshots/
```

