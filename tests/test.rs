use std::str::FromStr;

use axum::http::HeaderValue;
use axum_forwarded_header::ForwardedHeader;

#[test]
fn test_parse_forwarded_header_with_for_field() {
    assert_eq!(
        ForwardedHeader::try_from(&HeaderValue::from_str("cheese").unwrap())
            .expect("Failed to parse input")
            .for_field,
        Vec::<String>::new(),
    );

    assert_eq!(
        ForwardedHeader::try_from(&HeaderValue::from_str("for=192.168.0.1").unwrap())
            .expect("Failed to parse input")
            .for_field,
        vec!["192.168.0.1".to_string()]
    );
    assert_eq!(
        ForwardedHeader::try_from(
            &HeaderValue::from_str("For=\"[2001:db8:cafe::17]:4711\"").unwrap()
        )
        .expect("Failed to parse input")
        .for_field,
        vec!["\"[2001:db8:cafe::17]:4711\"".to_string()]
    );
    let test_res = ForwardedHeader::try_from(
        &HeaderValue::from_str("for=192.0.2.60;proto=http;by=203.0.113.43").unwrap(),
    )
    .expect("Failed to parse input");
    assert_eq!(test_res.for_field, vec!["192.0.2.60".to_string()]);
    assert_eq!(test_res.proto, Some("http".to_string()));
    assert_eq!(
        ForwardedHeader::try_from(
            HeaderValue::from_str("for=192.0.2.43, for=198.51.100.17").unwrap()
        )
        .expect("Failed to parse input")
        .for_field,
        vec!["192.0.2.43".to_string(), "198.51.100.17".to_string()]
    );
}

#[test]
fn test_by_field() {
    let res = ForwardedHeader::try_from(
        &HeaderValue::from_str("for=192.0.2.43, for=198.51.100.17;by=1.2.3.4;monkeycheese=hello")
            .expect("Failed to generate header"),
    )
    .expect("Failed to parse header");
    assert_eq!(res.by, Some("1.2.3.4".to_string()));
}

#[test]
fn test_host_field() {
    let res = ForwardedHeader::try_from(
        &HeaderValue::from_str(
            "for=192.0.2.43, for=198.51.100.17;by=1.2.3.4;host=1.2.3.4;monkeycheese=hello",
        )
        .expect("Failed to generate header"),
    )
    .expect("Failed to parse header");
    assert_eq!(res.host, Some("1.2.3.4".to_string()));
}

#[test]
fn test_derive() {
    let res = ForwardedHeader::try_from(
        &HeaderValue::from_str(
            "for=192.0.2.43, for=198.51.100.17;by=1.2.3.4;host=1.2.3.4;monkeycheese=hello",
        )
        .expect("Failed to generate header"),
    )
    .expect("Failed to parse header");

    let debug_output = format!("{:?}", res);
    dbg!(&debug_output);
    assert!(!debug_output.contains("monkeycheese"));
}

#[test]
fn test_into() {
    let res: ForwardedHeader = (&HeaderValue::from_str(
        "for=192.0.2.43, for=198.51.100.17;by=1.2.3.4;host=1.2.3.4;monkeycheese=hello",
    )
    .expect("Failed to generate header"))
        .try_into()
        .expect("whoops");
    println!("{:?}", res);
}

#[test]
fn test_for_as_ipaddr() {
    let res = ForwardedHeader::try_from(
        &HeaderValue::from_str("for=198.51.100.17").expect("Failed to get headervalue"),
    )
    .expect("Failed to parse header into ForwardedHeader");

    let ipaddr = res.for_as_ipaddr();
    assert_eq!(ipaddr.len(), 1);

    // now test with funky ipv6 addresses
    let res = ForwardedHeader::try_from(
        &HeaderValue::from_str("For=\"[2001:db8:cafe::17]:4711\"")
            .expect("Failed to get headervalue"),
    )
    .expect("Failed to parse header into ForwardedHeader");

    let ipaddr = res.for_as_ipaddr();
    dbg!(&ipaddr);
    assert_eq!(ipaddr.len(), 1);
    assert!(ipaddr[0].is_ipv6());
    assert!(ipaddr[0] == std::net::IpAddr::from_str("2001:db8:cafe::17").unwrap());
}
