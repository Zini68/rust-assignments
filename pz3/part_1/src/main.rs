use std::borrow::Cow;
use std::collections::HashMap;

trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}

#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u64,
    email: Cow<'static, str>,
    activated: bool,
}

struct InMemoryStorage {
    data: HashMap<u64, User>,
}

impl InMemoryStorage {
    fn new() -> Self {
        Self { data: HashMap::new() }
    }
}

impl Storage<u64, User> for InMemoryStorage {
    fn set(&mut self, key: u64, val: User) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &u64) -> Option<&User> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &u64) -> Option<User> {
        self.data.remove(key)
    }
}

struct UserRepositoryStatic<S: Storage<u64, User>> {
    storage: S,
}

impl<S: Storage<u64, User>> UserRepositoryStatic<S> {
    fn new(storage: S) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }

    fn update(&mut self, id: u64, mut user: User) {
        user.id = id;
        self.storage.set(id, user);
    }
}

struct UserRepositoryDynamic {
    storage: Box<dyn Storage<u64, User>>,
}

impl UserRepositoryDynamic {
    fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        Self { storage }
    }

    fn add(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    fn get(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn remove(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }

    fn update(&mut self, id: u64, mut user: User) {
        user.id = id;
        self.storage.set(id, user);
    }
}

fn main() {
    println!("Part 1 implementation.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_dispatch() {
        let storage = InMemoryStorage::new();
        let mut repo = UserRepositoryStatic::new(storage);
        
        let user = User { id: 1, email: Cow::Borrowed("test@test.com"), activated: true };
        repo.add(user.clone());
        
        assert_eq!(repo.get(1), Some(&user));
        
        let removed = repo.remove(1);
        assert_eq!(removed, Some(user));
        assert_eq!(repo.get(1), None);
    }

    #[test]
    fn test_dynamic_dispatch() {
        let storage = Box::new(InMemoryStorage::new());
        let mut repo = UserRepositoryDynamic::new(storage);
        
        let user = User { id: 2, email: Cow::Borrowed("dyn@test.com"), activated: false };
        repo.add(user.clone());
        
        assert_eq!(repo.get(2), Some(&user));
        
        let removed = repo.remove(2);
        assert_eq!(removed, Some(user));
        assert_eq!(repo.get(2), None);
    }
}
