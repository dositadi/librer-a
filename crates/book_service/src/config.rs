use envconfig::Envconfig;

#[derive(Clone, Envconfig)]
pub struct AppConf {
    #[envconfig(nested)]
    pub server: ServerConf,

    #[envconfig(nested)]
    pub db: DBConf,
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

#[derive(Clone, Envconfig)]
pub struct DBConf {
    #[envconfig(from = "DB_PROTOCOL", default = "postgres")]
    pub protocol: String,

    #[envconfig(from = "DB_HOST")]
    pub host: String,

    #[envconfig(from = "DB_PORT", default = "5432")]
    pub port: u16,

    #[envconfig(from = "POSTGRES_DB")]
    pub db_name: String,

    #[envconfig(from = "POSTGRES_USER")]
    pub user: String,

    #[envconfig(from = "POSTGRES_PASSWORD")]
    pub password: String,
}

impl DBConf {
    pub fn init() -> Self {
        DBConf::init_from_env().expect("Failed to load environment variables")
    }

    pub fn into_db_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.protocol,
            self.user,
            self.password,
            self.host,
            self.port,
            self.db_name
        )
    }
}
