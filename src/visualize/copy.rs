use super::Visualize;
use crate::size::Size;
use std::fmt::Display;

impl<'a, Name, Data> Clone for Visualize<'a, Name, Data>
where
    Name: Display,
    Data: Size,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, Name, Data> Copy for Visualize<'a, Name, Data>
where
    Name: Display,
    Data: Size,
{
}
