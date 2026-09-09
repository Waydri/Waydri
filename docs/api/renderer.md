# SoftwareRenderer API

The `SoftwareRenderer` provides CPU-based rendering for environments without GPU acceleration. It is the fallback renderer when Vulkan and GLES are unavailable.

## Construction

```rust
let renderer = SoftwareRenderer::new(1920, 1080)?;
```

Creates a software renderer with the specified dimensions. Allocates an internal framebuffer of `width * height * 4` bytes.

```rust
let renderer = SoftwareRenderer::new_with_fallback(1920, 1080)?;
```

Tries to use SIMD-optimized rendering paths. Falls back to scalar operations if the platform does not support the required instruction set.

## Methods

### `fill_rect(&mut self, rect: Rect, color: Color)`

Fills a rectangular area with a solid color. Clips the rectangle to the framebuffer bounds.

```rust
renderer.fill_rect(
    Rect::new(10, 10, 200, 100),
    Color::rgba(255, 128, 0, 255),
);
```

### `draw_line(&mut self, start: Point, end: Point, color: Color, thickness: u32)`

Draws a line between two points using Bresenham's algorithm with anti-aliasing. The line is rendered at the specified thickness in pixels.

### `fill_gradient(&mut self, rect: Rect, gradient: Gradient)`

Fills a rectangle with a gradient. Supports linear and radial gradients.

```rust
renderer.fill_gradient(
    Rect::new(0, 0, 1920, 1080),
    Gradient::Linear {
        start: Point::new(0, 0),
        end: Point::new(0, 1080),
        colors: (Color::BLACK, Color::rgb(20, 20, 30)),
    },
);
```

### `render_window(&mut self, window: &Window, source: &FrameBuffer, dest: Rect)`

Renders a window's buffer into the framebuffer at the specified destination rectangle. Handles scaling, opacity, and optional rounded corners.

### `frame(&self) -> &FrameBuffer`

Returns a reference to the current frame buffer. The buffer is in RGBA8 format, 4 bytes per pixel.

### `clear(&mut self, color: Color)`

Fills the entire framebuffer with the specified color.

### `resize(&mut self, width: u32, height: u32) -> Result<()>`

Resizes the framebuffer. Preserves existing content where possible. Returns an error if the allocation fails.

## DamageTracker

The `DamageTracker` records which regions of the screen have changed since the last frame. Only damaged regions are redrawn, saving CPU time.

```rust
let mut damage = DamageTracker::new(1920, 1080);

damage.mark_rect(Rect::new(100, 100, 300, 200));
damage.mark_rect(Rect::new(0, 0, 1920, 30));

let regions = damage.damaged_regions();
for region in regions {
    renderer.fill_rect(region, Color::TRANSPARENT);
    redraw_region(&mut renderer, region);
}

damage.clear();
```

### Methods

#### `mark_rect(&mut self, rect: Rect)`

Marks a rectangular region as damaged.

#### `mark_full(&mut self)`

Marks the entire screen as damaged. Forces a full redraw.

#### `damaged_regions(&self) -> &[Rect]`

Returns a list of merged, non-overlapping damaged regions.

#### `clear(&mut self)`

Clears all damage. Called after rendering is complete.

#### `is_damaged(&self) -> bool`

Returns `true` if any region is marked as damaged.

## Gradient

```rust
pub enum Gradient {
    Linear {
        start: Point,
        end: Point,
        colors: (Color, Color),
    },
    Radial {
        center: Point,
        radius: f64,
        colors: (Color, Color),
    },
}
```

## Color

```rust
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self { ... }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { ... }
    pub fn hex(hex: u32) -> Self { ... }
}
```

## Performance

The software renderer is designed for correctness over speed. For production use, prefer the Vulkan or GLES backends. The software renderer is useful for:

- Headless rendering and testing
- Environments without GPU support
- Off-screen compositing for screen capture
- Debugging rendering issues
