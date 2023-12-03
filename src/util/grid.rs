use std::ops::{Index, IndexMut};

#[derive(Clone)]
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

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(i, value)| ((i / self.dimensions.1, i % self.dimensions.1), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> + '_ {
        self.data
            .iter_mut()
            .enumerate()
            .map(|(i, value)| ((i / self.dimensions.1, i % self.dimensions.1), value))
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> + '_
    where
        T: Send,
    {
        self.data.chunks_mut(self.dimensions.1)
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.data.iter()
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
}

#[derive(Clone)]
pub struct BufferedRingGrid<T> {
    inner: Grid<T>,
    pub dimensions: (usize, usize),
}
impl<T> BufferedRingGrid<T> {
    pub fn from_dimensions(dimensions: (usize, usize), value: T) -> Self
    where
        T: Clone,
    {
        Self {
            dimensions,
            inner: Grid::from_dimensions((dimensions.0 + 2, dimensions.1 + 2), value),
        }
    }

    pub fn size(&self) -> usize {
        self.dimensions.0 * self.dimensions.1
    }

    pub fn count_neighbours_with_diagonals<F>(&self, f: F, y: usize, x: usize) -> u32
    where
        F: Fn(&T) -> bool,
    {
        assert!(y < self.dimensions.0);
        assert!(x < self.dimensions.1);
        let mut count = 0;
        for j in 0..3 {
            for i in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }

                // SAFETY:
                // The guard at the top of the function verified the dimension constraints.
                // The construction method for the struct should have verified that the data size matches the dimensions.
                let value = unsafe {
                    self.inner
                        .data
                        .get_unchecked((y + j) * self.inner.dimensions.1 + x + i)
                };
                if f(value) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn has_neighbours_with_diagonals<F>(&self, f: F, y: usize, x: usize) -> bool
    where
        F: Fn(&T) -> bool,
    {
        self.count_neighbours_with_diagonals(f, y, x) > 0
    }

    pub fn iter(&self) -> impl Iterator<Item = ((usize, usize), &T)> + '_ {
        self.inner.iter().filter_map(|(pos, value): (_, &T)| {
            if pos.0 == 0
                || pos.1 == 0
                || pos.0 == self.dimensions.0 + 1
                || pos.1 == self.dimensions.1 + 1
            {
                return None;
            }
            Some(((pos.0 - 1, pos.1 - 1), value))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = ((usize, usize), &mut T)> + '_ {
        self.inner
            .iter_mut()
            .filter_map(|(pos, value): (_, &mut T)| {
                if pos.0 == 0
                    || pos.1 == 0
                    || pos.0 == self.dimensions.0 + 1
                    || pos.1 == self.dimensions.1 + 1
                {
                    return None;
                }
                Some(((pos.0 - 1, pos.1 - 1), value))
            })
    }

    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> + '_
    where
        T: Send,
    {
        self.inner.rows_mut().enumerate().filter_map(|(j, row)| {
            if j == 0 || j == self.dimensions.0 + 1 {
                None
            } else {
                Some(&mut row[1..self.dimensions.1 + 1])
            }
        })
    }

    pub fn values(&self) -> impl Iterator<Item = &T> + '_ {
        self.iter().map(|(_, value)| value)
    }
}

impl<A> Extend<((usize, usize), A)> for BufferedRingGrid<A> {
    fn extend<T: IntoIterator<Item = ((usize, usize), A)>>(&mut self, iter: T) {
        iter.into_iter().for_each(|(position, value)| {
            self[position.0][position.1] = value;
        });
    }
}

impl<T> Index<usize> for BufferedRingGrid<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index + 1][1..self.dimensions.1 + 1]
    }
}

impl<T> IndexMut<usize> for BufferedRingGrid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index + 1][1..self.dimensions.1 + 1]
    }
}
