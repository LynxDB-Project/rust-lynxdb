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
use std::sync::atomic::{AtomicU32, Ordering};

pub const __METHOD__FIND_BY_KEY_CF_COLUMN: u8 = 1;
pub const __METHOD__FIND_MULTI_COLUMNS: u8 = 2;
pub const __METHOD__INSERT: u8 = 3;
pub const __METHOD__DELETE: u8 = 6;
// TODO...

const __FLAG__CLIENT_REQUEST: u8 = 1;
const SERIAL: AtomicU32 = AtomicU32::new(0);

pub struct Request<'a> {
    method: u8,
    blocks: VecDeque<Node<'a>>,
    len: u32,
}

impl<'a> Request<'a> {
    pub fn new(method: u8) -> Request<'a> {
        Request {
            method,
            blocks: VecDeque::new(),
            len: 5, // serial(4 bytes), client request flag(1 bytes)
        }
    }

    fn append(&mut self, has_len: bool, data: &'a [u8]) {
        if has_len {
            self.len += 4;
        }

        self.len += data.len() as u32;
        self.blocks.push_back(Node::new(has_len, data));
    }

    pub fn append_var_str(&mut self, s: &'a str) {
        let data = s.as_bytes();
        self.append(true, data);
    }

    pub fn write(&self, tcp_stream: &mut TcpStream) -> Result<u32> {
        let mut write_len: u32 = 0;

        write_len += tcp_stream.write(&self.len.to_be_bytes())? as u32;
        write_len += tcp_stream.write(&__FLAG__CLIENT_REQUEST.to_be_bytes())? as u32;
        write_len += tcp_stream.write(&SERIAL.fetch_add(1, Ordering::SeqCst).to_be_bytes())? as u32;
        write_len += tcp_stream.write(&self.method.to_be_bytes())? as u32;

        // write data
        for node in &self.blocks {
            if node.has_len {
                let bytes = (node.bytes.len() as u32).to_be_bytes();
                write_len += tcp_stream.write(&bytes)? as u32;
            }
            write_len += tcp_stream.write(node.bytes)? as u32;
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
    use crate::request::{__METHOD__DELETE, Request};

    #[test]
    fn test_001() {
        let mut request = Request::new(__METHOD__DELETE);
        request.append_var_str("test");
        assert_eq!(request.len, 13);
    }
}

