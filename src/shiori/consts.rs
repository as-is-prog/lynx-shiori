pub enum RequestMethod {
    GET,    /* GET method */
    NOTIFY, /* Notify method */
}
impl RequestMethod {
    pub(super) fn as_str(&self) -> &str {
        match &self {
            RequestMethod::GET => "GET",
            RequestMethod::NOTIFY => "NOTIFY",
        }
    }
}

pub enum SenderType {
    INTERNAL,    /* application internal */
    EXTERNAL,    /* application external */
    SAKURAAPI,   /* Sakura API */
    EMBED,       /* \![embed] tag */
    RAISE,       /* \![raise] tag */
    PROPERTY,    /* Reference Property System event */
    PLUGIN,      /* Plugin */
    SSTP,        /* SSTP */
    COMMUNICATE, /* Communicate */
}
impl SenderType {
    pub(super) fn as_str(&self) -> &str {
        match &self {
            SenderType::INTERNAL => "internal",
            SenderType::EXTERNAL => "",
            SenderType::SAKURAAPI => todo!(),
            SenderType::EMBED => todo!(),
            SenderType::RAISE => todo!(),
            SenderType::PROPERTY => todo!(),
            SenderType::PLUGIN => todo!(),
            SenderType::SSTP => todo!(),
            SenderType::COMMUNICATE => todo!(),
        }
    }
}

pub enum SecurityLevel {}

pub enum GhostExecuteStatus {}

pub(super) const SHIORI_REQUEST_HEADER_STARTS: &str = "SHIORI/";
pub(super) const SHIORI_REQUEST_HEADER_VERSION_3_0: &str = "SHIORI/3.0";
