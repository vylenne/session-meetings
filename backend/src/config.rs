use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub app_jwt_secret: String,
    pub jitsi_jwt_app_id: String,
    pub jitsi_jwt_app_secret: String,
    pub jitsi_domain: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            app_jwt_secret: env::var("APP_JWT_SECRET").expect("APP_JWT_SECRET must be set"),
            jitsi_jwt_app_id: env::var("JITSI_JWT_APP_ID").unwrap_or_else(|_| "session_meeting".into()),
            jitsi_jwt_app_secret: env::var("JITSI_JWT_APP_SECRET").expect("JITSI_JWT_APP_SECRET must be set"),
            jitsi_domain: env::var("JITSI_DOMAIN").unwrap_or_else(|_| "meet.localhost".into()),
        }
    }
}
