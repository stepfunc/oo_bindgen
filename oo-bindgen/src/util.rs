pub struct WithLast<I>
where
    I: Iterator,
{
    iter: I,
    next: Option<I::Item>,
}

impl<I> WithLast<I>
where
    I: Iterator,
{
    fn drop_last(self) -> DropLast<I> {
        DropLast { inner: self }
    }
}

pub struct DropLast<I>
where
    I: Iterator,
{
    inner: WithLast<I>,
}

impl<I> Iterator for DropLast<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            None => None,
            Some((item, last)) => {
                if last {
                    None
                } else {
                    Some(item)
                }
            }
        }
    }
}

pub trait WithLastIndication: Iterator + Sized {
    fn with_last(self) -> WithLast<Self>;

    fn drop_last(self) -> DropLast<Self>;
}

impl<I> WithLastIndication for I
where
    I: Iterator,
{
    fn with_last(mut self) -> WithLast<Self> {
        let first = self.next();
        WithLast {
            iter: self,
            next: first,
        }
    }

    fn drop_last(self) -> DropLast<I> {
        self.with_last().drop_last()
    }
}

impl<I> Iterator for WithLast<I>
where
    I: Iterator,
{
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            None => None,
            Some(e) => match self.iter.next() {
                None => Some((e, true)),
                Some(f) => {
                    self.next = Some(f);
                    Some((e, false))
                }
            },
        }
    }
}
