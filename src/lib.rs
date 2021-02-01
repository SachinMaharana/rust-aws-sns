use rusoto_core::Region;
use rusoto_sns::{MessageAttributeValue, PublishInput, PublishResponse, Sns, SnsClient};
use std::fmt;
use std::{collections::HashMap, env};
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
    pub async fn send(
        &self,
        message: String,
        phone_number: String,
    ) -> anyhow::Result<PublishResponse> {
        verify_credentials()?;
        let aws_region = get_aws_region()?;

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

#[derive(Debug)]
pub enum Error {
    MissingCredential(Credential),
}

#[derive(Debug)]
pub enum Credential {
    AwsAccessKeyId,
    AwsSecretAccessKey,
    AwsRegion,
    All,
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingCredential(Credential::AwsRegion) => {
                write!(f, "AWS_REGION env var is required.")
            }
            Error::MissingCredential(Credential::AwsAccessKeyId) => {
                write!(f, "AWS_ACCESS_KEY_ID env var is required.")
            }
            Error::MissingCredential(Credential::AwsSecretAccessKey) => {
                write!(f, "AWS_SECRET_ACCESS_KEY env var is required.")
            }
            Error::MissingCredential(Credential::All) => {
                write!(
                    f,
                    "AWS_SECRET_ACCESS_KEY, AWS_REGION, AWS_ACCESS_KEY_ID env var is required."
                )
            }
        }
    }
}

fn verify_credentials() -> Result<(), Error> {
    let access_key = env::var("AWS_ACCESS_KEY_ID").ok();
    let secret_key = env::var("AWS_SECRET_ACCESS_KEY").ok();

    match (access_key, secret_key) {
        (Some(_), Some(_)) => Ok(()),
        (Some(_), None) => Err(Error::MissingCredential(Credential::AwsSecretAccessKey)),
        (None, Some(_)) => Err(Error::MissingCredential(Credential::AwsAccessKeyId)),
        (None, None) => Err(Error::MissingCredential(Credential::All)),
    }
}
fn get_aws_region() -> Result<String, Error> {
    let region = env::var("AWS_REGION").ok();

    match region {
        Some(region) => Ok(region),
        None => Err(Error::MissingCredential(Credential::AwsRegion)),
    }
}
