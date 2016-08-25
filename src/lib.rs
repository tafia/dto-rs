extern crate rustc_serialize;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct AccessTokenDTO {
    app_id: String,
    scopes: String,
    access_token: String,
    token_type: TokenTypeDTO,
    expiration: i64,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, RustcDecodable, RustcEncodable)]
pub enum TokenTypeDTO {
    Bearer,
}
