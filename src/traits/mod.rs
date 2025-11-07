#[allow(clippy::suspicious_operation_groupings)]
pub trait GenericWindowComparison<T: PartialOrd> {
    fn start_time(&self) -> T;
    fn end_time(&self) -> T;

    fn clash_check(
        this: &impl GenericWindowComparison<T>,
        others: &[impl GenericWindowComparison<T>],
    ) -> bool
    where
        Self: Sized,
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

    fn clash_check_boxed(
        this: &impl GenericWindowComparison<T>,
        others: &[Box<dyn GenericWindowComparison<T>>],
    ) -> bool
    where
        Self: Sized,
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
