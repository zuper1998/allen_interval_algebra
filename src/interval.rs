use std::fmt;
use std::fmt::Formatter;
use log::{warn};


/// Interval definition with Allen's interval algebra
/// more info: https://en.wikipedia.org/wiki/Allen%27s_interval_algebra
#[derive(Debug,Clone,Copy)]
pub struct Interval<TimeType>{
    pub start: TimeType,
    pub end: TimeType
}

impl<TimeType: fmt::Display> fmt::Display for Interval<TimeType> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{} {}",self.start,self.end)
    }
}


impl<TimeType: std::cmp::PartialOrd> Interval<TimeType>{
    pub fn new(start: TimeType, end: TimeType) -> Self {
        if start>end{
            warn!("start time is before end time for interval, switching them for now...");
            return Interval{
                start: end,
                end: start
            }

        }
        Interval{
            start,
            end
        }


    }

    pub fn precede(&self,other: Interval<TimeType>) -> bool {
        self.end < other.start
    }

    pub fn meet(&self,other: Interval<TimeType>) -> bool {
        self.end == other.start
    }

    pub fn  overlaps(&self,other: Interval<TimeType>) -> bool {
        self.end > other.start && self.end < other.end
    }

    pub fn  starts(&self,other: Interval<TimeType>) -> bool {
        self.start == other.start && self.end < other.end
    }

    pub fn  during(&self,other: Interval<TimeType>) -> bool {
        self.start > other.start && self.end < other.end
    }

    pub fn  finishes(&self,other: Interval<TimeType>) -> bool {
        self.end == other.end && self.start > other.start
    }

    pub fn  equal(&self,other: Interval<TimeType>) -> bool {
        self.start == other.start && self.end == other.end
    }




}