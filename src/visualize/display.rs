use super::{Direction::*, Visualize};
use crate::size::Size;
use std::fmt::{Display, Error, Formatter};

impl<'a, Name, Data> Display for Visualize<'a, Name, Data>
where
    Name: Display,
    Data: Size + Into<u64>,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let write = |line: &String| writeln!(formatter, "{}", line);
        match self.direction {
            BottomUp => self.rows().iter().rev().try_for_each(write),
            TopDown => self.rows().iter().try_for_each(write),
        }
    }
}
