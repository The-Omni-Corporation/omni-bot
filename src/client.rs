//======---------------------------------------------------------------======//
//                                                                           //
// Copyright (c) 2023 The Omni Authors. All rights reserved.                 //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-2-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::commands::GENERAL_GROUP;
use crate::config::Config;
use crate::handler::Handler;
use serenity::client::Client;
use serenity::framework::StandardFramework;
use serenity::prelude::GatewayIntents;

pub async fn build_client(config: &Config) -> Client {
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    Client::builder(&config.user.token, intents)
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix("!"))
                .group(&GENERAL_GROUP),
        )
        .await
        .expect("error creating client")
}
