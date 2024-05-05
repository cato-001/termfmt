use std::fmt::Display;

pub struct Lines<Value> {
    value: Value,
}

impl<Display0, Display1> Lines<(Display0, Display1)>
where
    Display0: Display,
    Display1: Display,
{
    pub fn new(value: (Display0, Display1)) -> Self {
        Self { value }
    }
}

impl<Display0, Display1> Display for Lines<(Display0, Display1)>
where
    Display0: Display,
    Display1: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.value.0)?;
        write!(f, "{}", self.value.1)
    }
}

impl<Display0, Display1, Display2> Lines<(Display0, Display1, Display2)>
where
    Display0: Display,
    Display1: Display,
    Display2: Display,
{
    pub fn new(value: (Display0, Display1, Display2)) -> Self {
        Self { value }
    }
}

impl<Display0, Display1, Display2> Display for Lines<(Display0, Display1, Display2)>
where
    Display0: Display,
    Display1: Display,
    Display2: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.value.0)?;
        writeln!(f, "{}", self.value.1)?;
        write!(f, "{}", self.value.2)
    }
}
