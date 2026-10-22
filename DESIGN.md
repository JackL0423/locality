# Design for program to Rotate image using Array2

## Functions

### Transform

```rust
pub fn transform(
    src: &Array2<Rgb>,
    dst_w: usize,
    dst_h: usize,
    order: &Order,
    map: impl Fn(usize, usize) -> (usize, usize),
) -> Array2<Rgb> {
    let blank = Rgb {
        red: 0,
        green: 0,
        blue: 0,
    };
    let (w, h) = (src.get_width(), src.get_height());
    let mut dst = Array2::new(dst_w, dst_h, blank);
    let s = src.as_slice();
    let d = dst.as_mut_slice();
    let mut copy = |c: usize, r: usize| {
        let (dc, dr) = map(c, r);
        d[dr * dst_w + dc] = s[r * w + c].clone();
    };
    match order {
        Order::Row => {
            for r in 0..h {
                for c in 0..w {
                    copy(c, r)
                }
            }
        }
        Order::Col => {
            for c in 0..w {
                for r in 0..h {
                    copy(c, r)
                }
            }
        }
        Order::Block => {
            for tr in (0..h).step_by(TILE) {
                for tc in (0..w).step_by(TILE) {
                    for r in tr..(tr + TILE).min(h) {
                        for c in tc..(tc + TILE).min(w) {
                            copy(c, r)
                        }
                    }
                }
            }
        }
    }
    dst
}
```

#### Thought process

- Every transformation is a mapping from a source coordinate `(col, row)` to a destination coordinate, so one generic function does the copying and each transformation only supplies its `map`.
- Make a blank destination of size `dst_w x dst_h`, visit every source pixel, and copy it to the mapped location.
- Loops give `(col, row)` directly, and both arrays are indexed through their row-major slices (`row * width + col`), so there is no per-pixel division or `Option` bounds check.
- `Order` picks the traversal so the orders can be compared for locality:
  - row-major: reads in memory order; writes jump when rows become columns (rotate 90/270, transpose).
  - column-major: the reverse, reads jump and writes are in order.
  - block-major (tiled): visits `TILE x TILE` squares (`TILE = 32`). A tile's source and destination cache lines all fit in L1, so both sides reuse every line they load. Default for rotate 90/270 and transpose.
- Rotate 180 maps rows to rows, so row-major is already in memory order on both sides; it is the default for 180, but any order can be passed to compare. The flips always use row-major.

### Rotate 90

```rust
pub fn rotate90(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (h - j - 1, i))
}
```

#### Thought process

- Rotating 90° clockwise swaps the dimensions, so the destination is `h` wide and `w` tall.
- Column `i` of the source becomes row `i` of the destination.
- Row `j` becomes a column counted from the right edge, so the top row (`j = 0`) ends up in the last column, `h - 1`.
- So `(i, j) -> (h - j - 1, i)`.

### Rotate 180

```rust
pub fn rotate180(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, w, h, o, |i, j| (w - i - 1, h - j - 1))
}
```

#### Thought process

- Rotating 180° keeps the dimensions the same (`w x h`).
- Each pixel moves to the opposite corner, so both the column and the row are mirrored: the first column becomes the last and the first row becomes the last.
- So `(i, j) -> (w - i - 1, h - j - 1)`.

### Rotate 270

```rust
pub fn rotate270(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (j, w - i - 1))
}
```

#### Thought process

- Rotating 270° clockwise (90° counter-clockwise) swaps the dimensions, so the destination is `h` wide and `w` tall.
- Row `j` of the source becomes column `j` of the destination.
- Column `i` becomes a row counted from the bottom, so the leftmost column (`i = 0`) ends up in the last row, `w - 1`.
- So `(i, j) -> (j, w - i - 1)`. This is the reverse of rotate 90.

### flip horizontal

### flip vertical

### transpose

```rust
pub fn transpose(src: &Array2<Rgb>, o: &Order) -> Array2<Rgb> {
    let (w, h) = (src.get_width(), src.get_height());
    transform(src, h, w, o, |i, j| (j, i))
}
```

#### Thought process

- Transposing reflects the image across its main diagonal (top-left to bottom-right), so the dimensions swap to `h x w`.
- The column and row just trade places, with nothing mirrored.
- So `(i, j) -> (j, i)`.
