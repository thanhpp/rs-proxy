use std::{collections::HashMap, error::Error, fs};

use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProxyConfig {
    name: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct GroupConfig {
    prefix: String,
    proxy_names: Option<Vec<String>>,
    destinations: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub proxies: Option<Vec<ProxyConfig>>,
    pub groups: Option<Vec<GroupConfig>>,
}

impl Config {
    pub fn from_yaml(path: &str) -> Result<Config, Box<dyn Error>> {
        let data = fs::read_to_string(path)?;
        let cfg = serde_yaml::from_str::<Config>(&data)?;

        Ok(cfg)
    }
}

pub fn validate_groups(
    gr_cfgs: &Vec<GroupConfig>,
    clients_map: &HashMap<String, Client>,
) -> Result<(), Box<dyn Error>> {
    for gr in gr_cfgs.iter() {
        let proxy_names = match &gr.proxy_names {
            None => return Err(format!("group contains no proxy name").into()),
            Some(x) => x,
        };

        for name in proxy_names.iter() {
            if !clients_map.contains_key(name) {
                return Err(format!("client {} not exist", name).into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_yaml() -> Result<(), String> {
        let res = Config::from_yaml("config.yaml");
        if let Err(e) = res {
            return Err(format!("{}", e));
        }
        Ok(())
    }
}

pub struct ProxyClient {
    name: String,
    client: reqwest::Client,
}

impl ProxyClient {
    pub fn from_config(cfg: &ProxyConfig) -> Result<ProxyClient, Box<dyn Error>> {
        if cfg.name.is_empty() {
            return Err("empty proxy config name".into());
        }
        if cfg.url.is_empty() {
            return Err("empty proxy config url".into());
        }

        let proxy_client = Client::builder()
            .proxy(Proxy::all(cfg.url.clone())?)
            .build()?;

        Ok(ProxyClient {
            name: cfg.name.clone(),
            client: proxy_client,
        })
    }
}

pub fn build_proxy_client_hash_map(
    proxies_config: &Vec<ProxyConfig>,
) -> Result<HashMap<String, Client>, Box<dyn Error>> {
    if proxies_config.is_empty() {
        return Err("empty proxies config".into());
    }
    let mut client_map: HashMap<String, Client> = HashMap::new();
    for cfg in proxies_config.iter() {
        let client = ProxyClient::from_config(cfg)?;
        if client_map.contains_key(&client.name) {
            return Err("duplicate proxy client name".into());
        }
        client_map.insert(client.name.clone(), client.client);
    }

    Ok(client_map)
}
