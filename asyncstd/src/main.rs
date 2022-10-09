use lib::utils::setup_logger;

mod cli;
mod commands;

/// https://github.com/fede1024/rust-rdkafka/blob/master/examples/runtime_async_std.rs

#[async_std::main]
async fn main() {
    let matches = cli::get_matches();

    match matches.subcommand() {
        Some(("process", sub_matches)) => {
            setup_logger(true, sub_matches.get_one::<String>("log-conf"));
            commands::process(sub_matches).await;
        }
        _ => {
            unimplemented!();
        }
    }
}
