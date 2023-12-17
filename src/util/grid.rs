use crate::util::position::Position;
use num::integer::div_rem;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    pub dimensions: (usize, usize),
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn from_dimensions(dimensions: (usize, usize), value: T) -> Self
    where
        T: Clone,
    {
        Self {
            dimensions,
            data: vec![value; dimensions.0 * dimensions.1],
        }
    }
    pub fn from_rows<Rows, Cells>(rows: Rows) -> Self
    where
        Rows: IntoIterator<Item = Cells>,
        Cells: IntoIterator<Item = T>,
    {
        let mut data = vec![];
        let mut width = None;

        for row in rows {
            data.extend(row);
            width.get_or_insert(data.len());
        }

        let width = match width {
            None => {
                return Self {
                    dimensions: (0, 0),
                    data,
                }
            }
            Some(width) => width,
        };

        assert_eq!(data.len() % width, 0);

        Self {
            dimensions: (data.len() / width, width),
            data,
        }
    }

    pub fn size(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (Position, &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, value)| (div_rem(i, self.dimensions.1).into(), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> + '_ {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, value)| (div_rem(i, self.dimensions.1).into(), value))
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn positions(&self) -> impl DoubleEndedIterator<Item = Position> {
        let dimensions = self.dimensions;
        (0..self.data.len()).map(move |i| div_rem(i, dimensions.1).into())
    }

    fn index(&self, pos: &Position) -> usize {
        pos.0 * self.dimensions.1 + pos.1
    }

    #[inline]
    pub fn get(&self, pos: &Position) -> &T {
        &self.data[self.index(pos)]
    }

    pub fn get_mut(&mut self, pos: &Position) -> &mut T {
        let i = self.index(pos);
        &mut self.data[i]
    }

    #[inline]
    pub fn set(&mut self, pos: &Position, value: T) {
        let idx = self.index(pos);
        self.data[idx] = value;
    }

    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.data.chunks_exact(self.dimensions.1)
    }

    pub fn get_row(&self, j: usize) -> &[T] {
        &self.data[j * self.dimensions.1..(j + 1) * self.dimensions.1]
    }

    pub fn transposed(&self) -> Self
    where
        T: Copy,
    {
        Grid::from_rows(
            (0..self.dimensions.1)
                .map(|i| (0..self.dimensions.0).map(move |j| self.data[j * self.dimensions.1 + i])),
        )
    }
}
impl<T> Index<usize> for Grid<T> {
    type Output = [T];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index * self.dimensions.1..(index + 1) * self.dimensions.1]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index * self.dimensions.1..(index + 1) * self.dimensions.1]
    }
}

impl<A> Extend<((usize, usize), A)> for Grid<A> {
    fn extend<T: IntoIterator<Item = ((usize, usize), A)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(position, value)| {
            self[position.0][position.1] = value;
        });
    }
}

impl Grid<bool> {
    pub fn from_points<I: Iterator<Item = (usize, usize)> + Clone>(iter: I) -> Self {
        let mut max_x = usize::MIN;
        let mut max_y = usize::MIN;
        for (y, x) in iter.clone() {
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }

        let mut grid = Self::from_dimensions((max_y, max_x), false);
        grid.extend(iter.map(|pos| (pos, true)));
        grid
    }

    #[inline]
    pub fn contains(&self, pos: &Position) -> bool {
        *self.get(pos)
    }

    pub fn count(&self) -> usize {
        self.values().filter(|value| **value).count()
    }
}

pub trait CellDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl CellDisplay for bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            true => '█',
            false => ' ',
        })
    }
}

impl<T> Display for Grid<T>
where
    T: CellDisplay,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data
            .chunks_exact(self.dimensions.1)
            .try_for_each(|row| {
                row.iter().try_for_each(|value| value.fmt(f))?;
                f.write_char('\n')
            })
    }
}

impl Extend<(usize, usize)> for Grid<bool> {
    fn extend<T: IntoIterator<Item = (usize, usize)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|position: (usize, usize)| {
            self[position.0][position.1] = true;
        });
    }
}

impl Extend<Position> for Grid<bool> {
    fn extend<I: IntoIterator<Item = Position>>(&mut self, iter: I) {
        iter.into_iter().for_each(|position| {
            self[position.y()][position.x()] = true;
        });
    }
}

pub struct BackedGrid<'a, I> {
    data: &'a [I],
    pub dimensions: (usize, usize),
    row_stride: usize,
}

impl<'a, I> BackedGrid<'a, I> {
    pub fn from_data_and_row_separator(data: &'a [I], separator: I) -> Self
    where
        I: Eq,
    {
        let width = data
            .iter()
            .position(|value| *value == separator)
            .unwrap_or(data.len());
        let row_stride = width + 1;
        Self {
            data,
            dimensions: (data.len().div_ceil(row_stride), width),
            row_stride,
        }
    }

    fn index(&self, pos: &Position) -> usize {
        pos.0 * self.row_stride + pos.1
    }

    #[inline]
    pub fn get<T>(&self, pos: &Position) -> T
    where
        &'a I: Into<T>,
    {
        (&self.data[self.index(pos)]).into()
    }

    pub fn iter<T>(&'a self) -> impl Iterator<Item = (Position, T)> + '_
    where
        &'a I: Into<T>,
    {
        self.data.iter().enumerate().filter_map(move |(i, value)| {
            let pair = div_rem(i, self.row_stride);
            if pair.1 >= self.dimensions.1 {
                None
            } else {
                Some((pair.into(), value.into()))
            }
        })
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        self.data.iter().enumerate().filter_map(move |(i, _)| {
            let pair = div_rem(i, self.row_stride);
            if pair.1 >= self.dimensions.1 {
                None
            } else {
                Some(pair.into())
            }
        })
    }
}
