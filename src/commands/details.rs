use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, INPUT_FILE_DOES_NOT_EXIST, INPUT_IS_NOT_FILE,
};
use crate::commands::ExecutableCommand;
use anyhow::{bail, Context, Result};
use clap::Parser;
use image::io::Reader;
use image::{GenericImageView, ImageFormat};
use owo_colors::{OwoColorize, Stream};
use std::path::PathBuf;

/// Print detailed information about an image in a pretty format.
#[derive(Debug, Clone, Parser)]
pub struct DetailsCommand {
    /// A path on disk to the image that should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,
}

impl ExecutableCommand for DetailsCommand {
    fn run(self) -> Result<()> {
        if !self.input_path.exists() {
            bail!(INPUT_FILE_DOES_NOT_EXIST);
        }
        if !self.input_path.is_file() {
            bail!(INPUT_IS_NOT_FILE);
        }

        let file_size = self.input_path.metadata()?.len();
        let image_format = ImageFormat::from_path(&self.input_path)?;
        let image = Reader::open(&self.input_path)?
            .decode()
            .context(ERROR_IMGDECODE_CTX)?;
        let dimensions = image.dimensions();

        println!(
            "{}",
            self.input_path
                .file_name()
                .context("failed to obtain filename")?
                .to_str()
                .context("failed to converto regular string")?
                .if_supports_color(Stream::Stdout, |text| text.cyan())
                .if_supports_color(Stream::Stdout, |text| text.bold())
        );
        println!("  * Format: {:?}", image_format);
        println!("  * Size (bytes): {:?}", file_size);
        println!("  * Dimensions: {}x{}", dimensions.0, dimensions.1);
        println!("  * Color Type: {:?}", image.color());

        Ok(())
    }
}
