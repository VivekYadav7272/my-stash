mod client_conn;
mod users;

use futures::TryStreamExt;
use mongodb::{
    bson::{self, doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::{Deserialize, Serialize};
use users::*;
use vercel_runtime::{run, Body, Error as VercelError, Request, Response, StatusCode};

#[tokio::main]
async fn main() -> Result<(), VercelError> {
    run(get_vault).await
    // run(handler).await
}

pub async fn get_vault(r: Request) -> Result<Response<Body>, VercelError> {
    let Some(auth_token) = get_auth_token(&r) else {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Auth token not provided, or is not valid UTF-8".into())?);
    };

    let database = client_conn::get_conn().await?;

    // unless the user is TEST_USER, all the usernames and passwords would be encrypted.
    // since this function simply returns the whole vault, this isn't a problem.
    // however, when the person requires only a specific UserToken, then we need to
    // somehow filter it. However, for that we would require the encryption key, which
    // we don't have.
    // So we have laid down on the model that the user would simply send us the encrypted username blob,
    // and we'll match against that.
    // For some reason this feels hacky, but it works just fine and is secure.

    #[cfg(debug_assertions)]
    let hashed_auth_token = if auth_token == "TEST_USER" {
        // this guy ain't encrypted. just send it aloong.
        auth_token
    } else {
        // pbkdf2 this shit
        "dummy value for now"
    };

    #[cfg(not(debug_assertions))]
    let hashed_auth_token = "dummy value for now";

    // Since converting to a UserToken and then serialising it is needlessly expensive,
    // afterall we just need to dump the tokens into the response stream.
    // so though this introduces the possibility of letting through a malformed
    // UserToken (since parsing into one would act as a validation step),
    // we choose not to do this to reduce overhead.
    let user_tokens = database.collection::<bson::Bson>(hashed_auth_token);

    let mut cursor = user_tokens.find(None, None).await?;
    let mut all_tokens = vec![];
    while let Some(token) = cursor.try_next().await? {
        all_tokens.push(token);
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(bson::Bson::Array(all_tokens).to_string().into())?)
}

fn get_auth_token(r: &Request) -> Option<&str> {
    r.headers()
        .get("auth_key")
        .and_then(|value| value.to_str().ok())
}
