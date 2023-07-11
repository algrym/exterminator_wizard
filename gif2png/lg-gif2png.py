#!/usr/bin/env python3
#
# Originally from https://pastebin.com/besPa147 by https://opengameart.org/users/amadorprograma
#
import glob
from PIL import Image

# Find all gifs which have an underscore in the middle and save the prefix names
prefixes = set()
for filename in glob.glob("last-guardian-sprites/*.gif"):
    sprite = Image.new("RGBA", (32, 32))
    name, _ = filename.split(".")

    src_img = name + ".gif"
    img = Image.open(src_img)

    sprite.paste(img, (0, 0))

    # Set transparency of composite image
    pixdata = sprite.load()
    width, height = sprite.size
    for y in range(height):
        for x in range(width):
             if pixdata[x, y] == (255, 255, 255, 255):
                   pixdata[x, y] = (255, 255, 255, 0)
    sprite.save(name + ".png", "PNG")
