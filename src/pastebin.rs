use std::path::Path;

use crate::auth;
use crate::error::Result;
use crate::logs;
use crate::sys;

const LIMIT: u64 = 16 << 10;

// Pastebin API values (only the ones in use).
const PRIVATE_UNLISTED: &str = "1";
const EXPIRE_1W: &str = "1W";

/// Upload `file` as an unlisted paste and return its URL.
pub fn upload(file: &Path) -> Result<String> {
    let paste_code = logs::tail(file, LIMIT)?;

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
