//! Print one RFC 4122 UUID v4 to stdout.

use std::io::{self, Write};
use std::process::ExitCode;

const LINE_LEN: usize = 39;

fn main() -> ExitCode {
    let Some(line) = generate_line() else {
        return ExitCode::from(1);
    };
    let mut stdout = io::stdout();
    if stdout.write_all(&line).is_err() || stdout.flush().is_err() {
        return ExitCode::from(1);
    }
    ExitCode::SUCCESS
}

fn generate_line() -> Option<[u8; LINE_LEN]> {
    let mut bytes = [0u8; 16];
    if !fill_random(&mut bytes) {
        return None;
    }
    apply_rfc4122_v4(&mut bytes);
    let mut line = [0u8; LINE_LEN];
    format_uuid(&bytes, &mut line);
    Some(line)
}

fn apply_rfc4122_v4(bytes: &mut [u8; 16]) {
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
}

fn format_uuid(bytes: &[u8; 16], out: &mut [u8; LINE_LEN]) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out[0] = b'{';
    let mut cursor = 1;
    for (index, byte) in bytes.iter().copied().enumerate() {
        if matches!(index, 4 | 6 | 8 | 10) {
            out[cursor] = b'-';
            cursor += 1;
        }
        out[cursor] = HEX[(byte >> 4) as usize];
        out[cursor + 1] = HEX[(byte & 0x0f) as usize];
        cursor += 2;
    }
    out[cursor] = b'}';
    out[cursor + 1] = b'\n';
}

#[cfg(windows)]
fn fill_random(buf: &mut [u8]) -> bool {
    const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x0000_0002;
    let status = unsafe {
        BCryptGenRandom(
            core::ptr::null_mut(),
            buf.as_mut_ptr(),
            buf.len() as u32,
            BCRYPT_USE_SYSTEM_PREFERRED_RNG,
        )
    };
    status == 0
}

#[cfg(windows)]
#[link(name = "bcrypt")]
unsafe extern "system" {
    fn BCryptGenRandom(
        h_algorithm: *mut core::ffi::c_void,
        pb_buffer: *mut u8,
        cb_buffer: u32,
        dw_flags: u32,
    ) -> i32;
}

#[cfg(not(windows))]
fn fill_random(buf: &mut [u8]) -> bool {
    use std::fs::File;
    use std::io::Read;

    File::open("/dev/urandom")
        .and_then(|mut file| file.read_exact(buf))
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::{apply_rfc4122_v4, format_uuid, generate_line, LINE_LEN};

    #[test]
    fn formats_nil_with_version_and_variant() {
        let mut bytes = [0u8; 16];
        apply_rfc4122_v4(&mut bytes);
        let mut line = [0u8; LINE_LEN];
        format_uuid(&bytes, &mut line);
        assert_eq!(&line, b"{00000000-0000-4000-8000-000000000000}\n");
    }

    #[test]
    fn sets_version_and_variant_bits() {
        let mut bytes = [0xff; 16];
        apply_rfc4122_v4(&mut bytes);
        assert_eq!(bytes[6] & 0xf0, 0x40);
        assert_eq!(bytes[8] & 0xc0, 0x80);
    }

    #[test]
    fn generated_line_matches_uuid_v4() {
        let line = generate_line().expect("random source");
        let text = std::str::from_utf8(&line).unwrap();
        assert!(text.ends_with('\n'));
        let body = &text[..text.len() - 1];
        assert_eq!(body.len(), 38);
        assert!(body.starts_with('{') && body.ends_with('}'));
        let inner = body[1..37].as_bytes();
        for (index, byte) in inner.iter().copied().enumerate() {
            if matches!(index, 8 | 13 | 18 | 23) {
                assert_eq!(byte, b'-');
            } else {
                assert!(byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase());
            }
        }
        assert_eq!(inner[14], b'4');
        assert!(matches!(inner[19], b'8' | b'9' | b'a' | b'b'));
    }
}
