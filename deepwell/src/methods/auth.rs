/*
 * methods/auth.rs
 *
 * DEEPWELL - Wikijump API provider and database manager
 * Copyright (C) 2019-2023 Wikijump Team
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use super::prelude::*;
use crate::services::authentication::{
    AuthenticateUserOutput, AuthenticationService, LoginUser, LoginUserMfa,
    LoginUserOutput, MultiFactorAuthenticateUser, MultiFactorConfigure,
};
use crate::services::session::{
    CreateSession, InvalidateOtherSessions, RenewSession, SessionInputOutput,
    VerifySession,
};
use crate::services::user::GetUser;
use crate::services::{Error, MfaService, SessionService};
use crate::web::UserIdQuery;

pub async fn auth_login(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let LoginUser {
        authenticate,
        ip_address,
        user_agent,
    } = req.body_json().await?;

    // Don't allow empty passwords.
    //
    // They are never valid, and are potentially indicative of the user
    // entering the password in the name field instead, which we do
    // *not* want to be logging.
    if authenticate.password.is_empty() {
        tide::log::error!("User submitted empty password in auth request");
        return Err(TideError::from_str(StatusCode::BadRequest, ""));
    }

    // All authentication issue should return the same error.
    //
    // If anything went wrong, only allow a generic backend failure
    // to avoid leaking internal state.
    //
    // The only three possible responses to this method should be:
    // * success
    // * invalid authentication
    // * server error
    let result = AuthenticationService::auth_password(&ctx, authenticate).await;
    let AuthenticateUserOutput { needs_mfa, user_id } = match result {
        Ok(output) => output,
        Err(error) => {
            let status_code = match error {
                Error::InvalidAuthentication => StatusCode::Forbidden,
                _ => {
                    tide::log::error!(
                        "Unexpected error during user authentication: {error}",
                    );

                    StatusCode::InternalServerError
                }
            };

            return Err(TideError::from_str(status_code, ""));
        }
    };

    let login_complete = !needs_mfa;
    tide::log::info!(
        "Password authentication for user ID {user_id} succeeded (login complete: {login_complete})",
    );

    let session_token = SessionService::create(
        &ctx,
        CreateSession {
            user_id,
            ip_address,
            user_agent,
            restricted: !login_complete,
        },
    )
    .await?;

    let body = Body::from_json(&LoginUserOutput {
        session_token,
        needs_mfa,
    })?;

    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_session_get(req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let UserIdQuery { user_id } = req.query()?;

    let sessions = SessionService::get_all(&ctx, user_id).await.to_api()?;

    let body = Body::from_json(&sessions)?;
    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_session_validate(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let VerifySession {
        session_token,
        user_id,
    } = req.body_json().await?;

    SessionService::verify(&ctx, &session_token, user_id)
        .await
        .to_api()?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn auth_session_renew(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let input: RenewSession = req.body_json().await?;

    let session_token = SessionService::renew(&ctx, input).await.to_api()?;

    let body = Body::from_json(&SessionInputOutput { session_token })?;
    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_session_invalidate_others(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let InvalidateOtherSessions {
        session_token,
        user_id,
    } = req.body_json().await?;

    let invalidated = SessionService::invalidate_others(&ctx, &session_token, user_id)
        .await
        .to_api()?;

    let body = Body::from_json(&invalidated)?;
    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_logout(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let SessionInputOutput { session_token } = req.body_json().await?;

    SessionService::invalidate(&ctx, session_token).await?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn auth_mfa_verify(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);
    let LoginUserMfa {
        session_token,
        totp_or_code,
        ip_address,
        user_agent,
    } = req.body_json().await?;

    tide::log::info!(
        "Verifying user's MFA for login (temporary session token {session_token})"
    );

    let user = AuthenticationService::auth_mfa(
        &ctx,
        MultiFactorAuthenticateUser {
            session_token: &session_token,
            totp_or_code: &totp_or_code,
        },
    )
    .await?;

    let new_session_token = SessionService::renew(
        &ctx,
        RenewSession {
            old_session_token: session_token,
            user_id: user.user_id,
            ip_address,
            user_agent,
        },
    )
    .await?;

    let body = Body::from_json(&SessionInputOutput {
        session_token: new_session_token,
    })?;

    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_mfa_setup(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let GetUser { user: reference } = req.body_json().await?;
    let user = UserService::get(&ctx, reference.borrow()).await.to_api()?;
    let output = MfaService::setup(&ctx, &user).await.to_api()?;

    let body = Body::from_json(&output)?;
    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}

pub async fn auth_mfa_disable(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let MultiFactorConfigure { session_token } = req.body_json().await?;
    let user = SessionService::get_user(&ctx, &session_token, false)
        .await
        .to_api()?;

    MfaService::disable(&ctx, user.user_id).await.to_api()?;

    txn.commit().await?;
    Ok(Response::new(StatusCode::NoContent))
}

pub async fn auth_mfa_reset_recovery(mut req: ApiRequest) -> ApiResponse {
    let txn = req.database().begin().await?;
    let ctx = ServiceContext::new(&req, &txn);

    let MultiFactorConfigure { session_token } = req.body_json().await?;
    let user = SessionService::get_user(&ctx, &session_token, false)
        .await
        .to_api()?;

    let output = MfaService::reset_recovery_codes(&ctx, &user)
        .await
        .to_api()?;

    let body = Body::from_json(&output)?;
    let response = Response::builder(StatusCode::Ok).body(body).into();
    txn.commit().await?;
    Ok(response)
}
