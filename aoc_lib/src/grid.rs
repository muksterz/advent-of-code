use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
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
            end: self.num_rows() - 1,
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
            end: self.num_cols() - 1,
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
        println!("Index: {index}");
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

pub struct RowsIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowsIter<'grid, T> {
    type Item = Row<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let r = self.end;
            self.end -= 1;
            Some(Row::new(self.grid, r))
        }
    }
}

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

pub struct RowsMutIter<'rows, 'grid, T> {
    grid: &'rows mut RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'rows, 'grid, T> Iterator for RowsMutIter<'rows, 'grid, T> {
    type Item = RowMut<'rows, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let r = self.end;
            self.end -= 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, r))
        }
    }
}

pub struct RowsMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowsMutIntoIter<'grid, T> {
    type Item = RowMut<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
            let r = self.end;
            self.end -= 1;
            Some(RowMut::new(unsafe { self.grid.unchecked_copy() }, r))
        }
    }
}

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

pub struct ColumnsIter<'grid, T> {
    grid: RawGridRef<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnsIter<'grid, T> {
    type Item = Column<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            Some(Column::new(self.grid, c))
        }
    }
}

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

pub struct ColumnsMutIter<'col, 'grid, T> {
    grid: &'col mut RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'col, 'grid, T> Iterator for ColumnsMutIter<'col, 'grid, T> {
    type Item = ColumnMut<'col, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            Some(ColumnMut::new(unsafe { self.grid.unchecked_copy() }, c))
        }
    }
}

pub struct ColumnsMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnsMutIntoIter<'grid, T> {
    type Item = ColumnMut<'grid, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            Some(ColumnMut::new(unsafe { self.grid.unchecked_copy() }, c))
        }
    }
}

pub struct Row<'grid, T> {
    grid: RawGridRef<'grid, T>,
    row: i64,
}
impl<'grid, T> Row<'grid, T> {
    fn new(grid: impl Into<RawGridRef<'grid, T>>, row: i64) -> Self {
        Self {
            grid: grid.into(),
            row,
        }
    }
    pub fn iter(&self) -> RowIter<'grid, T> {
        RowIter {
            row: *self,
            start: 0,
            end: self.grid.row_size() - 1,
        }
    }
    pub const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(self.row, index)
    }
    pub fn get(&self, index: i64) -> Option<&'grid T> {
        self.grid.get(self.coord_of(index))
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

pub struct RowIter<'grid, T> {
    row: Row<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let s = self.start;
            self.start += 1;
            Some(self.row.get(s).unwrap())
        }
    }
}
impl<'grid, T> DoubleEndedIterator for RowIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let s = self.end;
            self.end -= 1;
            self.row.get(s)
        }
    }
}
impl<'grid, T> Clone for RowIter<'grid, T> {
    fn clone(&self) -> Self {
        Self {
            row: self.row,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct RowMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    row: i64,
}
impl<'grid, T> RowMut<'grid, T> {
    fn new(grid: impl Into<RawGridMut<'grid, T>>, row: i64) -> Self {
        Self {
            grid: grid.into(),
            row,
        }
    }
    pub fn get(&self, index: i64) -> Option<&T> {
        self.grid.get(Coord::new(self.row, index))
    }
    pub fn get_mut(&mut self, index: i64) -> Option<&mut T> {
        self.grid.get_mut(Coord::new(self.row, index))
    }
    pub fn iter(&self) -> RowIter<T> {
        RowIter {
            row: Row::new(self.grid.as_ref(), self.row),
            start: 0,
            end: self.grid.row_size() - 1,
        }
    }
    pub fn iter_mut<'row>(&'row mut self) -> RowMutIter<'grid, 'row, T> {
        let size = self.grid.row_size() - 1;
        RowMutIter {
            grid: &mut self.grid,
            row: self.row,
            start: 0,
            end: size,
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
        let end = self.grid.row_size() - 1;
        RowMutIntoIter {
            grid: self.grid,
            row: self.row,
            start: 0,
            end,
        }
    }
}

pub struct RowMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    row: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for RowMutIntoIter<'grid, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, c)) }
        }
    }
}

