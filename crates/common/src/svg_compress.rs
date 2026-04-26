//! Lightweight SVG dictionary compression for database storage.
//!
//! Prefix `"s1:"` marks a dictionary-compressed SVG. Raw `"<svg..."` passes through unchanged.

const SVG_DICT: &[(&str, &str)] = &[
    // Ordered longest-first to avoid partial-match collisions during decompression
    ("~X", "xmlns=\"http://www.w3.org/2000/svg\""),
    ("~D", "dominant-baseline=\"central\""),
    ("~s", "stroke=\"currentColor\""),
    ("~f", "fill=\"currentColor\""),
    ("~n", "fill=\"none\""),
    ("~m", "text-anchor=\"middle\""),
    ("~e", "text-anchor=\"end\""),
    ("~d", "stroke-dasharray=\"6,4\""),
    ("~o", "stroke-opacity=\""),
    ("~i", "font-style=\"italic\""),
    ("~F", "fill-opacity=\"0.15\""),
    ("~w", "stroke-width=\""),
    ("~z", "font-size=\""),
    ("~v", "viewBox=\""),
    ("~M", "style=\"max-width:"),
    ("~L", "<line "),
    ("~T", "<text "),
    ("~C", "<circle "),
    ("~P", "<path d=\""),
    ("~Q", "<polyline points=\""),
    ("~G", "<polygon points=\""),
    ("~E", "</text>"),
    ("~g", "class=\"g\""),
    ("~a", "class=\"a\""),
    ("~t", "class=\"t\""),
];

/// Decompress a dictionary-compressed SVG, or return raw SVG unchanged.
pub fn decompress_svg(s: &str) -> String {
    let body = match s.strip_prefix("s1:") {
        Some(b) => b,
        None => return s.to_string(),
    };
    let mut out = body.to_string();
    for &(token, expansion) in SVG_DICT {
        out = out.replace(token, expansion);
    }
    out
}

/// Compress a raw SVG using the same dictionary `decompress_svg` inverts.
/// Iterates `SVG_DICT` in the declared longest-first order, replacing each
/// expansion with its short token. Output is prefixed with `"s1:"`.
pub fn compress_svg(s: &str) -> String {
    let mut out = s.to_string();
    for &(token, expansion) in SVG_DICT {
        out = out.replace(expansion, token);
    }
    format!("s1:{out}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_compressed() {
        let original = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 200" fill="none" stroke="currentColor"><line class="g" stroke-width="2" x1="0" y1="100" x2="400" y2="100"/><text class="t" text-anchor="middle" dominant-baseline="central" font-size="14" fill="currentColor" x="200" y="50">Hello</text></svg>"#;
        // This is the exact output of Python compress_svg(original)
        let compressed = r#"s1:<svg ~X ~v0 0 400 200" ~n ~s>~L~g ~w2" x1="0" y1="100" x2="400" y2="100"/>~T~t ~m ~D ~z14" ~f x="200" y="50">Hello~E</svg>"#;
        assert_eq!(decompress_svg(compressed), original);
    }

    #[test]
    fn raw_svg_passthrough() {
        let raw = r#"<svg viewBox="0 0 10 10"><rect/></svg>"#;
        assert_eq!(decompress_svg(raw), raw);
    }

    #[test]
    fn empty_string_passthrough() {
        assert_eq!(decompress_svg(""), "");
    }

    #[test]
    fn compress_matches_python_fixture() {
        let original = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 200" fill="none" stroke="currentColor"><line class="g" stroke-width="2" x1="0" y1="100" x2="400" y2="100"/><text class="t" text-anchor="middle" dominant-baseline="central" font-size="14" fill="currentColor" x="200" y="50">Hello</text></svg>"#;
        let expected = r#"s1:<svg ~X ~v0 0 400 200" ~n ~s>~L~g ~w2" x1="0" y1="100" x2="400" y2="100"/>~T~t ~m ~D ~z14" ~f x="200" y="50">Hello~E</svg>"#;
        assert_eq!(compress_svg(original), expected);
    }

    #[test]
    fn compress_decompress_roundtrip() {
        let original = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 400 200" fill="none" stroke="currentColor"><line class="g" stroke-width="2" x1="0" y1="100" x2="400" y2="100"/></svg>"#;
        assert_eq!(decompress_svg(&compress_svg(original)), original);
    }
}
