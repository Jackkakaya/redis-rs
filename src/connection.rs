use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::client::Command;
use crate::error::Error;
use crate::Result;

pub struct Connection {
    pub conn: TcpStream,
    pub reader: BufReader<TcpStream>,
}

impl Connection {
    const SINGLE_STRINGS: u8 = b'+';
    const ERRORS: u8 = b'-';
    const INTEGERS: u8 = b':';
    const BULK_STRINGS: u8 = b'$';
    const ARRAYS: u8 = b'*';

    pub fn new(stream: TcpStream) -> Result<Connection> {
        let reader = BufReader::new(stream.try_clone()?);

        Ok(Connection { conn: stream, reader })
    }

    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Connection> {
        let stream = TcpStream::connect(addr)?;

        Self::new(stream)
    }

    pub fn execute(&mut self, cmd: Command) -> Result<Reply> {
        self.send(cmd)?;
        self.receive()
    }

    fn send(&mut self, cmd: Command) -> Result<()> {
        let send_data = cmd.to_vec();
        self.conn.write_all(&send_data)?;
        Ok(())
    }

    fn receive(&mut self) -> Result<Reply> {
        let mut buffer = Vec::new();
        self.reader.read_until(b'\n', &mut buffer)?;
        if buffer.is_empty() {
            return Err(Error::Resp);
        }
        let buffer = &buffer[0..buffer.len() - 2];

        let reply = match buffer[0] {
            Self::SINGLE_STRINGS => Reply::new(ReplyKind::SingleStrings, Vec::from(&buffer[1..])),
            Self::ERRORS => Reply::new(ReplyKind::Errors, Vec::from(&buffer[1..])),
            Self::INTEGERS => Reply::new(ReplyKind::Integers, Vec::from(&buffer[1..])),
            Self::BULK_STRINGS => Reply::new(
                ReplyKind::BulkStrings,
                self.read_bulk(String::from_utf8_lossy(&buffer[1..]).parse::<i64>()?)?,
            ),
            Self::ARRAYS => todo!(),

            _ => unreachable!(),
        };

        Ok(reply)
    }

    fn read_bulk(&mut self, size: i64) -> Result<Vec<u8>> {
        if size < 0 {
            // return -1 when 'GET' an not exist key, raw data is: "$-1"
            return Err(Error::KeyNotFound);
        }

        let mut buf = vec![0; size as usize];
        self.reader.read_exact(&mut buf)?;
        Ok(buf)
    }
}

#[derive(Debug)]
pub enum ReplyKind {
    SingleStrings,
    Errors,
    Integers,
    BulkStrings,
    Arrays,
}

#[derive(Debug)]
pub struct Reply {
    pub kind: ReplyKind,
    pub data: Vec<u8>,
}

impl Reply {
    pub fn new(kind: ReplyKind, data: Vec<u8>) -> Self {
        Reply { kind, data }
    }
}
