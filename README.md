# rust-aws-sns

crate rust-aws-sns provides a small wrapper around AWS SNS to make SMS usage more friendly.

# Example Usage

```rust
use rust_aws_sns::{SmsType, SMS};

#[tokio::main]
async fn main() {
    let s = SMS {
        // ..Default::default() can also pass default values
        sms_type: SmsType::Transactional,
        sender_id: "".into(),
        max_price: 0.01,
    };
    let res = s.send("hello".into(), "917559981534".into()).await;
    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }
}
```

---

_Licence MIT_
