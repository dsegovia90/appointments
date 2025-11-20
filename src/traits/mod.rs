#[allow(clippy::suspicious_operation_groupings)]
pub trait GenericWindowComparison<T: PartialOrd> {
    fn start_time(&self) -> T;
    fn end_time(&self) -> T;

    /// Check if this window clashes with any other window in the list.
    /// U and V both implement `GenericWindowComparison<T>`, but can be different types.
    fn clash_check<U, V>(this: &U, others: &[V]) -> bool
    where
        U: GenericWindowComparison<T>,
        V: GenericWindowComparison<T>,
    {
        others.iter().any(|other| {
            // Check if this start_time is between other start_time and end_time
            if this.start_time() >= other.start_time() && this.start_time() < other.end_time() {
                return true;
            }
            // Check if this end_time is between other start_time and end_time
            if this.end_time() > other.start_time() && this.end_time() <= other.end_time() {
                return true;
            }
            // Check if this is wrapping around other
            if this.start_time() <= other.start_time() && this.end_time() >= other.end_time() {
                return true;
            }
            false
        })
    }
}
