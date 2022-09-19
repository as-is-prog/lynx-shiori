use std::char::from_u32;
use strum_macros::{Display, EnumString};

pub(super) const SHIORI_PROTOCOL_VERSION_STARTS: &str = "SHIORI/";
pub(super) const SHIORI_PROTOCOL_VERSION_3_0: &str = "SHIORI/3.0";

#[derive(EnumString, Display)]
pub enum RequestMethod {
    #[strum(serialize = "GET")]
    Get, /* GET method */
    #[strum(serialize = "NOTIFY")]
    Notify, /* Notify method */
}

pub(super) const SHIORI_PROTOCOL_HEADER_CHARSET_STARTS: &str = "Charset";

pub(super) const SHIORI_PROTOCOL_HEADER_SENDER_STARTS: &str = "Sender";

pub(super) const SHIORI_PROTOCOL_HEADER_SENDER_TYPE_STARTS: &str = "SenderType";

#[derive(EnumString, Display)]
pub enum SenderType {
    #[strum()]
    Internal, /* application internal */
    External,      /* application external */
    SakuraApi,     /* Sakura API */
    Embed,         /* \![embed] tag */
    Raise,         /* \![raise] tag */
    Property,      /* Reference Property System event */
    Plugin,        /* Plugin */
    Sstp,          /* SSTP */
    Communicate,   /* Communicate */
    Other(String), /* (original) other. */
    None,
}

// impl SenderType {
//     pub(super) fn to_string(&self) -> String {
//         match &self {
//             SenderType::Internal => "internal".to_string(),
//             SenderType::External => "external".to_string(),
//             SenderType::SakuraApi => "sakuraapi".to_string(),
//             SenderType::Embed => "embed".to_string(),
//             SenderType::Raise => "raise".to_string(),
//             SenderType::Property => "property".to_string(),
//             SenderType::Plugin => "plugin".to_string(),
//             SenderType::Sstp => "sstp".to_string(),
//             SenderType::Communicate => "communicate".to_string(),
//             SenderType::Other(str) => str.to_string(),
//             SenderType::None => String::new(),
//         }
//     }
// }

pub(super) const SHIORI_PROTOCOL_HEADER_SECULITY_LEVEL_STARTS: &str = "SecurityLevel";
pub enum SecurityLevel {
    Local,
    External,
    Other(String), /* (original) other */
    None,
}
impl SecurityLevel {
    pub(super) fn to_string(&self) -> String {
        match &self {
            SecurityLevel::Local => "local".to_string(),
            SecurityLevel::External => "external".to_string(),
            SecurityLevel::Other(str) => str.to_string(),
            SecurityLevel::None => String::new(),
        }
    }
}

/* ex: opening(communicate/dialog) */
pub enum GhostExecuteStatusOpeningKind {
    Communicate,   /* communicate box */
    Input,         /* input box */
    Teach,         /* teach box */
    Dialog,        /* dialog. ex: file select dialog, color picker... */
    Other(String), /* (original) other. */
    None,
}
impl GhostExecuteStatusOpeningKind {
    pub(super) fn to_string(&self) -> String {
        match &self {
            GhostExecuteStatusOpeningKind::Communicate => "communicate".to_string(),
            GhostExecuteStatusOpeningKind::Input => "input".to_string(),
            GhostExecuteStatusOpeningKind::Teach => "teach".to_string(),
            GhostExecuteStatusOpeningKind::Dialog => "dialog".to_string(),
            GhostExecuteStatusOpeningKind::Other(str) => str.to_string(),
            GhostExecuteStatusOpeningKind::None => String::new(),
        }
    }
}

