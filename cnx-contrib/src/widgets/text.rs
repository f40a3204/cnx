use anyhow::Result;
use byte_unit::{Byte, ByteUnit};
use cnx::text::{Attributes, Text};
use cnx::widgets::{Widget, WidgetStream};
use nix::sys::statvfs::statvfs;
use std::time::Duration;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;


impl TextBox {
    /// Creates a new [`TextBox`] widget.
    ///
    /// Arguments
    ///
    /// * `attr` - Represents `Attributes` which controls properties like
    /// `Font`, foreground and background color etc.
    ///
    /// * `path` - The text to be displayed
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate cnx;
    /// #
    /// # use cnx::*;
    /// # use cnx::text::*;
    /// # use cnx_contrib::widgets::disk_usage::*;
    /// # use anyhow::Result;
    /// #
    /// # fn run() -> Result<()> {
    /// let attr = Attributes {
    ///     font: Font::new("SourceCodePro 21"),
    ///     fg_color: Color::white(),
    ///     bg_color: None,
    ///     padding: Padding::new(8.0, 8.0, 0.0, 0.0),
    /// };
    ///
    /// let mut cnx = Cnx::new(Position::Top);
    /// cnx.add_widget(DiskUsage::new(attr, "Hello World");
    /// # Ok(())
    /// # }
    /// # fn main() { run().unwrap(); }
    /// ```
    pub fn new(
        attr: Attributes,
        text: String,
    ) -> Self {
        Self { attr, text }
    }

    fn tick(&self) -> Result<Vec<Text>> {
        Ok(text)
    }
}

impl Widget for TextBox {
    fn into_stream(self: Box<Self>) -> Result<WidgetStream> {
        let ten_seconds = Duration::from_secs(10);
        let interval = time::interval(ten_seconds);
        let stream = IntervalStream::new(interval).map(move |_| self.tick());

        Ok(Box::pin(stream))
    }
}
