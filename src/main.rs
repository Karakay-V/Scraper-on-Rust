use std::any::Any;
use std::fmt::Display;
use std::time::Duration;
use fantoccini::elements::Element;
use fantoccini::Locator;

mod user;
use user::{type_username, type_password};

mod login;
use login::fun;

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let username = type_username();
    let password = type_password();
    fun(username,password).await
}

// println!("{:?}", c.get_all_cookies().await?.value());
