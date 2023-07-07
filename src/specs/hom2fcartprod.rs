use core::iter::Peekable;

/// Two-fold [Cartesian product] of [iterators] that are "homogeneous" in the sense that
/// they iterate over the same type of items. Notice that if the elements in the iterators repeat,
/// the resulting iterator will repeat as well.
/// 
/// ## Examples
/// 
/// ### Manual iteration
/// 
/// ```
/// use cart_prod::specs::Hom2FCartProd;
/// 
/// let it1 = 0..=1;
/// let it2 = 0..=1;
/// 
/// let mut it = Hom2FCartProd::new(it1, it2);
/// 
/// assert_eq!(it.next(), Some([0, 0]));
/// assert_eq!(it.next(), Some([0, 1]));
/// assert_eq!(it.next(), Some([1, 0]));
/// assert_eq!(it.next(), Some([1, 1]));
/// assert_eq!(it.next(), None);
/// ```
/// 
/// ### For loop with pattern matching
/// 
/// ```
/// use cart_prod::specs::Hom2FCartProd;
/// use core::fmt::Write;
/// 
/// let it1 = 0..=1;
/// let it2 = 0..=1;
/// 
/// let mut s = String::new();
/// 
/// for [el1, el2] in Hom2FCartProd::new(it1, it2) {
///    // The panic is intentional to keep the example simple.
///    writeln!(s, "{el1} {el2}").unwrap();
/// }
/// 
/// assert_eq!(s, "0 0\n0 1\n1 0\n1 1\n");
/// ```
/// 
/// [Cartesian product]: https://en.wikipedia.org/wiki/Cartesian_product
/// [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
pub struct Hom2FCartProd<Item, I1,I2>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
{
    curr_it1: Peekable<I1>,
    curr_it2: I2,
    // Original iterator 1 is not required because
    // the traversal over it1 happens only once.
    orig_it2: I2,
}

impl<Item, I1, I2> Hom2FCartProd<Item, I1, I2>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
{
    /// Creates a new [`Hom2FCartProd`] from two iterators.
    pub fn new(it1: I1, it2: I2) -> Self {
        Self {
            curr_it1: it1.peekable(),
            curr_it2: it2.clone(),
            orig_it2: it2,
        }
    }
}

impl<Item, I1, I2> Iterator for Hom2FCartProd<Item, I1, I2>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
{
    type Item = [Item;2];

    fn next(&mut self) -> Option<Self::Item> {
        let mut el1 = self.curr_it1.peek()?.clone();
        let el2 = match self.curr_it2.next() {
            Some(el2) => el2,
            None => {
                let _ = self.curr_it1.next()?;
                el1 = self.curr_it1.peek()?.clone();
                self.curr_it2 = self.orig_it2.clone();
                self.curr_it2.next()?
            }
        };
        Some([el1, el2])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min1, max1) = self.curr_it1.size_hint();
        let (min2, max2) = self.curr_it2.size_hint();
        let min = min1.saturating_mul(min2);
        let max = match (max1, max2) {
            (Some(max1), Some(max2)) => max1.checked_mul(max2),
            _ => None,
        };
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_empty() {
        let it1 = 0..0;
        let it2 = 0..2;
        let mut it = Hom2FCartProd::new(it1, it2);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_size_hint_simple() {
        let it1 = 0..2;
        let it2 = 0..2;
        let it = Hom2FCartProd::new(it1, it2);
        assert_eq!(it.size_hint(), (4, Some(4)));
    }

    #[test]
    fn test_size_hint_empty() {
        let it1 = 0..0;
        let it2 = 0..2;
        let it = Hom2FCartProd::new(it1, it2);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }
}
