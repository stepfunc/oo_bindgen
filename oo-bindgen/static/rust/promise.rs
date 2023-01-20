pub(crate) trait FutureInterface<V, E> : Sized  + Send + Sync + 'static {
    fn dropped() -> E;
    fn complete(&self, result: Result<V,E>);
}

struct DropSafeCell<F, V, E> where F : FutureInterface<V,E> {
    value: Option<F>,
    _v : std::marker::PhantomData<V>,
    _e : std::marker::PhantomData<E>,
}

impl<F, V, E> Drop for DropSafeCell<F, V, E> where F : FutureInterface<V, E> {
    fn drop(&mut self) {
        if let Some(f) = self.value.take() {
            f.complete(Err(F::dropped()));
        }
    }
}

pub(crate) fn make_promise<V, E>(future: impl FutureInterface<V, E>) -> impl FnOnce(Result<V,E>) {
    let mut cell = DropSafeCell {
        value: Some(future),
        _v: Default::default(),
        _e: Default::default(),
    };
    move |res| {
        if let Some(f) = cell.value.take() {
            f.complete(res);
        }
    }
}



