use std::{collections::HashMap, sync::Arc, time::Duration};

use lazy_static::lazy_static;
use reqwest::Client;
use rocket::{
    futures::{stream, StreamExt},
    tokio::{spawn, sync::RwLock, task::JoinHandle, time::interval},
};

use crate::config::{read_members, Member};

lazy_static! {
    static ref CLIENT: Client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap();
}

#[derive(Clone)]
pub enum Health {
    Ok,
    SiteUnreachable,
    SlugMismatch(String),
    NoWebringEmbed,
}

pub async fn check_health(url: &str, slug: &str) -> Health {
    let response = match CLIENT.get(url).send().await {
        Ok(body) => body,
        Err(_) => return Health::SiteUnreachable,
    };
    let body = match response.text().await {
        Ok(body) => body,
        Err(_) => return Health::SiteUnreachable,
    };

    const URL_PREFIX: &str = "https://overengineering.kognise.dev/embed/";
    let mut count = 0;
    for (offset, _) in body.match_indices(URL_PREFIX) {
        let offset = offset + URL_PREFIX.len();

        let mut body_slug = String::with_capacity(slug.len());
        for char in body[offset..].chars() {
            if char == '\'' || char == '"' || char == '?' || body_slug.len() >= 64 {
                break;
            }
            body_slug.push(char);
        }

        if body_slug != slug {
            return Health::SlugMismatch(body_slug);
        }

        count += 1;
    }

    if count == 0 {
        Health::NoWebringEmbed
    } else {
        Health::Ok
    }
}

pub struct MemberManager {
    members: Arc<RwLock<Vec<Member>>>,
    health: Arc<RwLock<HashMap<String, Health>>>,
    interval_handle: JoinHandle<()>,
}

impl MemberManager {
    pub fn new() -> Self {
        let members = Arc::new(RwLock::new(vec![]));
        let health = Arc::new(RwLock::new(HashMap::new()));

        let interval_handle = spawn(Self::health_check_task(members.clone(), health.clone()));

        Self {
            members,
            health,
            interval_handle,
        }
    }

    pub async fn members(&self) -> Vec<(Member, Option<Health>)> {
        *self.members.write().await = read_members().await.expect("failed to read members");
        let health = self.health.read().await;
        self.members
            .read()
            .await
            .iter()
            .map(|member| (member.clone(), health.get(&member.slug).cloned()))
            .collect()
    }

    pub async fn health_check_task(
        members: Arc<RwLock<Vec<Member>>>,
        health: Arc<RwLock<HashMap<String, Health>>>,
    ) {
        let mut interval = interval(Duration::from_secs(60));
        let mut is_first = true;

        loop {
            interval.tick().await;

            if is_first {
                println!("Performing first healthcheck...");
            }
            let new_members = read_members().await.expect("failed to read members");
            let member_data: Vec<(String, String)> = new_members
                .iter()
                .map(|m| (m.slug.clone(), m.url.clone()))
                .collect();
            *members.write().await = new_members;

            *health.write().await = stream::iter(member_data)
                .map(async |(slug, url)| {
                    let health = check_health(&url, &slug).await;
                    (slug, health)
                })
                .buffer_unordered(8)
                .collect()
                .await;
            if is_first {
                println!("First healthcheck completed!");
                is_first = false;
            }
        }
    }
}

impl Drop for MemberManager {
    fn drop(&mut self) {
        self.interval_handle.abort();
    }
}
