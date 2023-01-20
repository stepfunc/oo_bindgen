pub(crate) trait FutureType<V> {
    fn on_drop() -> V;
    fn complete(self, result: V);
}

pub(crate) struct Promise<T, V> where T: FutureType<V> {
    inner: Option<T>,
    _v : std::marker::PhantomData<V>,
}

impl<T,V> Drop for Promise<T, V> where T: FutureType<V> {
    fn drop(&mut self) {
        if let Some(cb) = self.inner.take() {
            cb.complete(T::on_drop());
        }
    }
}

impl<T,V> Promise<T, V> where T: FutureType<V> {
    pub(crate) fn new(inner: T) -> Self {
        Self {
            inner: Some(inner),
            _v : Default::default(),
        }
    }

    pub(crate) fn complete(mut self, result: V) {
        if let Some(x) = self.inner.take() {
            x.complete(result);
        }
    }
}


