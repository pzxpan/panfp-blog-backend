use deadpool_postgres::Pool;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub log: slog::Logger,
}
