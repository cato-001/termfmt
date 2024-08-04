use std::io::stdout;

use serde::Serialize;

pub enum TermFmt<Bundle> {
    Direct(DirectTermFmt),
    Bundled(BundledTermFmt, Bundle),
}

pub enum DirectTermFmt {
    Plain,
    Interactive,
}

pub enum BundledTermFmt {
    Json,
}

pub trait BundleFmt: Serialize + Sized {
    type Config;

    fn new(config: Self::Config) -> Self;

    fn clear(&mut self);
}

impl<Bundle> TermFmt<Bundle>
where
    Bundle: Serialize,
    Bundle: BundleFmt,
{
    pub fn plain() -> Self {
        Self::Direct(DirectTermFmt::Plain)
    }

    pub fn interactive() -> Self {
        Self::Direct(DirectTermFmt::Interactive)
    }

    pub fn json(bundle: Bundle) -> Self {
        Self::Bundled(BundledTermFmt::Json, bundle)
    }

    pub fn push(&mut self, modify: impl Fn(&mut Bundle)) {
        let TermFmt::Bundled(_, bundle) = self else {
            return;
        };
        modify(bundle);
    }

    pub fn flush(&mut self) -> eyre::Result<()> {
        let TermFmt::Bundled(fmt, bundle) = self else {
            return Ok(());
        };
        match fmt {
            BundledTermFmt::Json => serde_json::to_writer(stdout(), bundle)?,
        }
        bundle.clear();
        Ok(())
    }
}
