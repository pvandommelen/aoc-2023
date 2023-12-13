use crate::util::position::{Direction, Position};
use std::fmt::{Display, Formatter};
use std::hash::Hasher;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
    pub dimensions: (usize, usize),
    data: Vec<T>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GridPosition {
    offset: usize,
    row_stride: usize,
}
impl From<GridPosition> for (usize, usize) {
    fn from(value: GridPosition) -> Self {
        (value.y(), value.x())
    }
}

impl GridPosition {
    fn with_offset(self, offset: usize) -> Self {
        Self {
            offset,
            row_stride: self.row_stride,
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

    pub fn iter(&self) -> impl Iterator<Item = (GridPosition, &T)> + '_ {
        self.data.iter().enumerate().map(|(i, value)| {
            (
                GridPosition {
                    offset: i,
                    row_stride: self.dimensions.1,
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
                },
                value,
            )
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
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

    pub fn render(&self) -> String {
        String::from_iter(self.data.chunks_exact(self.dimensions.1).map(|row| {
            let mut s = String::from_iter(row.iter().map(|value| match value {
                true => '█',
                false => ' ',
            }));
            s.push('\n');
            s
        }))
    }

    #[inline]
    pub fn contains(&self, pos: &GridPosition) -> bool {
        *self.get(pos)
    }

    pub fn count(&self) -> usize {
        self.values().filter(|value| **value).count()
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
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
                })
            }
        })
    }
}
