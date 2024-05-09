use std::fmt::Display;

pub trait DataDisplay {
    fn message(&self) -> impl Display;
    fn values(&self) -> impl IntoIterator<Item = impl Display>;
}
