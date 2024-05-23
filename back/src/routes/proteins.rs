use crate::auth::auth_guard::AuthenticatedUser;

#[get("/protected")]
pub fn protected_route(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", user.username)
}
