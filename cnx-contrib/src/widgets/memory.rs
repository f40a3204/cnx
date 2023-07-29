use anyhow::Result;
use async_stream::try_stream;
use cnx::text::{Attributes, Text};
use cnx::widgets::{Widget, WidgetStream};
use std::time::Duration;
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use tokio::time;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};


/// Represents Weather widget used to show current weather information.
pub struct Memory {
    attr: Attributes,
    render: Option<Box<dyn Fn(u64) -> String>>,
}


impl Memory{
    /// Creates a new [`Weather`] widget.
    ///
    /// Arguments
    ///
    /// * `attr` - Represents `Attributes` which controls properties like
    /// `Font`, foreground and background color etc.
    ///
    /// * `station_code` - Represents weather station code from the
    /// Federal Climate Complex ISD. You can find your place's station
    /// code by getting the information from either [NOAA's
    /// archive](https://www1.ncdc.noaa.gov/pub/data/noaa/isd-history.txt)
    /// or [Internet Archive's
    /// data](https://web.archive.org/web/20210522235412/https://www1.ncdc.noaa.gov/pub/data/noaa/isd-history.txt)
    /// of the same link.
    ///
    /// * `render` - We use the closure to control the way output is
    /// displayed in the bar. [`WeatherInfo`] represents the current
    /// weather details of the particular station.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate cnx;
    /// #
    /// # use cnx::*;
    /// # use cnx::text::*;
    /// # use cnx_contrib::widgets::weather::*;
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
    /// cnx.add_widget(Weather::new(attr, "VOBL".into(),  None));
    /// # Ok(())
    /// # }
    /// # fn main() { run().unwrap(); }
    /// ```
    pub fn new(
        attr: Attributes,
        render: Option<Box<dyn Fn(u64) -> String>>,
    ) -> Memory {
        Memory {
            attr,
            render,
        }
    }


    fn tick(&mut self) -> Result<Vec<Text>> {
        let mut sys = System::new_all();
        sys.refresh_memory();
        let text = sys.total_memory();

        Ok(text)

    }
}

impl Widget for Memory {
    fn into_stream(mut self: Box<Self>) -> Result<WidgetStream> {
        let ten_seconds = Duration::from_secs(10);
        let interval = time::interval(ten_seconds);
        let stream = IntervalStream::new(interval).map(move |_| self.tick());
        Ok(Box::pin(stream))
    }
}
