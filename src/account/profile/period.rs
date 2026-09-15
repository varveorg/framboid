use time::Date;

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
}