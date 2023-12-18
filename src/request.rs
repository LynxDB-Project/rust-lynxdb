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
use std::io::{Result, Write};
use std::net::TcpStream;

pub struct Request<'a> {
    blocks: VecDeque<Node<'a>>,
    has_len: bool,
    len: u32,
}

impl<'a> Request<'a> {
    pub fn new(has_len: bool) -> Request<'a> {
        Request {
            blocks: VecDeque::new(),
            has_len,
            len: 0,
        }
    }

    fn append(&mut self, has_len: bool, data: &'a [u8]) {
        self.blocks.push_back(Node::new(has_len, data));
    }

    pub fn append_var_str(&mut self, s: &'a str) {
        let data = s.as_bytes();
        self.append(true, data);
    }

    pub fn write(&self, tcp_stream: &mut TcpStream) -> Result<u32> {
        let mut write_len: u32 = 0;

        let len_bytes = self.len.to_be_bytes();

        let result = tcp_stream.write(&len_bytes);
        match result {
            Ok(len) => { write_len += len as u32 }
            Err(e) => return Err(e)
        }

        for node in &self.blocks {
            if node.has_len {
                let len_bytes = (node.bytes.len() as u32).to_be_bytes();
                let result = tcp_stream.write(&len_bytes);

                match result {
                    Ok(len) => { write_len += len as u32 }
                    Err(e) => return Err(e)
                }
            }

            let result = tcp_stream.write(node.bytes);
            match result {
                Ok(len) => { write_len += len as u32 }
                Err(e) => return Err(e)
            }
        }

        Ok(write_len)
    }
}

struct Node<'a> {
    has_len: bool,
    bytes: &'a [u8],
}

impl<'a> Node<'a> {
    fn new(has_len: bool, bytes: &[u8]) -> Node {
        Node {
            has_len,
            bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::request::Request;

    #[test]
    fn test_001() {
        let request = Request::new(true);
    }
}

