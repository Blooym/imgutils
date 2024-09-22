use crate::commands::messages::{
    ERROR_IMGDECODE_CTX, ERROR_IMGREAD_CTX, ERROR_IMGSAVE_CTX, ERROR_IMGTYPEPARSE_CTX,
    INPUT_FILE_DOES_NOT_EXIST, INPUT_IS_NOT_FILE, OUTPUT_ALREADY_EXISTS,
};
use crate::commands::ExecutableCommand;
use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use image::{imageops::FilterType, ImageFormat, ImageReader};
use std::path::PathBuf;

/// Resize an image, optionally specifying maintaining aspect ratio.
#[derive(Debug, Clone, Parser)]
pub struct ResizeCommand {
    /// A path on disk to the file image should be loaded.
    #[arg(short = 'i', long = "input")]
    pub input_path: PathBuf,

    /// A path on disk to where the output imageshould be placed.
    /// The image will automatically converted to file type of the
    /// file extension if possible.
    #[arg(short = 'o', long = "output")]
    pub output_path: PathBuf,

    /// The new width of the image.
    #[arg(long = "width")]
    pub width: u32,

    /// The new height of the image.
    #[arg(long = "height")]
    pub height: u32,

    /// The type of sampling filter to use when resizing this image.
    #[arg(long = "filter-type", default_value = "nearest")]
    pub filter_type: ValueEnumFilterType,

    /// Maintain the image's aspect ratio while resizing it.
    #[arg(long = "keep-aspect-ratio")]
    pub keep_aspect_ratio: bool,

    /// Overwrite any existing file at the output path.
    #[arg(long = "overwrite", default_value_t = false)]
    pub overwrite: bool,
}

impl ExecutableCommand for ResizeCommand {
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

        match self.keep_aspect_ratio {
            true => {
                image = image.resize(self.width, self.height, self.filter_type.to_image_variant())
            }
            false => {
                image = image.resize_exact(
                    self.width,
                    self.height,
                    self.filter_type.to_image_variant(),
                );
            }
        };

        image
            .save_with_format(&self.output_path, output_format)
            .context(ERROR_IMGSAVE_CTX)?;

        Ok(())
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ValueEnumFilterType {
    /// Nearest Neighbor
    Nearest,

    /// Linear Filter
    Triangle,

    /// Cubic Filter
    CatmullRom,

    /// Gaussian Filter
    Gaussian,

    /// Lanczos with window 3
    Lanczos3,
}

impl ValueEnumFilterType {
    pub fn to_image_variant(&self) -> FilterType {
        match self {
            ValueEnumFilterType::Nearest => FilterType::Nearest,
            ValueEnumFilterType::Triangle => FilterType::Triangle,
            ValueEnumFilterType::CatmullRom => FilterType::CatmullRom,
            ValueEnumFilterType::Gaussian => FilterType::Gaussian,
            ValueEnumFilterType::Lanczos3 => FilterType::Lanczos3,
        }
    }
}
