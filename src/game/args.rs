use clap::Parser;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    #[clap(short, long, name = "width", value_parser, default_value_t = 8)]
    pub width: usize,
    #[clap(short = 'H', long, name = "height", value_parser, default_value_t = 8)]
    pub height: usize,
    #[clap(short, long, name = "mines", value_parser, default_value_t = 10)]
    pub mines: u8,
    #[clap(short, long, name = "duds", value_parser, default_value_t = 0)]
    pub duds: u8,
}
