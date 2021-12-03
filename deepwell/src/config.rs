/*
 * config.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2021 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use clap::{App, Arg};
use std::net::SocketAddr;
use std::num::NonZeroU32;
use std::process;

#[derive(Debug, Clone)]
pub struct Config {
    /// Whether the logger should be enabled or not.
    pub logger: bool,

    /// The address the server will be hosted on.
    pub address: SocketAddr,

    /// The number of requests allowed per IP per minute.
    pub rate_limit_per_minute: NonZeroU32,

    /// The secret to bypass the rate-limit.
    /// An empty value means to disable bypassing.
    pub rate_limit_secret: String,
}

impl Config {
    pub fn load() -> Self {
        let matches = App::new("DEEPWELL")
            .arg(
                Arg::with_name("disable-log")
                    .short("q")
                    .long("quiet")
                    .long("disable-log")
                    .help("Disable logging output."),
            )
            .arg(
                Arg::with_name("host")
                    .short("h")
                    .long("host")
                    .long("hostname")
                    .takes_value(true)
                    .default_value("::")
                    .help("What host to listen on."),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .default_value("2747")
                    .help("What port to listen on."),
            )
            .arg(
                Arg::with_name("ratelimit-min")
                    .short("r")
                    .long("requests-per-minute")
                    .takes_value(true)
                    .default_value("20")
                    .help("How many requests are allowed per IP address per minute."),
            )
            .arg(
                Arg::with_name("ratelimit-secret")
                    .long("rate-limit-secret")
                    .takes_value(true)
                    .default_value("")
                    .help("A token which can be used by internal services to bypass the rate-limit."),
            )
            .get_matches();

        let logger = !matches.is_present("disable-log");

        let host_value = matches
            .value_of("host")
            .expect("No hostname in argument matches");
        let host = match host_value.parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid IP address: {}", host_value);
                process::exit(1);
            }
        };

        let port_value = matches
            .value_of("port")
            .expect("No port in argument matches");
        let port = match port_value.parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Invalid port number: {}", port_value);
                process::exit(1);
            }
        };

        let address = SocketAddr::new(host, port);

        let rate_limit_value = matches
            .value_of("ratelimit-min")
            .expect("No ratelimit per-minute in argument matches");
        let rate_limit_per_minute = match rate_limit_value.parse() {
            Ok(value) => value,
            Err(_) => {
                eprintln!(
                    "Invalid number of requests per minute: {}",
                    rate_limit_value
                );
                process::exit(1);
            }
        };

        let rate_limit_secret = matches
            .value_of("ratelimit-secret")
            .expect("No ratelimit secret in argument matches")
            .to_owned();

        Config {
            logger,
            address,
            rate_limit_per_minute,
            rate_limit_secret,
        }
    }
}
