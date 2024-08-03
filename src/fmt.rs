use std::io::{self, stdout};
use std::marker::PhantomData;

use serde::Serialize;

pub enum TermFmt<Data, Bundle> {
    Plain(PlainFmt<Data>),
    Interactive(InteractiveFmt<Data>),
    Json(JsonFmt<Bundle>),
    Csv(CsvFmt<Bundle>),
}

pub struct PlainFmt<Data> {
    data: PhantomData<Data>,
}

pub struct InteractiveFmt<Data> {
    data: PhantomData<Data>,
}

pub struct JsonFmt<Bundle> {
    bundle: Bundle,
}

pub struct CsvFmt<Bundle> {
    bundle: Bundle,
}

pub trait DataFmt {
    fn plain(self);
    fn interactive(self);
}

pub trait BundleFmt: Serialize + Sized {
    type Data;
    type Config;

    fn new(config: Self::Config) -> Self;

    fn push(&mut self, value: Self::Data);
    fn clear(&mut self);

    fn json<Writer>(&self, writer: Writer) -> eyre::Result<()>
    where
        Writer: io::Write,
    {
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    fn csv<Writer>(&self, mut writer: csv::Writer<Writer>) -> eyre::Result<()>
    where
        Writer: io::Write,
    {
        writer.serialize(self)?;
        Ok(())
    }
}

impl<Data, Bundle> TermFmt<Data, Bundle>
where
    Data: DataFmt,
    Bundle: Serialize,
    Bundle: BundleFmt<Data = Data>,
{
    pub fn plain() -> Self {
        Self::Plain(PlainFmt {
            data: PhantomData::default(),
        })
    }

    pub fn interactive() -> Self {
        Self::Interactive(InteractiveFmt {
            data: PhantomData::default(),
        })
    }

    pub fn json(bundle: Bundle) -> Self {
        Self::Json(JsonFmt::new(bundle))
    }

    pub fn csv(bundle: Bundle) -> Self {
        Self::Csv(CsvFmt::new(bundle))
    }

    pub fn output(&mut self, data: Data) {
        match self {
            Self::Plain(_) => data.plain(),
            Self::Interactive(_) => data.interactive(),
            Self::Json(fmt) => fmt.output(data),
            Self::Csv(fmt) => fmt.output(data),
        }
    }

    pub fn flush(&mut self) -> eyre::Result<()> {
        match self {
            Self::Json(fmt) => fmt.flush(),
            Self::Csv(fmt) => fmt.flush(),
            _ => Ok(()),
        }
    }
}

impl<Bundle, Data> JsonFmt<Bundle>
where
    Bundle: Serialize,
    Bundle: BundleFmt<Data = Data>,
{
    pub fn new(unit: Bundle) -> Self {
        Self { bundle: unit }
    }

    pub fn output(&mut self, data: Data) {
        self.bundle.push(data);
    }

    pub fn flush(&mut self) -> eyre::Result<()> {
        self.bundle.json(stdout())?;
        self.bundle.clear();
        Ok(())
    }
}

impl<Bundle, Data> CsvFmt<Bundle>
where
    Bundle: Serialize,
    Bundle: BundleFmt<Data = Data>,
{
    pub fn new(unit: Bundle) -> Self {
        Self { bundle: unit }
    }

    pub fn output(&mut self, data: Data) {
        self.bundle.push(data);
    }

    pub fn flush(&mut self) -> eyre::Result<()> {
        let writer = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(stdout());
        self.bundle.csv(writer)?;
        self.bundle.clear();
        Ok(())
    }
}
