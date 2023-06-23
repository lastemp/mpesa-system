use actix_web::web;
use chrono::prelude::*;
use mysql::Pool;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct RegisterUrlData {
    ShortCode: String,
    ResponseType: String,
    ConfirmationURL: String,
    ValidationURL: String,
}

#[derive(Deserialize, Debug)]
struct AuthTokenResponseData {
    access_token: Option<String>,
    expires_in: Option<String>,
}

#[derive(Deserialize, Debug)]
struct RegisterUrlResponseData {
    OriginatorCoversationID: Option<String>,
    ConversationID: Option<String>,
    ResponseDescription: Option<String>,
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

pub async fn generate_auth_token(
    data: web::Data<Pool>,
    api_key: String,
    api_url: String,
) -> std::result::Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .get(api_url)
        .header(CONTENT_TYPE, "text/plain")
        .header(ACCEPT, "application/json")
        .header("Authorization", api_key)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<AuthTokenResponseData>().await?;

                    let access_token = &my_output.access_token.as_ref().unwrap_or(&k);
                    let expires_in = &my_output.expires_in.as_ref().unwrap_or(&k);

                    let expires_in: u32 = match expires_in.parse::<u32>() {
                        Ok(a) => a,
                        Err(e) => 0,
                    };

                    crate::db_layer::create_mpesa_access_token(
                        &data,
                        access_token.to_string(),
                        expires_in,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(())
}

pub async fn register_url(
    data: web::Data<Pool>,
    register_url_details: RegisterUrlInputDetails,
) -> std::result::Result<(), reqwest::Error> {
    let access_token: String = register_url_details.access_token;
    let api_url: String = register_url_details.api_url;
    let short_code: String = register_url_details.short_code;
    let response_type: String = register_url_details.response_type;
    let confirmation_url: String = register_url_details.confirmation_url;
    let validation_url: String = register_url_details.validation_url;

    let register_url_data = RegisterUrlData {
        ShortCode: short_code,
        ResponseType: response_type,
        ConfirmationURL: confirmation_url,
        ValidationURL: validation_url,
    };
    let client = reqwest::Client::new();
    // "%Y-%m-%d %H:%M:%S" i.e "yyyy-MM-dd HH:mm:ss"
    // "%Y-%m-%d %H:%M:%S%.3f" i.e "yyyy-MM-dd HH:mm:ss.SSS"
    let date_to_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    let res = client
        .post(api_url)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .header("Authorization", access_token)
        .json(&register_url_data)
        .send()
        //.await?; //The "?" after the await returns errors immediately and hence will not be captured on match clause below
        .await;

    match res {
        Err(e) => {
            println!("server not responding");
        }
        Ok(response) => {
            match response.status() {
                StatusCode::OK => {
                    let date_from_mpesa = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
                    let k = String::from(""); //Default value.

                    let my_output = response.json::<RegisterUrlResponseData>().await?;

                    let originator_coversation_id =
                        &my_output.OriginatorCoversationID.as_ref().unwrap_or(&k);
                    let conversation_id = &my_output.ConversationID.as_ref().unwrap_or(&k);
                    let response_description =
                        &my_output.ResponseDescription.as_ref().unwrap_or(&k);

                    crate::db_layer::create_mpesa_register_url(
                        &data,
                        originator_coversation_id.to_string(),
                        conversation_id.to_string(),
                        response_description.to_string(),
                        register_url_data.ShortCode,
                        register_url_data.ConfirmationURL,
                        register_url_data.ValidationURL,
                        date_to_mpesa,
                        date_from_mpesa,
                    );
                }
                s => println!("Received response status: {:?}", s),
            }
        }
    };

    Ok(())
}
