// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public
// License, version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

use std::{env, path::PathBuf};
use structopt::StructOpt;
use tokio::fs;
use webmention::Client;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();

    let opts = CLI::from_args();
    let res = match opts.cmd {
        Cmd::Send { source, target } => {
            let mut client = Client::builder().source(source.as_str());

            for t in target {
                client = client.target(t.as_str());
            }

            client.build()?.send().await?
        }
        Cmd::Query { source, output } => {
            let res = Client::builder()
                .source(source.as_str())
                .build()?
                .send()
                .await?;

            if let Some(dest) = output {
                let root = env::current_dir()?.join(dest);
                fs::create_dir_all(&root).await?;
                let dest = root.join("links.json");
                //let mut file = fs::File::create(dest).await?;
                //file.write_all(res.to_bytes()).await?;
                log::debug!("Wrote output to {:?}", &dest);
            }

            res
        }
    };

    if !opts.silent {
        println!("{:#?}", res);
    }

    Ok(())
}

/// CLI tool to send and retrieve Webmentions
#[derive(StructOpt)]
struct CLI {
    #[structopt(long, short = "s")]
    silent: bool,
    #[structopt(subcommand)]
    cmd: Cmd,
}

#[derive(StructOpt)]
enum Cmd {
    /// Send a Webmention to one (or many) targets
    Send {
        /// URL indicating the Webmention source
        source: String,
        /// URL indicating the Webmention target
        target: Vec<String>,
    },
    /// Query for Webmentions of a specific URL
    Query {
        /// Source URL for endpoint discovery
        source: String,
        /// Optionally specify a folder to save Webmention data
        output: Option<PathBuf>,
    },
}
