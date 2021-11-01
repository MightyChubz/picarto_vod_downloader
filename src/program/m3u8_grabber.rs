use log::Level::Info;
use m3u8_rs::parse_playlist;
use m3u8_rs::playlist::{MediaSegment, Playlist};
use reqwest::blocking::{Client, Response};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use scraper::{Html, Selector};
use serde_json::{to_string_pretty, Value};
use std::fs::write;
use std::rc::Rc;
use url::Url;

const USER_HEADER: &str = "picarto_vod_downloader 0.9.0 / MightyChubz / Contact: theradcast16@gmail.com for more information on how this program works.";

/// This class should only be initialized once and then cloned after from there on out.
/// Creating another copy of this class introduces unneeded memory allocation and an entirely new client
/// for no valid reasons.
#[derive(Clone)]
pub struct Sender {
    client: Rc<Client>, // This is Rc encase I need to clone it at all in the future.
}

impl Sender {
    pub fn new() -> Self {
        Sender {
            client: Rc::new(Client::new()),
        }
    }

    /// Sends GET request to the `url` and returns the [Response]
    ///
    /// # Arguments
    ///
    /// * `url`: A [&str] containing the url to send the get request to.
    ///
    /// returns: Response of request sent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let sender = Sender::new();
    /// let response = sender.get("https://tools.learningcontainer.com/sample-json.json");
    /// let json = response.json();
    /// ```
    pub fn get(&self, url: &str) -> Response {
        debug!("Sending url: {}", url);
        self.client
            .get(url)
            .header(USER_AGENT, USER_HEADER)
            .send()
            .expect("Sent call through client failed!")
    }

    pub fn get_with_auth(&self, url: &str, header: (&str, &str)) -> Response {
        debug!("Sending url: {}", url);
        self.client
            .get(url)
            .header(USER_AGENT, USER_HEADER)
            .header(header.0, header.1)
            .send()
            .expect("Sent call through client failed!")
    }

    pub fn post(&self, url: &str, query: &[(&str, &str)]) -> Response {
        debug!("Sending url POST method: {}", url);
        self.client
            .post(url)
            .query(query)
            .header(USER_AGENT, USER_HEADER)
            .send()
            .expect("Unable to send request!")
    }
}

/// This class combs through the html of the input url and finds the m3u8 link, which it will then
/// format and return for the m3u8 grabber to use.
pub struct Grabber {
    sender: Sender,
}

impl Grabber {
    pub fn new(sender: Sender) -> Self {
        Grabber { sender }
    }

    /// Searches for and grabs video based off of channel name and video id.
    ///
    /// # Arguments
    ///
    /// * `channel_name`: The channel name to look for
    /// * `video_id`: The video id to grab
    ///
    /// returns: String with url to the m3u8 file.
    pub fn grab(&self, channel_name: &str, video_id: &str) -> String {
        let url = format!(
            "https://api.picarto.tv/api/v1/channel/name/{}/videos",
            channel_name
        );
        let text: Value = self.sender.get(&url).json().unwrap();
        info!("Grabbed channel videos from api...");

        let entry = text
            .as_array()
            .unwrap()
            .iter()
            .find(|e| e.as_object().unwrap()["id"].to_string().as_str() == video_id.trim())
            .unwrap();

        entry["file"].to_string()
    }
}

/// This is the grabber used to grab all the needed m3u8 files, trying to locate the segments of the video.
pub struct M3U8Grabber {
    sender: Sender,
}

impl M3U8Grabber {
    pub fn new(sender: Sender) -> Self {
        M3U8Grabber { sender }
    }

    /// This is a function that handles most of the manual calls needed, finding the master playlist
    /// and finding the media playlist through the script request before returning the grabbed segments.
    pub fn grab_segments(&self, m3u8_url: &str, access_token: &str) -> (Url, Vec<MediaSegment>) {
        debug!(
            "Attempting to parse and grab m3u8 file with url: {}",
            m3u8_url
        );
        loop {
            let bytes = self
                .sender
                .get_with_auth(
                    m3u8_url,
                    (AUTHORIZATION.as_str(), &format!("Bearer {}", access_token)),
                )
                .bytes()
                .unwrap();
            write("index.m3u8", &bytes).unwrap();
            let m3u8 = parse_playlist(bytes.as_ref());
            // debug!("Chosen url: {}", url);
            // trace!("Data: {:#?}", bytes.as_ref());

            unimplemented!();
            // match m3u8 {
            //     Ok((_, Playlist::MasterPlaylist(pl))) => {
            //         // This is only grabbing the first variant since there is no quality setting for
            //         // Picarto.
            //         let variant = pl.variants.first().unwrap();
            //         url.path_segments_mut().unwrap().pop().push("/");
            //         url = url.join(&variant.uri).unwrap();
            //
            //         debug!("Variant url: {}", url);
            //     }
            //     Ok((_, Playlist::MediaPlaylist(pl))) => {
            //         // Assuming that the second request is the media playlist, we can grab the segments
            //         // and then return it with the url in a tuple.
            //         url.path_segments_mut().unwrap().pop();
            //         debug!("Media url: {}", url);
            //         return (url, pl.segments);
            //     }
            //     Err(_) => {
            //         panic!("Unable to parse m3u8!");
            //     }
            // };
        }
    }
}
