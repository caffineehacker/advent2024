use std::ops::{Add, Sub};

use num_traits::{CheckedAdd, CheckedSub};

#[allow(non_camel_case_types)]
pub type Position_i64 = Position<i64>;

#[allow(non_camel_case_types)]
pub type Position_usize = Position<usize>;

#[allow(non_camel_case_types)]
pub type Position_isize = Position<isize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position<T> {
    pub x: T,
    pub y: T,
}

impl<T> Position<T> {
    pub fn try_from<F: TryInto<T>>(f: Position<F>) -> Result<Self, <F as TryInto<T>>::Error> {
        Ok(Self {
            x: f.x.try_into()?,
            y: f.y.try_into()?,
        })
    }

    pub fn from<F: Into<T>>(f: Position<F>) -> Self {
        Self {
            x: f.x.into(),
            y: f.y.into(),
        }
    }
}

impl From<Position<usize>> for Position<isize> {
    fn from(f: Position<usize>) -> Self {
        Position {
            x: f.x.try_into().unwrap(),
            y: f.y.try_into().unwrap(),
        }
    }
}

impl<T: Add<Output = T>> Add for Position<T> {
    type Output = Position<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: CheckedAdd<Output = T>> CheckedAdd for Position<T> {
    fn checked_add(&self, rhs: &Self) -> Option<Self::Output> {
        Some(Position {
            x: self.x.checked_add(&rhs.x)?,
            y: self.y.checked_add(&rhs.y)?,
        })
    }
}

impl<T: Sub<Output = T>> Sub for Position<T> {
    type Output = Position<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: CheckedSub> CheckedSub for Position<T> {
    fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        Some(Position {
            x: self.x.checked_sub(&rhs.x)?,
            y: self.y.checked_sub(&rhs.y)?,
        })
    }
}

impl<T: CheckedSub + Default> Position<T> {
    pub fn checked_sub_x(&self, x: T) -> Option<Self> {
        self.checked_sub(&Self::x(x))
    }

    pub fn checked_sub_y(&self, y: T) -> Option<Self> {
        self.checked_sub(&Self::y(y))
    }
}

impl<T: Default> Position<T> {
    pub fn x(x: T) -> Self {
        Position { x, y: T::default() }
    }

    pub fn y(y: T) -> Self {
        Position { x: T::default(), y }
    }
}

impl<T: Add<Output = T> + Default> Position<T> {
    pub fn add_x(self, x: T) -> Self {
        self + Position::<T>::x(x)
    }

    pub fn add_y(self, y: T) -> Self {
        self + Position::<T>::y(y)
    }
}

impl<T: Add<Output = T> + std::cmp::PartialOrd + Copy> Position<T> {
    pub fn limited_add_y(&self, y: T, limit: Self) -> Option<Self> {
        let new_pos = Position {
            x: self.x,
            y: self.y + y,
        };

        if new_pos.y >= limit.y {
            return None;
        }

        Some(new_pos)
    }

    pub fn limited_add_x(&self, x: T, limit: Self) -> Option<Self> {
        let new_pos = Position {
            x: self.x + x,
            y: self.y,
        };

        if new_pos.x >= limit.x {
            return None;
        }

        Some(new_pos)
    }
}

impl<T: Sub<Output = T> + Default> Position<T> {
    pub fn sub_x(self, x: T) -> Self {
        self - Position::<T>::x(x)
    }

    pub fn sub_y(self, y: T) -> Self {
        self - Position::<T>::y(y)
    }
}
