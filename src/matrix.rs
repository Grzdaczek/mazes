#[derive(Debug, Clone, PartialEq)]
pub struct AdjacencyMatrix<T> {
    data: Vec<T>,
    size: usize,
}

impl<T> AdjacencyMatrix<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            size: 0,
        }
    }

    pub fn push_default(&mut self)
    where
        T: Default,
    {
        let len = Self::chunk_len(self.size);
        self.data.extend((0..len).map(|_| T::default()));
        self.size += 1;
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        *self.get_mut(x, y) = value;
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        let (chunk, position) = if x > y { (x, y) } else { (y, x + y) };
        let offset = Self::chunk_offset(chunk);

        &self.data[offset + position]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut T {
        let (chunk, position) = if x > y { (x, y) } else { (y, x + y) };
        let offset = Self::chunk_offset(chunk);

        &mut self.data[offset + position]
    }

    pub fn remove(&mut self, remove: usize) {
        assert!(remove < self.size);

        let new_data = self
            .data
            .drain(..)
            .enumerate()
            .scan(
                (0, 0, 0, 0),
                |(next_chunk, next_offset, offset, chunk), (i, el)| {
                    if *next_offset == i {
                        *chunk = *next_chunk;
                        *next_chunk += 1;
                        *offset = *next_offset;
                        *next_offset = Self::chunk_offset(*next_chunk);
                    }

                    let inner_i = i - *offset;
                    let chunk = *chunk;

                    Some((chunk, inner_i, el))
                },
            )
            .filter_map(|pos| match pos {
                (ch, _, _) if ch == remove => None,
                (ch, inn_i, _) if inn_i == remove && ch > remove => None,
                (ch, inn_i, _) if inn_i == (2 * remove) + 1 && ch > remove => None,
                (_, _, el) => Some(el),
            })
            .collect();

        self.data = new_data;
    }

    #[inline]
    pub fn chunk_len(chunk: usize) -> usize {
        (chunk * 2) + 1
    }

    #[inline]
    pub fn chunk_offset(chunk: usize) -> usize {
        chunk.pow(2)
    }

    pub fn iter(&self) -> AdjacencyMatrixIterator<T> {
        AdjacencyMatrixIterator::new(self)
    }
}

pub struct AdjacencyMatrixIterator<'a, T> {
    matrix: &'a AdjacencyMatrix<T>,
    index: usize,
    inner_index: usize,
    chunk: usize,
}

impl<'a, T> AdjacencyMatrixIterator<'a, T> {
    pub fn new(matrix: &'a AdjacencyMatrix<T>) -> Self {
        Self {
            matrix,
            index: 0,
            inner_index: 0,
            chunk: 0,
        }
    }
}

impl<'a, T> Iterator for AdjacencyMatrixIterator<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next = if self.index < self.matrix.data.len() {
            let (x, y) = if self.chunk > self.inner_index {
                (self.chunk, self.inner_index)
            } else {
                (self.inner_index - self.chunk, self.chunk)
            };

            Some((x, y, &self.matrix.data[self.index]))
        } else {
            None
        };

        self.index += 1;
        self.inner_index += 1;
        if self.index >= AdjacencyMatrix::<T>::chunk_offset(self.chunk + 1) {
            self.chunk += 1;
            self.inner_index = 0;
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create() {
        let matrix = AdjacencyMatrix::<u8>::new();
        assert_eq!(matrix.data, vec![]);
        assert_eq!(matrix.size, 0);
    }

    #[test]
    fn can_push_default() {
        let mut matrix = AdjacencyMatrix::<u8>::new();
        matrix.push_default();
        assert_eq!(matrix.data, vec![0]);
        assert_eq!(matrix.size, 1);

        matrix.push_default();
        assert_eq!(matrix.data, vec![0, 0, 0, 0]);
        assert_eq!(matrix.size, 2);

        matrix.push_default();
        assert_eq!(matrix.data, vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(matrix.size, 3);
    }

    fn create_set() -> AdjacencyMatrix<u8> {
        let mut matrix = AdjacencyMatrix::<u8>::new();
        matrix.push_default();
        matrix.push_default();
        matrix.push_default();
        matrix.set(0, 0, 00);
        matrix.set(0, 1, 01);
        matrix.set(0, 2, 02);
        matrix.set(1, 0, 10);
        matrix.set(1, 1, 11);
        matrix.set(1, 2, 12);
        matrix.set(2, 0, 20);
        matrix.set(2, 1, 21);
        matrix.set(2, 2, 22);

        matrix
    }

    #[test]
    fn can_get_and_set() {
        let matrix = create_set();

        assert_eq!(matrix.data, vec![00, 10, 01, 11, 20, 21, 02, 12, 22]);
    }

    #[test]
    fn can_remove() {
        let mut matrix = create_set();
        matrix.remove(1);

        assert_eq!(matrix.data, vec![00, 20, 02, 22]);
    }

    #[test]
    fn can_iter() {
        let matrix = create_set();
        let diff: Vec<(usize, usize, &u8)> = vec![
            (0, 0, &00),
            (1, 0, &10),
            (0, 1, &01),
            (1, 1, &11),
            (2, 0, &20),
            (2, 1, &21),
            (0, 2, &02),
            (1, 2, &12),
            (2, 2, &22),
        ];

        assert!(matrix.iter().eq(diff.into_iter()));
    }
}
