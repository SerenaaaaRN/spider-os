#!/usr/bin/env python3
# =============================================================================
# ⚠️  DUMMY ASSET GENERATOR — BUKAN HASIL FINAL
# -----------------------------------------------------------------------------
# Ini placeholder supaya Phase 4 bisa ditutup dulu. Sebelum build Phase 6/7 di
# VM, GANTI file di assets-final/ dengan desain final (lihat docs/desain.md §4
# dan assets-final/README-GANTI-ASET.txt). Menjalankan ulang skrip ini AKAN
# menimpa PNG di assets-final/.
#
# Palet diambil persis dari tools/spider-sense/src/render/ansi.rs.
# Butuh: Python 3 + Pillow + numpy (sudah ada di host Windows).
# =============================================================================

from __future__ import annotations

import math
import os
import random
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "assets-final"
PREVIEW = ROOT / "docs" / "screenshots"

# Palet kanonik (docs/desain.md §2) — xterm 88/174/223/180/138/108/179/1;31
MAROON = "#870000"
ROSE = "#D78787"
CREAM = "#FFD7AF"
SAND = "#D7AF87"
DUSK = "#AF8787"
SAGE = "#87AF87"
AMBER = "#D7AF5F"
ALERT = "#FF5555"
CANVAS = "#17100F"
PANEL = "#2A1A18"
DEEP = "#4A0B0B"


def rgb(hex_color: str) -> tuple[int, int, int]:
    h = hex_color.lstrip("#")
    return tuple(int(h[i : i + 2], 16) for i in (0, 2, 4))  # type: ignore[return-value]