pub struct RowMutIter<'grid, 'row, T> {
    grid: &'row mut RawGridMut<'grid, T>,
    row: i64,
    start: i64,
    end: i64,
}
impl<'grid, 'row, T> Iterator for RowMutIter<'grid, 'row, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            // SAFETY: Iterator never returns the same index twice
            unsafe { self.grid.get_mut_unbound(Coord::new(self.row, c)) }
        }
    }
}

pub struct Column<'grid, T> {
    grid: RawGridRef<'grid, T>,
    col: i64,
}
impl<'grid, T> Column<'grid, T> {
    fn new(grid: impl Into<RawGridRef<'grid, T>>, col: i64) -> Self {
        Self {
            grid: grid.into(),
            col,
        }
    }
    pub fn iter(&self) -> ColumnIter<'grid, T> {
        ColumnIter {
            col: *self,
            start: 0,
            end: self.grid.col_size() - 1,
        }
    }
    pub const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(index, self.col)
    }
    pub fn get(&self, index: i64) -> Option<&'grid T> {
        self.grid.get(self.coord_of(index))
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

pub struct ColumnIter<'grid, T> {
    col: Column<'grid, T>,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnIter<'grid, T> {
    type Item = &'grid T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let c = self.start;
            self.start += 1;
            Some(self.col.get(c).unwrap())
        }
    }
}
impl<'grid, T> DoubleEndedIterator for ColumnIter<'grid, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        } else {
            let c = self.end;
            self.end -= 1;
            Some(self.col.get(c).unwrap())
        }
    }
}
impl<'grid, T> Clone for ColumnIter<'grid, T> {
    fn clone(&self) -> Self {
        Self {
            col: self.col,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct ColumnMut<'grid, T> {
    grid: RawGridMut<'grid, T>,
    col: i64,
}
impl<'grid, T> ColumnMut<'grid, T> {
    fn new(grid: impl Into<RawGridMut<'grid, T>>, col: i64) -> Self {
        Self {
            grid: grid.into(),
            col,
        }
    }
    pub const fn coord_of(&self, index: i64) -> Coord {
        Coord::new(index, self.col)
    }
    pub fn get(&self, index: i64) -> Option<&T> {
        self.grid.get(self.coord_of(index))
    }
    pub fn get_mut(&mut self, index: i64) -> Option<&mut T> {
        self.grid.get_mut(self.coord_of(index))
    }

    pub fn iter(&self) -> ColumnIter<T> {
        ColumnIter {
            col: Column::new(self.grid.as_ref(), self.col),
            start: 0,
            end: self.grid.col_size() - 1,
        }
    }
    pub fn iter_mut<'col>(&'col mut self) -> ColumnMutIter<'grid, 'col, T> {
        let end = self.grid.col_size() - 1;
        ColumnMutIter {
            grid: &mut self.grid,
            col: self.col,
            start: 0,
            end,
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
        let end = self.grid.col_size() - 1;
        ColumnMutIntoIter {
            grid: self.grid,
            col: self.col,
            start: 0,
            end,
        }
    }
}

pub struct ColumnMutIter<'grid, 'col, T> {
    grid: &'col mut RawGridMut<'grid, T>,
    col: i64,
    start: i64,
    end: i64,
}
impl<'grid, 'col, T> Iterator for ColumnMutIter<'grid, 'col, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
        if self.start > self.end {
            None
        } else {
            let r = self.end;
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(r, self.col)) }
        }
    }
}

pub struct ColumnMutIntoIter<'grid, T> {
    grid: RawGridMut<'grid, T>,
    col: i64,
    start: i64,
    end: i64,
}
impl<'grid, T> Iterator for ColumnMutIntoIter<'grid, T> {
    type Item = &'grid mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
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
            let r = self.end;
            self.end -= 1;
            unsafe { self.grid.get_mut_unbound(Coord::new(r, self.col)) }
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
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

        for r in grid.rows_mut() {
            for v in r.iter().copied().zip(arr) {
                assert_eq!(v.0, v.1)
            }
            arr.rotate_left(1)
        }
    }
}
