use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GithubUser {
    pub id: usize,

    pub name: Option<String>,

    #[serde(rename(deserialize = "bio"))]
    pub biography: Option<String>,

    #[serde(rename(deserialize = "public_repos"))]
    pub repos_count: usize,

    pub followers: usize,

    pub following: usize,

    pub avatar_url: String,

    #[serde(rename(deserialize = "html_url"))]
    pub profile_url: String,
}
