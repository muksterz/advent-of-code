use std::{
    cmp::Ordering,
    fmt::Debug,
    marker::PhantomData,
    ops::{
        Add, AddAssign, Bound, Index, IndexMut, Mul, MulAssign, Range, RangeBounds, Sub, SubAssign,
    }, panic::UnwindSafe,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self::new(0, 0, Vec::new())
    }
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize, data: Vec<T>) -> Self {
        let size = rows * cols;
        assert_eq!(data.len(), size);
        Self {
            rows,
            cols,
            data: Vec::new(),
        }
    }
    pub const fn build() -> GridBuilder<T> {
        GridBuilder::new()
    }
    pub const fn size(&self) -> (i64, i64) {
        (self.num_rows(), self.num_cols())
    }
    pub const fn num_rows(&self) -> i64 {
        self.rows as i64
    }
    pub const fn num_cols(&self) -> i64 {
        self.cols as i64
    }

    fn coord_to_index(&self, coord: Coord) -> Option<usize> {
        coord_to_index(coord, self.num_rows(), self.num_cols())
    }
    fn raw_ref(&self) -> RawGridRef<T> {
        let ptr = self.data.as_ptr();
        RawGridRef {
            data: PhantomData,
            ptr,
            rows: self.num_rows(),
            cols: self.num_cols(),
        }
    }
    fn raw_mut(&mut self) -> RawGridMut<T> {
        let ptr = self.data.as_mut_ptr();
        RawGridMut {
            marker: PhantomData,
            ptr,
            rows: self.num_rows(),
            cols: self.num_cols(),
        }
    }
    pub fn get(&self, coord: Coord) -> Option<&T> {
        self.data.get(self.coord_to_index(coord)?)
    }
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut T> {
        let index = self.coord_to_index(coord)?;
        self.data.get_mut(index)
    }

    pub fn row(&self, index: i64) -> Row<T> {
        Row::new(self, index)
    }
    pub fn row_mut(&mut self, index: i64) -> RowMut<T> {
        RowMut::new(self, index)
    }
    pub fn col(&self, index: i64) -> Column<T> {
        Column::new(self, index)
    }
    pub fn col_mut(&mut self, index: i64) -> ColumnMut<T> {
        ColumnMut::new(self, index)
    }

    pub fn rows(&self) -> Rows<T> {
        Rows {
            grid: self.raw_ref(),
            start: 0,
            end: self.num_rows(),
        }
    }
    pub fn rows_mut(&mut self) -> RowsMut<T> {
        let end = self.raw_mut().row_size();
        RowsMut {
            grid: self.raw_mut(),
            start: 0,
            end,
        }
    }
    pub fn cols(&self) -> Columns<T> {
        Columns {
            grid: self.raw_ref(),
            start: 0,
            end: self.num_cols(),
        }
    }
    pub fn cols_mut(&mut self) -> ColumnsMut<T> {
        let end = self.raw_mut().row_size();
        ColumnsMut {
            grid: self.raw_mut(),
            start: 0,
            end,
        }
    }
}

impl<T> IndexMut<Coord> for Grid<T> {
    #[track_caller]
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of Bounds")
    }
}
impl<T> Index<Coord> for Grid<T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: Coord) -> &Self::Output {
        self.get(index).expect("Index out of Bounds")
    }
}

impl<T, const R: usize, const C: usize> From<[[T; C]; R]> for Grid<T> {
    fn from(value: [[T; C]; R]) -> Self {
        Self {
            cols: C,
            rows: R,
            data: value.into_iter().flatten().collect(),
        }
    }
}

pub struct GridBuilder<T> {
    rows: Vec<Vec<T>>,
}
impl<T> GridBuilder<T> {
    pub const fn new() -> Self {
        Self { rows: Vec::new() }
    }
    #[track_caller]
    pub fn finish(self) -> Grid<T> {
        let rows = self.rows.len();
        let cols = self.rows.iter().map(Vec::len).max().unwrap_or(0);
        for r in self.rows.iter() {
            if r.len() != cols {
                panic!("Uneven row lengths: Expected len {cols} found {}", r.len())
            }
        }

        Grid {
            data: self.rows.into_iter().flatten().collect(),
            rows,
            cols,
        }
    }
    pub fn push(&mut self, v: T) {
        if self.rows.is_empty() {
            self.push_empty_row();
        }
        self.rows.last_mut().unwrap().push(v)
    }
    pub fn push_empty_row(&mut self) {
        self.rows.push(Vec::new())
    }
    pub fn push_row(&mut self, row: Vec<T>) {
        self.rows.push(row)
    }
}

fn coord_to_index(c: Coord, rows: i64, cols: i64) -> Option<usize> {
    if c.row < 0 || c.col < 0 {
        return None;
    }
    if c.row >= rows || c.col >= cols {
        return None;
    }

    let row = c.row as usize;
    let col = c.col as usize;
    let index = row * cols as usize + col;
    Some(index)
}

