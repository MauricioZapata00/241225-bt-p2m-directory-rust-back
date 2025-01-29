use rocket::serde::{Deserialize as RocketDeserialize, Serialize as RocketSerialize};


#[derive(Debug, Clone, RocketSerialize, RocketDeserialize)]
pub struct AccountDto {
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    #[serde(rename = "bankCode")]
    pub bank_code: String,
}

impl AccountDto {
    pub fn new(account_number: String, bank_code: String) -> Self {
        Self {
            account_number,
            bank_code,
        }
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for AccountDto {
//     type Error = String;
//
//     async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
//         // First check content type
//         match request.content_type() {
//             Some(ct) if ct.is_json() => {
//                 // We can't read the body here because FromRequest doesn't have access to Data
//                 // Instead, we should return Forward to let Json<AccountDto> handle it
//                 Outcome::Success(Self)
//             }
//             Some(_) => Outcome::Error((
//                 Status::UnsupportedMediaType,
//                 "Content-Type must be application/json".to_string(),
//             )),
//             None => Outcome::Error((
//                 Status::BadRequest,
//                 "Content-Type header missing".to_string(),
//             )),
//         }
//     }
// }