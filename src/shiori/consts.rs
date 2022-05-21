pub enum RequestMethod {
    Get,    /* GET method */
    Notify, /* Notify method */
}
impl RequestMethod {
    pub(super) fn as_str(&self) -> &str {
        match &self {
            RequestMethod::Get => "GET",
            RequestMethod::Notify => "NOTIFY",
        }
    }
}

pub(super) const SHIORI_REQUEST_CHARSET_STARTS: &str = "Charset: ";

pub(super) const SHIORI_REQUEST_SENDER_STARTS: &str = "Sender: ";

pub(super) const SHIORI_REQUEST_SENDER_TYPE_STARTS: &str = "SenderType: ";
pub enum SenderType {
    Internal,    /* application internal */
    External,    /* application external */
    SakuraApi,   /* Sakura API */
    Embed,       /* \![embed] tag */
    Raise,       /* \![raise] tag */
    Property,    /* Reference Property System event */
    Plugin,      /* Plugin */
    Sstp,        /* SSTP */
    Communicate, /* Communicate */
    None,        /* (original) */
}
impl SenderType {
    pub(super) fn as_str(&self) -> &str {
        match &self {
            SenderType::Internal => "internal",
            SenderType::External => "external",
            SenderType::SakuraApi => "sakuraapi",
            SenderType::Embed => "embed",
            SenderType::Raise => "raise",
            SenderType::Property => "property",
            SenderType::Plugin => "plugin",
            SenderType::Sstp => "sstp",
            SenderType::Communicate => "communicate",
            SenderType::None => "",
        }
    }
}

pub(super) const SHIORI_REQUEST_SECULITY_LEVEL_STARTS: &str = "SecurityLevel: ";
pub enum SecurityLevel {
    Local,
    External,
    None, /* (original) */
}
impl SecurityLevel {
    pub(super) fn as_str(&self) -> &str {
        match &self {
            SecurityLevel::Local => "GET",
            SecurityLevel::External => "NOTIFY",
            SecurityLevel::None => "",
        }
    }
}

pub enum GhostExecuteStatusOpeningKind {
    Communicate, /* communicate box */
    Input,       /* input box */
    Teach,       /* teach box */
    Dialog,      /* dialog. ex: file select dialog, color picker... */
}
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
    Balloon(String),
}

pub(super) const SHIORI_REQUEST_HEADER_STARTS: &str = "SHIORI/";
pub(super) const SHIORI_REQUEST_HEADER_VERSION_3_0: &str = "SHIORI/3.0";
