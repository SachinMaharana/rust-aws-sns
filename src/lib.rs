use rusoto_core::Region;
use rusoto_sns::{MessageAttributeValue, PublishInput, PublishResponse, Sns, SnsClient};
use std::{borrow::Cow, fmt};
use std::{collections::HashMap, env};

// Type of SMS delivery
pub enum SmsType {
    // Promotional are non-critical messages, such as marketing messages.
    // Amazon SNS optimizes the message delivery to incur the lowest cost.
    Promotional,
    // Transactional messages are critical messages that support
    // customer transactions, such as one-time passcodes for multi-factor authentication.
    // Amazon SNS optimizes the message delivery to achieve the highest reliability.
    Transactional,
}

impl SmsType {
    fn value(&self) -> String {
        match *self {
            SmsType::Transactional => "Transactional".into(),
            SmsType::Promotional => "Promotional".into(),
        }
    }
}

// SMS configures an SNS SMS client.
pub struct SMS<'a> {
    pub sms_type: SmsType,
    pub sender_id: Cow<'a, str>,
    pub max_price: f64,
}

// Defaults.
impl<'a> Default for SMS<'a> {
    fn default() -> Self {
        SMS {
            sms_type: SmsType::Transactional,
            sender_id: "".into(),
            max_price: 0.01,
        }
    }
}

impl<'a> SMS<'a> {
    fn get_client(&self) -> anyhow::Result<SnsClient> {
        let aws_region = get_aws_region()?;
        let client = SnsClient::new(aws_region.parse::<Region>()?);
        Ok(client)
    }

    // Send `message` to `number`.
    pub async fn send<'b, S>(&self, message: S, phone_number: S) -> anyhow::Result<PublishResponse>
    where
        S: Into<Cow<'b, str>>,
    {
        verify_credentials()?;

        let mut attrs: HashMap<String, MessageAttributeValue> = HashMap::new();

        if self.sender_id != "" {
            attrs.insert(
                "AWS.SNS.SMS.SenderID".into(),
                rusoto_sns::MessageAttributeValue {
                    data_type: "String".to_string(),
                    string_value: Some(self.sender_id.to_string()),
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
            message: message.into().to_string(),
            phone_number: Some(phone_number.into().to_string()),
            message_attributes: Some(attrs),
            ..Default::default()
        };

        let client = self.get_client()?;
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
