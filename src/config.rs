//======---------------------------------------------------------------======//
//                                                                           //
// Copyright 2022 Evan Cox <evanacox00@gmail.com>. All rights reserved.      //
//                                                                           //
// Use of this source code is governed by a BSD-style license that can be    //
// found in the LICENSE.txt file at the root of this project, or at the      //
// following link: https://opensource.org/licenses/BSD-3-Clause              //
//                                                                           //
//======---------------------------------------------------------------======//

use serde::Deserialize;
use serenity::prelude::TypeMapKey;
use std::env;
use std::fs;
use toml;
use tracing::metadata::Level;

#[derive(Copy, Clone, Eq, PartialEq, Deserialize)]
pub enum LogLevel {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

impl Into<Level> for LogLevel {
    fn into(self) -> Level {
        match self {
            Self::Trace => Level::TRACE,
            Self::Debug => Level::DEBUG,
            Self::Info => Level::INFO,
            Self::Warn => Level::WARN,
            Self::Error => Level::ERROR,
        }
    }
}

#[derive(Deserialize)]
pub struct User {
    pub token: String,
}

#[derive(Deserialize)]
pub struct Options {
    pub log_level: LogLevel,
    pub owners: Vec<u64>,
}

#[derive(Deserialize)]
pub struct Config {
    pub user: User,
    pub options: Options,
}

impl TypeMapKey for Config {
    type Value = Options;
}

pub fn read_config() -> Config {
    let cwd = env::current_dir().expect("can't access current working directory");
    let file = fs::read_to_string(cwd.join("config.toml")).expect("can't read 'config.toml'");

    toml::from_str(&file).expect("unable to parse 'config.toml', check that format is correct")
}
