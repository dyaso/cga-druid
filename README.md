# cga-druid
Conformal Geometric Algebra experiments in Rust. Uses:

 * Druid to get a window and receive input events
 * Piet to draw in 2D
 * Ganja.js's generated Rust code for the conformal 2D algebra

I want to make UIs based around vector graphics instead of pixels, so a simple drawing app seems an obvious project.

### Progress

2020-10-28 Reading "[A Covariant Approach to Geometry using Geometric Algebra](Covarient Approach to Geometry Using Geometric Algebra.pdf)"

### Roadmap

 * Turtle graphics or PostScript style image description language
 * Ganja 'coffee shop' style interactive playground (with Lua for describing scenes in place of JS)
 * UI elements like buttons and a drawing canvas

### Questions

How axis-aligned bounding boxes should work (for UI elements), or just use circles for everything? Any problems with ellipses produced via non-uniform scaling of coordinates? Normalization.

