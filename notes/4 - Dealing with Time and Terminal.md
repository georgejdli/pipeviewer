# 4.1 Working with Instant
* use time elapsed from an instant to measure total time

# 4.2 Calculate a Delta Time Using Instants
* want to measure the duration between two instants to measure time elapsed between runs

# 4.3 Use Durations to Create an Ergonimic Timer Struct
* Duration - represent span of time, typically used for system timeouts
    * subtract one instance from another, basically same usage as section 4.2

# 4.4 Using Timer, Output Progress Statistics at a Steady Rate
* define a trait to output seconds in a time format
    * trait is implemented for concrete f64 type so we can chain `as_time()` method
    * `start.elapsed().as_secs().as_time()`

# 4.5 Applying Crossterm and Colorizing the Output
* crossterm works on both Unix and Windows terminals
* use io::stderr instead of eprint! macro to display errors
* use execute! macro to run commands