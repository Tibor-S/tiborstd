use std::{
    ops::{Deref, DerefMut},
    rc::{self, Rc},
    sync::{
        self, Arc, LockResult, Mutex, MutexGuard, RwLock, RwLockReadGuard,
        RwLockWriteGuard, TryLockError,
    },
};

pub type RcR<T> = Rc<T>;
pub type RcRW<T> = Rc<RwLock<T>>;
pub type RcM<T> = Rc<Mutex<T>>;
pub type ArcR<T> = Arc<T>;
pub type ArcRW<T> = Arc<RwLock<T>>;
pub type ArcM<T> = Arc<Mutex<T>>;

pub trait Readable<T> {
    type ReadGuard<'a>: Deref<Target = T>
    where
        Self: 'a;
    type TryGuard<G>;
    type Weak: WeakContainer<Strong = Self>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>>;
    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>>;

    fn read<'a>(&'a self) -> Self::ReadGuard<'a>
    where
        Self::TryGuard<Self::ReadGuard<'a>>: Recoverable<Self::ReadGuard<'a>>,
    {
        self.try_read().recover()
    }

    fn read_non_blocking<'a>(&'a self) -> Option<Self::ReadGuard<'a>>
    where
        Self::TryGuard<Self::ReadGuard<'a>>: Recoverable<Self::ReadGuard<'a>>,
    {
        self.try_read_non_blocking().map(|res| res.recover())
    }
}

impl<T> Readable<T> for Rc<T> {
    type ReadGuard<'a>
        = &'a T
    where
        Self: 'a;
    type TryGuard<G> = G;
    type Weak = rc::Weak<T>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        self
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        Some(self)
    }
}

impl<T> Readable<T> for Rc<RwLock<T>> {
    type ReadGuard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;
    type Weak = rc::Weak<RwLock<T>>;

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

impl<T> Readable<T> for Rc<Mutex<T>> {
    type ReadGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;
    type Weak = rc::Weak<Mutex<T>>;

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

impl<T> Readable<T> for Arc<T> {
    type ReadGuard<'a>
        = &'a T
    where
        Self: 'a;
    type TryGuard<G> = G;
    type Weak = sync::Weak<T>;

    fn try_read<'a>(&'a self) -> Self::TryGuard<Self::ReadGuard<'a>> {
        self
    }

    fn try_read_non_blocking<'a>(&'a self) -> Option<Self::TryGuard<Self::ReadGuard<'a>>> {
        Some(self)
    }
}

impl<T> Readable<T> for Arc<RwLock<T>> {
    type ReadGuard<'a>
        = RwLockReadGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;
    type Weak = sync::Weak<RwLock<T>>;

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

impl<T> Readable<T> for Arc<Mutex<T>> {
    type ReadGuard<'a>
        = MutexGuard<'a, T>
    where
        Self: 'a;
    type TryGuard<G> = LockResult<G>;
    type Weak = sync::Weak<Mutex<T>>;

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

    fn write<'a>(&'a self) -> Self::WriteGuard<'a>
    where
        Self::TryGuard<Self::WriteGuard<'a>>: Recoverable<Self::WriteGuard<'a>>,
    {
        self.try_write().recover()
    }

    fn write_non_blocking<'a>(&'a self) -> Option<Self::WriteGuard<'a>>
    where
        Self::TryGuard<Self::WriteGuard<'a>>: Recoverable<Self::WriteGuard<'a>>,
    {
        self.try_write_non_blocking().map(|res| res.recover())
    }
}

impl<T> Writable<T> for Rc<RwLock<T>> {
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

impl<T> Writable<T> for Rc<Mutex<T>> {
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

impl<T> Writable<T> for Arc<RwLock<T>> {
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

impl<T> Writable<T> for Arc<Mutex<T>> {
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

pub trait WeakContainer {
    type Strong;

    fn upgrade(&self) -> Option<Self::Strong>;
}

impl<T> WeakContainer for rc::Weak<T> {
    type Strong = Rc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}

impl<T> WeakContainer for sync::Weak<T> {
    type Strong = Arc<T>;

    fn upgrade(&self) -> Option<Self::Strong> {
        self.upgrade()
    }
}
