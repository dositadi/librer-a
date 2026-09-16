use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub server: ServerConf,
}

impl AppConf {
    pub fn init() -> Self {
        Self::init_from_env().expect("Failed to load configuration!, check the .env file")
    }
}

#[derive(Clone, Envconfig)]
pub struct ServerConf {
    #[envconfig(from = "SERVER_PORT")]
    pub port: u16,

    #[envconfig(from = "SERVER_ALLOWED_METHODS")]
    pub allowed_methods: String,

    #[envconfig(from = "SERVER_ALLOWED_ORIGINS")]
    pub allowed_origins: String,

    #[envconfig(from = "SERVER_ALLOWED_HEADERS")]
    pub allowed_headers: String,

    #[envconfig(from = "SERVER_DEFAULT_BODY_LIMIT")]
    pub default_body_limits: usize,
}

impl ServerConf {
    pub fn to_addr(&self) -> String {
        format!("0.0.0.0:{}", self.port)
    }
}