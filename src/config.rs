use dotenv::dotenv;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub port: u16,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env");
        let jwt_maxage = std::env::var("JWT_MAXAGE")
            .expect("JWT_MAXAGE must be set in .env")
            .parse::<i64>()
            .expect("JWT_MAXAGE must be a valid integer");

        Config {
            database_url,
            jwt_secret,
            jwt_maxage,
            port: 8080,
        }
    }
}