/* ex: Status: choosing,balloon(0=0) */
pub(super) const SHIORI_PROTOCOL_HEADER_STATUS_STARTS: &str = "Status";
pub enum GhostExecuteStatus {
    Talking,                                     /* talking */
    Choosing,                                    /* choosing */
    Minimizing,                                  /* minimizing */
    Induction,                                   /* \![enter,inductionmode] */
    Passive,                                     /* \![enter,passivemode] */
    TimeCritical,                                /* \t => time critical session. */
    NoUserBreak,                                 /* \![enter,nouserbreak] */
    Online,                                      /* network connecting */
    Opening(Vec<GhostExecuteStatusOpeningKind>), /* dialog or inputbox opening. */
    Balloon(String), /* Balloon(TODO: ID群をパースしてVec<{char_id, balloon_id}にする>) */
    Other(String),   /* (original) other. */
    None,
}
impl GhostExecuteStatus {
    pub(super) fn to_string(&self) -> String {
        match &self {
            GhostExecuteStatus::Talking => "talking".to_string(),
            GhostExecuteStatus::Choosing => "choosing".to_string(),
            GhostExecuteStatus::Minimizing => "minimizing".to_string(),
            GhostExecuteStatus::Induction => "induction".to_string(),
            GhostExecuteStatus::Passive => "passive".to_string(),
            GhostExecuteStatus::TimeCritical => "timecritical".to_string(),
            GhostExecuteStatus::NoUserBreak => "nouserbreak".to_string(),
            GhostExecuteStatus::Online => "online".to_string(),
            GhostExecuteStatus::Opening(kinds) => format!(
                "opening({})",
                kinds
                    .iter()
                    .map(|k| k.to_string())
                    .collect::<Vec<String>>()
                    .join("/")
            ),
            GhostExecuteStatus::Balloon(str) => format!("balloon({})", str).to_string(),
            GhostExecuteStatus::Other(str) => str.to_string(),
            GhostExecuteStatus::None => String::new(),
        }
    }
}

pub(super) const SHIORI_PROTOCOL_HEADER_ID_STARTS: &str = "ID";
pub(super) const SHIORI_PROTOCOL_HEADER_BASE_ID_STARTS: &str = "BaseID";
pub(super) const SHIORI_PROTOCOL_HEADER_REFERENCE_STARTS: &str = "Reference";

pub(super) const SHIORI_PROTOCOL_STATUS_CODE_200: &str = "200 OK";
pub(super) const SHIORI_PROTOCOL_STATUS_CODE_204: &str = "204 No Content";
pub(super) const SHIORI_PROTOCOL_STATUS_CODE_311: &str = "311 Not Enough";
pub(super) const SHIORI_PROTOCOL_STATUS_CODE_312: &str = "312 Advice";
pub(super) const SHIORI_PROTOCOL_STATUS_CODE_400: &str = "400 Bad Request";
pub(super) const SHIORI_PROTOCOL_STATUS_CODE_500: &str = "500 Internal Server Error";

pub(super) const SHIORI_PROTOCOL_HEADER_VALUE_STARTS: &str = "Value";

pub(super) const SHIORI_PROTOCOL_HEADER_VALUE_NOTIFY_STARTS: &str = "ValueNotify";

pub(super) const SHIORI_PROTOCOL_HEADER_MARKER_STARTS: &str = "Marker";

pub(super) const SHIORI_PROTOCOL_HEADER_ERROR_LEVEL_STARTS: &str = "ErrorLevel";
pub(super) const SHIORI_PROTOCOL_HEADER_ERROR_LEVEL_SEPARATOR_BYTE: u32 = 1;

pub enum ErrorLevel {
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Other(String), /* (original) */
    None,
}
impl ErrorLevel {
    pub(super) fn get_separator() -> char {
        from_u32(1).expect("internal error")
    }
    pub(super) fn to_string(&self) -> String {
        match &self {
            ErrorLevel::Info => "info".to_string(),
            ErrorLevel::Notice => "notice".to_string(),
            ErrorLevel::Warning => "warning".to_string(),
            ErrorLevel::Error => "error".to_string(),
            ErrorLevel::Critical => "critical".to_string(),
            ErrorLevel::Other(str) => str.to_string(),
            ErrorLevel::None => String::new(),
        }
    }
}
