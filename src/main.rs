//======---------------------------------------------------------------======//
//                                                                           //
// Copyright (c) 2023 The Omni Authors. All rights reserved.                 //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-2-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

mod client;
mod commands;
mod config;
mod handler;

use crate::client::build_client;
use crate::config::{read_config, Config};
use tracing::instrument;
use tracing::log::*;
use tracing_subscriber::filter::LevelFilter;

#[tokio::main]
#[instrument]
async fn main() {
    let config = read_config();

    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::from_level(config.options.log_level.into()))
        .init();

    let mut client = build_client(&config).await;

    client.data.write().await.insert::<Config>(config.options);

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
