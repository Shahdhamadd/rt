# rt

A small ray tracer I wrote in Rust. You describe a scene (some shapes, a light and a camera) in a plain text file and it draws a picture of it into a `.ppm` image file. It's plain Rust with no external crates, so `cargo` is the only thing you need.

This page explains what it can do, how to run it, and how to write your own scenes. You don't need to know any Rust to make your own images: a scene is just a short text file.

## What it can do

- **Four shapes:** sphere, cube, flat plane and cylinder. Each one can go anywhere, at any size, in any color.
- **Scene files.** Scenes are written in simple text files, so you can make a new one without touching the code or recompiling.
- **A camera** you can put anywhere and point at anything.
- **A light** whose position and brightness you choose.
- **Shading.** Surfaces facing the light are bright and surfaces turned away from it get darker, which is what makes a sphere look round instead of looking like a flat red circle.
- **Highlights.** Objects can have a small white shiny spot where the light catches them, like a real ball or a polished surface.
- **Shadows.** Objects block the light, so they throw shadows onto the floor and onto each other. The light isn't a single tiny point (it's a small square, 1.5 units wide by default), so the shadow edges come out soft rather than razor sharp.
- **Smooth edges.** Every pixel is sampled 36 times (a 6×6 grid) and averaged, so the outlines of shapes don't look jagged.
- **Any resolution** you like, set from the command line.
- **Uses every CPU core.** The rows of the picture are shared out between all your cores, so big images render many times faster.
- **Reflections (bonus).** Any object can be made a bit reflective or a full mirror. Turn this on with `-r`.

## Running it

```bash
cargo run --release -- <scene file> [width] [height] [-r] [-o image.ppm]
```

- `<scene file>` is the text file that describes the scene. The ready-made ones are in the `scenes/` folder. If you leave it out you get `scenes/all.txt`.
- `width` and `height` are the size of the picture in pixels. The default is 400×300.
- `-o image.ppm` saves the picture straight into that file.
- `-r` turns reflections on. They're off by default to keep normal renders quick.
- `--release` matters. The debug build works, but it's a lot slower.

A few examples:

```bash
cargo run --release -- scenes/all.txt 800 600 -o all.ppm
cargo run --release -- scenes/sphere.txt 200 150 -o quick-test.ppm
cargo run --release -- scenes/mirror.txt 800 600 -r -o mirror.ppm
```

While it runs it prints `Rows left: ...` so you can see it's working. An 800×600 image takes under a second on my laptop. When I'm just trying things out I use something small like 200×150, which is instant.

I'd stick to a 4:3 size (400×300, 800×600, 1200×900 and so on). Other sizes work fine, but the ready-made scenes were framed for 4:3, so a very wide or very tall picture can cut things off.

**Always use `-o` on Windows.** If you leave `-o` out, the picture is printed to the terminal instead, and you can catch it with `> image.ppm`. That works in Git Bash and `cmd`, but PowerShell's `>` adds a few invisible bytes to the start of the file and image viewers then won't open it. `-o` writes the file itself, so it works the same in every terminal.

To look at a `.ppm` you can use GIMP, IrfanView, a PPM viewer extension in VS Code, or any online PPM viewer.

## The example images

The `images/` folder has the five pictures I rendered, all at 800×600:

- `1-sphere.ppm`: a red sphere on a grey floor. (`scenes/sphere.txt`)
- `2-plane-cube.ppm`: a blue cube on the floor, with the light turned down to half brightness so it's darker than the first image. (`scenes/cube.txt`)
- `3-all-objects.ppm`: a sphere, a cube, a cylinder and the floor together. (`scenes/all.txt`)
- `4-all-objects-moved-camera.ppm`: exactly the same scene as 3, seen from the right side. (`scenes/all-moved.txt`)
- `bonus-reflection.ppm`: a mirror sphere and a slightly reflective floor. (`scenes/mirror.txt` with `-r`)

You can re-make any of them with `cargo run --release -- <scene file> 800 600 -o file.ppm`.

## How it works (the short version)

Think of the picture as a window made of tiny squares, one per pixel, with the camera standing behind it. For each pixel the program:

