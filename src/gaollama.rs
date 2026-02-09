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

use reqwest::Client;
use serde_json::json;

pub struct GAOllama {
   pub model: String,
   pub query: String
}

impl GAOllama {
   pub async fn query() -> Option<String> {
      let client = Client::new();

      let payload = json!({
         "model": "qwen3-coder-next",
         "prompt": "What is your name?"
      });

      let response = client
         .post("http://localhost:11434/api/generate")
         .json(&payload)
         .send()
         .await
         .ok()?
         .text()
         .await
         .ok()?;

      Some(response)
   }
}
