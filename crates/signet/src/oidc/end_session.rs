use crate::auth::session::{
    clear_session_cookie, cookie_value, destroy_session, SESSION_COOKIE,
};
use crate::error::{AppError, AppResult};
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect, Response};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EndSessionQuery {
    #[allow(dead_code)]
    id_token_hint: Option<String>,
    post_logout_redirect_uri: Option<String>,
    state: Option<String>,
    client_id: Option<String>,
}

pub async fn end_session(
    State(state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
    Query(q): Query<EndSessionQuery>,
) -> AppResult<Response> {
    let jar = if let Some(token) = cookie_value(&headers, SESSION_COOKIE) {
        destroy_session(&state.pool, &token).await?;
        jar.add(clear_session_cookie(state.config.cookie_secure))
    } else {
        jar
    };

    // Redirect back to the client only if the URI is allow-listed for that client.
    if let (Some(client_id), Some(uri)) = (&q.client_id, &q.post_logout_redirect_uri) {
        let allowed: Option<Vec<String>> = sqlx::query_scalar(
            "SELECT post_logout_redirect_uris FROM client_apps WHERE client_id = $1",
        )
        .bind(client_id)
        .fetch_optional(&state.pool)
        .await?;
        if let Some(list) = allowed {
            if list.iter().any(|u| u == uri) {
                let mut url =
                    url::Url::parse(uri).map_err(|_| AppError::bad_request("bad redirect uri"))?;
                if let Some(s) = &q.state {
                    url.query_pairs_mut().append_pair("state", s);
                }
                return Ok((jar, Redirect::temporary(url.as_str())).into_response());
            }
        }
    }

    Ok((jar, Redirect::temporary("/")).into_response())
}
