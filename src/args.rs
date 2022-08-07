use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(value_parser)]
    pub name: String,

    #[clap(long, short, value_parser, env = "GITHUB_ACCESS_TOKEN")]
    pub token: String,
}
