mod command;
mod waiter;
mod cli;

use structopt::StructOpt;
use std::process::{exit};
use num_format::{SystemLocale, ToFormattedString};
use cli::InputOptions;

fn main() {
    let opt: InputOptions = InputOptions::from_args();

    if opt.linear && opt.backoff {
        abort("must use linear or backoff mode, not both");
    }

    let mut waiter = waiter::create_waiter(&opt);
    let mut command = command::create_command(&opt)
        .unwrap_or_else(|e| abort(e));

    let locale = SystemLocale::default()
        .unwrap_or_else(|_| abort("failed to determine system locale"));

    loop {
        let result = command::run_command(&mut command);

        match result {
            Ok(exit_status) => {
                if exit_status.success() {
                    break;
                }
            }
            Err(error) => abort(error.to_string().as_str())
        }

        if opt.verbose {
            eprintln!("waitfor: waiting {}ms to retry", waiter.next_delay().to_formatted_string(&locale));
        }
        waiter.wait();
    }
}

fn abort(error: &str) -> ! {
    eprintln!("waitfor: error: {}", error);
    exit(1);
}
