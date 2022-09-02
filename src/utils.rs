/*
  f(x,y): n -> x + (y * cols)
  f(n): x -> (n mod cols).floor()
  f(n): y -> (n / cols).floor()
*/

pub fn x_and_y_to_index(x: usize, y: usize, cols: usize) -> usize {
    (x + y * cols) as usize
}

pub fn index_to_x_and_y(index: usize, cols: usize) -> (usize, usize) {
    (index % cols as usize, (index / cols) as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_convert_x_and_y_to_index() {
        assert_eq!(x_and_y_to_index(0, 0, 4), 0);
        assert_eq!(x_and_y_to_index(1, 0, 4), 1);
        assert_eq!(x_and_y_to_index(2, 0, 4), 2);
        assert_eq!(x_and_y_to_index(3, 0, 4), 3);
        assert_eq!(x_and_y_to_index(0, 1, 4), 4);
        assert_eq!(x_and_y_to_index(1, 1, 4), 5);
        assert_eq!(x_and_y_to_index(2, 1, 4), 6);
        assert_eq!(x_and_y_to_index(3, 1, 4), 7);
        assert_eq!(x_and_y_to_index(0, 2, 4), 8);
    }

    #[test]
    fn it_convert_index_to_x_and_y() {
        assert_eq!(index_to_x_and_y(0, 4), (0, 0));
        assert_eq!(index_to_x_and_y(1, 4), (1, 0));
        assert_eq!(index_to_x_and_y(2, 4), (2, 0));
        assert_eq!(index_to_x_and_y(3, 4), (3, 0));
        assert_eq!(index_to_x_and_y(4, 4), (0, 1));
        assert_eq!(index_to_x_and_y(5, 4), (1, 1));
        assert_eq!(index_to_x_and_y(6, 4), (2, 1));
        assert_eq!(index_to_x_and_y(7, 4), (3, 1));
        assert_eq!(index_to_x_and_y(8, 4), (0, 2));
    }
}
