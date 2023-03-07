//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use crate::config::Config;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;
use tracing::log::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let options = data.get::<Config>().unwrap();
    let id = ctx.shard_id;
    let response = format!("Pong! Shard #{id} active. :ping_pong:");

    if options.owners.contains(&msg.author.id.0) {
        info!("user is owner: {:?}", msg.author);
    }

    if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
        warn!("Error sending message: {:?}", why);

        Err(CommandError::from(why))
    } else {
        Ok(())
    }
}

#[group]
#[commands(ping)]
pub struct General;
