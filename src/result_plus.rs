
pub trait OnError<E> {
    fn on_error<F: FnOnce(&E)>(self, f: F) -> Self;
}

impl<T, E> OnError<E> for Result<T, E> {
    fn on_error<F: FnOnce(&E)>(self, f: F) -> Self {
        match &self {
            Ok(_) => self,
            Err(e) => {
                f(e);
                self
            },
        }
    }
}

impl<T> OnError<()> for Option<T> {
    fn on_error<F: FnOnce(&())>(self, f: F) -> Self {
        match &self {
            Some(_) => self,
            None => {
                f(&());
                self
            },
        }
    }
}
