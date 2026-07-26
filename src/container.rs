use std::{
    ops::{Deref, DerefMut},
    rc::{self, Rc},
    sync::{
        self, Arc, LockResult, Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
        TryLockError,
    },
};

#[derive(Default, Debug, Clone)]
pub struct RcR<T>(Rc<T>);

impl<T> RcR<T> {
    pub fn new(value: T) -> Self {
        RcR(Rc::new(value))
    }
}

impl<T> Deref for RcR<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RcR<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkRcR<T>(rc::Weak<T>);

impl<T> WkRcR<T> {
    pub fn new() -> Self {
        WkRcR(rc::Weak::new())
    }
}

impl<T> Deref for WkRcR<T> {
    type Target = rc::Weak<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkRcR<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct RcRW<T>(Rc<RwLock<T>>);

impl<T> RcRW<T> {
    pub fn new(value: T) -> Self {
        RcRW(Rc::new(RwLock::new(value)))
    }
}

impl<T> Deref for RcRW<T> {
    type Target = Rc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RcRW<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkRcRW<T>(rc::Weak<RwLock<T>>);

impl<T> WkRcRW<T> {
    pub fn new() -> Self {
        WkRcRW(rc::Weak::new())
    }
}

impl<T> Deref for WkRcRW<T> {
    type Target = rc::Weak<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkRcRW<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct RcM<T>(Rc<Mutex<T>>);

impl<T> RcM<T> {
    pub fn new(value: T) -> Self {
        RcM(Rc::new(Mutex::new(value)))
    }
}

impl<T> Deref for RcM<T> {
    type Target = Rc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RcM<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkRcM<T>(rc::Weak<Mutex<T>>);

impl<T> WkRcM<T> {
    pub fn new() -> Self {
        WkRcM(rc::Weak::new())
    }
}

impl<T> Deref for WkRcM<T> {
    type Target = rc::Weak<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkRcM<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct ArcR<T>(Arc<T>);

impl<T> ArcR<T> {
    pub fn new(value: T) -> Self {
        ArcR(Arc::new(value))
    }
}

impl<T> Deref for ArcR<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ArcR<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkArcR<T>(sync::Weak<T>);

impl<T> WkArcR<T> {
    pub fn new() -> Self {
        WkArcR(sync::Weak::new())
    }
}

impl<T> Deref for WkArcR<T> {
    type Target = sync::Weak<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkArcR<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct ArcRW<T>(Arc<RwLock<T>>);

impl<T> ArcRW<T> {
    pub fn new(value: T) -> Self {
        ArcRW(Arc::new(RwLock::new(value)))
    }
}

impl<T> Deref for ArcRW<T> {
    type Target = Arc<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ArcRW<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkArcRW<T>(sync::Weak<RwLock<T>>);

impl<T> WkArcRW<T> {
    pub fn new() -> Self {
        WkArcRW(sync::Weak::new())
    }
}

impl<T> Deref for WkArcRW<T> {
    type Target = sync::Weak<RwLock<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkArcRW<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct ArcM<T>(Arc<Mutex<T>>);

impl<T> ArcM<T> {
    pub fn new(value: T) -> Self {
        ArcM(Arc::new(Mutex::new(value)))
    }
}

impl<T> Deref for ArcM<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ArcM<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Debug, Clone)]
pub struct WkArcM<T>(sync::Weak<Mutex<T>>);

impl<T> WkArcM<T> {
    pub fn new() -> Self {
        WkArcM(sync::Weak::new())
    }
}

impl<T> Deref for WkArcM<T> {
    type Target = sync::Weak<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for WkArcM<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait Readable<T> {
    type Resolved;
    type ReadGuard<'a>: Deref<Target = T>
    where
        Self: 'a;
    type TryGuard<G>: Recoverable<G>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>>;
    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>>;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a> {
        self.try_read().recover()
    }

    fn read_non_blocking<'a>(&'a self) -> Option<Self::ReadGuard<'a>> {
        self.try_read_non_blocking().map(|res| res.recover())
    }
}

impl<T> Readable<T> for T {
    type Resolved = T;
    type ReadGuard<'a>
        = &'a T
    where
        Self: 'a;
    type TryGuard<G> = G;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        self
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        Some(self)
    }
}

impl<T> Readable<T> for RcR<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = &'a T
    where
        Self: 'a;
    type TryGuard<G> = G;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        self
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        Some(self)
    }
}

impl<T> Readable<T> for RcRW<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        RwLock::read(self)
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        match RwLock::try_read(self) {
            Ok(rw) => Some(Ok(rw)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Readable<T> for RcM<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        Mutex::lock(self)
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        match Mutex::try_lock(self) {
            Ok(m) => Some(Ok(m)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Readable<T> for ArcR<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = &'a T
    where
        Self: 'a;
    type TryGuard<G> = G;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        self
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        Some(self)
    }
}

impl<T> Readable<T> for ArcRW<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        RwLock::read(self)
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        match RwLock::try_read(self) {
            Ok(rw) => Some(Ok(rw)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Readable<T> for ArcM<T> {
    type Resolved = T;
    type ReadGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        Mutex::lock(self)
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        match Mutex::try_lock(self) {
            Ok(m) => Some(Ok(m)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

pub trait Writable<T>: Readable<T> {
    type WriteGuard<'a>: DerefMut<Target = T>
    where
        Self: 'a;

    fn try_write<'a>(&'a self) -> Self::TryGuard<Self::WriteGuard<'a>>;
    fn try_write_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::WriteGuard<'a>>>;

    fn write<'a>(&'a self) -> Self::WriteGuard<'a> {
        self.try_write().recover()
    }

    fn write_non_blocking<'a>(&'a self) -> Option<Self::WriteGuard<'a>> {
        self.try_write_non_blocking().map(|res| res.recover())
    }
}

impl<T> Writable<T> for RcRW<T> {
    type WriteGuard<'a>
        = RwLockWriteGuard<'a, T>
    where
        Self: 'a;

    fn try_write<'a>(&'a self) -> Self::TryGuard<Self::WriteGuard<'a>> {
        RwLock::write(self)
    }

    fn try_write_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::WriteGuard<'a>>> {
        match RwLock::try_write(self) {
            Ok(rw) => Some(Ok(rw)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Writable<T> for RcM<T> {
    type WriteGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;

    fn try_write<'a>(&'a self) -> Self::TryGuard<Self::WriteGuard<'a>> {
        Mutex::lock(self)
    }

    fn try_write_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::WriteGuard<'a>>> {
        match Mutex::try_lock(self) {
            Ok(m) => Some(Ok(m)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Writable<T> for ArcRW<T> {
    type WriteGuard<'a>
        = RwLockWriteGuard<'a, T>
    where
        Self: 'a;

    fn try_write<'a>(&'a self) -> Self::TryGuard<Self::WriteGuard<'a>> {
        RwLock::write(self)
    }

    fn try_write_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::WriteGuard<'a>>> {
        match RwLock::try_write(self) {
            Ok(rw) => Some(Ok(rw)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

impl<T> Writable<T> for ArcM<T> {
    type WriteGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;

    fn try_write<'a>(&'a self) -> Self::TryGuard<Self::WriteGuard<'a>> {
        Mutex::lock(self)
    }

    fn try_write_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::WriteGuard<'a>>> {
        match Mutex::try_lock(self) {
            Ok(m) => Some(Ok(m)),
            Err(TryLockError::Poisoned(p)) => Some(Err(p)),
            Err(TryLockError::WouldBlock) => None,
        }
    }
}

pub trait Recoverable<T> {
    fn recover(self) -> T;
}

impl<T> Recoverable<T> for T {
    fn recover(self) -> T {
        self
    }
}

impl<T> Recoverable<T> for LockResult<T> {
    fn recover(self) -> T {
        self.unwrap_or_else(|e| e.into_inner())
    }
}

pub trait ReadContainer: Readable<Self::Item> {
    type Item;
}

impl<T> ReadContainer for RcR<T> {
    type Item = T;
}

impl<T> ReadContainer for RcRW<T> {
    type Item = T;
}

impl<T> ReadContainer for RcM<T> {
    type Item = T;
}

impl<T> ReadContainer for ArcR<T> {
    type Item = T;
}

impl<T> ReadContainer for ArcRW<T> {
    type Item = T;
}

impl<T> ReadContainer for ArcM<T> {
    type Item = T;
}
