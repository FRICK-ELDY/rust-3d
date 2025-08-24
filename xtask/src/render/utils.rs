//! summary: 文字列ユーティリティ
//! path: xtask/src/render/utils.rs

pub fn to_unix_sep(p: &str) -> String {
    p.replace('\\', "/")
}

pub fn url_encode_path(rel_unix: &str) -> String {
    use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

    // "/" を含む GitHub パス用の安全集合
    const PATH_SAFE: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'/')
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');

    utf8_percent_encode(rel_unix, PATH_SAFE).to_string()
}

pub fn sanitize_md_cell(s: &str) -> String {
    s.replace('|', "\\|").trim().to_string()
}
