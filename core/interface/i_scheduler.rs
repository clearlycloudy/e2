pub trait IScheduler {
    type Item;
    fn new( items: &[Self::Item] ) -> Self;
}
