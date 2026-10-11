use std::fs::{self, File};
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

use crate::auth;
use crate::error::Result;
use crate::sys;

const LIMIT: u64 = 16 << 10;

// Pastebin API values (only the ones in use).
const PRIVATE_UNLISTED: &str = "1";
const EXPIRE_1W: &str = "1W";

/// Content to paste: the whole file, or its last `LIMIT` bytes starting at a
/// line boundary.
fn paste_code(file: &Path) -> io::Result<String> {
    let size = fs::metadata(file)?.len();
    if size <= LIMIT {
        return Ok(String::from_utf8_lossy(&fs::read(file)?).into_owned());
    }
    let mut f = File::open(file)?;
    f.seek(SeekFrom::Start(size - LIMIT))?;
    let mut buf = Vec::new();
    f.read_to_end(&mut buf)?;
    let text = String::from_utf8_lossy(&buf);
    let start = text.find('\n').map_or(0, |i| i + 1);
    Ok(text[start..].to_string())
}

/// Upload `file` as an unlisted paste and return its URL.
pub fn upload(file: &Path) -> Result<String> {
    let paste_code = paste_code(file)?;

    let user_key = ureq::post("https://pastebin.com/api/api_login.php")
        .send_form(&[
            ("api_dev_key", auth::PASTEBIN_DEV_KEY),
            ("api_user_name", auth::PASTEBIN_USER_NAME),
            ("api_user_password", auth::PASTEBIN_USER_PASSWORD),
        ])?
        .into_string()?;

    let name = file
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let folder: String = sys::hostname().chars().take(8).collect();

    let url = ureq::post("https://pastebin.com/api/api_post.php")
        .send_form(&[
            ("api_dev_key", auth::PASTEBIN_DEV_KEY),
            ("api_option", "paste"),
            ("api_paste_code", &paste_code),
            ("api_user_key", &user_key),
            ("api_paste_name", &name),
            ("api_paste_private", PRIVATE_UNLISTED),
            ("api_paste_expire_date", EXPIRE_1W),
            ("api_folder_key", &folder),
        ])?
        .into_string()?;
    Ok(url)
}