#[derive(Debug)]
struct RawGridRef<'grid, T> {
    data: PhantomData<&'grid T>,
    ptr: *const T,
    rows: i64,
    cols: i64,
}
impl<'grid, T> RawGridRef<'grid, T> {
    fn row_size(&self) -> i64 {
        self.cols
    }
    fn col_size(&self) -> i64 {
        self.rows
    }
    fn coord_to_index(&self, c: Coord) -> Option<usize> {
        coord_to_index(c, self.rows, self.cols)
    }
    fn get(&self, c: Coord) -> Option<&'grid T> {
        let index = self.coord_to_index(c)?;
        // SAFETY: In bounds due to invarients
        unsafe { self.ptr.add(index).as_ref() }
    }
}
impl<'grid, T> Clone for RawGridRef<'grid, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'grid, T> Copy for RawGridRef<'grid, T> {}
impl<'grid, T> From<&'grid Grid<T>> for RawGridRef<'grid, T> {
    fn from(value: &'grid Grid<T>) -> Self {
        value.raw_ref()
    }
}
unsafe impl <'grid, T: Sync> Send for RawGridRef<'grid, T> {}
unsafe impl <'grid, T: Sync> Sync for RawGridRef<'grid, T> {}
impl <'grid, T: UnwindSafe> UnwindSafe for RawGridRef<'grid, T> {}

#[derive(Debug)]
struct RawGridMut<'grid, T> {
    marker: PhantomData<&'grid mut T>,
    ptr: *mut T,
    rows: i64,
    cols: i64,
}
impl<'grid, T> RawGridMut<'grid, T> {
    fn coord_to_index(&self, c: Coord) -> Option<usize> {
        coord_to_index(c, self.rows, self.cols)
    }
    fn get(&self, c: Coord) -> Option<&'grid T> {
        let index = self.coord_to_index(c)?;
        // SAFETY: In bounds due to invarients
        unsafe { self.ptr.add(index).as_ref() }
    }
    fn get_mut(&mut self, c: Coord) -> Option<&mut T> {
        // SAFETY: In bounds due to invarients
        // Lifetime is tied to &mut self so no other borrows will overlap
        unsafe { self.get_mut_unbound(c) }
    }
    /// # Safety
    /// This function must not be called with multiple overlapping coords
    unsafe fn get_mut_unbound(&mut self, c: Coord) -> Option<&'grid mut T> {
        let index = self.coord_to_index(c)?;
        // SAFETY: In bounds due to invarients
        unsafe { self.ptr.add(index).as_mut() }
    }
    /// # Safety
    /// The returned grid must not issue mutable refs issued by any grid made with this lifetime
    unsafe fn unchecked_copy(&mut self) -> Self {
        Self {
            marker: self.marker,
            ptr: self.ptr,
            rows: self.rows,
            cols: self.cols,
        }
    }
    fn row_size(&self) -> i64 {
        self.cols
    }
    fn col_size(&self) -> i64 {
        self.rows
    }

    fn as_ref(&self) -> RawGridRef<T> {
        RawGridRef {
            data: PhantomData,
            ptr: self.ptr as *const _,
            rows: self.rows,
            cols: self.cols,
        }
    }
}
impl<'grid, T> From<&'grid mut Grid<T>> for RawGridMut<'grid, T> {
    fn from(value: &'grid mut Grid<T>) -> Self {
        value.raw_mut()
    }
}
unsafe impl <'grid, T: Send> Send for RawGridMut<'grid, T> {}
unsafe impl <'grid, T: Sync> Sync for RawGridMut<'grid, T> {}
impl <'grid, T: UnwindSafe> UnwindSafe for RawGridMut<'grid, T> {}


#[derive(Debug)]
pub struct Rows<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Rows<'grid, T> {
    pub fn iter(&self) -> RowsIter<'grid, T> {
        RowsIter {
            grid: self.grid,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'grid, T> Clone for Rows<'grid, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'grid, T> Copy for Rows<'grid, T> {}
impl<'grid, T> IntoIterator for Rows<'grid, T> {
    type Item = Row<'grid, T>;

    type IntoIter = RowsIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug)]
pub struct RowsIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowsIter<'grid, T> {
    type Item = Row<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            Some(Row::new(self.grid, r))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for RowsIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(Row::new(self.grid, self.end))
        }
    }
}

