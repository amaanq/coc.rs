// manage a session
pub const BASE_DEV_URL: &str = "https://developer.clashofclans.com/api";
const IP_URL: &str = "https://api.ipify.org";

pub fn login(username: String, password: String){
    reqwest::Client::new().post(format!("{}/login",BASE_DEV_URL));
}

