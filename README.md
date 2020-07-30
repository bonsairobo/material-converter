# Material Converter

This is mostly a personal tool for use with the [Amethyst Voxel Mapper](https://github.com/amethyst/voxel-mapper).

I often find free PBR materials on the Internet, but they aren't in a proper
format for use with my shaders. So I can use this tool to convert them depending
on the type of texture. It can also vertically concatenate textures for use as
"array textures" (like sampler2DArray in GLSL).

```
material-converter 0.1.0
Tool for converting common material image formats.

USAGE:
    material-converter <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    convert-images         Given an assignment of material attribute to each image file path, convert the images to
                           the proper formats and give them canonical names
    feeling-lucky          Do a guess-input, convert-images, and make-array-material all in sequence, assuming all
                           input materials will be compatible (same size and set of attributes)
    guess-input            Given a directory of images, try to assign image files to their material attribute using
                           heuristics like file name. Produces a RON file to be used as input to the convert-images
                           command. Prints to stdout when it can't guess what a file is for or if there are
                           conflicting files
    help                   Prints this message or the help of the given subcommand(s)
    make-array-material    Combine multiple directories of images into a single directory of images, where the
                           images of each material attribute are concatenated vertically for use as an array texture
                           (sampler2DArray in GLSL). Assumes that images of the same attribute also have the same
                           file name (as ensured by the convert-images command)
```