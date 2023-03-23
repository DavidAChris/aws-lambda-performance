cargo lambda build --release
cargo lambda deploy --iam-role arn:aws:iam::<account number>:role/<role_name> rust-lambda