use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

pub struct Conn {
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
}

impl Conn {
    pub fn from(stream: TcpStream) -> Self {
        let stream_clone = stream.try_clone().expect("clone failed...");
        let reader = BufReader::new(stream);
        let writer = BufWriter::new(stream_clone);
        Conn { reader, writer }
    }

    pub fn read_line(&mut self, buf: &mut String) {
        self.reader.read_line(buf).unwrap();
    }
    pub fn read_exact(&mut self, buf: &mut [u8]) {
        self.reader.read_exact(buf).unwrap();
    }
    pub fn write_all(&mut self, buf: &[u8]) {
        self.writer.write_all(buf).unwrap();
    }
    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }
}
