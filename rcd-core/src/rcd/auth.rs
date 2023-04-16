use tracing::debug;
use rcdproto::rcdp::{AuthRequest, RevokeReply, TokenReply};

use crate::rcd::Rcd;

pub async fn auth_for_token(core: &Rcd, request: AuthRequest) -> TokenReply {
    let auth_result = core.verify_login(request.clone());
    let mut jwt = String::from("");
    let mut expiration = String::from("");
    let mut is_successful = false;

    if auth_result.0 {
        let result = core.dbi().create_token_for_login(&request.user_name);
        jwt = result.0;
        expiration = result.1.to_rfc3339();
        is_successful = true;

        debug!("created rcd token for {}", &request.user_name);
    }

    TokenReply {
        is_successful,
        expiration_utc: expiration,
        jwt,
    }
}

pub async fn revoke_token(core: &Rcd, request: AuthRequest) -> RevokeReply {
    let auth_result = core.verify_login(request.clone());
    let jwt = request.jwt;
    let mut is_successful = false;

    if auth_result.0 {
        let result = core.dbi().revoke_token(&jwt);
        is_successful = result;
    }

    RevokeReply { is_successful }
}
