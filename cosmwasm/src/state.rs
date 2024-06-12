use cw_storage_plus::Map;

pub const USER_COUNT: Map<String, u64> = Map::new("count");