use std::env;

use minecraft_utils::mojang_api::user::validate_username;
use minecraft_utils::mojang_api::{get_username_uuid, Profile};

fn main() {
    let name_uuid = env::args()
        .skip(1)
        .next()
        .expect("username or uuid must be provided as an argument");

    let is_uuid = name_uuid.len() > 16;
    let uuid = if is_uuid {
        name_uuid.replace('-', "")
    } else {
        validate_username(&name_uuid).expect("invalid username");
        get_username_uuid(&name_uuid).expect("unable to fetch uuid from username")
    };

    let profile = Profile::fetch(&uuid).expect("unable to fetch user profile.");

    println!("uuid: {}", profile.id);
    println!("name: {}", profile.name);
    println!(
        "skin model: {}",
        profile.slim_model().then(|| "alex").unwrap_or("steve")
    );
    println!("skin url: {}", profile.textures().skin.url);
    println!(
        "cape url: {}",
        profile.textures().cape.as_ref().map_or("", |v| &v.url)
    );
}