#[derive(Debug)]
pub struct RowsMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> RowsMut<'grid, T> {
    pub fn iter(&self) -> RowsIter<T> {
        RowsIter {
            grid: self.grid.as_ref(),
            start: self.start,
            end: self.end,
        }
    }
    pub fn iter_mut<'rows>(&'rows mut self) -> RowsMutIter<'rows, 'grid, T> {
        RowsMutIter {
            grid: &mut self.grid,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'grid, T> IntoIterator for RowsMut<'grid, T> {
    type Item = RowMut<'grid, T>;

    type IntoIter = RowsMutIntoIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowsMutIntoIter {
            grid: self.grid,
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Debug)]
pub struct RowsMutIter<'rows, 'grid, T> {
    grid: &'rows mut RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'rows, 'grid, T> Iterator for RowsMutIter<'rows, 'grid, T> {
    type Item = RowMut<'rows, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, r))
        }
    }
}
impl<'rows, 'grid, T> DoubleEndedIterator for RowsMutIter<'rows, 'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, self.end))
        }
    }
}

#[derive(Debug)]
pub struct RowsMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowsMutIntoIter<'grid, T> {
    type Item = RowMut<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, r))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for RowsMutIntoIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            self.end -= 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, self.end))
        }
    }
}

#[derive(Debug)]
pub struct Columns<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Columns<'grid, T> {
    pub fn iter(&self) -> ColumnsIter<'grid, T> {
        ColumnsIter {
            grid: self.grid,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'grid, T> IntoIterator for Columns<'grid, T> {
    type Item = Column<'grid, T>;

    type IntoIter = ColumnsIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> Clone for Columns<'grid, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'grid, T> Copy for Columns<'grid, T> {}

#[derive(Debug)]
pub struct ColumnsIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnsIter<'grid, T> {
    type Item = Column<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(Column::new(self.grid, c))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for ColumnsIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(Column::new(self.grid, self.end))
        }
    }
}

#[derive(Debug)]
pub struct ColumnsMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> ColumnsMut<'grid, T> {
    pub fn iter(&self) -> ColumnsIter<T> {
        ColumnsIter {
            grid: self.grid.as_ref(),
            start: self.start,
            end: self.end,
        }
    }
    pub fn iter_mut<'col>(&'col mut self) -> ColumnsMutIter<'col, 'grid, T> {
        ColumnsMutIter {
            grid: &mut self.grid,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'grid, T> IntoIterator for ColumnsMut<'grid, T> {
    type Item = ColumnMut<'grid, T>;

    type IntoIter = ColumnsMutIntoIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColumnsMutIntoIter {
            grid: self.grid,
            start: self.start,
            end: self.end,
        }
    }
}

#[derive(Debug)]
pub struct ColumnsMutIter<'col, 'grid, T> {
    grid: &'col mut RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'col, 'grid, T> Iterator for ColumnsMutIter<'col, 'grid, T> {
    type Item = ColumnMut<'col, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(ColumnMut::new(unsafe { self.grid.unchecked_copy() }, c))
        }
    }
}
impl<'col, 'grid, T> DoubleEndedIterator for ColumnsMutIter<'col, 'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(ColumnMut::new(
                unsafe { self.grid.unchecked_copy() },
                self.end,
            ))
        }
    }
}

#[derive(Debug)]
pub struct ColumnsMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnsMutIntoIter<'grid, T> {
    type Item = ColumnMut<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(ColumnMut::new(unsafe { self.grid.unchecked_copy() }, c))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for ColumnsMutIntoIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            Some(ColumnMut::new(
                unsafe { self.grid.unchecked_copy() },
                self.end,
            ))
        }
    }
}

pub struct Row<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
    row: i64,
}
impl<'grid, T> Row<'grid, T> {
    fn new(grid: impl Into<RawGridRef<'grid, T>>, row: i64) -> Self {
        let grid = grid.into();
        Self {
            grid,
            start: 0,
            end: grid.row_size(),
            row,
        }
    }
    pub fn iter(&self) -> RowIter<'grid, T> {
        RowIter {
            grid: self.grid,
            row: self.row,
            start: self.start,
            end: self.grid.row_size(),
        }
    }
    pub const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(self.row, index + self.start)
    }
    #[track_caller]
    pub fn get(&self, index: i64) -> Option<&'grid T> {
        self.grid.get(self.coord_of(index))
    }
    #[track_caller]
    pub fn slice<I: RangeBounds<i64>>(&self, range: I) -> Self {
        let start = match range.start_bound() {
            Bound::Included(&i) => self.start + i,
            Bound::Excluded(&i) => self.start + i + 1,
            Bound::Unbounded => self.start,
        };
        let end = match range.end_bound() {
            Bound::Included(&i) => self.start + i + 1,
            Bound::Excluded(&i) => self.start + i,
            Bound::Unbounded => self.end,
        };
        assert!(self.start <= start);
        assert!(end <= self.end);
        Self {
            grid: self.grid,
            start,
            end,
            row: self.row,
        }
    }
}
impl<'grid, T> Index<i64> for Row<'grid, T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: i64) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<'grid, T> Clone for Row<'grid, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'grid, T> Copy for Row<'grid, T> {}
impl<'grid, T> IntoIterator for Row<'grid, T> {
    type Item = &'grid T;

