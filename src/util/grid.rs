use crate::util::position::{Direction, Position};
use std::fmt::{Display, Formatter, Write};
use std::hash::Hasher;
use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Grid<T> {
    pub dimensions: (usize, usize),
    data: Vec<T>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GridPosition {
    offset: usize,
    row_stride: usize,
    dimensions: (usize, usize),
}
impl From<GridPosition> for (usize, usize) {
    fn from(value: GridPosition) -> Self {
        (value.y(), value.x())
    }
}

impl GridPosition {
    pub fn from_grid_and_position<T>(grid: &Grid<T>, pos: &(usize, usize)) -> Self {
        Self {
            offset: pos.0 * grid.dimensions.1 + pos.1,
            dimensions: grid.dimensions,
            row_stride: grid.dimensions.1,
        }
    }

    fn with_offset(self, offset: usize) -> Self {
        Self {
            offset,
            dimensions: self.dimensions,
            row_stride: self.row_stride,
        }
    }

    pub fn checked_moved(&self, direction: &Direction) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.offset < self.row_stride {
                    return None;
                }
                Some(self.with_offset(self.offset - self.row_stride))
            }
            Direction::Right => {
                if self.offset % self.row_stride > self.dimensions.1 - 2 {
                    return None;
                }
                Some(self.with_offset(self.offset + 1))
            }
            Direction::Down => {
                if self.offset >= (self.dimensions.0 - 1) * self.row_stride {
                    return None;
                }
                Some(self.with_offset(self.offset + self.row_stride))
            }
            Direction::Left => {
                if self.offset % self.row_stride == 0 {
                    return None;
                }
                Some(self.with_offset(self.offset - 1))
            }
        }
    }
}

impl std::hash::Hash for GridPosition {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.offset);
    }
}

impl Position for GridPosition {
    #[inline]
    fn y(&self) -> usize {
        self.offset / self.row_stride
    }
    #[inline]
    fn x(&self) -> usize {
        self.offset % self.row_stride
    }

    #[must_use]
    fn with_direction(self, direction: &Direction) -> Self {
        match direction {
            Direction::Up => self.with_offset(self.offset - self.row_stride),
            Direction::Down => self.with_offset(self.offset + self.row_stride),
            Direction::Right => self.with_offset(self.offset + 1),
            Direction::Left => self.with_offset(self.offset - 1),
        }
    }
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

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (GridPosition, &T)> + '_ {
        self.data.iter().enumerate().map(|(i, value)| {
            (
                GridPosition {
                    offset: i,
                    row_stride: self.dimensions.1,
                    dimensions: self.dimensions,
                },
                value,
            )
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (GridPosition, &mut T)> + '_ {
        self.data.iter_mut().enumerate().map(|(i, value)| {
            (
                GridPosition {
                    offset: i,
                    row_stride: self.dimensions.1,
                    dimensions: self.dimensions,
                },
                value,
            )
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
    }

    pub fn positions(&self) -> impl DoubleEndedIterator<Item = GridPosition> {
        let dimensions = self.dimensions;
        (0..self.data.len()).map(move |i| GridPosition {
            offset: i,
            row_stride: dimensions.1,
            dimensions,
        })
    }

    #[inline]
    pub fn get<P: Position>(&self, pos: &P) -> &T {
        let offset = pos.y() * self.dimensions.1 + pos.x();
        &self.data[offset]
    }

    #[inline]
    pub fn set<P: Position>(&mut self, pos: &P, value: T) {
        let offset = pos.y() * self.dimensions.1 + pos.x();
        self.data[offset] = value;
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
    pub fn contains(&self, pos: &GridPosition) -> bool {
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

impl<T: Position> Extend<T> for Grid<bool> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
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

    #[inline]
    pub fn get<T>(&self, pos: &GridPosition) -> T
    where
        &'a I: Into<T>,
    {
        (&self.data[pos.offset]).into()
    }

    pub fn iter<T>(&'a self) -> impl Iterator<Item = (GridPosition, T)> + '_
    where
        &'a I: Into<T>,
    {
        self.data.iter().enumerate().filter_map(move |(i, value)| {
            if i % self.row_stride >= self.dimensions.1 {
                None
            } else {
                Some((
                    GridPosition {
                        offset: i,
                        row_stride: self.row_stride,
                        dimensions: self.dimensions,
                    },
                    value.into(),
                ))
            }
        })
    }

    pub fn positions(&self) -> impl Iterator<Item = GridPosition> + '_ {
        self.data.iter().enumerate().filter_map(move |(i, _)| {
            if i % self.row_stride >= self.dimensions.1 {
                None
            } else {
                Some(GridPosition {
                    offset: i,
                    row_stride: self.row_stride,
                    dimensions: self.dimensions,
                })
            }
        })
    }
}
