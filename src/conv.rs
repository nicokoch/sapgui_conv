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
        let mut res = String::new();

        if !self.router.is_empty() {
            if self.router.starts_with(Self::HOST_PREFIX) {
                res.push_str(&self.router);
            } else {
                res.push_str(&format!("{}{}", Self::HOST_PREFIX, self.router));
            }
            res.push_str(&format!("{}3299", Self::PORT_PREFIX));
        }

        if !self.appl_server.is_empty() {
            res.push_str(&format!("{}{}", Self::HOST_PREFIX, self.appl_server));
            if self.instance_id.is_empty() {
                res.push_str(&format!("{}3200", Self::PORT_PREFIX));
            } else {
                res.push_str(&format!("{}32{}", Self::PORT_PREFIX, self.instance_id));
            }
            if !self.system_id.is_empty() {
                res.push_str(&format!("&systemName={}", self.system_id));
            }
        }

        if !self.client.is_empty() {
            res.push_str(&format!("&clnt={}", self.client));
        }
        if !self.user.is_empty() {
            res.push_str(&format!("&user={}", self.user));
        }
        res
    }
}
