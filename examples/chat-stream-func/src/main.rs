use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::{StreamExt, TryStreamExt};
use std::error::Error;
use std::{io::{stdout, Write}, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content("Write a marketing blog praising and introducing Rust library async-openai")
            .role(Role::User)
            .build()?])
        .build()?;
    let chat_stream = client.chat().create_stream(request).await?;

    let mut lock = stdout();
    chat_stream
        .map_err(|err| writeln!(lock, "error: {err}").unwrap())
        .for_each(|response| {
            response.choices.iter().for_each(|chat_choice| {
                if let Some(ref content) = chat_choice.delta.content {
                    write!(lock, "{}", content).unwrap();
                }
            });
        })
        .await
}
