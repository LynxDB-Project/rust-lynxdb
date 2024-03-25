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

use std::io::Result;
use std::net::TcpStream;

pub struct Response {
    method: u8,
}

impl Response {
    pub fn read(tcp_stream: &mut TcpStream) -> Result<Response> {
        Ok(Response { method: 0 })
    }

    pub fn to_str(&self) -> &str {
        // TODO
        let s = "";
        s
    }
}
