use core::iter::Peekable;

/// Three-fold [Cartesian product] of [iterators] that are "homogeneous" in the sense that
/// they iterate over the same type of items. Notice that if the elements in the iterators repeat,
/// the resulting iterator will repeat as well.
/// 
/// ## Examples
/// 
/// ### Manual iteration
/// 
/// ```
/// use cart_prod::specs::Hom3FCartProd;
/// 
/// let it1 = 0..=1;
/// let it2 = 0..=1;
/// let it3 = 0..=1;
/// 
/// let mut it = Hom3FCartProd::new(it1, it2, it3);
/// 
/// assert_eq!(it.next(), Some([0, 0, 0]));
/// assert_eq!(it.next(), Some([0, 0, 1]));
/// assert_eq!(it.next(), Some([0, 1, 0]));
/// assert_eq!(it.next(), Some([0, 1, 1]));
/// assert_eq!(it.next(), Some([1, 0, 0]));
/// assert_eq!(it.next(), Some([1, 0, 1]));
/// assert_eq!(it.next(), Some([1, 1, 0]));
/// assert_eq!(it.next(), Some([1, 1, 1]));
/// assert_eq!(it.next(), None);
/// ```
/// 
/// ### For loop with pattern matching
/// 
/// ```
/// use cart_prod::specs::Hom3FCartProd;
/// use core::fmt::Write;
/// 
/// let it1 = 0..=1;
/// let it2 = 0..=1;
/// let it3 = 0..=1;
/// 
/// let mut s = String::new();
/// 
/// for [el1, el2, el3] in Hom3FCartProd::new(it1, it2, it3) {
///    // The panic is intentional to keep the example simple.
///    writeln!(s, "{el1} {el2} {el3}").unwrap();
/// }
/// 
/// assert_eq!(s, "0 0 0\n0 0 1\n0 1 0\n0 1 1\n1 0 0\n1 0 1\n1 1 0\n1 1 1\n");
/// ```
/// 
/// [Cartesian product]: https://en.wikipedia.org/wiki/Cartesian_product
/// [iterators]: https://doc.rust-lang.org/book/ch13-02-iterators.html
pub struct Hom3FCartProd<Item, I1,I2,I3>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
    I3: Iterator<Item=Item> + Clone,
{
    curr_it1: Peekable<I1>,
    curr_it2: Peekable<I2>,
    curr_it3: I3,
    // Original iterator 1 is not required because
    // the traversal over it1 happens only once.
    orig_it2: I2,
    orig_it3: I3,
}

impl<Item, I1, I2, I3> Hom3FCartProd<Item, I1, I2, I3>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
    I3: Iterator<Item=Item> + Clone,
{
    /// Creates a new [`Hom3FCartProd`] from three iterators.
    pub fn new(it1: I1, it2: I2, it3: I3) -> Self {
        Self {
            curr_it1: it1.peekable(),
            curr_it2: it2.clone().peekable(),
            orig_it2: it2,
            curr_it3: it3.clone(),
            orig_it3: it3,
        }
    }
}

impl<Item, I1, I2, I3> Iterator for Hom3FCartProd<Item, I1, I2, I3>
where
    Item: Clone,
    I1: Iterator<Item=Item>,
    I2: Iterator<Item=Item> + Clone,
    I3: Iterator<Item=Item> + Clone,
{
    type Item = [Item;3];

    fn next(&mut self) -> Option<Self::Item> {
        let el1 = self.curr_it1.peek()?.clone();
        let mut el2 = self.curr_it2.peek()?.clone();
        if let Some(el3) = self.curr_it3.next() {
            return Some([el1, el2, el3]);
        }
        drop(self.curr_it2.next()?);
        if let Some(el2) = self.curr_it2.peek().map(Clone::clone) {
            self.curr_it3 = self.orig_it3.clone();
            return Some([el1, el2, self.curr_it3.next()?]);
        }
        drop(self.curr_it1.next()?);
        if let Some(el1) = self.curr_it1.peek().map(Clone::clone) {
            self.curr_it3 = self.orig_it3.clone();
            self.curr_it2 = self.orig_it2.clone().peekable();
            el2 = self.curr_it2.peek()?.clone();
            return Some([el1, el2, self.curr_it3.next()?]);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (min1, max1) = self.curr_it1.size_hint();
        let (min2, max2) = self.curr_it2.size_hint();
        let (min3, max3) = self.curr_it3.size_hint();
        let minima = [min1, min2, min3];
        let maxima = [max1, max2, max3];
        let min = minima.iter().copied().fold(1usize, |prod, x| {
            prod.saturating_mul(x)
        });
        let max = maxima.iter().copied().fold(Some(1usize), |opt_prod, opt_x| {
            match opt_x {
                Some(x) => opt_prod.and_then(|prod| prod.checked_mul(x)),
                None => None,
            }
        });
        (min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let it1 = 0..=3;
        let it2 = 0..=2;
        let it3 = 0..=1;
        let mut it = Hom3FCartProd::new(it1, it2, it3);
        assert_eq!(it.next(), Some([0, 0, 0]));
        assert_eq!(it.next(), Some([0, 0, 1]));
        assert_eq!(it.next(), Some([0, 1, 0]));
        assert_eq!(it.next(), Some([0, 1, 1]));
        assert_eq!(it.next(), Some([0, 2, 0]));
        assert_eq!(it.next(), Some([0, 2, 1]));
        assert_eq!(it.next(), Some([1, 0, 0]));
        assert_eq!(it.next(), Some([1, 0, 1]));
        assert_eq!(it.next(), Some([1, 1, 0]));
        assert_eq!(it.next(), Some([1, 1, 1]));
        assert_eq!(it.next(), Some([1, 2, 0]));
        assert_eq!(it.next(), Some([1, 2, 1]));
        assert_eq!(it.next(), Some([2, 0, 0]));
        assert_eq!(it.next(), Some([2, 0, 1]));
        assert_eq!(it.next(), Some([2, 1, 0]));
        assert_eq!(it.next(), Some([2, 1, 1]));
        assert_eq!(it.next(), Some([2, 2, 0]));
        assert_eq!(it.next(), Some([2, 2, 1]));
        assert_eq!(it.next(), Some([3, 0, 0]));
        assert_eq!(it.next(), Some([3, 0, 1]));
        assert_eq!(it.next(), Some([3, 1, 0]));
        assert_eq!(it.next(), Some([3, 1, 1]));
        assert_eq!(it.next(), Some([3, 2, 0]));
        assert_eq!(it.next(), Some([3, 2, 1]));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_empty() {
        let it1 = 0..=3;
        let it2 = 0..=2;
        let it3 = 0..0;
        let mut it = Hom3FCartProd::new(it1, it2, it3);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_size_hint_simple() {
        let it1 = 0..2;
        let it2 = 0..2;
        let it3 = 0..3;
        let it = Hom3FCartProd::new(it1, it2, it3);
        assert_eq!(it.size_hint(), (12, Some(12)));
    }

    #[test]
    fn test_size_hint_empty() {
        let it1 = 0..2;
        let it2 = 0..2;
        let it3 = 0..0;
        let it = Hom3FCartProd::new(it1, it2, it3);
        assert_eq!(it.size_hint(), (0, Some(0)));
    }
}