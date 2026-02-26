import termrender
print("termrender imported successfully!")
print("dir(termrender):", dir(termrender))

v1 = termrender.Vec2(10, 20)
v2 = termrender.Vec2(5, 5)
v3 = v1 + v2

print(f"Vec2 addition result: x={v3.x}, y={v3.y}")
