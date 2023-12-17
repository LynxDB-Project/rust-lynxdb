/*
 * Copyright 2023 Baili Zhang.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::VecDeque;
use std::net::TcpStream;

pub struct DataBlocks<'a> {
    blocks: VecDeque<Node<'a>>,
}

impl<'a> DataBlocks<'a> {
    pub fn new() -> DataBlocks<'a> {
        DataBlocks {
            blocks: VecDeque::new(),
        }
    }

    pub fn append_var_str(&self, s: &str) {}

    pub fn write(&self, tcp_stream: &TcpStream) {}
}

struct Node<'a> {
    data: &'a [u8],
}

impl<'a> Node<'a> {
    pub fn data(&self) -> &[u8] {
        self.data
    }
}

