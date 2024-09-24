## RUST exercise
### Image Processing
#### Image crop and resizing
```rust
// crop the original image to the x, y, w, z.
let img = imageops::crop(&mut img, x, y, w, z).to_image();
let img2 = imageops::crop(&mut imag, heigh, length, filter).to_image()
```
- Example Code: [image_thumb](./image_thumb/)

## Thread Handling