    type IntoIter = RowIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> IntoIterator for &Row<'grid, T> {
    type Item = &'grid T;

    type IntoIter = RowIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> Sequence for Row<'grid, T> {
    type Index = i64;

    type Value = T;

    type Slice<'a> = Self where Self: 'a;

    fn get(&self, index: Self::Index) -> Option<&Self::Value> {
        self.get(index)
    }
    fn bounds(&self) -> Range<Self::Index> {
        0..(self.end - self.start)
    }
    fn slice<I: RangeBounds<Self::Index>>(&self, range: I) -> Self {
        self.slice(range)
    }
}
impl<'grid, T: Debug> Debug for Row<'grid, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[derive(Debug)]
pub struct RowIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    row: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            self.grid.get(Coord::new(self.row, c))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for RowIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            self.grid.get(Coord::new(self.row, self.end))
        }
    }
}
impl<'grid, T> Clone for RowIter<'grid, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            row: self.row,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct RowMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
    row: i64,
}
impl<'grid, T> RowMut<'grid, T> {
    fn new(grid: impl Into<RawGridMut<'grid, T>>, row: i64) -> Self {
        let grid = grid.into();
        let end = grid.row_size();
        Self {
            grid,
            start: 0,
            end,
            row,
        }
    }
    const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(self.row, self.start + index)
    }
    #[track_caller]
    pub fn get(&self, index: i64) -> Option<&T> {
        self.grid.get(self.coord_of(index))
    }
    #[track_caller]
    pub fn get_mut(&mut self, index: i64) -> Option<&mut T> {
        self.grid.get_mut(self.coord_of(index))
    }
    pub fn as_ref(&self) -> Row<T> {
        Row {
            grid: self.grid.as_ref(),
            start: self.start,
            end: self.end,
            row: self.row,
        }
    }
    pub fn iter(&self) -> RowIter<T> {
        self.as_ref().iter()
    }
    pub fn iter_mut<'row>(&'row mut self) -> RowMutIter<'grid, 'row, T> {
        RowMutIter {
            grid: &mut self.grid,
            row: self.row,
            start: self.start,
            end: self.end,
        }
    }
    #[track_caller]
    pub fn slice<I: RangeBounds<i64>>(&self, range: I) -> Row<T> {
        self.as_ref().slice(range)
    }
    #[track_caller]
    pub fn slice_mut<I: RangeBounds<i64>>(self, range: I) -> RowMut<'grid, T> {
        let start = match range.start_bound() {
            Bound::Included(&i) => self.start + i,
            Bound::Excluded(&i) => self.start + i + 1,
            Bound::Unbounded => self.start,
        };
        let end = match range.end_bound() {
            Bound::Included(&i) => self.start + i + 1,
            Bound::Excluded(&i) => self.start + i,
            Bound::Unbounded => self.end,
        };
        assert!(self.start <= start);
        assert!(end <= self.end);
        // SAFETY: Lifetime of `RowMut` is bound by &mut self
        // so self is inaccessable while `RowMut` is alive
        RowMut {
            grid: self.grid,
            start,
            end,
            row: self.row,
        }
    }
}
impl<'grid, T> Index<i64> for RowMut<'grid, T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: i64) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<'grid, T> IndexMut<i64> for RowMut<'grid, T> {
    #[track_caller]
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<'grid, T> IntoIterator for RowMut<'grid, T> {
    type Item = &'grid mut T;
    type IntoIter = RowMutIntoIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        RowMutIntoIter {
            grid: self.grid,
            row: self.row,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'rows, 'grid, T> IntoIterator for &'rows RowMut<'grid, T> {
    type Item = &'rows T;

    type IntoIter = RowIter<'rows, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> Sequence for RowMut<'grid, T> {
    type Index = i64;

    type Value = T;

    type Slice<'a> = Row<'a, T> where Self :'a;

    fn get(&self, index: Self::Index) -> Option<&Self::Value> {
        self.get(index)
    }
    fn bounds(&self) -> Range<Self::Index> {
        0..(self.end - self.start)
    }
    fn slice<I: RangeBounds<Self::Index>>(&self, range: I) -> Self::Slice<'_> {
        self.slice(range)
    }
}
impl<'grid, T> SequenceMut for RowMut<'grid, T> {
    type SliceMut = RowMut<'grid, T>;

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Value> {
        self.get_mut(index)
    }

    fn slice_mut<I: RangeBounds<Self::Index>>(self, range: I) -> Self::SliceMut {
        self.slice_mut(range)
    }
}
impl<'grid, T: Debug> Debug for RowMut<'grid, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[derive(Debug)]
pub struct RowMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    row: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowMutIntoIter<'grid, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            // SAFETY: Iterator never returns the same value twice
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, c)) }
        }
    }
}
impl<'grid, T> DoubleEndedIterator for RowMutIntoIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, self.end)) }
        }
    }
}

