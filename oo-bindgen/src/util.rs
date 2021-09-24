pub struct WithLast<I>
where
    I: Iterator,
{
    iter: I,
    next: Option<I::Item>,
}

pub trait WithLastIndication: Iterator + Sized {
    fn with_last(self) -> WithLast<Self>;
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
