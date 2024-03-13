use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let cf = req.cf().expect("Failed to get Cloudflare settings");
    let (lat, lon) = cf.coordinates().unwrap_or_default();

    let access_token = match env.secret("MAPBOX_ACCESS_TOKEN") {
        Ok(secret) => secret.to_string(),
        Err(_) => return Response::error("Failed to load secret.", 500),
    };

    let url = format!(
        "https://api.mapbox.com/styles/v1/mapbox/streets-v12/static/pin-l+555555({lon},{lat})/{lon},{lat},9,0/500x500@2x?access_token={access_token}"
    );

    let cache = Cache::default();
    match cache.get(&url, false).await {
        Ok(Some(response)) => return Ok(response),
        Ok(None) => (), // Continue if there's no cache hit
        Err(_) => return Response::error("Cache lookup failed.", 500),
    }

    let parsed_url = match Url::parse(&url) {
        Ok(url) => url,
        Err(_) => return Response::error("Failed to parse URL.", 500),
    };

    let mut response = match Fetch::Url(parsed_url).send().await {
        Ok(res) => res,
        Err(_) => return Response::error("Failed to load the image.", 500),
    };

    if cache.put(&url, response.cloned().unwrap()).await.is_err() {
        // Logging the error might be helpful here, but we choose to ignore it as per the user's scenario.
        // Log the error or take action as needed.
    }

    Ok(response)
}
