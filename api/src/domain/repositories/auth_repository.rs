pub trait AuthRepository {
    fn create_authentication(&self) -> anyhow::Result<()>;
}
