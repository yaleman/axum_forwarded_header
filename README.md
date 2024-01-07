# axum-forwarded-header

Functionality for parsing the "Forwarded" header into a struct you can get info from.

Per [RFC7239 Section 4](https://www.rfc-editor.org/rfc/rfc7239#section-4).

## Usage

Either build the struct manually, or parse a `HeaderValue` from the `http` crate using `.try_from()`. It exposes use `TryFrom` instead of `From` because there's a `to_str` it needs to do and I'd rather not put an `expect()` call into.

See the tests for examples.
