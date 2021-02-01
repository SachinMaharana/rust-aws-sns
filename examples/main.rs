use rust_aws_sns::{SmsType, SMS};

#[tokio::main]
async fn main() {
    let s = SMS {
        ..Default::default()
    };
    let res = s.send("hello".into(), "91xxxxxx15xx".into()).await;
    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }
}
