use time::Date;

#[derive(Clone, Copy)]
pub struct Period {
    start: Date,
    end: Option<Date>
}

impl Period {
    /// Creates a `Date` using an optional terminating `Date`.
    pub fn new(start: Date, end: Option<Date>) -> Self {
        Self {
            start,
            end
        }
    }

    /// Returns a copy of the contained start `Date`.
    pub fn start(&self) -> Date {
        self.start
    }

    /// Returns a copy of the contained `Option` with a possible copy of the end date.
    pub fn end(&self) -> Option<Date> {
        self.end
    }
}