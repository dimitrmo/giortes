use rss::Channel;
use serde::Serialize;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

const GIORTES_ENDPOINT: &'static str = "https://www.giortes.gr/rss/si_av_me_el.xml";

#[derive(Clone, Debug, Serialize)]
pub struct Giortes {
    updated_at: u128,
    copyright: String,
    names: Vec<String>,
}

impl Default for Giortes {

    fn default() -> Self {
        Giortes {
            updated_at: 0,
            copyright: String::from(""),
            names: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Eortologio {
    giortes: Box<Giortes>,
}

impl Eortologio {
    pub fn get_giortes(&self) -> &Giortes {
        self.giortes.as_ref()
    }

    async fn fetch_feed_async(&self) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::get(GIORTES_ENDPOINT).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    fn fetch_feed(&self) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::blocking::get(GIORTES_ENDPOINT)
            .unwrap()
            .bytes()
            .unwrap();
        let channel = Channel::read_from(&content[..])?;
        Ok(channel)
    }

    pub async fn fetch_giortes_async(&self) -> Giortes {
        let channel = self.fetch_feed_async().await.unwrap();
        let copyright = channel.copyright.unwrap();

        let epoch = SystemTime::now().duration_since(UNIX_EPOCH);
        let epoch_updated_at = epoch.unwrap().as_millis();
        let mut giortes = Giortes::default();
        giortes.copyright = copyright;
        giortes.updated_at = epoch_updated_at;

        for item in channel.items {
            let title = item.title.unwrap();
            let title_parts = title.split_once(":").unwrap();
            let names = title_parts.1.trim();
            giortes.names.push(names.to_string());
        }

        giortes
    }

    pub fn fetch_giortes(&self) -> Giortes {
        let channel = self.fetch_feed().unwrap();
        let copyright = channel.copyright.unwrap();

        let epoch = SystemTime::now().duration_since(UNIX_EPOCH);
        let epoch_updated_at = epoch.unwrap().as_millis();
        let mut giortes = Giortes::default();
        giortes.copyright = copyright;
        giortes.updated_at = epoch_updated_at;

        for item in channel.items {
            let title = item.title.unwrap();
            let title_parts = title.split_once(":").unwrap();
            let names = title_parts.1.trim();
            giortes.names.push(names.to_string());
        }

        giortes
    }

    pub async fn refresh_giortes_async(&mut self) -> &Giortes {
        let giortes = Box::new(self.fetch_giortes_async().await);
        let _old = std::mem::replace(&mut self.giortes, giortes);
        self.get_giortes()
    }

    pub fn refresh_giortes(&mut self) -> &Giortes {
        let giortes = Box::new(self.fetch_giortes());
        let _old = std::mem::replace(&mut self.giortes, giortes);
        self.get_giortes()
    }
}
