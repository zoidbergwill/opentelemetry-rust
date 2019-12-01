//! # OpenTelemetry Print Exporter
//!
//! This exporter currently delegates to the [rustracing_print library]
//! which implements the [OpenTracing API].
//!
//! [rustracing_print library]: https://github.com/sile/rustracing_print
//! [OpenTracing API]: https://opentracing.io/
use crate::exporter::trace;
use crate::sdk;
use std::any;

/// Print exporter
#[derive(Debug)]
pub struct Exporter {
    pretty_print: bool
}

impl Exporter {
    /// Create a new exporter builder.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Default `Exporter` with initialized sender.
    pub fn init_default() -> Self {
        Exporter::builder().init()
    }
}

impl trace::SpanExporter for Exporter {
    type Span = sdk::Span;

    /// Ignored because spans export themselves on drop currently.
    fn export(&self, _batch: Vec<Self::Span>) -> Result<(), ()> {
        for span  in _batch {
            println!("{:?}", span);
        }
        Ok(())
    }

    /// Ignored for now.
    fn shutdown(&self) {}

    /// Allows `Exporter` to be downcast from trait object.
    fn as_any(&self) -> &dyn any::Any {
        self
    }
}

/// Print Exporter Builder
#[derive(Debug)]
pub struct Builder {
    pretty_print: bool
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            pretty_print: false,
        }
    }
}

impl Builder {
    /// Enable pretty printing encoded spans
    pub fn with_pretty_print(self, pretty_print: bool) -> Self {
        Builder {
            pretty_print,
            ..self
        }
    }

    /// Create a new exporter from the builder
    pub fn init(self) -> Exporter {
        let Builder {
            pretty_print,
        } = self;

        Exporter {
            pretty_print,
        }
    }
}
