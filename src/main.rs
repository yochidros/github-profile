use clap::Parser;

use args::Args;
use handler::fetch_user;

use self::github::GithubUser;

mod args;
mod github;
mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let args = Args::parse();
    let name: String = args.name;
    let user = fetch_user(name).await?;
    print_userinfo(user);

    Ok(())
}

fn print_userinfo(user: GithubUser) {
    println!("Name: {}", user.name.unwrap_or(String::from("-")));
    println!("AvatorIconUrl: {}", user.avatar_url);
    println!(
        "Bio: {}",
        user.biography
            .unwrap_or(String::from("-"))
            .replace("\n", "")
    );
    println!("Followers: {}", user.followers);
    println!("Followings: {}", user.following);
    println!("Public Repository Count: {}", user.repos_count);
    println!("Profile Url: {}", user.profile_url);
}
