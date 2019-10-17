// homebrew shitty parser combinator

pub struct TryParse<'a, T, E = (), V = ()> {
    head: &'a [T],
    current: &'a [T],
    value: Option<Result<V, E>>,
}

impl<'a, T, E, V> TryParse<'a, T, E, V> {
    fn new(head: &'a [T]) -> Self {
        TryParse {
            head,
            current: head,
            value: None,
        }
    }

    fn or<F, U, E_>(self, f: F) -> TryParse<'a, T, E_, U>
    where
        F: Fn(&'a [T]) -> Result<(U, &'a [T]), E_>,
        U: From<V>,
    {
        match self.value {
            None | Some(Err(_)) => match f(self.current) {
                Ok((t, current)) => TryParse {
                    head: self.head,
                    current,
                    value: Some(Ok(t)),
                },
                Err(e) => TryParse {
                    head: self.head,
                    current: self.current,
                    value: Some(Err(e)),
                },
            },
            Some(Ok(v)) => TryParse {
                head: self.head,
                current: self.current,
                value: Some(Ok(v.into())),
            },
        }
    }

    fn try_parse<F, U, E_>(&self, f: F) -> TryParse<'a, T, E_, U>
    where
        F: Fn(&'a [T]) -> Result<(U, &'a [T]), E_>,
    {
        match f(self.head) {
            Ok((t, current)) => TryParse {
                head: self.head,
                current,
                value: Some(Ok(t)),
            },
            Err(e) => TryParse {
                head: self.head,
                current: self.current,
                value: Some(Err(e)),
            },
        }
    }

    fn many<F, U, E_>(&self, n: usize, f: F) -> TryParse<'a, T, E_, Vec<U>>
    where
        F: Fn(&'a [T]) -> Result<(U, &'a [T]), E_>,
    {
        let current;
        let err;
        let mut items = vec![];
        
        loop {
            let res = self.try_parse::<_, U, _>(&f);
            match res.value {
                Some(Ok(v)) => {
                    items.push(v);
                },
                Some(Err(e)) => {
                    current = res.current;
                    err = e;
                    break;
                },
                _ => {
                    unreachable!()
                }
            }
        }

        if n <= items.len() {
            TryParse {
                head: self.head,
                current,
                value: Some(Ok(items))
            }
        } else {
            TryParse {
                head: self.head,
                current: self.current,
                value: Some(Err(err))
            }
        }
    }

    fn then<F, U, E_>(self, f: F) -> TryParse<'a, T, E_, (V, U)>
    where
        F: Fn(&'a [T]) -> Result<(U, &'a [T]), E_>,
        E_: From<E>,
    {
        match self.value {
            None => {
                panic!("then() called without any value.");
            }
            Some(Err(e)) => TryParse {
                head: self.head,
                current: self.current,
                value: Some(Err(e.into())),
            },
            Some(Ok(v)) => match f(self.current) {
                Ok((u, current)) => TryParse {
                    head: self.head,
                    current,
                    value: Some(Ok((v, u))),
                },
                Err(e) => TryParse {
                    head: self.head,
                    current: self.current,
                    value: Some(Err(e)),
                },
            },
        }
    }

    fn unveil(self) -> Result<(V, &'a [T]), E> {
        match self.value {
            None => panic!("unveil() without any value"),
            Some(Ok(v)) => Ok((v, self.current)),
            Some(Err(e)) => Err(e),
        }
    }
}
