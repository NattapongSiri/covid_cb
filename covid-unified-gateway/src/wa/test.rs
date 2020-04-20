use super::*;

#[test]
fn test_create_entity_builder() {
    println!("{:?}", EntityBuilder::builder("a".to_owned(), [2, 2], "abcd".to_owned()));
}

#[test]
fn test_build_entity_with_meta() {
    #[derive(Debug, Deserialize, Serialize)]
    struct LocalMeta {
        name: &'static str
    }
    println!("{:?}", 
        EntityBuilder::builder("b".to_owned(), [2, 2], "bbbb".to_owned())
                    .metadata(LocalMeta {name: "me"})
                    .confidence(0.5f32)
    );
}

#[test]
fn test_build_context() {
    #[derive(Debug, Serialize)]
    struct MyContext {
        name: &'static str
    }
    let ctx = ContextBuilder::builder()
                        .timezone("Asia/Bangkok".to_owned())
                        .user_defined(MyContext {name: "whoami"});
    println!("{:?}", ctx)
}

#[test]
fn test_build_user_input() {
    let ui = UserInputBuilder::builder()
                        .message_type(InputType::Text)
                        .text("yo")
                        .entities(vec![EntityBuilder::builder("unknown".to_owned(), [2, 2], "yo".to_owned()).build()]);
    println!("{:?}", ui)
}

#[test]
fn test_create_close_session() -> Result<(), CurlErr> {
    dotenv::dotenv().unwrap();
    let endpoint = std::env::var("WA_ENDPOINT").expect("Fail to find WA_ENDPOINT from environment variable");
    let id = std::env::var("WA_ID").expect("Fail to find WA_ID from environment variable");
    let api_key = std::env::var("WA_APIKEY").expect("Fail to find WA_APIKEY from environment variable");
    let version = std::env::var("WA_VERSION").expect("Fail to find WA_VERSION from environment variable");
    
    futures::executor::block_on(async {
        let session = WASession::new(endpoint, api_key, id, version).await?;
        session.close().await?;
        Ok(())
    })
}

#[test]
fn test_create_send_session() -> Result<(), CurlErr> {
    dotenv::dotenv().unwrap();
    let endpoint = std::env::var("WA_ENDPOINT").expect("Fail to find WA_ENDPOINT from environment variable");
    let id = std::env::var("WA_ID").expect("Fail to find WA_ID from environment variable");
    let api_key = std::env::var("WA_APIKEY").expect("Fail to find WA_APIKEY from environment variable");
    let version = std::env::var("WA_VERSION").expect("Fail to find WA_VERSION from environment variable");
    let msg = UserInputBuilder::builder()
                                    .text("greeting !")
                                    .build();
    futures::executor::block_on(async {
        let session = WASession::new(endpoint, api_key, id, version).await?;
        let result: SimpleWAResponse = session.send(&msg).await?;
        println!("{:?}", result);
        Ok(())
    })
}
#[test]
fn test_create_send_txt_session() -> Result<(), CurlErr> {
    dotenv::dotenv().unwrap();
    let endpoint = std::env::var("WA_ENDPOINT").expect("Fail to find WA_ENDPOINT from environment variable");
    let id = std::env::var("WA_ID").expect("Fail to find WA_ID from environment variable");
    let api_key = std::env::var("WA_APIKEY").expect("Fail to find WA_APIKEY from environment variable");
    let version = std::env::var("WA_VERSION").expect("Fail to find WA_VERSION from environment variable");
    
    futures::executor::block_on(async {
        let session = WASession::new(endpoint, api_key, id, version).await?;
        let result: SimpleWAResponse = session.send_txt("hey there").await?;
        println!("{:?}", result);
        Ok(())
    })
}

#[test]
fn test_create_reattach_session() -> Result<(), CurlErr> {
    dotenv::dotenv().unwrap();
    let endpoint = std::env::var("WA_ENDPOINT").expect("Fail to find WA_ENDPOINT from environment variable");
    let id = std::env::var("WA_ID").expect("Fail to find WA_ID from environment variable");
    let api_key = std::env::var("WA_APIKEY").expect("Fail to find WA_APIKEY from environment variable");
    let version = std::env::var("WA_VERSION").expect("Fail to find WA_VERSION from environment variable");
    let msg = UserInputBuilder::builder()
                                    .text("greeting !")
                                    .build();
    futures::executor::block_on(async {
        let session = WASession::new(endpoint.to_owned(), api_key.to_owned(), id.to_owned(), version.to_owned()).await?;
        let another_session = WASession::re_attach(endpoint, api_key, id, version, session.session_id);
        let result : SimpleWAResponse = another_session.send(&msg).await?;
        println!("{:?}", result);
        Ok(())
    })
}