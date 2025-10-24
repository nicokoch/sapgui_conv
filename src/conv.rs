#[derive(Default)]
pub struct SapGuiConnection {
    //pub desc: &'a str,
    pub system_id: String,
    pub appl_server: String,
    pub instance_id: String,
    pub router: String,
    pub client: String,
    pub user: String,
}

impl SapGuiConnection {
    const HOST_PREFIX: &'static str = "/H/";
    const PORT_PREFIX: &'static str = "/S/";
    pub fn as_connection_string(&self) -> String {
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

        let mut res = String::new();
        Self::append_key(&mut res, "conn", &conn);
        Self::append_key(&mut res, "systemName", system_id);
        Self::append_key(&mut res, "clnt", client);
        Self::append_key(&mut res, "user", user);
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
            res.push_str(&format!("{key}={val}"));
        }
    }
}
