# Mandelbrot

This is a sideproject where I learned about the Mandelbrot set.

Compile and run with

```
cargo run --release
```

This will generate and image called `fractal.png`.

### Notes

I used rayon to speedup the iterations. This gave a massive performance boost for large images.

### Resources and learning materials used

https://plus.maths.org/content/unveiling-mandelbrot-set

https://github.com/willi-kappler/mandel-rust

https://rosettacode.org/wiki/Mandelbrot_set#Rust
