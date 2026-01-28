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

pub fn repositories() -> Arc<Repositories> {
    Arc::clone(&REPOSITORIES.read())
}

pub fn set_repositories(new_repos: Repositories) {
    *REPOSITORIES.write() = Arc::new(new_repos);
}
