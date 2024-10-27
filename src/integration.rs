/*
    This code helps keeping the changes within the World Model and the Sim Engine aligned.
    It is the shared glue that keeps them working together, as they may individually change.
    
    In the long-term, the World Model can be user generated and configured,
    and will dynamically be interpreted by the SimEngine.
    
    In the short term, the World Model is hard-coded in Rust, and the SimEngine has certain
    hard-wired assumptions about that code. To make them play nicely together during the development evolution,
    is the job of this module.

*/


