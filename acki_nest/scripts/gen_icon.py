#!/usr/bin/env python3
"""Генерирует PNG-иконку AckiNest (256x256) без внешних зависимостей."""
import zlib
import struct
import os
import math

def chunk(tag, data):
    c = tag + data
    return struct.pack(">I", len(data)) + c + struct.pack(">I", zlib.crc32(c))

def make_png(path, w, h, pixel_fn):
    raw = bytearray()
    for y in range(h):
        raw += b"\x00"  # filter: none
        for x in range(w):
            raw += bytes(pixel_fn(x, y))
    ihdr = struct.pack(">IIBBBBB", w, h, 8, 2, 0, 0, 0)  # 8-bit RGB
    png = (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", zlib.compress(bytes(raw), 9))
        + chunk(b"IEND", b"")
    )
    with open(path, "wb") as f:
        f.write(png)

def main():
    out = os.path.join(os.path.dirname(__file__), "..", "src-tauri", "icons", "icon.png")
    os.makedirs(os.path.dirname(out), exist_ok=True)
    size = 256
    cx, cy = size / 2, size / 2

    def pixel(x, y):
        # Диагональный градиент: тёмно-синий (низ) -> фиолетовый (верх).
        t = (x + y) / (2 * size)
        r = int(24 + t * 80)
        g = int(38 + t * 60)
        b = int(92 + t * 160)
        # Скруглённые углы (прозрачность не поддерживаем, используем чёрный фон)
        dx, dy = x - cx, y - cy
        d = math.sqrt(dx * dx + dy * dy)
        if d > size * 0.46:
            # Кольцо-«гнездо» по краю
            if d > size * 0.5:
                return (10, 14, 32)
            return (60, 90, 180)
        # Внутренний круг с градиентом и «лункой» в центре
        if d < size * 0.16:
            return (255, 214, 90)  # золотое ядро
        ring = abs(d - size * 0.30)
        if ring < size * 0.035:
            return (120, 180, 255)  # светло-голубое кольцо
        return (r, g, b)

    make_png(out, size, size, pixel)
    print("OK:", out)

if __name__ == "__main__":
    main()
