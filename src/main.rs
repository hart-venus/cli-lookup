use std::env;
use openai_api_rust::*;
use openai_api_rust::chat::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args: String = args[1..].join(" ");

    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        max_tokens: Some(1024),
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,

        messages: vec![Message {
            role: Role::User,
            content: args,
        }],
    };

    let response = openai.chat_completion_create(&body).unwrap();
    let choice = response.choices.first().unwrap().message.as_ref().unwrap();

    println!("{}", choice.content);

}
