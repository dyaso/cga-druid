## cga-druid: Conformal Geometric Algebra experiments in Rust

I want to make UIs based around vector graphics instead of pixels, so a simple drawing app seems an obvious project.

### Plan

 * Turtle graphics or PostScript style image description language
 * Ganja 'coffee shop' style interactive playground
 * UI elements like buttons and a drawing canvas

### Progress

`2020-10-28` Reading "[A Covariant Approach to Geometry using Geometric Algebra](Covarient%20Approach%20to%20Geometry%20Using%20Geometric%20Algebra.pdf)"

### Questions

How axis-aligned bounding boxes should work (for UI elements), or just use circles for everything? Any problems with ellipses produced via non-uniform scaling of coordinates. Normalization.

#### What scripting language to use?

https://arewegameyet.rs/ecosystem/scripting/

The most important ability is writing expressions with multiple different products and unary operators (dual, reverse, complement, conjugate, involution, i'm not sure), so either easy operator overloading or effective processing of source code is a must (the latter likely meaning a Lisp).


