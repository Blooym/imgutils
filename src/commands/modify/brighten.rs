use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, ERROR_IMGSAVE_CTX, ERROR_IMGTYPEPARSE_CTX, INPUT_FILE_DOES_NOT_EXIST,
    INPUT_IS_NOT_FILE, OUTPUT_ALREADY_EXISTS,
};
use crate::commands::ExecutableCommand;
use anyhow::{bail, Context, Result};
use clap::Parser;
use image::{io::Reader, ImageFormat};
use std::path::PathBuf;

/// Brighten the pixels of an image.
#[derive(Debug, Clone, Parser)]
pub struct BrightenCommand {
    /// A path on disk to the image that should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,

    /// A path on disk to where the output imageshould be placed.
    /// The image will automatically converted to file type of the
    /// file extension if possible.
    #[arg(short = 'o', long = "output")]
    pub output_path: PathBuf,

    /// The amount to brighten each pixel by.
    /// Negative values decrease the brightness and positive values increase it.
    #[arg(long = "amount", allow_hyphen_values = true)]
    pub amount: i32,

    /// Overwrite any existing file at the output path.
    #[arg(long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}

impl ExecutableCommand for BrightenCommand {
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
        Reader::open(self.input_path)?
            .decode()
            .context(ERROR_IMGDECODE_CTX)?
            .brighten(self.amount)
            .save_with_format(self.output_path, output_format)
            .context(ERROR_IMGSAVE_CTX)?;

        Ok(())
    }
}
