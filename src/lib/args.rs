use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
        /// Show icons next to file names (--icons)
        #[arg(long, default_value_t = false)]
        pub icons: bool,
    /// Directory to list (default: current directory)
    #[arg(default_value = ".")]
    pub path: String,

    /// Use long format (-l)
    #[arg(short, long, default_value_t = false)]
    pub long: bool,
    /// Human-readable sizes (-h)
    /// Human-readable sizes (-h)

    /// Human-readable sizes (-h)
    #[arg(short = 'H', long, default_value_t = false)]
    pub human: bool,

    /// Show hidden files (-a)
    #[arg(short, long, default_value_t = false)]
    pub all: bool,

    /// Sort by: name, size, time
    #[arg(long, default_value = "name")]
    pub sort: String,

    /// Reverse sort order (-r)
    #[arg(short, long, default_value_t = false)]
    pub reverse: bool,

    /// Recursive listing (-R)
    #[arg(short = 'R', long, default_value_t = false)]
    pub recursive: bool,

}
