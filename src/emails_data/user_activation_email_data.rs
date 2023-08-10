use serde_derive::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserActivationEmailData {
    pub facebook_link: Option<String>,
    pub twitter_link: Option<String>,
    pub instagram_link: Option<String>,
    pub linked_in_link: Option<String>,
    pub first_contatct_line: Option<String>,
    pub second_contatct_line: Option<String>,
    pub account_activation_link: String,
}
