use anyhow::Result;
use rusoto_core::Region;
use rusoto_sns::{MessageAttributeValue, PublishInput, PublishResponse, Sns, SnsClient};
use std::{collections::HashMap, process};

pub enum SmsType {
    Promotional,
    Transactional,
}

impl SmsType {
    fn value(&self) -> String {
        match *self {
            SmsType::Promotional => "Promotional".into(),
            SmsType::Transactional => "Transactional".into(),
        }
    }
}

pub struct SMS {
    pub sms_type: SmsType,
    pub sender_id: String,
    pub max_price: f64,
}

impl Default for SMS {
    fn default() -> Self {
        SMS {
            sms_type: SmsType::Transactional,
            sender_id: "".into(),
            max_price: 0.01,
        }
    }
}

impl SMS {
    pub async fn send(&self, message: String, phone_number: String) -> Result<PublishResponse> {
        let aws_region = std::env::var("AWS_REGION").unwrap_or_else(|_| {
            println!(
                "Error: AWS_REGION env variable is required. \nTIP: export AWS_REGION=us-east-1"
            );
            process::exit(1);
        });
        let _aws_key = std::env::var("AWS_SECRET_ACCESS_KEY").unwrap_or_else(|_| {
            println!(
                "Error: AWS_SECRET_ACCESS_KEY env variable is required. \nTIP: export AWS_SECRET_ACCESS_KEY=xxxxx"
            );
            process::exit(1);
        });
        let _aws_access = std::env::var("AWS_ACCESS_KEY_ID").unwrap_or_else(|_| {
            println!(
                "Error: AWS_ACCESS_KEY_ID env variable is required. \nTIP: export AWS_ACCESS_KEY_ID=xxxxx"
            );
            process::exit(1);
        });

        let mut attrs: HashMap<String, MessageAttributeValue> = HashMap::new();

        if self.sender_id != "" {
            attrs.insert(
                "AWS.SNS.SMS.SenderID".into(),
                rusoto_sns::MessageAttributeValue {
                    data_type: "String".to_string(),
                    string_value: Some(self.sender_id.clone()),
                    binary_value: None,
                },
            );
        }

        attrs.insert(
            "AWS.SNS.SMS.MaxPrice".into(),
            rusoto_sns::MessageAttributeValue {
                data_type: "String".to_string(),
                string_value: Some(self.max_price.to_string()),
                binary_value: None,
            },
        );

        attrs.insert(
            "AWS.SNS.SMS.SMSType".into(),
            rusoto_sns::MessageAttributeValue {
                data_type: "String".to_string(),
                string_value: Some(self.sms_type.value()),
                binary_value: None,
            },
        );

        let params = PublishInput {
            message: message.into(),
            phone_number: Some(phone_number.into()),
            message_attributes: Some(attrs),
            ..Default::default()
        };

        let client = SnsClient::new(aws_region.parse::<Region>()?);
        Ok(client.publish(params).await?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
