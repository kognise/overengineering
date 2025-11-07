use rocket::tokio::fs::read_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemberColors {
    pub text: String,
    pub border: String,
    pub links: String,
}

impl Default for MemberColors {
    fn default() -> Self {
        Self {
            border: "#000000".to_string(),
            text: "#000000".to_string(),
            links: "#0000ee".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ParsedMember {
    pub name: String,
    pub url: String,
    pub colors: Option<MemberColors>,
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
            colors: member.colors.unwrap_or_default(),
            font_stack: member.font_stack,
            font_size: member.font_size,
            stylesheets: member.stylesheets.unwrap_or_default(),
        });
    }

    members.sort_by(|a, b| a.slug.cmp(&b.slug));

    Ok(members)
}
