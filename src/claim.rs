use crate::config::User;
use phf::phf_map;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use ureq::{serde_json::Value, Error};

static HEADERS: phf::Map<&'static str, &'static str> = phf_map! {
    "Accept" => "application/json, text/plain, */*",
    "Accept-Encoding" => "gzip, deflate",
    "Connection" => "keep-alive",
    "Referer" => "https://act.hoyolab.com/",
    "Origin" => "https://act.hoyolab.com",
    "User-Agent" => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "x-rpc-app_version" => "2.34.1",
    "x-rpc-client_type" => "4",
};

static CLAIM_URIS: phf::Map<&'static str, &'static str> = phf_map! {
    "genshin" => "https://sg-hk4e-api.hoyolab.com/event/sol/sign?lang=en-us&act_id=e202102251931481",
    "honkai" => "https://sg-public-api.hoyolab.com/event/mani/sign?lang=en-us&act_id=e202110291205111",
    "star_rail" => "https://sg-public-api.hoyolab.com/event/luna/os/sign?lang=en-us&act_id=e202303301540311",
    "zzz" => "https://sg-act-nap-api.hoyolab.com/event/luna/zzz/os/sign?lang=en-us&act_id=e202406031448091",
};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct HoyolabResponse {
    data: Option<Value>,
    message: Option<String>,
    retcode: i64,
}

pub fn claim(users: &[User]) -> anyhow::Result<()> {
    let agent = ureq::builder()
        .timeout(Duration::from_secs(10))
        .user_agent(HEADERS.get("User-Agent").unwrap())
        .https_only(true)
        .build();

    for user in users {
        let username = user.username.as_ref().unwrap_or(&user.ltuid);

        let mut url_map: HashMap<String, String> = HashMap::new();
        if user.genshin {
            url_map.insert(
                "genshin".to_string(),
                CLAIM_URIS.get("genshin").unwrap().to_string(),
            );
        }
        if user.honkai {
            url_map.insert(
                "honkai".to_string(),
                CLAIM_URIS.get("honkai").unwrap().to_string(),
            );
        }
        if user.star_rail {
            url_map.insert(
                "star_rail".to_string(),
                CLAIM_URIS.get("star_rail").unwrap().to_string(),
            );
        }
        if user.zzz {
            url_map.insert(
                "zzz".to_string(),
                CLAIM_URIS.get("zzz").unwrap().to_string(),
            );
        }

        for (game, url) in url_map.into_iter() {
            let cookie = format!("ltoken_v2={}; ltuid_v2={};", user.ltoken, user.ltuid);
            let resp = agent
                .post(url.as_str())
                .set("Accept", HEADERS.get("Accept").unwrap())
                .set("Accept-Encoding", HEADERS.get("Accept-Encoding").unwrap())
                .set("Connection", HEADERS.get("Connection").unwrap())
                .set("Referer", HEADERS.get("Referer").unwrap())
                .set("Origin", HEADERS.get("Origin").unwrap())
                .set(
                    "x-rpc-app_version",
                    HEADERS.get("x-rpc-app_version").unwrap(),
                )
                .set(
                    "x-rpc-client_type",
                    HEADERS.get("x-rpc-client_type").unwrap(),
                )
                .set("Cookie", &cookie)
                .call();
            match resp {
                Ok(resp) => {
                    let resp: HoyolabResponse = resp.into_json()?;
                    println!(
                        "User: {}, Game: {} - {}",
                        username,
                        game,
                        resp.message.unwrap_or("unknown".to_string())
                    );
                }
                Err(Error::Status(code, _)) => eprintln!(
                    "User: {}, Game: {} - Error during request. Status code: {}.",
                    username, game, code
                ),
                Err(_) => eprintln!(
                    "User: {}, Game: {} - Unknown error during request.",
                    username, game
                ),
            }
        }
    }
    Ok(())
}
