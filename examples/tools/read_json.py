import json
import sys

font = ""

with open(sys.argv[1], "r") as f:
    font = f.read()

font = json.loads(font)
for character in font["characters"]:
    # print(f"'{character}'", end=", ")
    char = font["characters"][character]
    print("CharCoord { ", f"x: {char['x']}, y: {char['y']}, w: {char['width']}, h: {char['height']}, originX: {char['originX']}, originY: {char['originY']}, advance: {char['advance']}", "},")