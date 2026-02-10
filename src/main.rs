/*
 * This file is part of git-ai.
 *
 * Copyright (c) 2026 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.

 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

pub mod gacli;
pub mod gagit;
pub mod gaollama;

use clap::Parser;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = gacli::GACli::parse();
    match args.cmd.as_str() {
        "gen-commit-msg" => {
            info!("{}", gagit::GAGit::read_staged().unwrap());
            let response = gagit::GAGit::read_staged();
            let ollama = gaollama::GAOllama::query_gen_commit_msg(response.unwrap()).await;
            info!("{}", ollama.unwrap());
        }
        _ => {}
    }
}
