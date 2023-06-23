use actix_web::web;
use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
pub struct ResponseStatus {
    pub status_code: u8,
	pub status_description: String,
}

pub fn create_mpesa_access_token(
    data: &web::Data<Pool>,
    access_token: String,
    expires_in: u32,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> bool {
    let mut successful: bool = false;

    match data.get_conn().and_then(|mut conn| {
        insert_update_mpesa_access_token(
            &mut conn,
            access_token,
            expires_in,
            date_to_mpesa,
            date_from_mpesa,
        )
    }) {
        Ok(x) => {
            successful = true;
        }
        Err(e) => println!(
            "Failed to open DB connection. create_mpesa_access_token {:?}",
            e
        ),
    }

    successful
}

pub fn create_mpesa_register_url(
    data: &web::Data<Pool>,
    originator_coversation_id: String,
    conversation_id: String,
    response_description: String,
    business_short_code: String,
    confirmation_url: String,
    validation_url: String,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> bool {
    let mut successful: bool = false;

    match data.get_conn().and_then(|mut conn| {
        insert_mpesa_register_url(
            &mut conn,
            originator_coversation_id,
            conversation_id,
            response_description,
            business_short_code,
            confirmation_url,
            validation_url,
            date_to_mpesa,
            date_from_mpesa,
        )
    }) {
        Ok(x) => {
            successful = true;
        }
        Err(e) => println!(
            "Failed to open DB connection. create_mpesa_access_token {:?}",
            e
        ),
    }

    successful
}

pub fn create_incoming_c2b_mpesa_validation_requests(
    data: &web::Data<Pool>,
    transaction_type: String,
    trans_id: String,
    trans_time: String,
    trans_amount: String,
    business_short_code: String,
    bill_ref_number: String,
    invoice_number: String,
    org_account_balance: String,
    third_party_trans_id: String,
    _msisdn: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    bill_type: String,
) -> ResponseStatus {
    let response_code: u8 = 1;
    let response_message: String = String::from("Error occured during processing, please try again.");
    
    let mut response_status = ResponseStatus {
        status_code: response_code,
        status_description: response_message,
    };

    match data.get_conn().and_then(|mut conn| {
        insert_incoming_c2b_mpesa_validation_requests(
            &mut conn,
            transaction_type,
            trans_id,
            trans_time,
            trans_amount,
            business_short_code,
            bill_ref_number,
            invoice_number,
            org_account_balance,
            third_party_trans_id,
            _msisdn,
            first_name,
            middle_name,
            last_name,
            bill_type,
        )
    }) {
        Ok(x) => {
            response_status.status_code = x.status_code;
            response_status.status_description = x.status_description;
        }
        Err(e) => println!(
            "Failed to open DB connection. create_incoming_c2b_mpesa_validation_requests {:?}",
            e
        ),
    }

    response_status
}

pub fn create_incoming_c2b_mpesa_confirmation_requests(
    data: &web::Data<Pool>,
    transaction_type: String,
    trans_id: String,
    trans_time: String,
    trans_amount: String,
    business_short_code: String,
    bill_ref_number: String,
    invoice_number: String,
    org_account_balance: String,
    third_party_trans_id: String,
    _msisdn: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    bill_type: String,
) -> ResponseStatus {
    let response_code: u8 = 1;
    let response_message: String = String::from("Error occured during processing, please try again.");
    
    let mut response_status = ResponseStatus {
        status_code: response_code,
        status_description: response_message,
    };

    match data.get_conn().and_then(|mut conn| {
        insert_incoming_c2b_mpesa_confirmation_requests(
            &mut conn,
            transaction_type,
            trans_id,
            trans_time,
            trans_amount,
            business_short_code,
            bill_ref_number,
            invoice_number,
            org_account_balance,
            third_party_trans_id,
            _msisdn,
            first_name,
            middle_name,
            last_name,
            bill_type,
        )
    }) {
        Ok(x) => {
            response_status.status_code = x.status_code;
            response_status.status_description = x.status_description;
        }
        Err(e) => println!(
            "Failed to open DB connection. create_incoming_c2b_mpesa_confirmation_requests {:?}",
            e
        ),
    }

    response_status
}


pub fn get_mpesa_access_token(data: &web::Data<Pool>) -> String {
    let mut access_token: String = String::from("");

    match data
        .get_conn()
        .and_then(|mut conn| select_mpesa_access_token_details(&mut conn))
    {
        Ok(x) => {
            access_token = x;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    access_token
}

pub fn get_settings_details(data: &web::Data<Pool>, param_key: String) -> String {
    let mut param_value: String = String::from("");

    if param_key.len() == 0 {
        return param_value;
    }

    let param_key = param_key.to_lowercase();

    match data
        .get_conn()
        .and_then(|mut conn| select_settings_details(&mut conn, param_key.to_string()))
    {
        Ok(x) => {
            param_value = x;
        }
        Err(e) => println!("Failed to open DB connection. {:?}", e),
    }

    param_value
}

fn insert_update_mpesa_access_token(
    conn: &mut PooledConn,
    access_token: String,
    expires_in: u32,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> std::result::Result<u8, mysql::error::Error> {
    conn.exec_drop(
        "call insertupdatempesaaccesstokentest (:myaccesstoken, :myexpiresin, :mydatetompesa, :mydatefrommpesa);",
        params! {
            "myaccesstoken" => access_token,
            "myexpiresin" => expires_in,
            "mydatetompesa" => date_to_mpesa,
			"mydatefrommpesa" => date_from_mpesa,
        },
    )
	.and_then(|_| Ok(1))
}

fn insert_mpesa_register_url(
    conn: &mut PooledConn,
    originator_coversation_id: String,
    conversation_id: String,
    response_description: String,
    business_short_code: String,
    confirmation_url: String,
    validation_url: String,
    date_to_mpesa: String,
    date_from_mpesa: String,
) -> std::result::Result<u8, mysql::error::Error> {
    conn.exec_drop(
        "call insertmpesaregisterurltest (:myoriginatorcoversationid, :myconversationid, :myresponsedescription, :mybusinessshortcode, :myconfirmationurl, :myvalidationurl, :mydatetompesa, :mydatefrommpesa);",
        params! {
            "myoriginatorcoversationid" => originator_coversation_id,
            "myconversationid" => conversation_id,
            "myresponsedescription" => response_description,
            "mybusinessshortcode" => business_short_code,
            "myconfirmationurl" => confirmation_url,
            "myvalidationurl" => validation_url,
            "mydatetompesa" => date_to_mpesa,
			"mydatefrommpesa" => date_from_mpesa,
        },
    )
	.and_then(|_| Ok(1))
}

fn insert_incoming_c2b_mpesa_validation_requests(
    conn: &mut PooledConn,
    transaction_type: String,
    trans_id: String,
    trans_time: String,
    trans_amount: String,
    business_short_code: String,
    bill_ref_number: String,
    invoice_number: String,
    org_account_balance: String,
    third_party_trans_id: String,
    _msisdn: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    bill_type: String,
) -> std::result::Result<ResponseStatus, mysql::error::Error> {
    let response_code: u8 = 1;
    let response_message: String = String::from("Error occured during processing, please try again.");
    
    let mut response_status = ResponseStatus {
        status_code: response_code,
        status_description: response_message,
    };
    conn.exec_map(
        "call insertincomingc2bmpesavalidationrequeststest (:mytransactiontype, :mytransid, :mytranstime, :mytransamount, :mybusinessshortcode, :mybillrefnumber, :myinvoicenumber, :myorgaccountbalance, :mythirdpartytransid, :mymsisdn, :myfirstname, :mymiddlename, :mylastname, :mybilltype, :responsecode, :responsemessage);",
		params! {
            "mytransactiontype" => transaction_type,
            "mytransid" => trans_id,
            "mytranstime" => trans_time,
            "mytransamount" => trans_amount,
            "mybusinessshortcode" => business_short_code,
            "mybillrefnumber" => bill_ref_number,
            "myinvoicenumber" => invoice_number,
            "myorgaccountbalance" => org_account_balance,
            "mythirdpartytransid" => third_party_trans_id,
            "mymsisdn" => _msisdn,
            "myfirstname" => first_name,
            "mymiddlename" => middle_name,
            "mylastname" => last_name,
            "mybilltype" => bill_type,
            "responsecode" => 1, // output param
            "responsemessage" => String::from(""), // output param
        },
        |(responsecode, responsemessage)| {
            response_status = 
            ResponseStatus {
				status_code: responsecode,
				status_description: responsemessage,
			};
        },
        )
	.and_then(|_| Ok(response_status))
}

fn insert_incoming_c2b_mpesa_confirmation_requests(
    conn: &mut PooledConn,
    transaction_type: String,
    trans_id: String,
    trans_time: String,
    trans_amount: String,
    business_short_code: String,
    bill_ref_number: String,
    invoice_number: String,
    org_account_balance: String,
    third_party_trans_id: String,
    _msisdn: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    bill_type: String,
) -> std::result::Result<ResponseStatus, mysql::error::Error> {
    let response_code: u8 = 1;
    let response_message: String = String::from("Error occured during processing, please try again.");
    
    let mut response_status = ResponseStatus {
        status_code: response_code,
        status_description: response_message,
    };
    conn.exec_map(
        "call insertincomingc2bmpesaconfirmationrequeststest (:mytransactiontype, :mytransid, :mytranstime, :mytransamount, :mybusinessshortcode, :mybillrefnumber, :myinvoicenumber, :myorgaccountbalance, :mythirdpartytransid, :mymsisdn, :myfirstname, :mymiddlename, :mylastname, :mybilltype, :responsecode, :responsemessage);",
		params! {
            "mytransactiontype" => transaction_type,
            "mytransid" => trans_id,
            "mytranstime" => trans_time,
            "mytransamount" => trans_amount,
            "mybusinessshortcode" => business_short_code,
            "mybillrefnumber" => bill_ref_number,
            "myinvoicenumber" => invoice_number,
            "myorgaccountbalance" => org_account_balance,
            "mythirdpartytransid" => third_party_trans_id,
            "mymsisdn" => _msisdn,
            "myfirstname" => first_name,
            "mymiddlename" => middle_name,
            "mylastname" => last_name,
            "mybilltype" => bill_type,
            "responsecode" => 1, // output param
            "responsemessage" => String::from(""), // output param
        },
        |(responsecode, responsemessage)| {
            response_status = 
            ResponseStatus {
				status_code: responsecode,
				status_description: responsemessage,
			};
        },
        )
	.and_then(|_| Ok(response_status))
}

fn select_mpesa_access_token_details(
    conn: &mut PooledConn,
) -> std::result::Result<String, mysql::error::Error> {
    let mut access_token: String = String::from("");

    conn.exec_map(
        "call getmpesaaccesstokentest(:myaccesstoken);",
        params! {
            "myaccesstoken" => String::from(""), // output param
        },
        |myaccesstoken|
        access_token = myaccesstoken,
    )
	.and_then(|_| Ok(access_token))
}

fn select_settings_details(
    conn: &mut PooledConn,
    param_key: String,
) -> std::result::Result<String, mysql::error::Error> {
    let mut param_value: String = String::from("");

    conn.exec_map(
        "call getsettings(:paramkey, :paramvalue);",
        params! {
            "paramkey" => param_key,
            "paramvalue" => String::from(""), // output param
        },
        |paramvalue|
        param_value = paramvalue,
    )
	.and_then(|_| Ok(param_value))
}

