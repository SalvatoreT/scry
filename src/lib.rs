use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let cf = req.cf().expect("Failed to get Cloudflare settings");
    let (lat, lon) = cf.coordinates().unwrap_or_default();
    let access_token = match env.secret("MAPBOX_ACCESS_TOKEN") {
        Ok(secret) => secret.to_string(),
        Err(_) => {
            return Response::error("Failed to load secret.", 500);
        }
    };
    let url = format!("https://api.mapbox.com/styles/v1/mapbox/light-v11/static/pin-l+555555({lon},{lat})/{lon},{lat},9,0/500x500@2x?access_token={access_token}");
    let parse = Url::parse(&url).expect("Failed to parse URL");
    let cache = Cache::default();
    match cache.get(url.clone(), false).await {
        Ok(cache_response) => {
            match cache_response {
                None => {
                    let response = Fetch::Url(parse).send().await;
                    match response {
                        Ok(mut result) => {
                            cache.put(url, result.cloned().unwrap()).await.unwrap();
                            Ok(result)
                        }
                        Err(_) => {
                            Response::error("Failed to load the image.", 500)
                        }
                    }
                }
                Some(response) => {
                    Ok(response)
                }
            }
        }
        Err(error) => {
            return Response::error(format!("{:?}", error), 500);
        }
    }
}
