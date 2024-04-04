/*
 * Copyright 2023-2024 Baili Zhang.
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

use std::io::Read;
use std::net::TcpStream;

pub struct Response<'a> {
    method: &'a mut [u8; 1],
    data: Vec<u8>,
}

impl Response {
    pub fn new() -> Response {
        Response { method: &mut [0; 1], data: Vec::new() }
    }

    pub fn read(&self, tcp_stream: &mut TcpStream) {
        tcp_stream.read_exact(self.method)?;
    }

    pub fn to_str(&self) -> &str {
        // TODO
        let s = "";
        s
    }
}
