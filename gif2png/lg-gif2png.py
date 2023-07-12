#!/usr/bin/env python3
#
# Originally from https://pastebin.com/besPa147 by https://opengameart.org/users/amadorprograma
#
import glob
from PIL import Image

# Find all the gifs in the source directory
prefixes = set()
for filename in glob.glob("last-guardian-sprites/*.gif"):
    sprite = Image.new("RGBA", (32, 32))
    name, _ = filename.split(".")

    img = Image.open(filename)
    sprite.paste(img, (0, 0))

    # Change alpha value of white pixels to 0
    # Snippet found at:
    # https://stackoverflow.com/questions/765736/how-to-use-pil-to-make-all-white-pixels-transparent
    pixdata = sprite.load()
    width, height = sprite.size
    for y in range(height):
        for x in range(width):
             if pixdata[x, y] == (255, 255, 255, 255):
                   pixdata[x, y] = (255, 255, 255, 0)
    sprite.save(name + ".png", "PNG")
