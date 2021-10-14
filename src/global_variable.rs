use crate::Config;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct GlobalConfigs;

impl TypeMapKey for GlobalConfigs {
    type Value = Arc<RwLock<Config>>;
}
