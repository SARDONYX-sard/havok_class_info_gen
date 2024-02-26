use std::io::{Read, Write};

/// Read a fixed size buffer from the reader and returns it as a [`String`].
pub(super) fn read_fix_str(mut br: impl Read, len: usize) -> std::io::Result<String> {
    let mut buf = vec![0; len];
    br.read_exact(&mut buf)?;
    let null_index = buf.iter().position(|&x| x == 0).unwrap_or(len);
    let string = String::from_utf8_lossy(&buf[..null_index]);
    Ok(string.into_owned())
}

/// Write from a fixed size buffer to the writer.
pub(super) fn write_fix_str(mut bw: impl Write, string: &str, len: usize) -> std::io::Result<()> {
    // Speed up the process by avoiding unnecessary heap allocation.
    if string.len() == len {
        bw.write_all(string.as_bytes())?;
    } else {
        let mut buf = string.as_bytes().to_vec();
        buf.resize(len, 0);
        bw.write_all(&buf)?;
    }

    Ok(())
}
