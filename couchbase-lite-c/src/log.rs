
use ffi::CBL_SetLogLevel;

pub enum LogLevel {
    LogDebug = 0,
    LogVerbose = 1,
    LogInfo = 2,
    LogWarning = 3,
    LogError = 4,
    LogNone = 5,
}

pub enum LogDomain {
    LogDomainAll = 0,
    LogDomainDatabase = 1,
    LogDomainQuery = 2,
    LogDomainReplicator = 3,
    LogDomainNetwork = 4,
}

pub fn set_log_level(level: LogLevel, domain: LogDomain) {
    unsafe { CBL_SetLogLevel(level as u8, domain as u8); }
}