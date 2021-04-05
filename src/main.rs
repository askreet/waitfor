use structopt::StructOpt;
use std::process::{exit, Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use num_format::{SystemLocale, ToFormattedString};

#[derive(StructOpt, Debug)]
#[structopt(name = "waitfor")]
struct InputOptions {
    /// Retry the command every <delay> milliseconds. (default)
    #[structopt(short, long)]
    linear: bool,

    /// Retry the command with exponential backoff.
    #[structopt(short, long)]
    backoff: bool,

    /// Milliseconds between executions (linear mode) or base rate in exponential backoff.
    #[structopt(short, long, default_value = "2500")]
    delay: u64,

    /// Rate at which to increase delay during exponential backoff mode.
    #[structopt(short, long, default_value = "2.5")]
    rate: f32,

    /// Write a message to stderr each time a delay is occurring.
    #[structopt(short, long)]
    verbose: bool,

    command: Vec<String>,
}

trait Delay {
    fn next_delay(&self) -> u64;
    fn wait(&mut self) -> ();
}

struct LinearWaiter {
    delay_ms: u64
}

impl Delay for LinearWaiter {
    fn next_delay(&self) -> u64 {
        self.delay_ms
    }

    fn wait(&mut self) -> () {
        sleep(Duration::from_millis(self.delay_ms))
    }
}

struct BackoffWaiter {
    next_delay_ms: u64,
    rate: f32,
}

impl Delay for BackoffWaiter {
    fn next_delay(&self) -> u64 {
        self.next_delay_ms
    }

    fn wait(&mut self) -> () {
        sleep(Duration::from_millis(self.next_delay_ms));
        self.next_delay_ms = (self.next_delay_ms as f32 * self.rate).floor() as u64;
    }
}

fn main() {
    let opt: InputOptions = InputOptions::from_args();

    if opt.linear && opt.backoff {
        println!("Linear and backoff are mutually exclusive.");
        exit(1);
    }

    let mut waiter = create_waiter(&opt);
    let mut command = create_command(&opt);

    let locale = SystemLocale::default().expect("waitfor: failed to determine system locale.");

    loop {
        let mut child = command.spawn()
            .expect("waitfor: failed to execute command");

        let result = child.wait()
            .expect("waitfor: error while waiting for command to complete");

        if result.success() {
            break;
        }

        if opt.verbose {
            eprintln!("waitfor: waiting {}ms to retry", waiter.next_delay().to_formatted_string(&locale));
        }
        waiter.wait();
    }
}

fn create_waiter(opt: &InputOptions) -> Box<dyn Delay> {
    if opt.backoff {
        Box::new(BackoffWaiter { next_delay_ms: opt.delay, rate: opt.rate })
    } else {
        Box::new(LinearWaiter { delay_ms: opt.delay })
    }
}

fn create_command(opt: &InputOptions) -> Command {
    let program = opt.command.first().expect("waitfor: no command specified").clone();
    let args = &opt.command[1..];
    let mut command = Command::new(program);

    command.args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    command
}
