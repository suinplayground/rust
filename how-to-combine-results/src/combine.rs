fn concat<T>(v1: Vec<T>, v2: Vec<T>) -> Vec<T> {
    let mut v = Vec::new();
    v.extend(v1);
    v.extend(v2);
    v
}

pub trait IntoMerge<T1, E> {
    fn into_merge(self) -> Result<(T1,), Vec<E>>;
}

impl<T1, E> IntoMerge<T1, E> for Result<T1, E> {
    fn into_merge(self) -> Result<(T1,), Vec<E>> {
        match self {
            Ok(v1) => Ok((v1,)),
            Err(e1) => Err(vec![e1]),
        }
    }
}

pub trait Merge2<T1, T2, E> {
    fn merge(self, other: Result<T2, E>) -> Result<(T1, T2), Vec<E>>;
}

impl<T1, T2, E> Merge2<T1, T2, E> for Result<(T1,), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T2, E>) -> Result<(T1, T2), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1,)), Ok((v2,))) => Ok((v1, v2)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge3<T1, T2, T3, E> {
    fn merge(self, other: Result<T3, E>) -> Result<(T1, T2, T3), Vec<E>>;
}

impl<T1, T2, T3, E> Merge3<T1, T2, T3, E> for Result<(T1, T2), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T3, E>) -> Result<(T1, T2, T3), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2)), Ok((v3,))) => Ok((v1, v2, v3)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge4<T1, T2, T3, T4, E> {
    fn merge(self, other: Result<T4, E>) -> Result<(T1, T2, T3, T4), Vec<E>>;
}

impl<T1, T2, T3, T4, E> Merge4<T1, T2, T3, T4, E> for Result<(T1, T2, T3), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T4, E>) -> Result<(T1, T2, T3, T4), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3)), Ok((v4,))) => Ok((v1, v2, v3, v4)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge5<T1, T2, T3, T4, T5, E> {
    fn merge(self, other: Result<T5, E>) -> Result<(T1, T2, T3, T4, T5), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, E> Merge5<T1, T2, T3, T4, T5, E> for Result<(T1, T2, T3, T4), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T5, E>) -> Result<(T1, T2, T3, T4, T5), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4)), Ok((v5,))) => Ok((v1, v2, v3, v4, v5)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge6<T1, T2, T3, T4, T5, T6, E> {
    fn merge(self, other: Result<T6, E>) -> Result<(T1, T2, T3, T4, T5, T6), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, T6, E> Merge6<T1, T2, T3, T4, T5, T6, E>
    for Result<(T1, T2, T3, T4, T5), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T6, E>) -> Result<(T1, T2, T3, T4, T5, T6), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4, v5)), Ok((v6,))) => Ok((v1, v2, v3, v4, v5, v6)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge7<T1, T2, T3, T4, T5, T6, T7, E> {
    fn merge(self, other: Result<T7, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, T6, T7, E> Merge7<T1, T2, T3, T4, T5, T6, T7, E>
    for Result<(T1, T2, T3, T4, T5, T6), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T7, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4, v5, v6)), Ok((v7,))) => Ok((v1, v2, v3, v4, v5, v6, v7)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge8<T1, T2, T3, T4, T5, T6, T7, T8, E> {
    fn merge(self, other: Result<T8, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, E> Merge8<T1, T2, T3, T4, T5, T6, T7, T8, E>
    for Result<(T1, T2, T3, T4, T5, T6, T7), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T8, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4, v5, v6, v7)), Ok((v8,))) => Ok((v1, v2, v3, v4, v5, v6, v7, v8)),
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge9<T1, T2, T3, T4, T5, T6, T7, T8, T9, E> {
    fn merge(self, other: Result<T9, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, E> Merge9<T1, T2, T3, T4, T5, T6, T7, T8, T9, E>
    for Result<(T1, T2, T3, T4, T5, T6, T7, T8), Vec<E>>
where
    E: Clone,
{
    fn merge(self, other: Result<T9, E>) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4, v5, v6, v7, v8)), Ok((v9,))) => {
                Ok((v1, v2, v3, v4, v5, v6, v7, v8, v9))
            }
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}

pub trait Merge10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, E> {
    fn merge(
        self,
        other: Result<T10, E>,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Vec<E>>;
}

impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, E> Merge10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, E>
    for Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9), Vec<E>>
where
    E: Clone,
{
    fn merge(
        self,
        other: Result<T10, E>,
    ) -> Result<(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Vec<E>> {
        match (self, other.into_merge()) {
            (Ok((v1, v2, v3, v4, v5, v6, v7, v8, v9)), Ok((v10,))) => {
                Ok((v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
            }
            (Err(e1), Err(e2)) => Err(concat(e1, e2)),
            (_, Err(e)) | (Err(e), _) => Err(e),
        }
    }
}
