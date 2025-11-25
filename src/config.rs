use std::hash::{DefaultHasher, Hash, Hasher};

use chrono::Utc;
use rocket::tokio::fs::read_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParsedMemberColors {
    pub text: Option<String>,
    pub border: Option<String>,
    pub links: Option<String>,
    pub on_links: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MemberColors {
    pub text: String,
    pub border: String,
    pub links: String,
    pub on_links: String,
}

impl MemberColors {
    pub fn fill_empty_from(from: Option<&ParsedMemberColors>) -> Self {
        Self {
            text: from
                .and_then(|f| f.text.clone())
                .unwrap_or("#000000".to_string()),
            border: from
                .and_then(|f| f.border.clone())
                .unwrap_or("#000000".to_string()),
            links: from
                .and_then(|f| f.links.clone())
                .unwrap_or("#0000ee".to_string()),
            on_links: from
                .and_then(|f| f.on_links.clone())
                .unwrap_or("#ffffff".to_string()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ParsedMember {
    pub name: String,
    pub url: String,
    pub colors: Option<ParsedMemberColors>,
    pub font_stack: Option<String>,
    pub font_size: Option<String>,
    pub stylesheets: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct Member {
    pub slug: String,
    pub name: String,
    pub url: String,
    pub colors: MemberColors,
    pub font_stack: Option<String>,
    pub font_size: Option<String>,
    pub stylesheets: Vec<String>,
}

pub async fn read_members() -> anyhow::Result<Vec<Member>> {
    let mut members = vec![];

    let mut files = read_dir("members").await?;
    while let Some(file) = files.next_entry().await? {
        let reader = std::fs::File::open(file.path())?;
        let member: ParsedMember = match serde_yaml::from_reader(reader) {
            Ok(member) => member,
            Err(err) => {
                eprintln!(
                    "skipping member {:?} due to format error: {}",
                    file.file_name(),
                    err
                );
                continue;
            }
        };

        let slug = file
            .file_name()
            .to_string_lossy()
            .split('.')
            .next()
            .unwrap()
            .to_string();
        members.push(Member {
            slug,
            name: member.name,
            url: member.url,
            colors: MemberColors::fill_empty_from(member.colors.as_ref()),
            font_stack: member.font_stack,
            font_size: member.font_size,
            stylesheets: member.stylesheets.unwrap_or_default(),
        });
    }

    members.sort_by_cached_key(|m| {
        let mut h = DefaultHasher::new();
        (&m.slug, Utc::now().date_naive()).hash(&mut h);
        h.finish()
    });

    Ok(members)
}
