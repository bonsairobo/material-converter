use clap::Parser;
use material_converter::{
    convert_images, feeling_lucky, guess_input, make_array_material, MaterialFormat,
};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Tool for converting common material image formats."
)]
enum Args {
    /// Use heuristics to guess the material attribute of each file.
    ///
    /// Given a directory of images, try to assign a material attribute to each
    /// image using heuristics like file name. Produces a RON file to be used
    /// as input to the convert-images command. Prints to stderr when it can't
    /// guess what a file is for or if there are conflicting files.
    GuessInput {
        /// The directory containing the input images.
        #[arg(short, long)]
        input: PathBuf,
        /// A RON file serialization of a `Vec<(MaterialImage, String)>`,
        /// containing the assignment guesses.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Convert images to the desired format.
    ///
    /// Given a file of assignments of material attribute to each image file
    /// path, convert the images to the proper formats and give them canonical
    /// names.
    ConvertImages {
        /// A RON file serialization of a `Vec<(MaterialImage, String)>`.
        #[arg(short, long)]
        assignments: PathBuf,
        /// The desired output material format.
        #[arg(short, long)]
        format: MaterialFormat,
        /// The output directory. Will be created if it does not exist.
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Combine multiple materials into an array material.
    ///
    /// Combine multiple directories of images into a single directory of
    /// images, where the images of each material attribute are concatenated
    /// vertically for use as an array texture (sampler2DArray in GLSL). Assumes
    /// that images of the same attribute also have the same file name (as
    /// ensured by the convert-images command).
    MakeArrayMaterial {
        /// The input directories.
        #[arg(short, long)]
        input: Vec<PathBuf>,
        /// The output directory. Will be created if it does not exist.
        #[arg(short, long)]
        output: PathBuf,
    },
    /// guess-input, convert-images, then make-array-material
    ///
    /// Assumes all input materials will be compatible (same size and set of
    /// attributes).
    FeelingLucky {
        /// The input directories.
        input: Vec<PathBuf>,
        /// The desired output material format.
        #[arg(short, long)]
        format: MaterialFormat,
        /// The output directory. Will be created if it does not exist.
        #[arg(short, long)]
        output: PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    match Args::parse() {
        Args::GuessInput {
            input: input_directory,
            output: output_file,
        } => guess_input(
            &input_directory,
            &output_file.unwrap_or_else(|| input_directory.join("guesses").with_extension("ron")),
        ),
        Args::ConvertImages {
            assignments: assignment_file,
            format,
            output: output_directory,
        } => convert_images(&assignment_file, &format, &output_directory),
        Args::MakeArrayMaterial {
            input: input_directories,
            output: output_directory,
        } => make_array_material(&input_directories, &output_directory),
        Args::FeelingLucky {
            input: input_directories,
            format: desired_material_format,
            output: output_directory,
        } => feeling_lucky(
            &input_directories,
            &desired_material_format,
            &output_directory,
        ),
    }
}
