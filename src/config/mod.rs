use dotenvy::dotenv;
use std::env;

pub mod mod_config {
    pub fn load_env() {
        dotenv().ok();
    }

    pub fn database_url() -> String {
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    }

    pub fn jwt_secret() -> String {
        std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
    }
}
