use std::fmt::Display;

pub(crate) struct SeparatedValuesFmt<Value> {
    value: Value,
}

impl<Value, Item> Display for SeparatedValuesFmt<Value>
where
    Value: Iterator<Item = Item>,
    Item: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for item in self.value {
            write!(f, "{}", item);
        }
        Ok(())
    }
}