#[derive(Debug)]
pub struct RowMutIter<'grid, 'row, T> {
    grid: &'row mut RawGridMut<'grid, T>,
    row: i64,
    start: i64,
    end: i64,
}
impl<'grid, 'row, T> Iterator for RowMutIter<'grid, 'row, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, c)) }
        }
    }
}
impl<'grid, 'row, T> DoubleEndedIterator for RowMutIter<'grid, 'row, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            // SAFETY: Iterator never returns the same index twice
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, self.end)) }
        }
    }
}

pub struct Column<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
    col: i64,
}
impl<'grid, T> Column<'grid, T> {
    fn new(grid: impl Into<RawGridRef<'grid, T>>, col: i64) -> Self {
        let grid = grid.into();
        Self {
            grid,
            start: 0,
            end: grid.col_size(),
            col,
        }
    }
    pub fn iter(&self) -> ColumnIter<'grid, T> {
        ColumnIter {
            grid: self.grid,
            col: self.col,
            start: self.start,
            end: self.end,
        }
    }
    pub const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(self.start + index, self.col)
    }
    #[track_caller]
    pub fn get(&self, index: i64) -> Option<&'grid T> {
        self.grid.get(self.coord_of(index))
    }
    #[track_caller]
    pub fn slice<I: RangeBounds<i64>>(&self, range: I) -> Self {
        let start = match range.start_bound() {
            Bound::Included(&i) => self.start + i,
            Bound::Excluded(&i) => self.start + i + 1,
            Bound::Unbounded => self.start,
        };
        let end = match range.end_bound() {
            Bound::Included(&i) => self.start + i + 1,
            Bound::Excluded(&i) => self.start + i,
            Bound::Unbounded => self.end,
        };
        assert!(self.start <= start);
        assert!(end <= self.end);
        Self {
            grid: self.grid,
            start,
            end,
            col: self.col,
        }
    }
}
impl<'grid, T> Index<i64> for Column<'grid, T> {
    type Output = T;
    #[track_caller]
    fn index(&self, index: i64) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<'grid, T> Clone for Column<'grid, T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<'grid, T> Copy for Column<'grid, T> {}
impl<'grid, T> IntoIterator for Column<'grid, T> {
    type Item = &'grid T;

    type IntoIter = ColumnIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> IntoIterator for &Column<'grid, T> {
    type Item = &'grid T;

    type IntoIter = ColumnIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> Sequence for Column<'grid, T> {
    type Index = i64;

    type Value = T;

    type Slice<'a> = Self where Self: 'a;

