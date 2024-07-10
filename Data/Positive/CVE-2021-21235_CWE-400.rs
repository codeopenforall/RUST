fn discard_exact(&mut self, mut len: usize) -> io::Result<()> {
        while len > 0 {
            let consume_len = match self.fill_buf() {
                Ok(buf) if buf.is_empty() =>
                    return Err(io::Error::new(
                        io::ErrorKind::UnexpectedEof, "unexpected EOF")),
                Ok(buf) => buf.len().min(len),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(e),
            };
            self.consume(consume_len);
            len -= consume_len;
        }
        Ok(())
    }