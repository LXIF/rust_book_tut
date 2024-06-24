// comments with three slashes use markdown for export to crates
// check it out with cargo doc --open
// run tests with cargo test

//! this comment comments the item it is enclosed in
//! typically used at root scope of lib or module

/// # The Wargle Module
/// the wargle module contains howdy
///
/// # Examples
/// ```
/// let arg = 7;
/// let answer = publish_crate::wargle::add_one(arg);
///
/// assert_eq!(8, answer);
/// ```

pub mod wargle {
    /// howdy is called like so:
    /// ```
    /// publish_crate::wargle::howdy()
    /// ```

    pub fn howdy() {
        println!("howdy")
    }

    /// adds one
    /// /// ```
    /// let arg = 10;
    /// let answer = add_one(arg);
    ///
    /// assert_eq!(11, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}
