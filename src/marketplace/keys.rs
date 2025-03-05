struct MarketplaceKeys<'a> {
    last_title_account_id: &'a str,
    last_minecraft_id: &'a str,
    last_device_id: &'a str,
    
    pub keys: Vec<PackKey<'a>>,
}

struct PackKey<'a> {
    uuid: &'a str,
    key: &'a str,
}

// Let's not break the marketplace drm today =)