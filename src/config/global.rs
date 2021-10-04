use crate::config::structs::InstanceInfo;
use once_cell::sync::OnceCell;

pub static INSTANCE: OnceCell<InstanceInfo> = OnceCell::new();

pub const CONFIG_FILE: &str = "./config/server/config.toml";
pub const PLATFORMS_FILE: &str = "./config/server/platforms.json";
pub const SCRUTINS_FILE: &str = "./data/scrutins.json";
pub const ACTEURS_FILE: &str = "./data/acteurs.json";

pub const IPV4_WHITELIST_FILE: &str = "./config/filters/generated/ip-whitelist.csv";
pub const IPV4_BLACKLIST_FILE: &str = "./config/filters/generated/ip-blacklist.csv";
pub const ASN_LIST_FILE: &str = "./config/filters/generated/asn-list.csv";
pub const ASN_BLACKLIST_FILE: &str = "./config/filters/generated/asn-blacklist.csv";

pub const AC_IP_NOT_WHITELISTED: u16 = 1000;
pub const AC_IPABUSEDB_FAIL: u16 = 2000;

// CSRF token lasts 48 hours
pub const CSRF_TTL: i64 = 172_800;

// minimum and maximum tolerated abuse code
// for a submission to be counted as valid
pub const AC_LOWEST: i16 = 0;
pub const AC_HIGHEST: i16 = 20;

pub const MIN_SUB_DURATION: i32 = 90;
pub const MAX_SUB_DURATION: i32 = 21_600;
