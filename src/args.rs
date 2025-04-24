use crate::tournament::format;
use std::path::PathBuf;

#[derive(clap::Parser, Clone, Debug, PartialEq, Eq)]
#[command(version, about, long_about)]
pub struct Args {
    /// Path to file with participants: '<player/team>,<seed>' syntax, where <seed> is an optional u16
    pub file: PathBuf,
    /// Format in which the Tournament shall be carried out
    #[arg(short, long, value_enum, default_value_t = format::Supported::DoubleElimination)]
    pub format: format::Supported,
    /// When to seed players, or if seed isn't provided: shuffle.
    /// NOTE: ignored if <FORMAT> is not elimination type
    #[arg(short, long, value_enum, default_value_t = WhenSeed::Initially)]
    pub seed: WhenSeed,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum WhenSeed {
    Always,
    Initially,
    Never,
}
impl WhenSeed {
    pub fn always(self) -> bool {
        self == Self::Always
    }
    pub fn initially(self) -> bool {
        self == Self::Initially
    }
    pub fn never(self) -> bool {
        self == Self::Never
    }
}
