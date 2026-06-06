use crate::error::{Error, Result};

/// Reads a complete SCPI response from accumulated bytes using IEEE 488.2 block framing.
pub fn extract_response(buffer: &[u8], terminator: &str) -> Result<(Vec<u8>, usize)> {
    if buffer.is_empty() {
        return Err(Error::Timeout);
    }

    if buffer[0] == b'#' {
        return read_block_response(buffer);
    }

    read_terminator_response(buffer, terminator)
}

fn read_block_response(buffer: &[u8]) -> Result<(Vec<u8>, usize)> {
    if buffer.len() < 2 {
        return Err(Error::Timeout);
    }

    let digit_count = (buffer[1] as char)
        .to_digit(10)
        .ok_or_else(|| Error::Parse("invalid block digit count".into()))?
        as usize;

    if digit_count == 0 {
        return read_indefinite_block(buffer);
    }

    let len_start = 2;
    let len_end = len_start + digit_count;
    if buffer.len() < len_end {
        return Err(Error::Timeout);
    }

    let len_str = std::str::from_utf8(&buffer[len_start..len_end])
        .map_err(|e| Error::Parse(e.to_string()))?;
    let data_len: usize = len_str
        .parse()
        .map_err(|_| Error::Parse("invalid block length".into()))?;

    let data_start = len_end;
    let data_end = data_start + data_len;
    if buffer.len() < data_end {
        return Err(Error::Timeout);
    }

    let payload = buffer[data_start..data_end].to_vec();
    Ok((payload, data_end))
}

fn read_indefinite_block(buffer: &[u8]) -> Result<(Vec<u8>, usize)> {
    // #0<data>\n
    let start = 2;
    if let Some(pos) = buffer[start..].iter().position(|&b| b == b'\n') {
        let end = start + pos + 1;
        let payload = buffer[start..start + pos].to_vec();
        return Ok((payload, end));
    }
    Err(Error::Timeout)
}

fn read_terminator_response(buffer: &[u8], terminator: &str) -> Result<(Vec<u8>, usize)> {
    let term_bytes = terminator.as_bytes();
    if term_bytes.is_empty() {
        return Ok((buffer.to_vec(), buffer.len()));
    }

    if let Some(pos) = find_subslice(buffer, term_bytes) {
        let payload = buffer[..pos].to_vec();
        let consumed = pos + term_bytes.len();
        return Ok((payload, consumed));
    }

    if buffer.contains(&b'\n') {
        let pos = buffer.iter().position(|&b| b == b'\n').unwrap();
        return Ok((buffer[..pos].to_vec(), pos + 1));
    }

    Err(Error::Timeout)
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_definite_block_with_embedded_newline() {
        // #1 + length "4" + 4-byte payload containing newline
        let data = b"#14\nemb\n";
        let (payload, consumed) = extract_response(data, "\n").unwrap();
        assert_eq!(payload, b"\nemb");
        assert_eq!(consumed, 7);
    }

    #[test]
    fn reads_ascii_terminated() {
        let data = b"3.300\n";
        let (payload, consumed) = extract_response(data, "\n").unwrap();
        assert_eq!(payload, b"3.300");
        assert_eq!(consumed, 6);
    }

    #[test]
    fn reads_indefinite_block() {
        let data = b"#0hello\n";
        let (payload, consumed) = extract_response(data, "\n").unwrap();
        assert_eq!(payload, b"hello");
        assert_eq!(consumed, data.len());
    }
}
