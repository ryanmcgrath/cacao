use std::fmt;

const MIMETYPE_PLAIN: &str = "text/plain";

/// [Web Compatible MimeTypes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types#important_mime_types_for_web_developers)
pub enum MimeType {
    CSS,
    CSV,
    HTML,
    ICO,
    JS,
    JSON,
    JSONLD,
    OCTETSTREAM,
    RTF,
    SVG,
}

impl std::fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mime = match self {
            MimeType::CSS => "text/css",
            MimeType::CSV => "text/csv",
            MimeType::HTML => "text/html",
            MimeType::ICO => "image/vnd.microsoft.icon",
            MimeType::JS => "text/javascript",
            MimeType::JSON => "application/json",
            MimeType::JSONLD => "application/ld+json",
            MimeType::OCTETSTREAM => "application/octet-stream",
            MimeType::RTF => "application/rtf",
            MimeType::SVG => "image/svg+xml",
        };
        write!(f, "{}", mime)
    }
}

impl MimeType {
    /// parse a URI suffix to convert text/plain mimeType to their actual web compatible mimeType.
    pub fn parse_from_uri(uri: &str) -> MimeType {
        let suffix = uri.split('.').last();
        match suffix {
            Some("bin") => Self::OCTETSTREAM,
            Some("css") => Self::CSS,
            Some("csv") => Self::CSV,
            Some("html") => Self::HTML,
            Some("ico") => Self::ICO,
            Some("js") => Self::JS,
            Some("json") => Self::JSON,
            Some("jsonld") => Self::JSONLD,
            Some("rtf") => Self::RTF,
            Some("svg") => Self::SVG,
            // Assume HTML when a TLD is found for eg. `protocol:://example.com`
            Some(_) => Self::HTML,
            // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
            // using octet stream according to this:
            None => Self::OCTETSTREAM,
        }
    }

    /// infer mimetype from content (or) URI if needed.
    pub fn parse(content: &[u8], uri: &str) -> String {
        let mime = match infer::get(&content) {
            Some(info) => info.mime_type(),
            None => MIMETYPE_PLAIN,
        };

        if mime == MIMETYPE_PLAIN {
            return Self::parse_from_uri(uri).to_string();
        }

        mime.to_string()
    }
}
