use serde::Serialize;

#[derive(Serialize)]
pub struct Version {
    pub build_profile: &'static str,
    pub features: Vec<&'static str>,
    pub version: &'static str,
}

impl Default for Version {
    fn default() -> Self {
        Self::new()
    }
}

impl Version {
    pub fn new() -> Self {
        Self {
            build_profile: env!("BUILD_PROFILE"),
            features: env!("BUILD_FEATURES").split(',').collect::<Vec<_>>(),
            version: env!("REPO_VERSION"),
        }
    }
}