def font(size: int, bold: bool = False) -> ImageFont.FreeTypeFont | ImageFont.ImageFont:
    candidates = [
        r"C:\Windows\Fonts\segoeuib.ttf" if bold else r"C:\Windows\Fonts\segoeui.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf" if bold else "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    ]
    for path in candidates:
        if os.path.exists(path):
            return ImageFont.truetype(path, size)
    return ImageFont.load_default()


def gradient(size: tuple[int, int], stops: list[str]) -> Image.Image:
    """Gradient diagonal yang diinterpolasi antar warna (numpy, cepat)."""
    w, h = size
    yy, xx = np.mgrid[0:h, 0:w]
    t = xx / max(w - 1, 1) * 0.5 + yy / max(h - 1, 1) * 0.5
    arr = np.zeros((h, w, 3), dtype=np.float64)
    pos = np.linspace(0, 1, len(stops))
    for i in range(len(stops) - 1):
        mask = (t >= pos[i]) & (t <= pos[i + 1])
        local = ((t[mask] - pos[i]) / (pos[i + 1] - pos[i]))[:, None]
        c0 = np.array(rgb(stops[i]), dtype=np.float64)
        c1 = np.array(rgb(stops[i + 1]), dtype=np.float64)
        arr[mask] = c0[None, :] * (1 - local) + c1[None, :] * local
    return Image.fromarray(arr.astype(np.uint8), "RGB")


def draw_web(
    draw: ImageDraw.ImageDraw,
    cx: int,
    cy: int,
    radii: list[int],
    color: tuple[int, int, int],
    width: int,
    spokes: int = 12,
) -> None:
    for r in radii:
        draw.ellipse([cx - r, cy - r, cx + r, cy + r], outline=color, width=width)
    outer = radii[-1]
    for i in range(spokes):
        a = math.tau * i / spokes
        x = cx + int(math.cos(a) * outer)
        y = cy + int(math.sin(a) * outer)
        draw.line([(cx, cy), (x, y)], fill=color, width=width)


def draw_city(draw: ImageDraw.ImageDraw, w: int, h: int, color: tuple[int, int, int]) -> None:
    rng = random.Random(7)
    x = 0
    while x < w:
        bw = rng.randint(46, 110)
        bh = rng.randint(60, 200)
        draw.rectangle([x, h - bh, x + bw, h], fill=color)
        for wx in range(x + 8, x + bw - 8, 18):
            for wy in range(h - bh + 12, h - 12, 24):
                if rng.random() > 0.55:
                    draw.rectangle([wx, wy, wx + 5, wy + 8], fill=rgb(SAND))
        x += bw + rng.randint(4, 16)


def draw_spider(
    draw: ImageDraw.ImageDraw,
    cx: float,
    cy: float,
    s: float,
    body: tuple[int, int, int],
    accent: tuple[int, int, int],
) -> None:
    lw = max(1, int(s * 0.07))
    draw.ellipse([cx - 0.5 * s, cy - 0.3 * s, cx + 0.5 * s, cy + 0.9 * s], fill=body, outline=accent, width=lw)
    draw.ellipse([cx - 0.26 * s, cy - 0.95 * s, cx + 0.26 * s, cy - 0.25 * s], fill=body, outline=accent, width=lw)
    for side in (-1, 1):
        for dx, dy in ((1.05, -0.9), (1.3, -0.1), (1.05, 0.75)):
            draw.line(
                [(cx + side * 0.35 * s, cy + 0.1 * s),
                 (cx + side * 0.8 * s, cy + dy * s * 0.5),
                 (cx + side * dx * s, cy + dy * s)],
                fill=accent,
                width=lw,
                joint="curve",
            )


def wallpaper(size: tuple[int, int], login: bool = False, grub: bool = False) -> Image.Image:
    w, h = size
    stops = [CANVAS, DEEP, MAROON] if not grub else [CANVAS, PANEL, DEEP]
    img = gradient(size, stops)
    d = ImageDraw.Draw(img)
    cx, cy = w // 2, int(h * 0.42)
    scale = w / 1920
    draw_web(d, cx, cy, [int(r * scale) for r in (120, 240, 360, 480, 600, 720)], rgb(DUSK), max(1, int(3 * scale)))
    draw_city(d, w, h, rgb(CANVAS))
    draw_spider(d, cx, cy - 40 * scale, 120 * scale, rgb(MAROON), rgb(ROSE))
    title = "SPIDEROS"
    d.text((cx, cy + 70 * scale), title, font=font(int(96 * scale), bold=True), fill=rgb(CREAM), anchor="mm")
    d.text(
        (cx, cy + 130 * scale),
        "DENGAN KEKUATAN BESAR DATANG TANGGUNG JAWAB BESAR",
        font=font(int(24 * scale)),
        fill=rgb(SAND),
        anchor="mm",
    )
    if login:
        overlay = Image.new("RGBA", size, (0, 0, 0, 0))
        od = ImageDraw.Draw(overlay)
        od.rectangle([0, 0, int(w * 0.42), h], fill=(9, 5, 5, 170))
        img = Image.alpha_composite(img.convert("RGBA"), overlay).convert("RGB")
    return img


def logo(size: int = 400) -> Image.Image:
    """Gambar logo langsung pada ukuran target (lebih bersih daripada resize)."""
    k = size / 400
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    draw_web(d, 200 * k, 190 * k, [int(r * k) for r in (40, 80, 120, 160)],
             rgb(DUSK) + (200,), max(1, round(2 * k)))
    draw_spider(d, 200 * k, 180 * k, 90 * k, rgb(MAROON) + (255,), rgb(ROSE) + (255,))
    d.text((200 * k, 335 * k), "SPIDEROS",
           font=font(max(8, round(30 * k)), bold=True), fill=rgb(CREAM) + (255,), anchor="mm")
    return img


def contact_sheet(images: dict[str, Image.Image]) -> Image.Image:
    sheet = Image.new("RGB", (1280, 1020), rgb(CANVAS))
    d = ImageDraw.Draw(sheet)
    d.text((24, 16), "PHASE 4 — PREVIEW ASET DUMMY (ganti sebelum build Phase 6)", font=font(24, bold=True), fill=rgb(CREAM))
    d.text((24, 50), "Palet: maroon #870000 | rose #D78787 | cream #FFD7AF | sand #D7AF87 | dusk #AF8787", font=font(15), fill=rgb(SAND))

    slots = [
        ("wallpaper.png", 24, 90, 610, 343),
        ("login.png", 654, 90, 610, 343),
        ("grub-bg.png", 24, 470, 610, 343),
        ("isolinux-splash.png", 654, 470, 400, 300),
        ("logo.png", 1070, 470, 190, 190),
        ("spideros-logo-small.png", 1070, 690, 128, 128),
    ]
    for name, x, y, tw, th in slots:
        thumb = images[name].convert("RGB")
        thumb.thumbnail((tw, th), Image.Resampling.LANCZOS)
        sheet.paste(thumb, (x, y))
        d.rectangle([x, y, x + tw, y + th], outline=rgb(MAROON), width=2)
        d.text((x + 4, y + th + 4), name, font=font(14), fill=rgb(CREAM))
    return sheet


def main() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)

    wp = wallpaper((1920, 1080))
    login = wallpaper((1920, 1080), login=True)
    grub = wallpaper((1920, 1080), grub=True)
    iso = wp.resize((640, 480), Image.Resampling.LANCZOS)
    mark = logo(400)
    small = logo(128)

    outputs = {
        "wallpaper.png": wp,
        "login.png": login,
        "grub-bg.png": grub,
        "isolinux-splash.png": iso,
        "logo.png": mark,
        "spideros-logo-small.png": small,
    }
    for name, img in outputs.items():
        path = OUT / name
        img.save(path)
        print(f"{name:28} {img.size[0]}x{img.size[1]}  {path.stat().st_size / 1024:8.1f} KB  (DUMMY)")

    sheet = contact_sheet(outputs)
    preview_path = PREVIEW / "P04-aset-preview-01.png"
    sheet.save(preview_path)
    print(f"\nPreview: {preview_path}")
    print("\nNOTE: INI ASSET DUMMY. Ganti sebelum build Phase 6 - lihat docs/desain.md section 4")


if __name__ == "__main__":
    main()
