use std::fmt::Display;

pub trait DataDisplay {
    fn plain(&self) -> impl Display;
    fn interactive(&self) -> impl Display;
    fn csv(&self) -> impl Display;
    fn logger(&self) -> impl Display;
}

pub struct ArrayFmt<Value> {
    value: Value,
}

impl<Value, Item> ArrayFmt<Value>
where
    Value: IntoIterator<Item = Item>,
    Item: Display,
{
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl<Value, Item> DataDisplay for ArrayFmt<Value>
where
    Value: IntoIterator<Item = Item>,
    Item: Display,
{
    fn plain(&self) -> impl Display {
        self.value.into_iter()
    }

    fn interactive(&self) -> impl Display {
        todo!()
    }

    fn csv(&self) -> impl Display {
        todo!()
    }

    fn logger(&self) -> impl Display {
        todo!()
    }
}
