/*
* This file is part of git-think.
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
use serde::Deserialize;
use serde_json::json;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_util::io::StreamReader;
use futures::TryStreamExt;

#[derive(Deserialize)]
struct Chunk {
    response: String,
}

pub struct GAOllama {
    pub model: String,
    pub query: String,
}

impl GAOllama {
   pub async fn query_gen_commit_msg(llm: &str, patch: &str) -> Option<String> {
      let prompt = format!(r#"
You are a tool that generates Git commit messages.
Output ONLY the commit message text.
Do not include explanations, confirmations, or any additional text:
{patch}"#);

      let client = Client::new();
      let payload = json!({
         "model": llm,
         "prompt": prompt,
         "options": {
            "temperature": 0,
            "num_thread": 12
         }
      });

      let resp = client
         .post("http://localhost:11434/api/generate")
         .json(&payload)
         .send()
         .await
         .ok()?;

      let stream = resp.bytes_stream();
      let reader = StreamReader::new(
         stream.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e)),
      );

      let mut lines = BufReader::new(reader).lines();
      let mut out = String::new();
      while let Ok(Some(line)) = lines.next_line().await {
         if let Ok(chunk) = serde_json::from_str::<Chunk>(&line) {
            out.push_str(&chunk.response);
         }
      }

      Some(out)
   }
}
