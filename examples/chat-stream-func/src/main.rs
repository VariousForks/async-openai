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

    // For reasons not documented in OpenAI docs / OpenAPI spec,
    // the response of streaming call is different and doesn't include all the same fields.

    // From Rust docs on print: https://doc.rust-lang.org/std/macro.print.html
    //
    //  Note that stdout is frequently line-buffered by default so it may be necessary
    //  to use io::stdout().flush() to ensure the output is emitted immediately.
    //
    //  The print! macro will lock the standard output on each call.
    //  If you call print! within a hot loop, this behavior may be the bottleneck of the loop.
    //  To avoid this, lock stdout with io::stdout().lock():

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
