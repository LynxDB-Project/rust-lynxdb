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

use std::io::{Read, Result, Write};
use std::net::{Shutdown, SocketAddrV4, TcpStream};

use crate::request::{
    __METHOD__DELETE, __METHOD__FIND_BY_KEY_CF_COLUMN, __METHOD__FIND_MULTI_COLUMNS, __METHOD__INSERT,
    Request,
};
use crate::response::Response;

pub struct Connection {
    tcp_stream: TcpStream,
}

impl Connection {
    fn new(tcp_stream: TcpStream) -> Connection {
        Connection { tcp_stream }
    }

    pub fn find(&mut self, key: &str, column_family: &str, column: &str) -> Result<&str> {
        let mut request = Request::new(__METHOD__FIND_BY_KEY_CF_COLUMN);
        request.append_var_str(key);
        request.append_var_str(column_family);
        request.append_var_str(column);

        request.write(&mut self.tcp_stream)?;

        let response = Response::read(&mut self.tcp_stream)?;

        let value = response.to_str();
        return Ok(value);
    }

    pub fn find_multi_columns(
        &mut self,
        key: &str,
        column_family: &str,
        columns: Vec<&str>,
    ) -> Result<()> {
        let mut request = Request::new(__METHOD__FIND_MULTI_COLUMNS);
        request.append_var_str(key);
        request.append_var_str(column_family);

        for column in columns {
            request.append_var_str(column);
        }

        request.write(&mut self.tcp_stream)?;

        let response = Response::read(&mut self.tcp_stream)?;

        return Ok(());
    }

    pub fn insert(
        &mut self,
        column_family: &str,
        column: &str,
        key_value_pairs: Vec<(&[u8], &[u8], u64)>,
    ) -> Result<()> {
        let mut request = Request::new(__METHOD__INSERT);
        request.append_var_str(column_family);
        request.append_var_str(column);

        for pair in key_value_pairs {
            request.append_var_arr_u8(pair.0);
            request.append_var_arr_u8(pair.1);
            request.append_raw_u64(pair.2);
        }

        request.write(&mut self.tcp_stream)?;

        let response = Response::read(&mut self.tcp_stream)?;

        // TODO: check response

        return Ok(());
    }

    pub fn delete(&mut self, key: &str, column_family: &str, column: &str) -> Result<()> {
        let mut request = Request::new(__METHOD__DELETE);
        request.append_var_str(key);
        request.append_var_str(column_family);
        request.append_var_str(column);

        request.write(&mut self.tcp_stream)?;

        let response = Response::read(&mut self.tcp_stream);

        // TODO: check response

        return Ok(());
    }

    pub fn close(&mut self) {
        self.tcp_stream.shutdown(Shutdown::Both).unwrap();
    }
}

pub fn connect(db_addr: SocketAddrV4) -> Result<Connection> {
    let tcp_stream = TcpStream::connect(db_addr)?;
    return Ok(Connection::new(tcp_stream));
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::net::{Ipv4Addr, SocketAddrV4};
    use std::thread;

    use super::*;

    fn init_connection() -> Connection {
        let host = Ipv4Addr::new(127, 0, 0, 1);
        let db_addr = SocketAddrV4::new(host, 7820);
        connect(db_addr).unwrap()
    }

    #[test]
    fn test_001() {
        let mut connection = init_connection();

        let pairs = Vec::from([("key".as_bytes(), "value".as_bytes(), 32)]);

        connection.insert("column_family", "column", pairs).unwrap();

        let time = time::Duration::from_secs(20);
        thread::sleep(time);
    }
}
