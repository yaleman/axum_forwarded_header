use axum::http::HeaderValue;

/// Fields from a "Forwarded" header per [RFC7239 sec 4](https://www.rfc-editor.org/rfc/rfc7239#section-4)
#[derive(Debug)]
pub struct ForwardedHeader {
    pub for_field: Vec<String>,
    pub by: Option<String>,
    pub host: Option<String>,
    pub proto: Option<String>,
}

impl ForwardedHeader {
    /// Return the 'for' headers as a list of [std::net::IpAddr]'s.
    pub fn for_as_ipaddr(self) -> Vec<std::net::IpAddr> {
        self.for_field
            .iter()
            .filter_map(|ip| {
                if ip.contains(']') {
                    // this is an IPv6 address, get what's between the []
                    ip.split(']')
                        .next()?
                        .split('[')
                        .last()?
                        .parse::<std::net::IpAddr>()
                        .ok()
                } else {
                    ip.parse::<std::net::IpAddr>().ok()
                }
            })
            .collect::<Vec<std::net::IpAddr>>()
    }
}

/// This parses the Forwarded header, and returns a list of the IPs in the "for=" fields.
/// Per [RFC7239 sec 4](https://www.rfc-editor.org/rfc/rfc7239#section-4)
impl TryFrom<HeaderValue> for ForwardedHeader {
    type Error = String;
    fn try_from(forwarded: HeaderValue) -> Result<ForwardedHeader, String> {
        ForwardedHeader::try_from(&forwarded)
    }
}

/// This parses the Forwarded header, and returns a list of the IPs in the "for=" fields.
/// Per [RFC7239 sec 4](https://www.rfc-editor.org/rfc/rfc7239#section-4)
impl TryFrom<&HeaderValue> for ForwardedHeader {
    type Error = String;
    fn try_from(forwarded: &HeaderValue) -> Result<ForwardedHeader, String> {
        let mut for_field: Vec<String> = Vec::new();
        let mut by: Option<String> = None;
        let mut host: Option<String> = None;
        let mut proto: Option<String> = None;
        // first get the k=v pairs
        forwarded
            .to_str()
            .map_err(|err| err.to_string())?
            .split(';')
            .for_each(|s| {
                let s = s.trim().to_lowercase();
                // The for value can look like this:
                // for=192.0.2.43, for=198.51.100.17
                // so we need to handle this case
                if s.starts_with("for=") || s.starts_with("for =") {
                    // we have a valid thing to grab
                    let chunks: Vec<String> = s
                        .split(',')
                        .filter_map(|chunk| chunk.trim().split('=').last().map(|c| c.to_string()))
                        .collect::<Vec<String>>();
                    for_field.extend(chunks);
                } else if s.starts_with("by=") {
                    by = s.split('=').last().map(|c| c.to_string());
                } else if s.starts_with("host=") {
                    host = s.split('=').last().map(|c| c.to_string());
                } else if s.starts_with("proto=") {
                    proto = s.split('=').last().map(|c| c.to_string());
                } else {
                    // probably need to work out what to do here
                }
            });

        Ok(ForwardedHeader {
            for_field,
            by,
            host,
            proto,
        })
    }
}
