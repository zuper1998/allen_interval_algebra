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

    //Create interval from two TimeTypes, will give a warning if end is before start and will swap
    // them around
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


    // Test if self precedes other
    pub fn precede(&self,other: Interval<TimeType>) -> bool {
        self.end < other.start
    }

    // Test if self meets other
    pub fn meet(&self,other: Interval<TimeType>) -> bool {
        self.end == other.start
    }

    // Test if self overlaps with other
    pub fn  overlaps(&self,other: Interval<TimeType>) -> bool {
        self.end > other.start && self.end < other.end
    }

    // Test if self starts other
    pub fn  starts(&self,other: Interval<TimeType>) -> bool {
        self.start == other.start && self.end < other.end
    }

    // Test if self is during other
    pub fn  during(&self,other: Interval<TimeType>) -> bool {
        self.start > other.start && self.end < other.end
    }

    // Test if self finishes other
    pub fn  finishes(&self,other: Interval<TimeType>) -> bool {
        self.end == other.end && self.start > other.start
    }

    // Test if self is equal with other
    pub fn  equal(&self,other: Interval<TimeType>) -> bool {
        self.start == other.start && self.end == other.end
    }




}