use time::Date;

#[derive(Clone, Copy)]
pub struct Period {
    start: Date,
    end: Option<Date>
}

impl Period {
    pub fn new(start: Date, end: Option<Date>) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn start(&self) -> Date {
        self.start
    }

    pub fn end(&self) -> Option<Date> {
        self.end
    }
}