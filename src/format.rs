use semver::Version;
use serde_json::json;

use crate::{check, config::PackToml, error::Error, Result};

pub fn get_format(ver: Version) -> Result<u32> {
    check!(ver: ">= 1.6.1, <= 1.8.9" => 1);
    check!(ver: ">= 1.9, <= 1.10.2" => 2);
    check!(ver: ">= 1.11, <= 1.12.2" => 3);
    check!(ver: ">= 1.13, <= 1.14.4" => 4);
    check!(ver: ">= 1.15, <= 1.16.1" => 5);
    check!(ver: ">= 1.16.2, <= 1.16.5" => 6);
    check!(ver: ">= 1.17, <= 1.17.1" => 7);
    check!(ver: ">= 1.18, <= 1.18.2" => 8);
    check!(ver: ">= 1.19, <= 1.19.2" => 9);
    check!(ver: ="1.19.3" => 12);
    check!(ver: ="1.19.4" => 13);
    check!(ver: ">= 1.20, <= 1.20.1" => 15);
    check!(ver: ="1.20.2" => 18);
    check!(ver: ">= 1.20.3, <= 1.20.4" => 22);
    check!(ver: ="1.20.5" => 32);

    Err(Error::UnknownVer(format!(
        "Unknown Minecraft version: {}",
        ver
    )))
}

pub fn generate_manifest(config: &PackToml) -> Result<String> {
    let desc = config
        .pack
        .description
        .clone()
        .unwrap_or(config.pack.name.clone());
    let format = get_format(Version::parse(&config.version.minecraft)?)?;

    let json = json!({
        "pack": {
            "pack_format": format,
            "description": desc,
        },
    });

    Ok(serde_json::to_string_pretty(&json)?)
}