1. Shoots a ray, which is just a straight line from the camera through that pixel and out into the scene.
2. Checks every object to see if the ray hits it, and keeps the closest hit. Each shape has its own bit of maths for this. The sphere solves a quadratic equation. The plane only needs a division. The cube checks whether the ray is inside the box's x, y and z ranges all at the same time. The cylinder is a circle seen from above, cut off at its height, with a lid on each end.
3. If the ray hits nothing, it paints sky: a light blue that fades to white near the horizon.
4. If it hits something, it works out how much light reaches that spot. First it sends a second ray from the spot towards the light. If something is in the way, the spot is in shadow and only gets a little background light. If nothing is in the way, it checks how directly the surface faces the light (using the dot product of the surface's normal and the direction to the light). The more it faces the light, the brighter it is.
5. For the highlight, it bounces the light's direction off the surface like a mirror would, and checks how close that bounced direction is to pointing back at the camera. If it points almost straight at the camera, a bit of white is added. Raising that closeness to a high power (40) keeps the shiny spot small.
6. With `-r`, if the object is reflective, the ray bounces off it like light off a mirror and the program repeats the steps above for the bounced ray (up to 5 bounces). Whatever the bounced ray sees gets mixed into the object's own color.

It does this 36 times per pixel at slightly different spots, averages the results, and writes the final color into the file as three numbers from 0 to 255.

To make it fast, the rows are shared out between threads, one per CPU core: with 4 cores, thread 1 draws rows 0, 4, 8..., thread 2 draws rows 1, 5, 9..., and so on. Each thread keeps its finished rows, and at the end they're put back in order and written to the file.

## Making your own scene

A scene is a text file with one thing per line. Here's `scenes/all.txt`:

```
# One of each object

camera  0 1.5 1.5
look_at 0 -0.3 -5
light   -4 5 0  1.0

sphere   -1.8 0 -4.5  1.0  1 0 0  shine 0.5
cube     1.8 -0.4 -4.5  1.2  0.2 0.4 1  shine 0.5
cylinder 0 -1 -6  0.6 1.8  0.2 0.8 0.3  shine 0.5
plane    0 -1 0  0 1 0  0.6 0.6 0.6
```

- Each line starts with a word that says what it is, followed by numbers.
- Anything after a `#` is a comment and is ignored, and so are empty lines.
- Spaces don't matter, so you can line things up however you like.
- Every scene needs exactly one `camera`, one `look_at` and one `light`, plus as many objects as you want.

If you get something wrong, it tells you which line, for example `line 7: 'cube' needs 7 numbers, found 4` or `line 3: 'red' is not a number`.

The easiest way to start is to copy one of the files in `scenes/` and change it. There's a full walkthrough at the end of this section.

### Positions and colors

Positions are three numbers: `x y z`.

- `x` goes left (negative) and right (positive)
- `y` goes down (negative) and up (positive)
- `z` goes towards you (positive) and away into the distance (negative)

In the ready-made scenes the floor is at `y = -1`, the camera sits near `z = 1`, and the objects are about 4 to 6 units in front of it (`z` from `-4` to `-6`). Those are good numbers to start from.

Colors are also three numbers: red, green and blue, each from `0` to `1`:

```
1 0 0          red
0.2 0.4 1      blue
0.2 0.8 0.3    green
0.6 0.6 0.6    grey
1 1 1          white
```

### Sphere

```
sphere  0 0 -4  1.0  1 0 0
```

| Numbers | Meaning |
|---|---|
| `0 0 -4` | center |
| `1.0` | radius |
| `1 0 0` | color (red) |

To make a sphere sit on the floor instead of sinking into it or floating above it, set the center's `y` to `-1 + radius`. With a radius of 1 that gives `y = 0`, which is what the example uses.

### Cube

```
cube  1.8 -0.4 -4.5  1.2  0.2 0.4 1
```

| Numbers | Meaning |
|---|---|
| `1.8 -0.4 -4.5` | center |
| `1.2` | length of each edge |
| `0.2 0.4 1` | color (blue) |

The cube is always lined up with the x, y and z axes, so it can't be rotated. To rest it on the floor, set the center's `y` to `-1 + size / 2`. Here that's `-1 + 0.6 = -0.4`.

### Flat plane

```
plane  0 -1 0  0 1 0  0.6 0.6 0.6
```

| Numbers | Meaning |
|---|---|
| `0 -1 0` | any point the plane passes through |
| `0 1 0` | the direction it faces (its normal) |
| `0.6 0.6 0.6` | color (grey) |

A plane goes on forever in every direction, so you don't give it a size. Instead you give it one point it passes through and the direction it faces. The example faces straight up from `y = -1`, which makes it a floor. It's the same floor every ready-made scene uses.

For a back wall that faces the camera, point it towards you instead:

```
plane  0 0 -10  0 0 1  0.9 0.85 0.7
```

The direction doesn't need to be exactly length 1. The code tidies it up for you.

### Cylinder

```
cylinder  0 -1 -6  0.6 1.8  0.2 0.8 0.3
```

| Numbers | Meaning |
|---|---|
| `0 -1 -6` | center of the bottom |
| `0.6` | radius |
| `1.8` | height |
| `0.2 0.8 0.3` | color (green) |

It always stands straight up and has a flat lid on the top and the bottom. Because the position is the bottom (not the middle, like the sphere and cube), putting it on the floor is easy: just use `y = -1`.

### Changing the brightness

```
light  -4 5 0  1.0
```

The first three numbers are where the light is. `-4 5 0` means up high and off to the left, which is why the shadows in the example images fall to the right. Move the light and the shadows move with it.

The fourth number is the brightness:

- `1.0` is normal, and what most of the scenes use
- `0.5` is half as bright. `scenes/cube.txt` uses this, which is why it looks darker than the sphere scene.
- anything above `1.0`, like `1.5`, makes it brighter. Very bright lights wash the colors out towards white.

Parts that are in shadow, or turned away from the light, never go completely black. There's always a little background light (`0.1`) so you can still see their shape.

You can add a fifth number to change the size of the light. It's `1.5` if you leave it out. A smaller light gives sharper shadow edges and a smaller, sharper highlight:

```
light  -4 5 0  1.0  0.2
```

### Moving the camera

```
camera   0 1.5 1.5
look_at  0 -0.3 -5
```

`camera` is where the camera stands. `look_at` is the point it's aimed at, and that point ends up in the middle of the picture. So you change the angle by moving one or both of them. You don't have to work out any rotation angles yourself.

Some things to try:

- To see the scene from the side, move the camera sideways and leave `look_at` where it is. That's the only difference between `all.txt` and `all-moved.txt`: the camera goes from `0 1.5 1.5` to `5 2 -0.5` and keeps looking at the same spot.
- To look down from higher up, raise the camera's `y`.
- To get closer, move the camera towards `look_at`. The field of view is fixed, so moving closer is how you zoom in.
- Keep `look_at` roughly in the middle of your objects, or they'll end up off to one side.

One thing to avoid: don't put the camera directly above `look_at` looking straight down. The camera uses "up" as a reference to work out which way is sideways, and if it's pointing straight down that breaks. Moving it even a little to one side fixes it.

### Making things shiny

Add `shine` and a number at the end of any object's line:

```
sphere  0 0 -4  1.0  1 0 0  shine 0.5
```

`0` means no highlight at all (the default) and `1` is a strong, bright one. Around `0.5` looks like plastic. I leave the floor at `0`, because a big shiny floor looks wet.

The highlight only shows where the light would bounce straight into the camera, so on curved shapes (spheres, the sides of cylinders) you'll always see it somewhere. On the flat faces of a cube you only see it from just the right angle.

### Making things reflective (bonus)

Add `reflect` and a number at the end of any object's line. `0` is completely matte (the default) and `1` is a perfect mirror. Anything in between mixes the object's own color with its reflection.

```
sphere  -2.2 0 -5  1.0  0.9 0.9 0.9  reflect 0.8  shine 0.8
```

You can use `reflect` and `shine` together, in either order. A small value like `reflect 0.2` on the floor looks nice, because you get a faint reflection of everything standing on it.

Remember that reflections only show up when you run with `-r`. Without it, reflective objects are drawn like normal ones.

### Putting it together: a new scene

Here's a full example. It's a scene with one of each shape, a back wall, a dimmer light on the right, and the camera a bit higher than usual. Save it as `scenes/my-scene.txt`:

```
# My scene: one of each shape, a dimmer light on the right

camera  0 3 2
look_at 0 -0.5 -5
light   3 6 0  0.8

sphere   -1.5 0 -5  1.0  1 0.5 0  shine 0.6
cube     1.5 -0.5 -4.5  1.0  0.6 0.2 0.8
cylinder 0 -1 -7  0.5 2.5  0.9 0.9 0.2  shine 0.3
plane    0 -1 0  0 1 0  0.6 0.6 0.6
plane    0 0 -12  0 0 1  0.9 0.85 0.7
```

And render it:

```bash
cargo run --release -- scenes/my-scene.txt 800 600 -o my-scene.ppm
```

No recompiling needed. Because the light is on the right this time (`x = 3`), the shadows will fall to the left.

## Where things are

- `scenes/`: the ready-made scene files (this is the folder you'll use most)
- `src/main.rs`: reads the command line, shares the rows out between threads, shoots the rays, does the lighting, highlights, shadows and reflections, and writes the image
- `src/scene.rs`: `Scene`, and the code that reads a scene file and turns each line into a shape, the light or the camera
- `src/vec3.rs`: `Vec3`, the three-number type used for positions, directions and colors
- `src/ray.rs`: a ray, which is a starting point plus a direction
- `src/camera.rs`: turns a pixel into a ray
- `src/light.rs`: the light's position, brightness and size
- `src/object.rs`: `Object`, which lets one list hold all four kinds of shape
- `src/sphere.rs`, `src/cube.rs`, `src/plane.rs`, `src/cylinder.rs`: each shape's hit test and surface direction