    fn get(&self, index: Self::Index) -> Option<&Self::Value> {
        self.get(index)
    }
    fn bounds(&self) -> Range<Self::Index> {
        0..(self.end - self.start)
    }
    fn slice<I: RangeBounds<Self::Index>>(&self, range: I) -> Self {
        self.slice(range)
    }
}
impl<'grid, T: Debug> Debug for Column<'grid, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[derive(Debug)]
pub struct ColumnIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    col: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            self.grid.get(Coord::new(r, self.col))
        }
    }
}
impl<'grid, T> DoubleEndedIterator for ColumnIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            self.grid.get(Coord::new(self.end, self.col))
        }
    }
}
impl<'grid, T> Clone for ColumnIter<'grid, T> {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid,
            col: self.col,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct ColumnMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
    col: i64,
}
impl<'grid, T> ColumnMut<'grid, T> {
    fn new(grid: impl Into<RawGridMut<'grid, T>>, col: i64) -> Self {
        let grid = grid.into();
        let end = grid.col_size();
        Self {
            grid,
            start: 0,
            end,
            col,
        }
    }
    const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(index + self.start, self.col)
    }
    #[track_caller]
    pub fn get(&self, index: i64) -> Option<&T> {
        self.grid.get(self.coord_of(index))
    }
    #[track_caller]
    pub fn get_mut(&mut self, index: i64) -> Option<&mut T> {
        self.grid.get_mut(self.coord_of(index))
    }
    pub fn as_ref(&self) -> Column<T> {
        Column {
            grid: self.grid.as_ref(),
            start: self.start,
            end: self.end,
            col: self.col,
        }
    }
    #[track_caller]
    pub fn slice<I: RangeBounds<i64>>(&self, range: I) -> Column<T> {
        self.as_ref().slice(range)
    }
    #[track_caller]
    pub fn slice_mut<I: RangeBounds<i64>>(self, range: I) -> ColumnMut<'grid, T> {
        let start = match range.start_bound() {
            Bound::Included(&i) => self.start + i,
            Bound::Excluded(&i) => self.start + i + 1,
            Bound::Unbounded => self.start,
        };
        let end = match range.end_bound() {
            Bound::Included(&i) => self.start + i + 1,
            Bound::Excluded(&i) => self.start + i,
            Bound::Unbounded => self.end,
        };
        // SAFETY: Lifetime of `RowMut` is bound by &mut self
        // so self is inaccessable while `RowMut` is alive
        ColumnMut {
            grid: self.grid,
            start,
            end,
            col: self.col,
        }
    }
    pub fn iter(&self) -> ColumnIter<T> {
        self.as_ref().iter()
    }
    pub fn iter_mut<'col>(&'col mut self) -> ColumnMutIter<'grid, 'col, T> {
        ColumnMutIter {
            grid: &mut self.grid,
            col: self.col,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'grid, T> Index<i64> for ColumnMut<'grid, T> {
    type Output = T;

    #[track_caller]
    fn index(&self, index: i64) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl<'grid, T> IndexMut<i64> for ColumnMut<'grid, T> {
    #[track_caller]
    fn index_mut(&mut self, index: i64) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl<'grid, T> IntoIterator for ColumnMut<'grid, T> {
    type Item = &'grid mut T;

    type IntoIter = ColumnMutIntoIter<'grid, T>;

    fn into_iter(self) -> Self::IntoIter {
        ColumnMutIntoIter {
            grid: self.grid,
            col: self.col,
            start: self.start,
            end: self.end,
        }
    }
}
impl<'col, 'grid, T> IntoIterator for &'col ColumnMut<'grid, T> {
    type Item = &'col T;

    type IntoIter = ColumnIter<'col, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'grid, T> Sequence for ColumnMut<'grid, T> {
    type Index = i64;

    type Value = T;

    type Slice<'a> = Column<'a, T> where Self :'a;

    fn get(&self, index: Self::Index) -> Option<&Self::Value> {
        self.get(index)
    }
    fn bounds(&self) -> Range<Self::Index> {
        0..(self.end - self.start)
    }

    fn slice<I: RangeBounds<Self::Index>>(&self, range: I) -> Self::Slice<'_> {
        self.slice(range)
    }
}
impl<'grid, T> SequenceMut for ColumnMut<'grid, T> {
    type SliceMut = ColumnMut<'grid, T>;

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Value> {
        self.get_mut(index)
    }

    fn slice_mut<I: RangeBounds<Self::Index>>(self, range: I) -> Self::SliceMut {
        self.slice_mut(range)
    }
}
impl<'grid, T: Debug> Debug for ColumnMut<'grid, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

#[derive(Debug)]
pub struct ColumnMutIter<'grid, 'col, T> {
    grid: &'col mut RawGridMut<'grid, T>,
    col: i64,
    start: i64,
    end: i64,
}
impl<'grid, 'col, T> Iterator for ColumnMutIter<'grid, 'col, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(r, self.col)) }
        }
    }
}
impl<'grid, 'col, T> DoubleEndedIterator for ColumnMutIter<'grid, 'col, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(self.end, self.col)) }
        }
    }
}

#[derive(Debug)]
pub struct ColumnMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    col: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnMutIntoIter<'grid, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.end {
            None
        } else {
            let r = self.start;
            self.start += 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(r, self.col)) }
        }
    }
}
impl<'grid, T> DoubleEndedIterator for ColumnMutIntoIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(self.end, self.col)) }
        }
    }
}

pub trait Sequence {
    type Index;
    type Value;
    type Slice<'a>: Sequence<Index = Self::Index, Value = Self::Value>
    where
        Self: 'a;

    fn get(&self, index: Self::Index) -> Option<&Self::Value>;
    fn bounds(&self) -> Range<Self::Index>;

    fn slice<I: RangeBounds<Self::Index>>(&self, range: I) -> Self::Slice<'_>;
}

pub trait SequenceMut: Sequence {
    type SliceMut: Sequence<Index = Self::Index, Value = Self::Value>;
    

    fn get_mut(&mut self, index: Self::Index) -> Option<&mut Self::Value>;

    fn slice_mut<I: RangeBounds<Self::Index>>(self, range: I) -> Self::SliceMut;

    fn swap<S>(&mut self, rhs: &mut S)
    where
        S: SequenceMut<Index = Self::Index, Value = Self::Value>,
        Self::Index: Step,
    {
        let mut lhs_i = Step::range_to_iter(self.bounds());
        let mut rhs_i = Step::range_to_iter(rhs.bounds());
        while let (Some(l), Some(r)) = (lhs_i.next(), rhs_i.next()) {
            let l = self.get_mut(l).unwrap();
            let r = rhs.get_mut(r).unwrap();
            std::mem::swap(l, r)
        }

        assert!(lhs_i.next().is_none());
        assert!(rhs_i.next().is_none());
    }
}

pub trait Step: Sized {
    type Iter: Iterator<Item = Self>;
    fn range_to_iter(range: Range<Self>) -> Self::Iter;
}

impl Step for i64 {
    type Iter = Range<i64>;

