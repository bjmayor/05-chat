use crate::{
    error::ErrorOutput,
    models::{CreateUser, SigninUser},
    AppError, AppState,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Serialize, Deserialize)]
pub struct AuthOutput {
    token: String,
}

#[utoipa::path(
    post,
    path = "/api/signup",
    responses(
        (status = 200, description = "User created", body = AuthOutput),
    )
)]
/// Create a new user in the chat system with email and password.
///
/// - If the email already exists, it will return 409.
/// - Otherwise, it will return 201 with a token.
/// - If the workspace doesn't exist, it will create one.
pub(crate) async fn signup_handler(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.create_user(&input).await?;
    let token = state.ek.sign(user)?;
    let body = Json(AuthOutput { token });
    Ok((StatusCode::CREATED, body))
}

#[utoipa::path(
    post,
    path = "/api/signin",
    responses(
        (status = 200, description = "User signed in", body = AuthOutput),
    )
)]
pub(crate) async fn signin_handler(
    State(state): State<AppState>,
    Json(input): Json<SigninUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.verify_user(&input).await?;
    let result = match user {
        Some(user) => {
            let token = state.ek.sign(user)?;
            (StatusCode::OK, Json(AuthOutput { token })).into_response()
        }
        None => {
            let body = Json(ErrorOutput::new("Invalid email or password"));
            (StatusCode::FORBIDDEN, body).into_response()
        }
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ErrorOutput;
    use anyhow::Result;
    use http_body_util::BodyExt;

    #[tokio::test]
    async fn signup_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("acme", "Tian Chen", "tyr@acme.org", "123456");
        let ret = signup_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::CREATED);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");
        Ok(())
    }

    #[tokio::test]
    async fn signup_duplicate_user_should_409() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateUser::new("acme", "Tyr Chen", "tchen@acme.org", "123456");
        let ret = signup_handler(State(state.clone()), Json(input.clone()))
            .await
            .into_response();
        assert_eq!(ret.status(), StatusCode::CONFLICT);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;

        assert_eq!(ret.error, "email already exists: tchen@acme.org");
        Ok(())
    }

    #[tokio::test]
    async fn signin_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let email = "tchen@acme.org";
        let password = "123456";
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await?
            .into_response();
        assert_eq!(ret.status(), StatusCode::OK);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: AuthOutput = serde_json::from_slice(&body)?;
        assert_ne!(ret.token, "");

        Ok(())
    }

    #[tokio::test]
    async fn signin_with_non_exist_user_should_403() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let email = "alice@acme.org";
        let password = "Hunter42";
        let input = SigninUser::new(email, password);
        let ret = signin_handler(State(state), Json(input))
            .await
            .into_response();
        println!("{:?}", ret);
        assert_eq!(ret.status(), StatusCode::FORBIDDEN);
        let body = ret.into_body().collect().await?.to_bytes();
        let ret: ErrorOutput = serde_json::from_slice(&body)?;
        assert_eq!(ret.error, "Invalid email or password");

        Ok(())
    }
}
