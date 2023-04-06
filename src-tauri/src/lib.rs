pub mod pb;

pub use pb::sign;

use sign::sign_client::SignClient;
use sign::LoginReq;
use sign::SignUpReq;

pub const ADDR: &str = "http://[2001:da8:c800:b20c:77ca:7dd0:6e36:168d]:8848";
// pub const ADDR: &str = "http://localhost:8848";
// pub const ADDR: &str = "http://192.168.137.1:8848";

async fn create_sign_client() -> Option<SignClient<tonic::transport::Channel>> {
    let endpoint = tonic::transport::Endpoint::new(ADDR)
        .unwrap()
        .connect_timeout(std::time::Duration::from_secs(5))
        .timeout(std::time::Duration::from_secs(5));
    let channel = match endpoint.connect().await {
        Ok(channel) => channel,
        Err(_) => {
            println!("Faild connect to server.");
            return None;
        }
    };
    Some(SignClient::new(channel))
}

pub async fn async_sign_in(username: &str, password: &str) -> bool {
    let mut client = match create_sign_client().await {
        Some(client) => client,
        None => return false,
    };
    let request = LoginReq {
        username: username.into(),
        password: password.into(),
    };
    if let Ok(response) = client.verify(request).await {
        println!("Response is {response:#?}");
        return response.into_inner().result;
    };
    false
}

pub async fn async_sign_up(username: &str, password: &str, email: &str) -> bool {
    let mut client = match create_sign_client().await {
        Some(client) => client,
        None => return false,
    };
    let request = SignUpReq {
        username: username.into(),
        password: password.into(),
        email: email.into(),
    };
    if let Ok(response) = client.register(request).await {
        println!("Response is {response:#?}");
        return response.into_inner().result;
    };
    false
}