    fn range_to_iter(range: Range<Self>) -> Self::Iter {
        range
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash)]
pub struct Coord {
    pub row: i64,
    pub col: i64,
}

impl Coord {
    pub const N: Coord = Coord::new(-1, 0);
    pub const S: Coord = Coord::new(1, 0);
    pub const E: Coord = Coord::new(0, 1);
    pub const W: Coord = Coord::new(0, -1);

    pub const fn new(row: i64, col: i64) -> Self {
        Self { row, col }
    }

    pub fn dist_manhatten(self, other: Coord) -> i64 {
        let y = self.row - other.row;
        let x = self.col - other.col;

        y.abs() + x.abs()
    }
    pub fn dist_taxicab(self, other: Coord) -> i64 {
        self.dist_manhatten(other)
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.row.cmp(&other.row), self.col.cmp(&other.col)) {
            (Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),

            (Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
            (Ordering::Less, Ordering::Equal) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Equal) => Some(Ordering::Equal),

            _ => None,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl Sub for Coord {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}
impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.row -= rhs.row;
        self.col -= rhs.col;
    }
}
impl Mul<i64> for Coord {
    type Output = Self;

    fn mul(mut self, rhs: i64) -> Self::Output {
        self *= rhs;
        self
    }
}
impl Mul<u64> for Coord {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        self * rhs as i64
    }
}
impl MulAssign<i64> for Coord {
    fn mul_assign(&mut self, rhs: i64) {
        self.row *= rhs;
        self.col *= rhs;
    }
}
impl MulAssign<u64> for Coord {
    fn mul_assign(&mut self, rhs: u64) {
        *self *= rhs as i64
    }
}

macro_rules! for_all_pairs {
    ($mac:ident: $($x:ident)*) => {
        // Duplicate the list
        for_all_pairs!(@inner $mac: $($x)*; $($x)*);
    };

    // The end of iteration: we exhausted the list
    (@inner $mac:ident: ; $($x:ident)*) => {};

    // The head/tail recursion: pick the first element of the first list
    // and recursively do it for the tail.
    (@inner $mac:ident: $head:ident $($tail:ident)*; $($x:ident)*) => {
        $(
            $mac!($head $x);
        )*
        for_all_pairs!(@inner $mac: $($tail)*; $($x)*);
    };
}

macro_rules! impl_partial_eq {
    ($lhs:ident $rhs:ident) => {
        impl<'grid1, 'grid2, L: PartialEq<R>, R> PartialEq<$rhs<'grid2, R>> for $lhs<'grid1, L> {
            fn eq(&self, other: &$rhs<R>) -> bool {
                let mut i1 = self.iter();
                let mut i2 = other.iter();
                while let (Some(l), Some(r)) = (i1.next(), i2.next()) {
                    if l != r {
                        return false;
                    }
                }
                i1.next().is_none() & i2.next().is_none()
            }
        }
    };
}

macro_rules! impl_eq {
    ($($ty:ident)*) => {
        $(
            impl <'grid, T: Eq> Eq for $ty<'grid, T> {}
        )*
    };
}

macro_rules! impl_slice {
    ($($ty:ident)*) => {
        $(
            impl <'grid, T: PartialEq<U>, U> PartialEq<[U]> for $ty<'grid, T> {
                fn eq(&self, other: &[U]) -> bool {
                    let mut i1 = self.iter();
                    let mut i2 = other.iter();
                    while let (Some(l), Some(r)) = (i1.next(), i2.next()) {
                        if l != r {
                            return false;
                        }
                    }
                    i1.next().is_none() & i2.next().is_none()
                }
            }
            impl <'grid, T: PartialEq<U>, U, const N: usize> PartialEq<[U; N]> for $ty<'grid, T> {
                fn eq(&self, other: &[U; N]) -> bool {
                    self.eq(other.as_slice())
                }
            }
        )*
    };
}

macro_rules! impl_traits {
    ($($ty:ident)*) => {
        for_all_pairs!(impl_partial_eq: $($ty)*);
        impl_eq!($($ty)*);
        impl_slice!($($ty)*);
    };
}

