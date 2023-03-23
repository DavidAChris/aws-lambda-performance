use aws_lambda_events::event::sqs::SqsEvent;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::Deserialize;

#[derive(Deserialize)]
struct QueueMsg {
    num: u64,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn lambda_handler(event: LambdaEvent<SqsEvent>) -> Result<(), Error> {
    // Extract some useful information from the request
    let msg = event.payload.records[0].body.to_owned();
    let msg: QueueMsg = serde_json::from_str(&msg.unwrap())?;
    println!("{}", fib_recur(msg.num));
    Ok(())
}

fn fib_recur(num: u64) -> u64 {
    return if num <= 1 {
        num
    } else {
        fib_recur(num - 1) + fib_recur(num - 2)
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(lambda_handler)).await
}
