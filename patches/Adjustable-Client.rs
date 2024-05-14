#[derive(Debug)]
struct ClientWrapper(reqwest::Client);
impl std::ops::Deref for ClientWrapper{
    type Target = reqwest::Client;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub trait Client: std::ops::Deref + std::fmt::Debug {}
impl<T:std::ops::Deref + std::fmt::Debug> Client for T {}