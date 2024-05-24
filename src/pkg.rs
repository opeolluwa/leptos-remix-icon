
use serde::{Deserialize, Serialize};

/// the icon size can be one of 10x, sm, lg ... etc
/// this ported directly from remixicon.css
/// .ri-lg { font-size: 1.3333em; line-height: 0.75em; vertical-align: -.0667em; }
/// .ri-xl { font-size: 1.5em; line-height: 0.6666em; vertical-align: -.075em; }
/// .ri-xxs { font-size: .5em; }
/// .ri-xs { font-size: .75em; }
/// .ri-sm { font-size: .875em }
/// .ri-sm { font-size: .875em }
/// .ri-1x { font-size: 1em; }
/// .ri-2x { font-size: 2em; }
/// .ri-3x { font-size: 3em; }
/// .ri-4x { font-size: 4em; }
/// .ri-5x { font-size: 5em; }
/// .ri-6x { font-size: 6em; }
/// .ri-7x { font-size: 7em; }
/// .ri-8x { font-size: 8em; }
/// .ri-9x { font-size: 9em; }
/// .ri-10x { font-size: 10em; }
/// .ri-fw { text-align: center; width: 1.25em; }
///
///

#[derive(Debug, Serialize, Deserialize, Default)]
/// An enum to represent CSS class variants for font size and text alignment.
pub enum IconSize {
    /// .ri-xl { font-size: 1.5em; line-height: 0.6666em; vertical-align: -.075em; }
    RiXl,
    /// .ri-xxs { font-size: .5em; }
    RiXxs,
    /// .ri-xs { font-size: .75em; }
    RiXs,
    /// .ri-sm { font-size: .875em; }
    RiSm,
    /// .ri-1x { font-size: 1em; }
    Ri1x,
    /// .ri-2x { font-size: 2em; }
    Ri2x,
    /// .ri-3x { font-size: 3em; }
    Ri3x,
    /// .ri-4x { font-size: 4em; }
    Ri4x,
    /// .ri-5x { font-size: 5em; }
    Ri5x,
    /// .ri-6x { font-size: 6em; }
    Ri6x,
    /// .ri-7x { font-size: 7em; }
    Ri7x,
    /// .ri-8x { font-size: 8em; }
    Ri8x,
    /// .ri-9x { font-size: 9em; }
    Ri9x,
    /// .ri-10x { font-size: 10em; }
    Ri10x,
    #[default]
    /// .ri-fw { text-align: center; width: 1.25em; }
    RiFw,
}

impl IconSize {
  pub  fn from_str(size: &str) -> Option<Self> {
        match size {
            "xl" => Some(IconSize::RiXl),
            "xxs" => Some(IconSize::RiXxs),
            "xs" => Some(IconSize::RiXs),
            "sm" => Some(IconSize::RiSm),
            "1x" => Some(IconSize::Ri1x),
            "2x" => Some(IconSize::Ri2x),
            "3x" => Some(IconSize::Ri3x),
            "4x" => Some(IconSize::Ri4x),
            "5x" => Some(IconSize::Ri5x),
            "6x" => Some(IconSize::Ri6x),
            "7x" => Some(IconSize::Ri7x),
            "8x" => Some(IconSize::Ri8x),
            "9x" => Some(IconSize::Ri9x),
            "10x" => Some(IconSize::Ri10x),
            "fw" => Some(IconSize::RiFw),
            _ => None,
        }
    }

  pub  fn as_str(&self) -> &'static str {
        match self {
            IconSize::RiXl => "ri-xl",
            IconSize::RiXxs => "ri-xxs",
            IconSize::RiXs => "ri-xs",
            IconSize::RiSm => "ri-sm",
            IconSize::Ri1x => "ri-1x",
            IconSize::Ri2x => "ri-2x",
            IconSize::Ri3x => "ri-3x",
            IconSize::Ri4x => "ri-4x",
            IconSize::Ri5x => "ri-5x",
            IconSize::Ri6x => "ri-6x",
            IconSize::Ri7x => "ri-7x",
            IconSize::Ri8x => "ri-8x",
            IconSize::Ri9x => "ri-9x",
            IconSize::Ri10x => "ri-10x",
            IconSize::RiFw => "ri-fw",
        }
    }
}

