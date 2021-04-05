use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "waitfor")]
pub struct InputOptions {
    /// Retry the command every <delay> milliseconds. (default)
    #[structopt(short, long)]
    pub linear: bool,

    /// Retry the command with exponential backoff.
    #[structopt(short, long)]
    pub backoff: bool,

    /// Milliseconds between executions (linear mode) or base rate in exponential backoff.
    #[structopt(short, long, default_value = "2500")]
    pub delay: u64,

    /// Rate at which to increase delay during exponential backoff mode.
    #[structopt(short, long, default_value = "2.5")]
    pub rate: f32,

    /// Write a message to stderr each time a delay is occurring.
    #[structopt(short, long)]
    pub verbose: bool,

    pub command: Vec<String>,
}
