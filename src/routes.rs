extern crate base64;

use crate::api_layer::{generate_auth_token, register_url};
use crate::{
    models::{C2bData, ConfirmationResponseData, RegisterUrlInputDetails, ValidationResponseData},
    persistence::{
        create_incoming_c2b_mpesa_confirmation_requests,
        create_incoming_c2b_mpesa_validation_requests, get_mpesa_access_token,
        get_settings_details,
    },
};
use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use mysql::*;
use serde::{Deserialize, Serialize};
use std::str;

const C2B_BILL_TYPE: &str = "C2B";

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    format!("")
}

#[get("/generateauth")]
pub(crate) async fn generate_auth(data: web::Data<Pool>) -> impl Responder {
    let api_key = get_api_key(&data);
    let api_url = get_settings_details(&data, String::from("authtokenurlmpesa"));

    tokio::spawn(async move {
        // Process each request concurrently.
        generate_auth_token(data, api_key, api_url).await;
    });

    format!("")
}

#[get("/registerclienturls")]
pub(crate) async fn register_client_urls(data: web::Data<Pool>) -> impl Responder {
    let register_url_details = get_register_url_details(&data);

    tokio::spawn(async move {
        // Process each request concurrently.
        register_url(data, register_url_details).await;
    });

    format!("")
}

#[post("/validationc2b")]
pub(crate) async fn validation_c2b(
    validation_data: web::Json<C2bData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value.

    let transaction_type = &validation_data.TransactionType;
    let trans_id = &validation_data.TransID;
    let trans_time = &validation_data.TransTime;
    let trans_amount = &validation_data.TransAmount;
    let business_short_code = &validation_data.BusinessShortCode;
    let bill_ref_number = &validation_data.BillRefNumber;
    let invoice_number = &validation_data.InvoiceNumber.as_ref().unwrap_or(&k);
    let org_account_balance = &validation_data.OrgAccountBalance;
    let third_party_trans_id = &validation_data.ThirdPartyTransID;
    let _msisdn = &validation_data.MSISDN;
    let first_name = &validation_data.FirstName;
    let middle_name = &validation_data.MiddleName;
    let last_name = &validation_data.LastName;
    let bill_type = &C2B_BILL_TYPE;

    let response_status = create_incoming_c2b_mpesa_validation_requests(
        &data,
        transaction_type.to_string(),
        trans_id.to_string(),
        trans_time.to_string(),
        trans_amount.to_string(),
        business_short_code.to_string(),
        bill_ref_number.to_string(),
        invoice_number.to_string(),
        org_account_balance.to_string(),
        third_party_trans_id.to_string(),
        _msisdn.to_string(),
        first_name.to_string(),
        middle_name.to_string(),
        last_name.to_string(),
        bill_type.to_string(),
    );

    let response_data = ValidationResponseData {
        ResultCode: response_status.status_code.to_string(),
        ResultDesc: response_status.status_description,
    };

    web::Json(response_data)
}

#[post("/confirmationc2b")]
pub(crate) async fn confirmation_c2b(
    confirmation_data: web::Json<C2bData>,
    req: HttpRequest,
    data: web::Data<Pool>,
) -> impl Responder {
    let k = String::from(""); //Default value.

    let transaction_type = &confirmation_data.TransactionType;
    let trans_id = &confirmation_data.TransID;
    let trans_time = &confirmation_data.TransTime;
    let trans_amount = &confirmation_data.TransAmount;
    let business_short_code = &confirmation_data.BusinessShortCode;
    let bill_ref_number = &confirmation_data.BillRefNumber;
    let invoice_number = &confirmation_data.InvoiceNumber.as_ref().unwrap_or(&k);
    let org_account_balance = &confirmation_data.OrgAccountBalance;
    let third_party_trans_id = &confirmation_data.ThirdPartyTransID;
    let _msisdn = &confirmation_data.MSISDN;
    let first_name = &confirmation_data.FirstName;
    let middle_name = &confirmation_data.MiddleName;
    let last_name = &confirmation_data.LastName;
    let bill_type = &C2B_BILL_TYPE;

    let response_status = create_incoming_c2b_mpesa_confirmation_requests(
        &data,
        transaction_type.to_string(),
        trans_id.to_string(),
        trans_time.to_string(),
        trans_amount.to_string(),
        business_short_code.to_string(),
        bill_ref_number.to_string(),
        invoice_number.to_string(),
        org_account_balance.to_string(),
        third_party_trans_id.to_string(),
        _msisdn.to_string(),
        first_name.to_string(),
        middle_name.to_string(),
        last_name.to_string(),
        bill_type.to_string(),
    );

    let response_data = ConfirmationResponseData {
        ResultCode: response_status.status_code,
        ResultDesc: response_status.status_description,
    };

    web::Json(response_data)
}

fn get_api_key(data: &web::Data<Pool>) -> String {
    let consumer_key_mpesa = get_settings_details(&data, String::from("consumerkeympesa"));
    let consumer_secret_mpesa = get_settings_details(&data, String::from("consumersecretmpesa"));
    let mut password: String = consumer_key_mpesa;
    let k = ":"; // Separator
    password.push_str(k);
    password.push_str(&consumer_secret_mpesa);
    let encodedpassword = general_purpose::STANDARD.encode(password);

    let mut api_key = String::from("Basic");
    let k = " "; // Separator
    api_key.push_str(k);
    api_key.push_str(&encodedpassword);

    api_key
}

fn get_register_url_details(data: &web::Data<Pool>) -> RegisterUrlInputDetails {
    let mut access_token = String::from("Bearer");
    let k = " "; // Separator
    let password: String = get_mpesa_access_token(&data);
    access_token.push_str(k);
    access_token.push_str(&password);
    let api_url = get_settings_details(&data, String::from("c2bregisterurlmpesa"));
    let short_code = get_settings_details(&data, String::from("c2bregisterbusinessshortcodempesa"));
    let response_type = get_settings_details(&data, String::from("c2bregisterresponsetypempesa"));
    let confirmation_url = get_settings_details(&data, String::from("confirmationc2burlmpesa"));
    let validation_url = get_settings_details(&data, String::from("validationc2burlmpesa"));

    let register_url_details = RegisterUrlInputDetails {
        access_token: access_token,
        api_url: api_url,
        short_code: short_code,
        response_type: response_type,
        confirmation_url: confirmation_url,
        validation_url: validation_url,
    };

    register_url_details
}
