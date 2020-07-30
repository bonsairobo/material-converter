use material_converter::{
    convert_images, feeling_lucky, guess_input, make_array_material, MaterialFormat,
};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "material-converter",
    about = "Tool for converting common material image formats."
)]
enum Opt {
    /// Given a directory of images, try to assign image files to their material attribute using
    /// heuristics like file name. Produces a RON file to be used as input to the convert-images
    /// command. Prints to stdout when it can't guess what a file is for or if there are conflicting
    /// files.
    GuessInput {
        /// The directory containing the input images.
        #[structopt(parse(from_os_str))]
        input_directory: PathBuf,
        /// A RON file serialization of a `Vec<(MaterialImage, String)>`, containing the assignment
        /// guesses.
        #[structopt(parse(from_os_str))]
        output_file: PathBuf,
    },
    /// Given an assignment of material attribute to each image file path, convert the images to the
    /// proper formats and give them canonical names.
    ConvertImages {
        /// A RON file serialization of a `Vec<(MaterialImage, String)>`.
        #[structopt(parse(from_os_str))]
        assignment_file: PathBuf,
        /// The material format to convert into.
        desired_material_format: MaterialFormat,
        /// Where to put the resulting image files. Will create the directory if it doesn't exist.
        #[structopt(parse(from_os_str))]
        output_directory: PathBuf,
    },
    /// Combine multiple directories of images into a single directory of images, where the images
    /// of each material attribute are concatenated vertically for use as an array texture
    /// (sampler2DArray in GLSL). Assumes that images of the same attribute also have the same file
    /// name (as ensured by the convert-images command).
    MakeArrayMaterial {
        input_directories: Vec<PathBuf>,
        #[structopt(parse(from_os_str))]
        output_directory: PathBuf,
    },
    /// Do a guess-input, convert-images, and make-array-material all in sequence, assuming all
    /// input materials will be compatible (same size and set of attributes).
    FeelingLucky {
        input_directories: Vec<PathBuf>,
        #[structopt(long)]
        desired_material_format: MaterialFormat,
        #[structopt(long, parse(from_os_str))]
        output_directory: PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    match opt {
        Opt::GuessInput {
            input_directory,
            output_file,
        } => guess_input(&input_directory, &output_file),
        Opt::ConvertImages {
            assignment_file,
            desired_material_format,
            output_directory,
        } => convert_images(
            &assignment_file,
            &desired_material_format,
            &output_directory,
        ),
        Opt::MakeArrayMaterial {
            input_directories,
            output_directory,
        } => make_array_material(&input_directories, &output_directory),
        Opt::FeelingLucky {
            input_directories,
            desired_material_format,
            output_directory,
        } => feeling_lucky(
            &input_directories,
            &desired_material_format,
            &output_directory,
        ),
    }
}
