use regex::Regex;
use thiserror::Error;

#[derive(Default, Debug)]
pub struct SapGuiConnection {
    //pub desc: &'a str,
    pub system_id: String,
    pub appl_server: String,
    pub instance_id: String,
    pub router: String,
    pub client: String,
    pub user: String,
    pub lang: String,
    pub activate_snc: bool,
    pub snc_name: String,
    pub disable_sso: bool,
}

impl SapGuiConnection {
    const HOST_PREFIX: &'static str = "/H/";
    const PORT_PREFIX: &'static str = "/S/";
    pub fn to_connection_string(&self) -> String {
        let router = if !self.router.is_empty() {
            let mut res = String::new();
            if self.router.starts_with(Self::HOST_PREFIX) {
                res.push_str(&self.router);
            } else {
                res.push_str(&format!("{}{}", Self::HOST_PREFIX, self.router));
            }
            res.push_str(&format!("{}3299", Self::PORT_PREFIX));
            res
        } else {
            String::new()
        };

        let appl_server = if !self.appl_server.is_empty() {
            let mut res = String::new();
            res.push_str(&format!("{}{}", Self::HOST_PREFIX, self.appl_server));
            if self.instance_id.is_empty() {
                res.push_str(&format!("{}3200", Self::PORT_PREFIX));
            } else {
                res.push_str(&format!("{}32{}", Self::PORT_PREFIX, self.instance_id));
            }
            res
        } else {
            String::new()
        };

        let conn = format!("{router}{appl_server}");

        let system_id = &self.system_id;
        let client = &self.client;
        let user = &self.user;
        let lang = &self.lang;

        let (sncqop, sncname, manual_login) = {
            if self.activate_snc {
                (
                    "9",
                    self.snc_name.as_str(),
                    if self.disable_sso { "true" } else { "false" },
                )
            } else {
                Default::default()
            }
        };

        let mut res = String::new();
        Self::append_key(&mut res, "conn", &conn);
        Self::append_key(&mut res, "systemName", system_id);
        Self::append_key(&mut res, "clnt", client);
        Self::append_key(&mut res, "user", user);
        Self::append_key(&mut res, "lang", lang);
        Self::append_key(&mut res, "sncqop", sncqop);
        Self::append_key(&mut res, "sncname", sncname);
        Self::append_key(&mut res, "manualLogin", manual_login);
        if res.is_empty() {
            String::from("conn=")
        } else {
            res
        }
    }

    fn append_key(res: &mut String, key: &str, val: &str) {
        if !val.is_empty() {
            if !res.is_empty() {
                res.push('&');
            }
            let val = val.trim();
            res.push_str(&format!("{key}={val}"));
        }
    }
}

pub struct Parser {
    re: Regex,
    re_conn: Regex,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"(.+?)=(.+?)(&|$)").expect("valid regex"),
            re_conn: Regex::new(r"(/H/(.+?)/S/([0-9]{4}))+?").expect("valid regex"),
        }
    }

    pub fn parse(&self, input: &str) -> Result<SapGuiConnection, ParseError> {
        let mut conn = SapGuiConnection::default();
        for (_, [key, val, _]) in self.re.captures_iter(input).map(|mat| mat.extract()) {
            self.parse_keyval(&mut conn, key, val)?;
        }
        Ok(conn)
    }

    fn parse_keyval(
        &self,
        conn: &mut SapGuiConnection,
        key: &str,
        val: &str,
    ) -> Result<(), ParseError> {
        match key {
            "clnt" => conn.client = val.to_owned(),
            "systemName" => conn.system_id = val.to_owned(),
            "user" => conn.user = val.to_owned(),
            "lang" => conn.lang = val.to_owned(),
            "conn" => self.parse_conn(conn, val)?,
            key => return Err(ParseError::UnknownKey(key.to_owned())),
        }
        Ok(())
    }

    fn parse_conn(&self, conn: &mut SapGuiConnection, val: &str) -> Result<(), ParseError> {
        let last = self.re_conn.captures_iter(val).last();
        if let Some(last) = last {
            let pos = last.get_match().start();
            let (_, [_, host, port]) = last.extract();
            conn.appl_server = host.to_owned();
            conn.instance_id = port[2..].to_owned();
            conn.router = val[..pos].to_owned();
        } else {
            return Err(ParseError::InvalidConn(val.to_owned()));
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("unknown key `{0}`")]
    UnknownKey(String),
    #[error("invalid conn `{0}`")]
    InvalidConn(String),
}
