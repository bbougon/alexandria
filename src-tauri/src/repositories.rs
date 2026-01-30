use crate::collections::collections::{CollectionRepository, CollectionRepositoryMemory};
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;

pub struct Repositories {
    collections: Arc<dyn CollectionRepository>,
}

impl Repositories {
    pub fn new(collections_repo: Arc<dyn CollectionRepository>) -> Self {
        Self {
            collections: collections_repo,
        }
    }

    pub fn collections(&self) -> Arc<dyn CollectionRepository> {
        Arc::clone(&self.collections)
    }
}

static REPOSITORIES: Lazy<RwLock<Arc<Repositories>>> = Lazy::new(|| {
    let default = Repositories::new(Arc::new(CollectionRepositoryMemory::new()));
    RwLock::new(Arc::new(default))
});

#[cfg(test)]
thread_local! {
    static TEST_REPOSITORIES: std::cell::RefCell<Option<Arc<Repositories>>> =
        const { std::cell::RefCell::new(None) };
}

pub fn repositories() -> Arc<Repositories> {
    #[cfg(test)]
    if let Some(r) = TEST_REPOSITORIES.with(|cell| cell.borrow().clone()) {
        return r;
    }

    Arc::clone(&REPOSITORIES.read())
}

pub fn set_repositories(new_repos: Repositories) {
    *REPOSITORIES.write() = Arc::new(new_repos);
}

#[cfg(test)]
pub struct RepositoriesGuard {
    previous: Option<Arc<Repositories>>,
}

#[cfg(test)]
impl Drop for RepositoriesGuard {
    fn drop(&mut self) {
        TEST_REPOSITORIES.with(|cell| {
            *cell.borrow_mut() = self.previous.take();
        });
    }
}

#[cfg(test)]
pub fn with_test_repositories(new_repos: Repositories) -> RepositoriesGuard {
    let new_arc = Arc::new(new_repos);
    let previous = TEST_REPOSITORIES.with(|cell| cell.borrow_mut().replace(new_arc));
    RepositoriesGuard { previous }
}
