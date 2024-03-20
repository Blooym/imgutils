mod blur;
mod brighten;
mod contrast;
mod crop;
mod flip;
mod grayscale;
mod hue;
mod invert;
mod reformat;
mod resize;
mod rotate;

use self::blur::BlurCommand;
use self::brighten::BrightenCommand;
use self::contrast::ContrastCommand;
use self::crop::CropCommand;
use self::flip::FlipCommand;
use self::grayscale::GrayscaleCommand;
use self::hue::HueCommand;
use self::invert::InvertCommand;
use self::reformat::ReformatCommand;
use self::resize::ResizeCommand;
use self::rotate::RotateCommand;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use super::ExecutableCommand;
use std::time::Duration;

/// A collection of commands that perform modifications to images.
#[derive(Debug, Parser)]
pub struct ModifyCommandBase {
    #[clap(subcommand)]
    subcommand: ModifySubcommand,
}

#[derive(Debug, Parser)]
pub enum ModifySubcommand {
    Blur(BlurCommand),
    Brighten(BrightenCommand),
    Contrast(ContrastCommand),
    Crop(CropCommand),
    Flip(FlipCommand),
    Grayscale(GrayscaleCommand),
    Invert(InvertCommand),
    Hue(HueCommand),
    Reformat(ReformatCommand),
    Resize(ResizeCommand),
    Rotate(RotateCommand),
}

const PROGRESSBAR_TEMPLATE_RUNNING: &str = "🌸 [{elapsed}] {spinner} {msg}";
const PROGRESSBAR_TEMPLATE_FINISHED: &str = "🌸 {msg} in {elapsed_precise}";
const PROGRESSBAR_TICK_RATE_MS: u64 = 400;

impl ExecutableCommand for ModifyCommandBase {
    fn run(self) -> Result<()> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_style(ProgressStyle::with_template(PROGRESSBAR_TEMPLATE_RUNNING)?);
        progress_bar.set_message("Now processing image, please wait...");
        progress_bar.enable_steady_tick(Duration::from_millis(PROGRESSBAR_TICK_RATE_MS));

        if let Err(err) = match self.subcommand {
            ModifySubcommand::Blur(cmd) => cmd.run(),
            ModifySubcommand::Brighten(cmd) => cmd.run(),
            ModifySubcommand::Contrast(cmd) => cmd.run(),
            ModifySubcommand::Crop(cmd) => cmd.run(),
            ModifySubcommand::Flip(cmd) => cmd.run(),
            ModifySubcommand::Grayscale(cmd) => cmd.run(),
            ModifySubcommand::Invert(cmd) => cmd.run(),
            ModifySubcommand::Hue(cmd) => cmd.run(),
            ModifySubcommand::Reformat(cmd) => cmd.run(),
            ModifySubcommand::Resize(cmd) => cmd.run(),
            ModifySubcommand::Rotate(cmd) => cmd.run(),
        } {
            progress_bar.finish_and_clear();
            return Err(err);
        }

        progress_bar.set_style(ProgressStyle::with_template(PROGRESSBAR_TEMPLATE_FINISHED)?);
        progress_bar.finish_with_message("Successfully processed image");

        Ok(())
    }
}
