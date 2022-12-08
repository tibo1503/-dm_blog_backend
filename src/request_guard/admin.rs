use rocket::request::{Request, FromRequest, Outcome};

pub struct Admin;

pub const TOKEN: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
pub const TOKEN_COOKIE_FIELD: &str = "token";

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = std::convert::Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user_token = req.cookies().get(TOKEN_COOKIE_FIELD);

        match user_token {
            Some(x) => {
                if x.value() == TOKEN {
                    Outcome::Success(Admin)
                } else {
                    Outcome::Forward(())
                }
            },
            _ => Outcome::Forward(())
        }
    }
}