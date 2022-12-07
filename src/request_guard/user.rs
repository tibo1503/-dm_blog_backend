use rocket::request::{Request, FromRequest, Outcome};
use crate::token_const;

pub struct User;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_token = req.cookies().get(token_const::TOKEN_COOKIE_FIELD);

        match user_token {
            Some(x) => {
                if x.value() == token_const::TOKEN {
                    Outcome::Success(User)
                } else {
                    Outcome::Forward(())
                }
            },
            _ => Outcome::Forward(())
        }
    }
}