use crate::commands::ExecutableCommand;
use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, INPUT_FILE_DOES_NOT_EXIST, INPUT_IS_NOT_FILE,
};
use anyhow::{Context, Result, bail};
use clap::Parser;
use image::{GenericImageView, ImageReader};
use std::path::PathBuf;

/// Print an image's dimensions formatted as 'WidthxHeight'.
#[derive(Debug, Clone, Parser)]
pub struct DimensionsCommand {
    /// A path on disk to the image that should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,
}

impl ExecutableCommand for DimensionsCommand {
    fn run(self) -> Result<()> {
        if !self.input_path.exists() {
            bail!(INPUT_FILE_DOES_NOT_EXIST);
        }
        if !self.input_path.is_file() {
            bail!(INPUT_IS_NOT_FILE);
        }

        let dimensions = ImageReader::open(&self.input_path)?
            .decode()
            .context(ERROR_IMGDECODE_CTX)?
            .dimensions(); // Width = 0, Height = 1
        println!("{}x{}", dimensions.0, dimensions.1);

        Ok(())
    }
}
