use clap::ValueEnum;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum DateStyle {
    /// yyyymmdd
    Plain,
    /// yyyy.mm.dd
    Dot,
    /// yyyy/mm/dd
    Slash,
    /// yyyy:mm:dd
    Colon,
}
