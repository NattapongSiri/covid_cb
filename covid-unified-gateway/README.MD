# covid-unified-gateway
A gateway for covid chatbot that implement in Rust.
There're 3 submodules in this project which shall be split, in the future, to separate crate.
- utils - A utilities function wrap around libcurl to send REST request
- wa - Watson Assistant related type
- wlt - Watson Language Translate related type

# How to build
It requires docker container call `muslrust`.
It came with nightlies Rust with some basic C lib compile by `musl` such as `libcurl`, `openssl`, etc.
## Step to build
1. docker run -v <YOUR_CLONED_DIR>:/app -it clux/muslrust /bin/sh
1. rustup toolchain install stable-x86_64-unknown-linux-musl
1. cd /app
1. cargo build --release --target x86_64-unknown-linux-musl
## How to run
Create `.env` file and put it preferable at root of covid-unified-gateway project.
Inside `.env` file, specify following info:
```
WLT_APIKEY=<YOUR_WLT_APIKEY>
WLT_ENDPOINT=<YOUR_WLT_ENDPOINT_URL>
WLT_VERSION=<YOUR_WLT_VERSION>
WLT_RETRY=<MAX_RETRY_ON_FAIL_WLT>
WA_APIKEY=<YOUR_WA_APIKEY>
WA_ID=<YOUR_WA_ASSISTANT_ID>
WA_ENDPOINT=<YOUR_WA_ENDPOINT_URL>
WA_VERSION=<YOUR_WA_API_VERSION>
WA_RETRY=<MAX_RETRY_ON_FAIL_WA>
```
## Step to deploy
Follow every step on step to build
1. cd target/x86_64-unknown-linux-musl/release
1. cp covid-unified-gateway exec
1. zip exec.zip <PATH_TO_DOTENV_FILE> exec
1. ibmcloud fn action update <YOUR_FUNCTION_NAME> ./exec.zip --native -t 15000 -m 128
Your cloud function shall be deployed as <YOUR_FUNCTION_NAME> on IBM Cloud Functions namespace.
If it fail, consider read the doc on how to login using IBM Cloud CLI and how to set Functions namespace with CLI.