use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, ERROR_IMGREAD_CTX, ERROR_IMGSAVE_CTX, ERROR_IMGTYPEPARSE_CTX,
    INPUT_FILE_DOES_NOT_EXIST, INPUT_IS_NOT_FILE, OUTPUT_ALREADY_EXISTS,
};
use crate::commands::ExecutableCommand;
use anyhow::{bail, Context, Result};
use clap::Parser;
use image::{io::Reader, ImageFormat};
use std::path::PathBuf;

/// Change an image's file format.
#[derive(Debug, Clone, Parser)]
pub struct FormatCommand {
    /// A path on disk to the image that should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,

    /// A path on disk to where the output image should be placed.
    /// You should specify the file extension of the image type you would
    /// like to convert to here (e.g. Converting to a png would mean adding .png to
    /// the end of the file name)
    #[arg(short = 'o', long = "output")]
    pub output_path: PathBuf,

    /// Overwrite any existing file at the output path.
    #[arg(long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}

impl ExecutableCommand for FormatCommand {
    fn run(self) -> Result<()> {
        if !self.input_path.exists() {
            bail!(INPUT_FILE_DOES_NOT_EXIST);
        }
        if !self.input_path.is_file() {
            bail!(INPUT_IS_NOT_FILE);
        }
        if self.output_path.exists() && !self.overwrite {
            bail!(OUTPUT_ALREADY_EXISTS);
        }

        let output_format =
            ImageFormat::from_path(&self.output_path).context(ERROR_IMGTYPEPARSE_CTX)?;
        Reader::open(self.input_path)
            .context(ERROR_IMGREAD_CTX)?
            .decode()
            .context(ERROR_IMGDECODE_CTX)?
            .save_with_format(&self.output_path, output_format)
            .context(ERROR_IMGSAVE_CTX)?;

        Ok(())
    }
}
