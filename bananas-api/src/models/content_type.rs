/*
 * OpenTTD Content API
 *
 * OpenTTD Content API
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "ai")]
    Ai,
    #[serde(rename = "ai-library")]
    AiLibrary,
    #[serde(rename = "base-graphics")]
    BaseGraphics,
    #[serde(rename = "base-music")]
    BaseMusic,
    #[serde(rename = "base-sounds")]
    BaseSounds,
    #[serde(rename = "game-script")]
    GameScript,
    #[serde(rename = "game-script-library")]
    GameScriptLibrary,
    #[serde(rename = "heightmap")]
    Heightmap,
    #[serde(rename = "newgrf")]
    Newgrf,
    #[serde(rename = "scenario")]
    Scenario,

}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            Self::Ai => String::from("ai"),
            Self::AiLibrary => String::from("ai-library"),
            Self::BaseGraphics => String::from("base-graphics"),
            Self::BaseMusic => String::from("base-music"),
            Self::BaseSounds => String::from("base-sounds"),
            Self::GameScript => String::from("game-script"),
            Self::GameScriptLibrary => String::from("game-script-library"),
            Self::Heightmap => String::from("heightmap"),
            Self::Newgrf => String::from("newgrf"),
            Self::Scenario => String::from("scenario"),
        }
    }
}

impl Default for ContentType {
    fn default() -> ContentType {
        Self::Ai
    }
}



