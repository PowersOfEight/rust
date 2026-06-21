/**

 A simple version of the Iterator trait to make type associations make more sense``

Look at [section 20.2.1 of the rust book](https://rust-book.cs.brown.edu/ch20-02-advanced-traits.html)
for more information regarding this.  We'll create it here so that we can see the association in action

*/
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
