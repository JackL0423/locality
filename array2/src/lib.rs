/// Two-dimensional array, stored internally in row-major order.
///
/// # Type Parameters
/// * `T`: the type of the elements in the array.
///
/// # Examples
///
/// ```
/// use array2::Array2;
/// let a = Array2::new(2, 3, 42_u32);
/// assert_eq!(a.get_width(), 2);
/// assert_eq!(a.get_height(), 3);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Returns the width of the array.
    ///
    /// # Returns
    ///
    /// * The width of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert_eq!(a.get_width(), 2);
    /// ```
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Returns the height of the array.
    ///
    /// # Returns
    ///
    /// * The height of the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert_eq!(a.get_height(), 3);
    /// ```
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Returns the total number of elements in the array.
    ///
    /// # Returns
    ///
    /// * The total number of elements in the array.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert_eq!(a.len(), 6);
    /// ```
    pub fn len(&self) -> usize {
        self.width * self.height
    }

    /// Returns a reference to the element at the specified row and column, or 'None' if out of bounds.
    ///
    /// # Arguments
    ///
    /// * `row`: the row index.
    /// * `col`: the column index.
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if the element exists.
    /// * `None` if the element is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert_eq!(a.get(0, 0), Some(&42_u32));
    /// assert_eq!(a.get(1, 1), Some(&42_u32));
    /// assert_eq!(a.get(3, 0), None);
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row >= self.height || col >= self.width {
            return None;
        }
        Some(&self.data[row * self.width + col])
    }

    /// Returns `true` if the array contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert!(!a.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Creates a new `Array2` containing identical values for every element.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * `val`: the value to fill every element with.
    ///
    /// # Returns
    ///
    /// * An `Array2` of appropriate dimensions containing copies of `val`.
    ///
    /// # Panics
    ///
    /// * If `width * height` overflows `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// ```
    pub fn new(width: usize, height: usize, val: T) -> Self {
        let len = width
            .checked_mul(height)
            .expect("Array2::new: width * height overflowed usize");
        let data = vec![val; len];
        Array2 {
            width,
            height,
            data,
        }
    }

    /// Constructor that creates an `Array2` from a row-major ordered vector.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * `data`: a row-major ordered vector containing the elements.
    ///
    /// # Returns
    ///
    /// * An `Array2` of appropriate dimensions containing the elements from `data`.
    ///
    /// # Panics
    ///
    /// * If `width * height` overflows `usize`.
    /// * If `data.len() != width * height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let data = vec![1, 2, 3, 4, 5, 6];
    /// let a = Array2::from_row_major(2, 3, data);
    /// assert_eq!(a.at(0, 1), Some(&2));
    /// ```
    pub fn from_row_major(width: usize, height: usize, data: Vec<T>) -> Self {
        let expected_len = width
            .checked_mul(height)
            .expect("Array2::from_row_major: width * height overflowed usize");
        assert_eq!(
            data.len(),
            expected_len,
            "Data length does not match dimensions"
        );
        Array2 {
            width,
            height,
            data,
        }
    }

    /// Constructor that creates an `Array2` from a column-major ordered vector.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * `data`: a column-major ordered vector containing the elements.
    ///
    /// # Returns
    ///
    /// * An `Array2` of appropriate dimensions containing the elements from `data`.
    ///
    /// # Panics
    ///
    /// * If `width * height` overflows `usize`.
    /// * If `data.len() != width * height`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// // Column-major: column 0 is [1, 2, 3], column 1 is [4, 5, 6].
    /// let data = vec![1, 2, 3, 4, 5, 6];
    /// let a = Array2::from_column_major(2, 3, data);
    /// assert_eq!(a.at(1, 0), Some(&2));
    /// assert_eq!(a.at(0, 1), Some(&4));
    /// ```
    pub fn from_column_major(width: usize, height: usize, data: Vec<T>) -> Self {
        let expected_len = width
            .checked_mul(height)
            .expect("Array2::from_column_major: width * height overflowed usize");
        assert_eq!(
            data.len(),
            expected_len,
            "Data length does not match dimensions"
        );
        // `data` is column-major; re-lay it out into this array's row-major
        // backing store so every other method can assume row-major storage.
        let mut row_major = data.clone();
        for col in 0..width {
            for row in 0..height {
                row_major[row * width + col] = data[col * height + row].clone();
            }
        }
        Array2 {
            width,
            height,
            data: row_major,
        }
    }

    /// Computes the row-major index for `(row, col)`, or `None` if either
    /// coordinate is out of bounds.
    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.height && col < self.width {
            Some(row * self.width + col)
        } else {
            None
        }
    }

    /// Returns a reference to the element at `(row, col)`.
    ///
    /// # Arguments
    ///
    /// * `row`: the row index (`0..get_height()`).
    /// * `col`: the column index (`0..get_width()`).
    ///
    /// # Returns
    ///
    /// * `Some(&T)` if `row` and `col` are in bounds, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// assert_eq!(a.at(0, 0), Some(&42));
    /// assert_eq!(a.at(3, 0), None);
    /// ```
    pub fn at(&self, row: usize, col: usize) -> Option<&T> {
        self.index(row, col).map(|i| &self.data[i])
    }

    /// Returns a mutable reference to the element at `(row, col)`.
    ///
    /// # Arguments
    ///
    /// * `row`: the row index (`0..get_height()`).
    /// * `col`: the column index (`0..get_width()`).
    ///
    /// # Returns
    ///
    /// * `Some(&mut T)` if `row` and `col` are in bounds, `None` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let mut a = Array2::new(2, 3, 0_u32);
    /// *a.at_mut(1, 1).unwrap() = 9;
    /// assert_eq!(a.at(1, 1), Some(&9));
    /// ```
    pub fn at_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        let idx = self.index(row, col)?;
        Some(&mut self.data[idx])
    }

    /// Sets the element at `(row, col)` to `val`.
    ///
    /// # Arguments
    ///
    /// * `row`: the row index (`0..get_height()`).
    /// * `col`: the column index (`0..get_width()`).
    /// * `val`: the value to store.
    ///
    /// # Returns
    ///
    /// * `true` if `row` and `col` were in bounds and the value was written,
    ///   `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let mut a = Array2::new(2, 3, 0_u32);
    /// assert!(a.put(1, 1, 9));
    /// assert_eq!(a.at(1, 1), Some(&9));
    /// assert!(!a.put(10, 10, 9));
    /// ```
    pub fn put(&mut self, row: usize, col: usize, val: T) -> bool {
        match self.index(row, col) {
            Some(idx) => {
                self.data[idx] = val;
                true
            }
            None => false,
        }
    }

    /// Returns an iterator over the elements of the array in row-major order.
    ///
    /// Since the array is stored in row-major order internally, this is a
    /// direct slice iterator with no per-element index arithmetic or bounds
    /// checks.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// let mut iter = a.iter_row_major();
    /// assert_eq!(iter.next(), Some(&42_u32));
    /// ```
    pub fn iter_row_major(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns an iterator over the elements of the array in column-major order.
    ///
    /// Column-major traversal of row-major storage inherently strides across
    /// memory (one cache line touched per element for large arrays); that
    /// access pattern can't be avoided without transposing the storage. This
    /// implementation still avoids recomputing `row * width + col` per
    /// element by stepping a slice with a fixed stride instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::new(2, 3, 42_u32);
    /// let mut iter = a.iter_column_major();
    /// assert_eq!(iter.next(), Some(&42_u32));
    /// ```
    pub fn iter_column_major(&self) -> impl Iterator<Item = &T> {
        let width = self.width;
        (0..width).flat_map(move |col| self.data[col..].iter().step_by(width))
    }

    /// Returns the elements as a row-major slice.
    ///
    /// Element `(row, col)` is at index `row * get_width() + col`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::from_row_major(2, 2, vec![1, 2, 3, 4]);
    /// assert_eq!(a.as_slice()[2], 3);
    /// ```
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns the elements as a mutable row-major slice.
    ///
    /// Element `(row, col)` is at index `row * get_width() + col`.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let mut a = Array2::new(2, 2, 0_u32);
    /// a.as_mut_slice()[3] = 9;
    /// assert_eq!(a.at(1, 1), Some(&9));
    /// ```
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    /// Consumes the array and returns its elements in row-major order,
    /// without copying.
    ///
    /// # Examples
    ///
    /// ```
    /// use array2::Array2;
    /// let a = Array2::from_row_major(2, 1, vec![1, 2]);
    /// assert_eq!(a.into_row_major(), vec![1, 2]);
    /// ```
    pub fn into_row_major(self) -> Vec<T> {
        self.data
    }

    // Prints data in row-major order for debugging purposes.
    #[allow(dead_code)]
    pub fn print_row_major(&self)
    where
        T: std::fmt::Debug,
    {
        for row in 0..self.height {
            for col in 0..self.width {
                if let Some(val) = self.at(row, col) {
                    print!("{:?} ", val);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_array2() {
        let a: Array2<u32> = Array2::new(2, 3, 42_u32);
        assert_eq!(a.get_width(), 2);
        assert_eq!(a.get_height(), 3);
        assert_eq!(a.len(), 6);
        assert!(!a.is_empty());
    }

    #[test]
    fn iterators_work() {
        let a: Array2<u32> = Array2::new(2, 3, 42_u32);
        let mut row_iter = a.iter_row_major();
        assert_eq!(row_iter.next(), Some(&42_u32));

        let mut col_iter = a.iter_column_major();
        assert_eq!(col_iter.next(), Some(&42_u32));
    }

    #[test]
    fn iter_row_major_order_matches_row_major_layout() {
        // width = 3, height = 2:
        // row 0: 0 1 2
        // row 1: 3 4 5
        let data: Vec<u32> = (0..6).collect();
        let a = Array2::from_row_major(3, 2, data);
        let collected: Vec<u32> = a.iter_row_major().copied().collect();
        assert_eq!(collected, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter_column_major_order_matches_column_major_traversal() {
        // Same grid as above:
        // row 0: 0 1 2
        // row 1: 3 4 5
        // Column-major visits col0 rows..., col1 rows..., col2 rows...
        let data: Vec<u32> = (0..6).collect();
        let a = Array2::from_row_major(3, 2, data);
        let collected: Vec<u32> = a.iter_column_major().copied().collect();
        assert_eq!(collected, vec![0, 3, 1, 4, 2, 5]);
    }

    #[test]
    fn from_row_major_works() {
        let data = vec![42_u32; 6];
        let a: Array2<u32> = Array2::from_row_major(2, 3, data);
        assert_eq!(a.get_width(), 2);
        assert_eq!(a.get_height(), 3);
        assert_eq!(a.len(), 6);
    }

    #[test]
    fn from_row_major_panics_on_wrong_length() {
        let data = vec![42_u32; 5];
        let result = std::panic::catch_unwind(|| {
            let _a: Array2<u32> = Array2::from_row_major(2, 3, data);
        });
        assert!(result.is_err());
    }

    #[test]
    fn from_column_major_works() {
        let data = vec![42_u32; 6];
        let a: Array2<u32> = Array2::from_column_major(2, 3, data);
        assert_eq!(a.get_width(), 2);
        assert_eq!(a.get_height(), 3);
        assert_eq!(a.len(), 6);
    }

    #[test]
    fn from_column_major_places_values_at_correct_coordinates() {
        // width = 2, height = 3, column-major input:
        // column 0 = [0, 1, 2], column 1 = [3, 4, 5]
        let data: Vec<u32> = (0..6).collect();
        let a = Array2::from_column_major(2, 3, data);
        assert_eq!(a.at(0, 0), Some(&0));
        assert_eq!(a.at(1, 0), Some(&1));
        assert_eq!(a.at(2, 0), Some(&2));
        assert_eq!(a.at(0, 1), Some(&3));
        assert_eq!(a.at(1, 1), Some(&4));
        assert_eq!(a.at(2, 1), Some(&5));
    }

    #[test]
    fn at_returns_correct_values_at_corners_and_interior() {
        let data: Vec<u32> = (0..12).collect();
        let a = Array2::from_row_major(4, 3, data);
        assert_eq!(a.at(0, 0), Some(&0));
        assert_eq!(a.at(0, 3), Some(&3));
        assert_eq!(a.at(2, 0), Some(&8));
        assert_eq!(a.at(2, 3), Some(&11));
        assert_eq!(a.at(1, 2), Some(&6));
    }

    #[test]
    fn at_returns_none_when_out_of_bounds() {
        let a: Array2<u32> = Array2::new(2, 3, 0);
        assert_eq!(a.at(3, 0), None);
        assert_eq!(a.at(0, 2), None);
        assert_eq!(a.at(3, 2), None);
    }

    #[test]
    fn put_writes_in_bounds_and_rejects_out_of_bounds() {
        let mut a: Array2<u32> = Array2::new(2, 3, 0);
        assert!(a.put(1, 1, 99));
        assert_eq!(a.at(1, 1), Some(&99));

        assert!(!a.put(3, 0, 1));
        assert!(!a.put(0, 2, 1));
    }

    #[test]
    fn at_mut_mutation_is_visible_through_at_and_iteration() {
        let mut a: Array2<u32> = Array2::new(2, 2, 0);
        *a.at_mut(0, 1).unwrap() = 7;
        assert_eq!(a.at(0, 1), Some(&7));
        assert_eq!(a.at_mut(5, 5), None);

        let collected: Vec<u32> = a.iter_row_major().copied().collect();
        assert_eq!(collected, vec![0, 7, 0, 0]);
    }

    #[test]
    fn handles_non_square_dimensions() {
        let wide = Array2::from_row_major(5, 1, (0..5).collect());
        assert_eq!(wide.at(0, 4), Some(&4));
        assert_eq!(wide.at(1, 0), None);

        let tall = Array2::from_row_major(1, 5, (0..5).collect());
        assert_eq!(tall.at(4, 0), Some(&4));
        assert_eq!(tall.at(0, 1), None);
    }
}
