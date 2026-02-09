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

use git2::{DiffOptions, Repository};
use log::{info, warn};

pub struct GAGit {

}

impl GAGit {
   pub fn read_staged() -> Option<String> {
      let repo = Repository::discover(".");
      match &repo {
         Err(e) => {
            warn!("Could not find repo: {:?}", e.message());
            return None
         },
         Ok(v) => {
            info!("Repo found");
         }
      };

      let repo = repo.unwrap();
      let index = match repo.index() {
         Err(e) => {
            warn!("Could not find index: {:?}", e);
            return None;
         },
         Ok(v) => {
            info!("Index found");
            v
         }
      };

      let head_tree = repo
        .head()
        .ok()
        .and_then(|h| h.peel_to_tree().ok());

      let mut diff_opts = DiffOptions::new();
      let diff = repo.diff_tree_to_index(
        head_tree.as_ref(),
        Some(&index),
        Some(&mut diff_opts),
      );
      let diff = match diff {
         Err(e) => {
            warn!("Failed to build diff: {:?}", e);
            return None;
         },
         Ok(v) => v
      };
      
      let mut ret = String::new();
      let _ = diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
         info!("{}", std::str::from_utf8(line.content()).unwrap());
         match std::str::from_utf8(line.content()) {
            Err(_) => {},
            Ok(v) => ret += v
         };
         true
      });

      return Some(ret)
   }
}
