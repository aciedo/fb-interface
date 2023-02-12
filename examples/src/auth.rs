use std::io::Write;

use interface::{
    auth::api::{
        ErrorRes, ErrorResRef, RegisterNumberReq, RegisterNumberRes, RegisterNumberResRef,
        VerifyNumberReq, VerifyNumberRes, VerifyNumberResRef,
    },
    Builder, ReadAsRoot,
};
use reqwest::StatusCode;

use crate::inputs::{HOST, NAME, NUMBER};

mod inputs;

// Copy `inputs_template.rs` to `inputs.rs` and fill in the values
#[tokio::main]
async fn main() {
    let client = Client::new().await;

    let start = std::time::Instant::now();
    let reg = match client.register_number(NAME, NUMBER).await {
        Ok(res) => {
            println!("Success: {:?}", res);
            res
        }
        Err(err) => panic!(
            "Error: {:?}",
            err.error.unwrap_or("No error provided".to_string())
        ),
    };
    println!("Time taken: {:?}", start.elapsed());

    // wait for user to enter code
    let code = read_line("Enter code: ").parse::<u64>().unwrap();
    let start = std::time::Instant::now();
    match client
        .verify_number(NUMBER, code * reg.multiplier as u64)
        .await
    {
        Ok(res) => println!("Success: {:?}", res),
        Err(err) => panic!(
            "Error: {:?}",
            err.error.unwrap_or("No error provided".to_string())
        ),
    }
    println!("Time taken: {:?}", start.elapsed());
}

struct Client {
    client: reqwest::Client,
}

impl Client {
    async fn new() -> Self {
        let client = reqwest::Client::new();
        // get a TLS connection
        client.head(HOST).send().await.unwrap();
        Self { client }
    }

    async fn register_number(
        &self,
        name: &str,
        number: &str,
    ) -> Result<RegisterNumberRes, ErrorRes> {
        let mut builder = Builder::new();

        println!("building body");
        let body = RegisterNumberReq::builder()
            .name(name)
            .number(number)
            .finish(&mut builder);

        println!("sending request");
        let res = self
            .client
            .post(format!("{HOST}/v1/auth/register_number"))
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

    async fn verify_number(&self, number: &str, code: u64) -> Result<VerifyNumberRes, ErrorRes> {
        let mut builder = Builder::new();
        println!("building body");
        let body = VerifyNumberReq::builder()
            .number(number)
            .code(code)
            .finish(&mut builder);

        println!("sending request");
        let res = self
            .client
            .post(format!("{HOST}/v1/auth/verify_number"))
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
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
