use leptos::{server, prelude::ServerFnError};

#[server]
pub async fn email_me(name: Option<String>, email: String, message: Option<String>) -> Result<(), ServerFnError> {
    use leptos_actix::extract;
    let req: actix_web::web::Data<reqwest::Client> = extract().await?;
    match req.post(format!("http://{}/email", std::env::var("EMAILER").unwrap_or_else(|_| "0.0.0.0:8100".to_string())))
    .json(&serde_json::json!({ "name": name, "email": email, "message": message }))
    .send()
    .await {
        Ok(res) => {
            match res.status() {
                reqwest::StatusCode::OK => (), 
                _ => return Err(ServerFnError::ServerError("Invalid data sent".to_string())),
            };
        },
        Err(e) => {
            log::error!("{:#?}", e);
            return Err(ServerFnError::ServerError("error emailing".to_string()))
        }
    };
    Ok(())
}
