mod sns;

#[tokio::main]
async fn main() {
    let s = sns::SMS {
        ..Default::default()
    };
    let res = s.send("Sachin M work".into(), "917559981534".into()).await;
    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }
}
