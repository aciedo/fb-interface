use std::io::Write;

use interface::{
    auth::*,
    planus::{Builder, ReadAsRoot},
};
use reqwest::{StatusCode, Client};

use crate::inputs::{HOST, NAME, NUMBER}; 

mod inputs;

// Copy `inputs_template.rs` to `inputs.rs` and fill in the values
#[tokio::main]
async fn main() {
    let client = Client::new();
    // make request to `/` to get a TLS connection
    client.get(HOST).send().await.unwrap();

    let start = std::time::Instant::now();
    let reg = match register_number(NAME, NUMBER, &client).await {
        Ok(res) => {
            println!("Success: {:?}", res);
            res
        },
        Err(err) => panic!(
            "Error: {:?}",
            err.error.unwrap_or("No error provided".to_string())
        ),
    };
    println!("Time taken: {:?}", start.elapsed());

    // wait for user to enter code
    let code = read_line("Enter code: ").parse::<u64>().unwrap();
    let start = std::time::Instant::now();
    match verify_number(NUMBER, code * reg.multiplier as u64, &client).await {
        Ok(res) => println!("Success: {:?}", res),
        Err(err) => panic!(
            "Error: {:?}",
            err.error.unwrap_or("No error provided".to_string())
        ),
    }
    println!("Time taken: {:?}", start.elapsed());
}

async fn register_number(
    name: &str, number: &str, client: &Client
) -> Result<RegisterNumberRes, ErrorRes> {
    let mut builder = Builder::new();

    println!("building body");
    let body = RegisterNumberReq::builder()
        .name(name)
        .number(number)
        .finish(&mut builder);

    println!("sending request");
    let res = client
        .post(format!("{HOST}/auth/register-number"))
        .body(builder.finish(body, None).to_vec())
        .send()
        .await
        .unwrap();

    let status = res.status();
    let bytes = res.bytes().await.unwrap();
    println!("got bytes with length {:?}", bytes.len());

    match status {
        StatusCode::OK => Ok(RegisterNumberResRef::read_as_root(&bytes)
            .unwrap()
            .try_into()
            .unwrap()),
        _ => Err(ErrorResRef::read_as_root(&bytes)
            .unwrap()
            .try_into()
            .unwrap()),
    }
}

async fn verify_number(number: &str, code: u64, client: &Client) -> Result<VerifyNumberRes, ErrorRes> {
    let mut builder = Builder::new();
    println!("building body");
    let body = VerifyNumberReq::builder()
        .number(number)
        .code(code)
        .finish(&mut builder);

    println!("sending request");
    let res = client
        .post(format!("{HOST}/auth/verify-number"))
        .body(builder.finish(body, None).to_vec())
        .send()
        .await
        .unwrap();
    println!("got response");

    let status = res.status();
    let bytes = res.bytes().await.unwrap();
    println!("got bytes with length {:?}", bytes.len());

    match status {
        StatusCode::OK => Ok(VerifyNumberResRef::read_as_root(&bytes)
            .unwrap()
            .try_into()
            .unwrap()),
        _ => Err(ErrorResRef::read_as_root(&bytes)
            .unwrap()
            .try_into()
            .unwrap()),
    }
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}