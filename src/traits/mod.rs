#[allow(clippy::suspicious_operation_groupings)]
pub trait GenericWindowComparison<T: PartialOrd> {
    fn start_time(&self) -> T;
    fn end_time(&self) -> T;

    /// Check if this window clashes with any other window in the list.
    fn clash_check(&self, others: &[impl GenericWindowComparison<T>]) -> bool {
        others.iter().any(|other| {
            // Check if `self` `start_time` is between other
            // start_time and end_time
            if self.start_time() >= other.start_time() && self.start_time() < other.end_time() {
                return true;
            }
            // Check if `self` `end_time` is between other
            // `start_time` and `end_time`
            if self.end_time() > other.start_time() && self.end_time() <= other.end_time() {
                return true;
            }
            // Check if `self` is wrapping around other
            if self.start_time() <= other.start_time() && self.end_time() >= other.end_time() {
                return true;
            }
            false
        })
    }
}