impl_traits!(Column ColumnMut Row RowMut);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows() {
        let mut grid = Grid::build();
        let mut arr = [1, 2, 3, 4, 5];
        for _ in 0..5 {
            grid.push_empty_row();
            for n in arr {
                grid.push(n);
            }
            arr.rotate_left(1);
        }

        let mut grid = grid.finish();

        for r in grid.rows() {
            assert_eq!(5, r.iter().count());
            assert_eq!(5, r.into_iter().count());
        }

        for mut r in grid.rows_mut() {
            for v in r.iter().copied().zip(arr) {
                assert_eq!(v.0, v.1)
            }
            assert_eq!(5, r.iter_mut().count());
            assert_eq!(5, r.into_iter().count());

            arr.rotate_left(1)
        }

        for c in grid.cols() {
            assert_eq!(5, c.iter().count());
            assert_eq!(5, c.into_iter().count());
        }

        for mut c in grid.cols_mut() {
            assert_eq!(5, c.iter().count());
            assert_eq!(5, c.iter_mut().count());
            assert_eq!(5, c.into_iter().count());
        }

        // Reversed Iter

        for r in grid.rows() {
            assert_eq!(5, r.iter().rev().count());
            assert_eq!(5, r.into_iter().rev().count());
        }

        for mut r in grid.rows_mut() {
            assert_eq!(5, r.iter().rev().count());
            assert_eq!(5, r.iter_mut().rev().count());
            assert_eq!(5, r.into_iter().rev().count());

            arr.rotate_left(1)
        }

        for c in grid.cols() {
            assert_eq!(5, c.iter().rev().count());
            assert_eq!(5, c.into_iter().rev().count());
        }

        for mut c in grid.cols_mut() {
            assert_eq!(5, c.iter().rev().count());
            assert_eq!(5, c.iter_mut().rev().count());
            assert_eq!(5, c.into_iter().rev().count());
        }
    }

    #[test]
    fn coord_ordering() {
        let c1 = Coord::new(1, 1);
        let c2 = Coord::new(2, 2);
        assert!(c1 < c2);
        assert!(c2 > c1);
        assert!(c1 <= c1);
        assert!(c1 >= c1);
    }

    #[test]
    fn row_col_eq() {
        let grid: Grid<i32> = [
            [1, 1, 1, 1, 1],
            [1, 2, 2, 2, 2],
            [1, 2, 3, 3, 3],
            [1, 2, 3, 4, 4],
            [1, 2, 3, 4, 5],
        ]
        .into();

        for i in 0..5 {
            assert_eq!(grid.row(i), grid.col(i));
        }
    }

    #[test]
    fn slicing() {
        let mut grid: Grid<i32> = [
            [1, 1, 1, 1, 1],
            [1, 2, 2, 2, 2],
            [1, 2, 3, 3, 3],
            [1, 2, 3, 4, 4],
            [1, 2, 3, 4, 5],
        ]
        .into();
        let sums = [3, 6, 9, 11, 12];

        for (r, s) in grid.rows().into_iter().zip(sums) {
            let r = r.slice(2..);
            assert_eq!(r.iter().sum::<i32>(), s);
            assert_eq!(r.into_iter().sum::<i32>(), s);
        }

        for (r, s) in grid.rows_mut().into_iter().zip(sums) {
            let mut r = r.slice_mut(2..);
            assert_eq!(r.iter().sum::<i32>(), s);
            assert_eq!(r.iter_mut().map(|s| &*s).sum::<i32>(), s);
            assert_eq!(r.into_iter().map(|s| &*s).sum::<i32>(), s);
        }

        for (c, s) in grid.cols().into_iter().zip(sums) {
            let c = c.slice(2..);
            assert_eq!(c.iter().sum::<i32>(), s);
            assert_eq!(c.into_iter().sum::<i32>(), s);
        }

        for (c, s) in grid.cols_mut().into_iter().zip(sums) {
            let mut c = c.slice_mut(2..);
            assert_eq!(c.iter().sum::<i32>(), s);
            assert_eq!(c.iter_mut().map(|s| &*s).sum::<i32>(), s);
            assert_eq!(c.into_iter().map(|s| &*s).sum::<i32>(), s);
        }
    
        let r = grid.row(0);
        let r1 = r.slice(3..);
        let r2 = r1.slice(..);
        assert_eq!(r1, r2);
    }

    #[test]
    fn swap() {
        let grid :Grid<u64> = [
            [1;5],[2;5],[3;5],[4;5],[5;5]
        ].into();

        let mut g1 = grid.clone();
        let mut rows = g1.rows_mut().into_iter();
        let mut first = rows.next().unwrap();
        let mut second = rows.next().unwrap();
        first.swap(&mut second);

        assert_eq!(g1.row(0).iter().sum::<u64>(), 10);
        assert_eq!(g1.row(1).iter().sum::<u64>(), 5);

        let mut g2 = grid.clone();
        let mut rows = g2.rows_mut().into_iter();
        let first = rows.next().unwrap();
        let second = rows.next().unwrap();
        first.slice_mut(2..).swap(&mut second.slice_mut(2..));
        assert_eq!(g2.row(0).iter().sum::<u64>(), 8, "{:?}", g2.row(0));
        assert_eq!(g2.row(1).iter().sum::<u64>(), 7, "{:?}", g2.row(1));

    }

    #[test]
    fn get() {
        let grid: Grid<_> = [[1, 2, 3, 4, 5]].into();
        let r = grid.row(0);
        let r = r.slice(2..);
        assert_eq!(3, r[0])
    }
}
