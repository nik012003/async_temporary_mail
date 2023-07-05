use chrono::NaiveDateTime;
use rand::seq::SliceRandom;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Deserializer};
use std::error::Error;
use std::fmt;

const API_URL: &str = "https://www.1secmail.com/api/v1/";

#[derive(Debug)]

pub struct TempEmail {
    email_adress: EmailAddr,
    client: reqwest::Client,
}

impl TempEmail {
    /// Creates a temporary email
    ///
    /// # Example
    ///
    /// ```
    /// let temp_mail = temporary_mail::TempEmail::new();
    /// ```
    pub fn new() -> Self {
        let mail = EmailAddr::new();
        Self::from_email_addr(mail)
    }
    /// Creates a temporary email from a given EmailAddr
    ///
    /// # Example
    ///
    /// ```
    /// let temp_mail = temporary_mail::TempEmail::new();
    /// ```
    pub fn from_email_addr(mail: EmailAddr) -> Self {
        let client = reqwest::Client::new();

        TempEmail {
            email_adress: mail,
            client,
        }
    }
    /// Return the address of the `TempEmail`
    ///
    /// # Example
    ///
    /// ```
    /// let temp_mail = temporary_mail::TempEmail::new();
    /// println!("{}", temp_mail.get_address());  // abc@host.com
    /// ```
    pub fn get_address(&self) -> EmailAddr {
        self.email_adress.clone()
    }

    /// Return a vector of received Emails
    ///
    /// # Example
    /// ```ignore
    /// let temp_mail = temporary_mail::TempEmail::new();
    /// if let Ok(mails) = temp_mail.get_inbox().await {
    ///     mails.iter().for_each(|mail| println!("{:?}", mail));
    /// }
    /// ```
    pub async fn get_inbox(&self) -> Result<Vec<Email>, Box<dyn Error + Send + Sync>> {
        let res: serde_json::Value = self
            .client
            .get(API_URL)
            .query(&[
                ("action", "getMessages"),
                ("login", &self.email_adress.user),
                ("domain", &self.email_adress.domain),
            ])
            .send()
            .await?
            .json()
            .await?;

        let ids = res
            .as_array()
            .ok_or("Can't convert to array")?
            .iter()
            .filter_map(|e| e.get("id"))
            .filter_map(|i| i.as_i64())
            .collect::<Vec<_>>();

        let mut inbox = Vec::new();

        for id in ids {
            let email: Email = self
                .client
                .get(API_URL)
                .query(&[
                    ("action", "readMessage"),
                    ("login", &self.email_adress.user),
                    ("domain", &self.email_adress.domain),
                    ("id", &id.to_string()),
                ])
                .send()
                .await?
                .json()
                .await?;

            inbox.push(email);
        }

        Ok(inbox)
    }
}

#[derive(Debug, Clone)]
pub struct EmailAddr {
    user: String,
    domain: String,
}

impl EmailAddr {
    fn new() -> Self {
        let user = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let domain = ["1secmail.com", "1secmail.net", "1secmail.org"]
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string();

        EmailAddr { user, domain }
    }
}

impl fmt::Display for EmailAddr {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}@{}", self.user, self.domain)
    }
}
#[derive(Debug, Deserialize)]
pub struct Email {
    pub from: String,
    #[serde(deserialize_with = "Email::date_from_string")]
    pub date: NaiveDateTime,
    pub id: i64,
    pub subject: String,
    #[serde(alias = "textBody")]
    pub text_body: String,
    #[serde(alias = "htmlBody")]
    pub html_body: String,
    pub body: String,
}

impl Email {
    fn date_from_string<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDateTime, D::Error> {
        let date_string: String = Deserialize::deserialize(d)?;
        Ok(NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn email_from_json() {
        let email: Result<super::Email, _> = serde_json::from_str(
            r#"{ 
            "from": "user@gmail.com",
            "date": "2022-02-23 12:50:18",
            "id": 29471701,
            "subject": "HI!",
            "text_body": "Hi, buddy",
            "html_body": "<div dir=\"ltr\">Hi, buddy</div>\n",
            "body": "<div dir=\"ltr\">Hi, buddy</div>\n" }"#,
        );
        assert!(email.is_ok());
    }
    #[test]
    fn api_responsive() {
        let client = reqwest::blocking::Client::new();
        let res = client.get(super::API_URL).send();
        assert!(res.is_ok());
    }
}
