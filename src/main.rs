use hyper::Body;
use hyper::{Request, Response};
use rs_proxy::{build_proxy_client_hash_map, validate_groups, Config};

#[tokio::main]
async fn main() {
    // read config
    let cfg = Config::from_yaml("config.yaml").unwrap();
    let proxy_clients = build_proxy_client_hash_map(&cfg.proxies.unwrap()).unwrap();

    if let Err(e) = validate_groups(&cfg.groups.unwrap(), &proxy_clients) {
        panic!("{}", e);
    }
}

fn get_path_prefix(path: &str) -> String {
    let splitted: Vec<&str> = path.split("/").collect();
    if splitted.len() > 2 {
        return splitted[1].to_string();
    }

    "".to_string()
}

async fn route(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let prefix = get_path_prefix(req.uri().path());

    let resp = Response::default();
    if prefix.len() == 0 {
        return Ok(resp);
    }

    // create a new request
    let path = req.uri().path().replacen(format!("\\{}", prefix), "", 1);

    Ok(Response::new("".into()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_path_prefix() {
        assert_eq!(get_path_prefix("/bsc/abc"), "bsc")
    }
}

// let proxy_client = Client::builder()
//     .proxy(Proxy::all("").unwrap()) // http://user:pass@host:port
//     .build()
//     .unwrap();

// let resp = proxy_client
//     .get("http://ipinfo.io")
//     .send()
//     .await
//     .unwrap()
//     .text()
//     .await
//     .unwrap();

// println!("{:#?}", resp);
