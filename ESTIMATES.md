# Estimates

## Cache Estimated Hit Rate

| | row-major access | column-major access |
| --- | --- | --- |
| 90-deg rotation | 2 | 3 |
| 180-deg rotation | 1 | 4 |

### Justification

180 row-major is fastest since it is reading a source row left to right writes destination row right to left, so both step through adjacent memory. Each cache line fully loaded before it's evicted on both sides.

Both 90-degree rotations will tie. No matter which traversal you choose, one side jumps. Row-major in order adn writes to destination columns. Column-major reads down source columns and writes in order.

180 column-major is slowest since both sides jump a full row on every step. So it has a miss on both the load and the store.

## Per Pixel Operation Estimates

Additions: 3
Subtractions: 4
Multiplications: 2
Divisions and Mods: 0
Comparisions: 3
Loads: 1
Hit Rate: ~95%
Stores: 4

## Actual timings

Data comes from running `./bench.sh` for the mallory.ppm

| case | min (ms) | avg (ms) | max (ms) |
| --- | --- | --- | --- |
| 90 row-major | 54.95 | 55.28 | 55.56 |
| 90 col-major | 52.60 | 53.05 | 53.42 |
| 90 block-major | 19.16 | 19.33 | 19.48 |
| 180 row-major | 10.79 | 11.63 | 12.35 |
| 180 col-major | 83.49 | 83.95 | 84.49 |
| 180 block-major | 17.36 | 17.55 | 17.81 |

### Did my measurements match?

For the most part my predictions did actually match the data that came out. 180-row-major did end up being the fastest by reading the source left to right and then writing to the destination right to left.

Both 90 degree for row/col-major were about the same withing ~3ms difference so my prediction also matched there as I explained in the `Justification` section they are doing similar operations.

As I predicted the 180-column major was by far the slowest operation since it had to jump the full rows on every step so it has a miss on both the load and store.
