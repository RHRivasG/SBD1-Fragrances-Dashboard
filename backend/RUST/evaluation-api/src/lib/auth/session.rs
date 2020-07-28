use rocket::http::Cookies;

pub fn get_session_of(session: &str, mut cookies: Cookies) -> Option<i32> {
    cookies
        .get(session)
        .map(|id|
             id
             .value()
             .parse::<i32>()
             .ok()
        )
        .flatten()
} 
