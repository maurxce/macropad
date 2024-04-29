use crate::consts::VENDOR_ID;
use crate::keyboard::LedColor;
use crate::parse;
use clap::{Args, Parser, Subcommand};
use std::num::ParseIntError;

#[derive(Parser)]
pub struct Options {
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub devel_options: DevelOptions,
}

#[derive(Args, Debug)]
#[clap(
    hide(true),
    next_help_heading = "Development options (use with caution!)"
)]
pub struct DevelOptions {
    #[arg(long, default_value_t=VENDOR_ID, value_parser=hex_or_decimal, hide=true)]
    pub vendor_id: u16,

    #[arg(long, value_parser=hex_or_decimal, default_value_t=0x8840, hide=true)]
    pub product_id: u16,

    #[arg(long, value_parser=parse_address, hide=true)]
    pub address: Option<(u8, u8)>,

    /// OUT endpoint address where data is written
    #[arg(long, default_value_t = 0x4, hide = true)]
    pub out_endpoint_address: u8,

    /// IN endpoint address where data is read
    #[arg(long, default_value_t = 0x84, hide = true)]
    pub in_endpoint_address: u8,

    #[arg(long, hide = true)]
    pub interface_number: Option<u8>,
}

pub fn hex_or_decimal(s: &str) -> Result<u16, ParseIntError> {
    if s.to_ascii_lowercase().starts_with("0x") {
        u16::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u16>()
    }
}

fn parse_address(s: &str) -> std::result::Result<(u8, u8), nom::error::Error<String>> {
    parse::from_str(parse::address, s)
}

#[derive(Subcommand)]
pub enum Command {
    /// Show supported keys and modifiers
    ShowKeys,

    /// Validate key mappings config
    Validate {
        /// Configuration file in ron format
        #[clap(short, long, default_value = "./mapping.ron")]
        config_file: String,
    },

    /// Program key mappings
    Program {
        /// Configuration file in ron format
        #[clap(short, long, default_value = "./mapping.ron")]
        config_file: String,
    },

    /// Read configuration from device
    Read {
        /// Layer to read data for (layer is one based; 0 reads all layers)
        #[clap(short, long, default_value_t = 0)]
        layer: u8,
    },

    /// Select LED backlight mode
    Led(LedCommand),
}

#[derive(Parser, Clone, Default, Debug)]
pub struct LedCommand {
    /// Index of LED modes
    /// --------0x8840----------
    /// 0 - LEDs off
    /// 1 - backlight always on with LedColor
    /// 2 - no backlight, shock with LedColor when key pressed
    /// 3 - no backlight, shock2 when LedColor when key pressed
    /// 4 - no backlight, light up key with LedColor when pressed
    /// 5 - backlight white always on
    /// --------0x8890---color is not supported-------
    /// 0 - LEDs off
    /// 1 - LED on for last pushed key
    /// 2 - cycle through colors & buttons
    #[clap(verbatim_doc_comment)]
    pub index: u8,

    // made this an option because the 884x supports color but the 8890
    // does not. defaults to Red, but since the 8890 does not accept
    // setting color, it just gets ignored
    /// Note: Not applicable for product id 0x8890
    /// Color to apply with mode
    #[arg(value_enum, verbatim_doc_comment)]
    pub led_color: Option<LedColor>,
}
