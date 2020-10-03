# 3.1 Applying Multithreading Feature to Your Project
* refactor code into library modules
* bin and lib are separate crates
    * so in main() need to refer to lib modules with "pipeview::"
    * in the lib modules refer to lib with "crate::"
* when dealing with string params favor &str slices (reference)

# 3.2 Separate Input, Statistics, and Output Threads
* split into three child threads: read, stats, write threads
    * the communicate shutdown via a mutex
    * they're all joined to the main thread (as a child thread) that exits
* code won't be fully working yet after this section
* rust has similar destructuring syntax as Javascript

# 3.3 Operation with MPSC Channels for Data Flow in Sequence
* replace mutex with channels for thread communication
* setup two channels: read -> stats, stats -> write
    * send buffers of u8 across channels
    * use an empty buffer as a "sentinel" to stop
    * `sentinel` - special value used as a condition of termination (in a loop or recursive function)
* tx and rx : transmitter and receiver 
    * can clone the tx side of channel since it's Multi Producer

# 3.4 Refactor Code to Use Crossbeam Channels
* multi-producer multi-consumer channels for message passing
    * not in std lib though but better performance
    * but used in Firefox (one of oldest projects in Rust)
* read thread sends only meta data to stats thread
    * unbounded channel, expect data to be small
* read thread sends entire data to write thread
    * this channel will be bounded to have back pressure