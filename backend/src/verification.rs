use crate::{email, models::User};
use tokio::sync::Mutex;

pub struct Verifications {
    inner: Mutex<Vec<(u16, User)>>,
}

impl Verifications {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Vec::new()),
        }
    }

    pub async fn registration(&self, user: User) -> Result<String, crate::Error> {
        let code: u16 = rand::random_range(0..9999);
        email::send(
            &user.email,
            "Email Verification",
            format!("Your code: {code}"),
        )
        .await?;

        let email = user.email.clone();
        self.inner.lock().await.push((code, user));

        Ok(email)
    }

    pub async fn verify(&self, code: u16) -> Option<User> {
        let mut lock = self.inner.lock().await;
        let idx = lock.iter().position(|x| x.0 == code)?;

        let user = lock.remove(idx).1;
        Some(user)
    }
}
