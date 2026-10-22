# array2 design notes

`src/lib.rs` implements `Array2<T>`, a fixed-size 2D array.

## Storage

- Backed by a single `Vec<T>` (`data`), plus `width`/`height`.
- Always stored **row-major**: element `(row, col)` lives at
  `data[row * width + col]`.
- `width`/`height` are set once at construction and never change — there is
  no resize method, so the `data.len() == width * height` invariant always
  holds.
- `from_column_major` accepts column-major input but transposes it into the
  same row-major layout at construction time, so every other method can
  assume row-major storage without a separate code path.

## Element access

- `index(row, col) -> Option<usize>` is the single private helper that
  bounds-checks and computes the flat offset; `at`, `at_mut`, and `put` all
  go through it so bounds-checking logic isn't duplicated.
- All three return `None`/`false` on out-of-range coordinates instead of
  panicking.

## Iteration

- `iter_row_major` is just `self.data.iter()` — row-major traversal order is
  identical to the backing `Vec`'s order, so no index arithmetic is needed.
- `iter_column_major` walks each column with
  `self.data[col..].iter().step_by(width)` instead of recomputing
  `row * width + col` per element. The stride access itself is inherent to
  column-major traversal of row-major storage (not fixable without
  transposing), but this avoids the redundant per-element multiply.

## Error handling conventions

- Construction (`new`, `from_row_major`, `from_column_major`) panics on bad
  input (`usize` overflow in `width * height`, or mismatched `data.len()`)
  since these are programmer errors at setup time.
- Per-element access (`at`, `at_mut`, `put`) never panics — bad coordinates
  are a normal runtime condition, so they return `Option`/`bool`.

## Testing

- `#[cfg(test)] mod tests` in `lib.rs` covers: construction, both iteration
  orders (checked against the full expected sequence, not just the first
  element), row-major/column-major construction round-tripping to the
  correct `(row, col)` values, in-bounds and out-of-bounds `at`/`at_mut`/
  `put`, and non-square dimensions.
- Every public method's doc comment also has a runnable example, checked by
  `cargo test` as a doctest.
