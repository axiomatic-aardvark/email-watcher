use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
struct Folder {
    name: String,
    id: u32,
    #[serde(rename = "isDef")]
    is_def: bool,
    #[serde(rename = "newMsgCount")]
    new_msg_count: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    status: u32,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mail {
    folders: Vec<Folder>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MailInfo {
    message: Message,
    mail: Mail,
}

impl MailInfo {
    async fn get() -> Self {
        let username = env::var("USERNAME").expect("Username must be set");
        let password = env::var("PASSWORD").expect("Password must be set");

        let url = format!(
            "https://api.abv.bg/api/checkMail/json?username={}&password={}",
            username, password
        );
        let url = Url::parse(&*url).unwrap();

        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<MailInfo>()
            .await
            .unwrap();

        response
    }
}

#[derive(Debug)]
enum LED {
    On,
    Off,
}

#[tokio::main]
async fn main() {
    let mut led = LED::Off;

    let mail_info_response = MailInfo::get().await;

    if mail_info_response.message.status != 0 {
        panic!("A fatal error occurred:  {:?}", mail_info_response.message.text)
    }

    let mut last_msg_count = mail_info_response.mail.folders[0].new_msg_count;
    sleep(Duration::from_secs(10));

    loop {
        let mail_info_response = MailInfo::get().await;

        if mail_info_response.message.status == 103 {
            panic!("A fatal error occurred {:?}", mail_info_response.message.text)
        } else {
            let msg_count = mail_info_response.mail.folders[0].new_msg_count;
            println!("debug {}", msg_count);

            if msg_count > last_msg_count {
                led = LED::On;
                last_msg_count = msg_count;
            }
        }

        println!("{:?}", led);

        sleep(Duration::from_secs(10));
        led = LED::Off;
    }
}
