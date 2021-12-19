mod state;

extern crate telegram_bot;
extern crate hyper;
extern crate hyper_rustls;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
use tokio::runtime::Runtime;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ResponseType {
    ProgramCompiled { program: String, rustc: String },
    ProgramCompileError { rustc: String }
}

#[derive(Serialize)]
pub struct PlaygroundRequest {
    code: String,
    version: String,
    optimize: String,
    test: bool,
    separate_output: bool,
    color: bool,
    backtrace: String
}

#[tokio::main]
async fn main() -> Result<(), telegram_bot::Error> {
    let token = std::env::var("TOKEN").expect("TOKEN env var must be set");
    let api = telegram_bot::Api::new(token);

    //println!("getMe: {:?}", api.get_me());

    let mut stream = api.stream();

    use futures::StreamExt;

    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let telegram_bot::UpdateKind::Message(message) = update.kind {
            if let telegram_bot::MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.send(telegram_bot::SendMessage::new(message.chat, format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                    .await?;
            }
        }
    }
    Ok(())
}
