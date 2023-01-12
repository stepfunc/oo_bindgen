pub(crate) trait FutureInterface : Sized  + Send + 'static {
    type Value;
    type Error;

    const ERROR_ON_DROP : Self::Error;

    fn success(&self, value: Self::Value);
    fn error(&self, err: Self::Error);
}

pub(crate) trait Promise<T> : Send + 'static {
    fn complete(&mut self, value: T);
}

pub(crate) fn wrap<V, E>(interface: impl FutureInterface<Error=E, Value=V>) -> impl Promise<Result<V, E>> {
    PromiseImpl {
        inner: Some(interface),
    }
}

struct PromiseImpl<T>  where T: FutureInterface {
    inner: Option<T>,
}

impl<T> Promise<Result<T::Value, T::Error>> for PromiseImpl<T> where T: FutureInterface {
    fn complete(&mut self, res: Result<T::Value, T::Error>) {
        if let Some(x) = self.inner.take() {
            match res {
                Ok(v) => x.success(v),
                Err(err) => x.error(err),
            }
        }
    }
}

impl<T> Drop for PromiseImpl<T> where T: FutureInterface {
    fn drop(&mut self) {
        if let Some(x) = self.inner.take() {
            x.error(T::ERROR_ON_DROP);
        }
    }
}
