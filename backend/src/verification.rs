use std::time::{Duration, Instant};

use crate::{email, models};
use eyre::Result;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub enum Kind {
    Register(models::User),
    PasswordChange(String),
}

#[derive(Debug, Clone)]
pub struct Request {
    code: u16,
    kind: Kind,
    created: Instant,
}

impl Request {
    pub async fn new(kind: Kind, email: &str) -> Result<Self, crate::Error> {
        let code: u16 = rand::random_range(0..9999);
        email::send(email, "Email Verification", format!("Your code: {code}")).await?;

        Ok(Self {
            code,
            kind,
            created: Instant::now(),
        })
    }

    pub fn expired(&self) -> bool {
        const TIMEOUT: Duration = Duration::from_secs(480);
        self.created.elapsed() > TIMEOUT
    }
}

pub struct Verifications {
    inner: RwLock<Vec<Request>>,
}

impl std::fmt::Debug for Verifications {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.try_read() {
            Ok(guard) => f
                .debug_struct("Verifications")
                .field("inner", &*guard)
                .finish(),
            Err(_) => f.write_str("Verifications { inner: <locked> }"),
        }
    }
}

impl Verifications {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Vec::new()),
        }
    }

    pub async fn index(&self, code: u16) -> Option<usize> {
        let lock = self.inner.read().await;
        lock.iter().position(|x| x.code == code)
    }

    pub async fn registration(&self, user: models::User) -> Result<String, crate::Error> {
        let email = user.email.clone();
        let request = Request::new(Kind::Register(user), &email).await?;
        self.inner.write().await.push(request);

        Ok(email)
    }

    pub async fn verify_registration(&self, code: u16) -> Option<models::User> {
        let idx = self.index(code).await?;
        let mut lock = self.inner.write().await;
        let Kind::Register(user) = lock.remove(idx).kind else {
            return None;
        };

        Some(user)
    }

    pub async fn prune(&self) {
        let mut lock = self.inner.write().await;
        lock.retain(|x| !x.expired());
    }
}
