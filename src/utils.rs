use std::collections::HashSet;
use serde::{Serialize, Deserialize};

use crate::packets::Reader;

use std::i32;

// class LoginResponse(TypedDict):
//     osu_token: str
//     response_body: bytes

pub struct LoginResponse {
    pub cho_token: String,
    pub response_body: Vec<u8>,
}

impl LoginResponse {
    pub fn new(cho_token: String, response_body: Vec<u8>) -> Self {
        LoginResponse {
            cho_token,
            response_body,
        }
    }
}

// class LoginData(TypedDict):
//     username: str
//     password_md5: bytes
//     osu_version: str
//     utc_offset: int
//     display_city: bool
//     pm_private: bool
//     osu_path_md5: str
//     adapters_str: str
//     adapters_md5: str
//     uninstall_md5: str
//     disk_signature_md5: str

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password_md5: String,
    pub osu_version: String,
    pub utc_offset: i32,
    pub display_city: bool,
    pub pm_private: bool,
    pub osu_path_md5: String,
    pub adapters_str: String,
    pub adapters_md5: String,
    pub uninstall_md5: String,
    pub disk_signature_md5: String,
}

pub fn parse_login_data<T>(data: T) -> LoginData 
    where T: AsRef<[u8]>{
    // (
    //     username,
    //     password_md5,
    //     remainder,
    // ) = data.decode().split("\n", maxsplit=2)

    let data = data.as_ref();

    let a = data.split(|&b| b == b'\n').collect::<Vec<&[u8]>>();
    let username = String::from_utf8(a[0].to_vec()).unwrap();
    let password_md5 = String::from_utf8(a[1].to_vec()).unwrap();
    let remainder = a[2].to_vec();

    // (
    //     osu_version,
    //     utc_offset,
    //     display_city,
    //     client_hashes,
    //     pm_private,
    // ) = remainder.split("|", maxsplit=4)

    let a = remainder.split(|&b| b == b'|').collect::<Vec<&[u8]>>();
    let osu_version = String::from_utf8(a[0].to_vec()).unwrap();
    //let utc_offset = i32::from_str(String::from_utf8(a[1].to_vec()).unwrap().as_str()).unwrap();
    let utc_offset = String::from_utf8(a[1].to_vec()).unwrap().parse::<i32>().unwrap();
    let display_city = String::from_utf8(a[2].to_vec()).unwrap() == "1";
    let client_hashes = a[3].to_vec(); 
    let pm_private = String::from_utf8(a[4].to_vec()).unwrap() == "1";

    // (
    //     osu_path_md5,
    //     adapters_str,
    //     adapters_md5,
    //     uninstall_md5,
    //     disk_signature_md5,
    // ) = client_hashes[:-1].split(":", maxsplit=4)

    let a = client_hashes.split(|&b| b == b':').collect::<Vec<&[u8]>>();
    let osu_path_md5 = String::from_utf8(a[0].to_vec()).unwrap();
    let adapters_str = String::from_utf8(a[1].to_vec()).unwrap();
    let adapters_md5 = String::from_utf8(a[2].to_vec()).unwrap();
    let uninstall_md5 = String::from_utf8(a[3].to_vec()).unwrap();
    let disk_signature_md5 = String::from_utf8(a[4].to_vec()).unwrap();

    LoginData { 
        username,
        password_md5,
        osu_version,
        utc_offset,
        display_city,
        pm_private,
        osu_path_md5,
        adapters_str,
        adapters_md5,
        uninstall_md5,
        disk_signature_md5,
    }
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
