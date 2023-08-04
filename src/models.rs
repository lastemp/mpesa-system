use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct C2bData {
    pub TransactionType: String,
    pub TransID: String,
    pub TransTime: String,
    pub TransAmount: String,
    pub BusinessShortCode: String,
    pub BillRefNumber: String,
    pub InvoiceNumber: Option<String>,
    pub OrgAccountBalance: String,
    pub ThirdPartyTransID: String,
    pub MSISDN: String,
    pub FirstName: String,
    pub MiddleName: String,
    pub LastName: String,
}

#[derive(Serialize)]
pub struct ValidationResponseData {
    pub ResultCode: String,
    pub ResultDesc: String,
}

#[derive(Serialize)]
pub struct ConfirmationResponseData {
    pub ResultCode: u8,
    pub ResultDesc: String,
}

#[derive(Serialize)]
pub struct RegisterUrlData {
    pub ShortCode: String,
    pub ResponseType: String,
    pub ConfirmationURL: String,
    pub ValidationURL: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthTokenResponseData {
    pub access_token: Option<String>,
    pub expires_in: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUrlResponseData {
    pub OriginatorCoversationID: Option<String>,
    pub ConversationID: Option<String>,
    pub ResponseDescription: Option<String>,
}

// This struct holds  Register Url processing data
pub struct RegisterUrlInputDetails {
    pub access_token: String,
    pub api_url: String,
    pub short_code: String,
    pub response_type: String,
    pub confirmation_url: String,
    pub validation_url: String,
}
