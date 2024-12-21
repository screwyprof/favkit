use crate::{
    finder::Target,
    system::favorites::{DisplayName, Url},
};

pub struct TargetUrl(pub Url, pub DisplayName);

impl From<TargetUrl> for Target {
    fn from(target: TargetUrl) -> Self {
        match target.0.to_string().as_str() {
            "nwnode://domain-AirDrop" => Target::AirDrop,
            path => Target::Custom {
                label: target.1.to_string(),
                path: path.to_string(),
            },
        }
    }
}
