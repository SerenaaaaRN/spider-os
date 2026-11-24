#!/usr/bin/env python3
# Menyatukan file-file phase di docs/modules/ menjadi satu file modul_guide.md.
# Edit file phase, lalu jalankan skrip ini. Jangan edit modul_guide.md langsung.

from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MOD = ROOT / "docs" / "modules"
OUT = MOD / "modul_guide.md"

HEADER = (
    "> Versi gabungan dari seluruh file phase di `docs/modules/`. "
    "Dihasilkan otomatis oleh `scripts/generate-modul.py`; "
    "sumber kebenaran ada di file-file phase. Untuk navigasi, lihat `docs/index.md`.\n\n"
)

ORDER = [
    "00-pengantar.md",
    "phase-01-persiapan.md",
    "phase-02-baseline.md",
    "phase-03-uji-baseline.md",
    "phase-04-desain-aset.md",
    "phase-05-paket-sistem.md",
    "phase-06-tema-desktop.md",
    "phase-07-branding-boot.md",
    "phase-08-branding-terminal.md",
    "phase-09-spider-sense.md",
    "phase-09.5-komik.md",
    "phase-09.6-game.md",
    "phase-10-rc1.md",
    "phase-11-usb-hardware.md",
    "phase-12-freeze.md",
    "phase-13-laporan.md",
]

LAMPIRAN_MARKER = "### Lampiran D: Checklist master"


def read(name: str) -> str:
    path = MOD / name
    if not path.is_file():
        raise SystemExit(f"File tidak ditemukan: {path}")
    with path.open("r", encoding="utf-8", newline="") as fh:
        return fh.read()


def main() -> None:
    parts = [read(name) for name in ORDER]

    # Urutan asli lampiran: A, B, C, D, E, F, G, H.
    # lampiran-01 berisi A+B dan D+E+F; lampiran-02 (C) harus disisipkan di antaranya.
    lampiran_01 = read("lampiran-01-referensi.md")
    if lampiran_01.count(LAMPIRAN_MARKER) != 1:
        raise SystemExit("Penanda Lampiran D tidak ditemukan tepat sekali")
    ab, def_ = lampiran_01.split(LAMPIRAN_MARKER, 1)
    parts.append(ab)
    parts.append(read("lampiran-02-struktur-repo.md"))
    parts.append(LAMPIRAN_MARKER + def_)
    parts.append(read("lampiran-03-hypervisor.md"))

    body = "".join(parts)
    with OUT.open("w", encoding="utf-8", newline="") as fh:
        fh.write(HEADER + body)

    lines = (HEADER + body).count("\n") + 1
    print(f"OK: {OUT.relative_to(ROOT)} ({lines} baris, {len(ORDER) + 4} file sumber)")


if __name__ == "__main__":
    main()
