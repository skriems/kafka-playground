use clap::{arg, ArgMatches, Command};

///
///
/// # Example
///
/// ```rust
/// let matches = process_command.get_matches_from(vec!["process", "-d" "backpacks"]).unwrap();
/// assert!(matches.contains_id("domain"));
/// ```
fn process_command() -> Command {
    Command::new("process")
        .about("process kafka messages")
        .arg(
            arg!(-d --domain <DOMAIN> "Limit processing to a specific domain (can be more than one!).\nIf not set, all are being processed.\n")
                .next_line_help(true)
                .env("ZEOU_DOMAINS")
                .value_delimiter(',')
                .value_parser(["articles", "backpacks", "circles", "events", "users"])
                .default_value("articles,backpacks,circles,events,users")
        )
        .arg(
            arg!(-b --brokers <BROKERS> "broker list in kafka format")
                .env("ZEOU_BROKER")
                .default_value("localhost:9092"),
        )
        .arg(
            arg!(-g --"group-id" <GROUP_ID> "consumer group id")
                .env("ZEOU_GROUP_ID")
                .default_value("async-std"))
        .arg(
            arg!(-l --"log-conf" <LOG_CONF> "configure the logging format (example: 'rdkafka=trace')")
        )
}

pub fn get_matches() -> ArgMatches {
    Command::new("zeou")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(process_command())
        .subcommand(
            Command::new("restore")
                .about("restore event stream to kafka")
                .arg_required_else_help(true)
                .arg(arg!(-d --domain <DOMAIN>).help("domain to restore")),
        )
        .subcommand(
            Command::new("backup")
                .about("backup event stream")
                .arg_required_else_help(true)
                .arg(arg!(-d --domain <DOMAIN>).help("domain to backup")),
        )
        .get_matches()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_command() {
        assert!(
            process_command()
            .get_matches_from(vec!["process", "-d", "backpacks"])
            .contains_id("domain")
        );

        assert_eq!(
            process_command()
                .get_matches_from(vec!["process", "-d", "backpacks"])
                .get_many::<String>("domain")
                .unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
            vec!["backpacks"]
        );

        assert_eq!(
            process_command()
                .get_matches_from(vec!["process", "-d", "backpacks", "-d", "articles"])
                .get_many::<String>("domain")
                .unwrap_or_default().map(|v| v.as_str()).collect::<Vec<_>>(),
            vec!["backpacks", "articles"]
        );
    }
}
