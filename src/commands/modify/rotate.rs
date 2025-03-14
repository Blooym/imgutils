use crate::commands::ExecutableCommand;
use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, ERROR_IMGREAD_CTX, ERROR_IMGSAVE_CTX, ERROR_IMGTYPEPARSE_CTX,
    INPUT_FILE_DOES_NOT_EXIST, INPUT_IS_NOT_FILE, OUTPUT_ALREADY_EXISTS,
};
use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use image::{ImageFormat, ImageReader};
use std::path::PathBuf;

/// Rotate an image clockwise by set degree increments.
#[derive(Debug, Clone, Parser)]
pub struct RotateCommand {
    /// A path on disk to the image that should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,

    /// A path on disk to where the output imageshould be placed.
    /// The image will automatically converted to file type of the
    /// file extension if possible.
    #[arg(short = 'o', long = "output")]
    pub output_path: PathBuf,

    /// The amount of degrees clockwise to rotate this image.
    #[arg(long = "amount")]
    pub amount: RotationAmountDegrees,

    /// Overwrite any existing file at the output path.
    #[arg(long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}

impl ExecutableCommand for RotateCommand {
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
        let mut image = ImageReader::open(self.input_path)
            .context(ERROR_IMGREAD_CTX)?
            .decode()
            .context(ERROR_IMGDECODE_CTX)?;

        match self.amount {
            RotationAmountDegrees::Nintey => image = image.rotate90(),
            RotationAmountDegrees::OneEighty => image = image.rotate180(),
            RotationAmountDegrees::TwoSeventy => image = image.rotate270(),
        };

        image
            .save_with_format(&self.output_path, output_format)
            .context(ERROR_IMGSAVE_CTX)?;

        Ok(())
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RotationAmountDegrees {
    #[clap(name = "90")]
    Nintey,
    #[clap(name = "180")]
    OneEighty,
    #[clap(name = "270")]
    TwoSeventy,
}
