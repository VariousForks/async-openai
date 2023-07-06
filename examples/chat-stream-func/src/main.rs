use async_openai::{
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use futures::StreamExt; // remove TryStreamExt
use std::error::Error;
use std::io::{stdout, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestMessageArgs::default()
            .content("Write a marketing blog praising and introducing Rust library async-openai")
            .role(Role::User)
            .build().unwrap()])
        .build().unwrap();
        
    let result = client.chat().create_stream(request).await;

    match result {
        Ok(mut chat_stream) => {
            let mut lock = stdout();
            while let Some(message) = chat_stream.next().await {
                match message {
                    Ok(response) => {
                        for chat_choice in response.choices.iter() {
                            if let Some(ref content) = chat_choice.delta.content {
                                write!(lock, "{}", content).unwrap();
                            }
                        }
                    }
                    Err(_) => {
                        writeln!(lock, "error: an error occurred while receiving a message from the stream").unwrap();
                    }
                }
            }
            Ok(())
        }
        Err(_) => Err(Box::from("An error occurred while creating a stream"))
    }
}
