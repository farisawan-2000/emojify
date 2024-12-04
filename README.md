# emojify
### Turn your images into elegant emoji grids
---
## Usage

You know how to build Rust apps. Two binaries are built: `emojify`, and `emojify-camera`

The simplest usage is to have an image and a width in mind.

`./emojify <output grid width> path/to/image.png`

`./emojify <output grid width> path/to/image.gif`

This will make an image `output.png` or `output.gif` in the same folder as the `emojify` program.

You can also use an image in your clipboard, or have it download an image/GIF from a URL in your clipboard by omitting a file path.

`./emojify <output grid width>`

This program utilizes multiprocessing for each pixel conversion, and is really fast as a result. This is why a second proof-of-concept binary, `emojify-camera`, is provided to show that conversion can happen in real time at certain resolutions.

`./emojify-camera <output grid width>`

## Custom Emoji/Image sets

`emojify` can use any emoji font (and also technically any image set at all, but you're on your own there).

To make a custom emoji font, reference the hashmap at `src/rgb2emoji.rs` (You will also want to regenerate it to update the average colors). An emoji image set folder has 16x16 representations of any emoji you want


## Limitations

- Compound emojis joined with a ZWJ are not tested.