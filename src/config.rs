use serenity::model::id::ChannelId;

pub struct Config {
    pub honey_pot_chs: Vec<ChannelId>,
    pub ng_words: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        let channel_ids: Vec<u64> =
            vec![896694513031086112, 896694568265863178, 896694584808198154];
        let ng_words = vec!["free", "nitro"];
        Config {
            honey_pot_chs: channel_ids.iter().map(|id| ChannelId::from(*id)).collect(),
            ng_words: ng_words
                .iter()
                .map(|str| str.to_string().to_lowercase())
                .collect(),
        }
    }
}
