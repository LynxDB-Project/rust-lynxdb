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

use std::net::{SocketAddrV4, TcpStream};

pub struct Connection {
    tcp_stream: TcpStream,
}

impl Connection {
    fn new(tcp_stream: TcpStream) -> Connection {
        Connection {
            tcp_stream
        }
    }
}

pub fn create_connection(db_addr: SocketAddrV4) -> Connection {
    let result = TcpStream::connect(db_addr);
    let tcp_stream = result.unwrap();
    Connection::new(tcp_stream)
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use super::*;

    #[test]
    fn test_001() {
        let db_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7820);
        let connection = create_connection(db_addr);
    }
}
