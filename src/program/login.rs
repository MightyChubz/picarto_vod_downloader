use crate::program::m3u8_grabber::Sender;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty, Value};
use std::fs::{read_to_string, write};
use std::path::Path;

const LOGIN_PATH: &str = "login.json";
const PICARTO_AUTHORIZE_URL: &str = "https://oauth.picarto.tv/authorize";
const PICARTO_TOKEN_URL: &str = "https://oauth.picarto.tv/token";

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub client_id: String,
    pub client_secret: String,
    pub state: String,
}

impl Login {
    pub fn login(&self, sender: Sender) -> String {
        // Picarto's auth setup is frustrating.... xp
        // TODO: If continued development is done, I need to figure out how to send this request without the page expiring.
        let auth_response = sender.post(
            PICARTO_AUTHORIZE_URL,
            &[
                ("client_id", &self.client_id),
                ("response_type", "code"),
                ("scope", "readpub readpriv"),
                (
                    "redirect_uri",
                    "https://api.picarto.tv/oauth-eg/redirect.php",
                ),
                ("state", &self.state),
            ],
        );
        let auth_url = auth_response.url();
        debug!("Query: {}", auth_url.query().unwrap());
        debug!("Body: {}", auth_response.text().unwrap());

        unimplemented!()
        // let auth_token = auth_url
        //     .query_pairs()
        //     .find(|e| e.0 == "code")
        //     .map(|e| e.1.to_string())
        //     .unwrap();
        //
        // let access: Value = sender
        //     .post(
        //         PICARTO_TOKEN_URL,
        //         &[
        //             ("grant_type", "authorization_code"),
        //             ("code", &auth_token),
        //             ("client_id", &self.client_id),
        //             ("client_secret", &self.client_secret),
        //             ("state", &self.state),
        //         ],
        //     )
        //     .json()
        //     .unwrap();
        // access["access_token"].to_string()
    }

    pub fn empty(&self) -> bool {
        self.client_secret.is_empty() || self.client_id.is_empty() || self.state.is_empty()
    }
}

impl Default for Login {
    fn default() -> Self {
        if Path::new(LOGIN_PATH).exists() {
            debug!("Loading {}...", LOGIN_PATH);
            from_str(&read_to_string(LOGIN_PATH).unwrap()).unwrap()
        } else {
            debug!("{} does not exist...", LOGIN_PATH);
            debug!("Creating {} with default params...", LOGIN_PATH);
            let login = Self {
                client_id: String::new(),
                client_secret: String::new(),
                state: String::new(),
            };

            write(LOGIN_PATH, to_string_pretty(&login).unwrap()).unwrap();

            login
        }
    }
}
