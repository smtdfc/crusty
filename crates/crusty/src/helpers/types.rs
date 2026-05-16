use moka::future::Cache;
use std::sync::{Arc, LazyLock, RwLock};
use tokio::sync::Mutex;

pub type LazyCacheLock<K, T> = LazyLock<Cache<K, Arc<Mutex<T>>>>;
pub type LazyRwLock<T> = LazyLock<RwLock<T>>;
pub type ArcMutex<T> = Arc<Mutex<T>>;

pub fn new_arc_mutex<T>(data: T) -> ArcMutex<T> {
    return Arc::new(Mutex::new(data));
}
