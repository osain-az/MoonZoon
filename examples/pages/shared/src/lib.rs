
use moonlight::*;

// ------ UpMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum UpMsg {
    NewTest {
        item: String,
    },

}

// ------ DownMsg ------

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum DownMsg {
    // ------ Auth ------
    NewItem(String),

}
