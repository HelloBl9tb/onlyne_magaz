use crate::models::User;
use crate::repository::UserRepository;

pub struct UserService {
    repository: UserRepository,
}

impl UserService {
    pub async fn new() -> Self {
        Self {
            repository: UserRepository::new().await,
        }
    }

    pub async fn get_user(&self, id: i32) -> Option<User> {
        self.repository.get_user(id).await
    }
}