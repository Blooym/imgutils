mod details;
mod dimensions;
mod modify;

use self::details::DetailsCommand;
use self::dimensions::DimensionsCommand;
use self::modify::ModifyCommandBase;

use anyhow::Result;
use clap::{
    builder::{
        styling::{AnsiColor, Effects},
        Styles,
    },
    Parser,
};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::BrightMagenta.on_default() | Effects::BOLD)
        .usage(AnsiColor::BrightMagenta.on_default() | Effects::BOLD)
        .literal(AnsiColor::White.on_default())
        .placeholder(AnsiColor::White.on_default())
}

/// Command-line utility for quickly getting information about and manipulating images.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about, styles = styles())]
pub struct ProcessCommandRoot {
    #[clap(subcommand)]
    pub cmd: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    Details(DetailsCommand),
    Dimensions(DimensionsCommand),
    Modify(ModifyCommandBase),
}

pub trait ExecutableCommand {
    /// Consume the instance of and run this command.
    fn run(self) -> Result<()>;
}

impl ExecutableCommand for ProcessCommandRoot {
    fn run(self) -> Result<()> {
        match self.cmd {
            Commands::Details(cmd) => cmd.run(),
            Commands::Dimensions(cmd) => cmd.run(),
            Commands::Modify(cmd) => cmd.run(),
        }
    }
}

mod messages {
    pub const INPUT_FILE_DOES_NOT_EXIST: &str = "Unable to find file at the given input path";
    pub const INPUT_IS_NOT_FILE: &str = "Input path did not lead to a file.";
    pub const OUTPUT_ALREADY_EXISTS: &str =
        "Output file already exists. Use --overwrite to replace it";

    pub const ERROR_IMGREAD_CTX: &str = "error occured whilst reading image";
    pub const ERROR_IMGDECODE_CTX: &str = "error occured whilst decoding image";
    pub const ERROR_IMGSAVE_CTX: &str = "error occured whilst saving image to disk";
    pub const ERROR_IMGTYPEPARSE_CTX: &str =
        "error occured whilst parsing image extension to determine filetype";
}
