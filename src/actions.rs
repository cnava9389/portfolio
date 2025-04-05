use leptos::{server, prelude::ServerFnError};

#[server]
pub async fn email_me(name: Option<String>, email: String, message: Option<String>) -> Result<(), ServerFnError> {
    println!("{:#?}, {:#?}, {:#?}", name, email, message);
    Ok(())
}
