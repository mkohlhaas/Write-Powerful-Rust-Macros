// Google AI: how to test rust code based on tla+?
https://share.google/aimode/uIsbfsSpZHiVnWVuL


Testing Rust code based on a TLA+ specification relies on Model-Based Testing (MBT). Because TLA+ is an abstract design language and cannot compile directly into Rust code, you bridge the gap by generating behavioral traces from your TLA+ specification and replaying them against your actual Rust implementation. [1, 2, 3, 4, 5] 
You can accomplish this through three main workflows using modern toolchains like tla_connect or Modelator. [1, 2] 
------------------------------
## Workflow 1: Trace Generation and Replay (Apalache → ITF → Rust) [2] 
This is the most common and robust approach. You use a symbolic model checker to generate valid state-transition paths from your TLA+ spec, convert them into an intermediate format, and feed them into a Rust test suite. [2, 6] 

  [ TLA+ Spec ] ──(Apalache)──> [ ITF Trace (JSON) ] ──> [ Rust Driver Test ]


   1. Generate Traces: Use [Apalache](https://github.com/informalsystems/modelator) or the Modelator engine to explore the TLA+ model. Instead of just checking for bugs, configure it to output successful execution traces or counterexamples into the Informal Trace Format (ITF) (a standard JSON format for system traces). [1, 2, 7, 8] 
   2. Implement a Test Driver: In your Rust tests/ directory, use the [tla_connect crate](https://docs.rs/tla-connect). You create a test harness (a "Driver") that maps TLA+ states and actions to your real Rust functions. [2, 7, 9] 
   3. Assert Equivalence: Read the JSON/ITF trace inside your Rust #[test] function. For every step in the trace, trigger the corresponding Rust method and assert that Rust’s internal state matches the TLA+ variables. [2, 10, 11] 


* What it catches: Implementation gaps where your Rust code fails to handle a valid edge-case sequence allowed by your design. [2] 

------------------------------
## Workflow 2: Post-Hoc Trace Validation (Rust → NDJSON → Apalache)
If your system is highly dynamic or relies on property-based testing tools (like the [proptest crate](https://www.rustfinity.com/blog/rust-testing-libraries)), you can reverse the direction. [2, 12] 

   1. Instrument the Rust Code: Add logging inside your Rust software that records state modifications as they occur.
   2. Dump Logs: Output these logs during a test run as Newline-Delimited JSON (NDJSON).
   3. Verify with TLA+: Use tla_connect to pass this runtime trace back into Apalache against a specific TraceSpec. The model checker treats your Rust trace as a sequence of states and mathematically verifies if that exact path is valid according to the TLA+ state machine. [2, 11] 


* What it catches: Safety violations where your Rust implementation executes an invalid sequence of operations completely forbidden by the spec. [2] 

------------------------------
## Workflow 3: Interactive Symbolic Testing (RPC) [7] 
For complex distributed logic or long-running protocol engines, you can tightly couple the two runtimes interactively. [2, 7] 

   1. Launch Apalache in server mode (Apalache JSON-RPC).
   2. Your Rust test framework queries the RPC server step-by-step to ask: "What are the next possible valid actions from this state?"
   3. Rust randomly picks one of those valid actions, executes its own native implementation, checks the results, and reports the new state back to the model checker. [2, 7, 10, 13] 

------------------------------
## Alternative: Rust-Native Model Checking
If managing a Java/TLA+ environment alongside Cargo becomes a bottleneck, consider rewriting or mirroring your spec in a native Rust framework: [14] 

* [stateright](https://github.com/stateright/stateright): An excellent choice if you want to skip TLA+ syntax entirely but keep the design philosophy. It is an embedded model checker for Rust. You write your actor system model directly in Rust, model-check it, and deploy the exact same code to production using its lightweight runtime. [15, 16, 17] 
* [tla-checker (tla-rs)](https://crates.io/crates/tla-checker): A lightweight TLA+ model checker written purely in Rust. It allows you to parse simple TLA+ specs and interact with state spaces natively inside your Rust build chain without invoking standard Java-based TLC. [18, 19, 20] 

To help tailor the best testing approach for your project, what type of system are you building in Rust (e.g., a distributed consensus protocol, an async network service, or a stateful data structure)? [15, 21, 22] 

[1] [https://github.com](https://github.com/informalsystems/modelator)
[2] [https://docs.rs](https://docs.rs/tla-connect)
[3] [https://crates.io](https://crates.io/crates/tla-connect)
[4] [https://www.reddit.com](https://www.reddit.com/r/rust/comments/pnemzn/rust_vs_tla/)
[5] [https://dl.acm.org](https://dl.acm.org/doi/10.1145/3360549)
[6] [https://arxiv.org](https://arxiv.org/pdf/2503.11236)
[7] [https://docs.rs](https://docs.rs/tla-connect)
[8] [https://protocols-made-fun.com](https://protocols-made-fun.com/tlaplus/2025/12/15/tftp-symbolic-testing.html)
[9] [https://arxiv.org](https://arxiv.org/html/2503.12511v2)
[10] [https://doc.rust-lang.org](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)
[11] [https://www.usenix.org](https://www.usenix.org/system/files/atc25-tang.pdf)
[12] [https://www.rustfinity.com](https://www.rustfinity.com/blog/rust-testing-libraries)
[13] [https://www.reddit.com](https://www.reddit.com/r/dotnet/comments/1gdwtus/a_comparison_of_rusts_borrow_checker_to_the_one/)
[14] [https://users.rust-lang.org](https://users.rust-lang.org/t/tla-checker-tla-model-checker-in-rust/138086)
[15] [https://github.com](https://github.com/stateright/stateright)
[16] [https://www.stateright.rs](https://www.stateright.rs/comparison-with-tlaplus.html)
[17] [https://news.ycombinator.com](https://news.ycombinator.com/item?id=40259185)
[18] [https://www.reddit.com](https://www.reddit.com/r/rust/comments/1qw85gk/tlachecker_a_tla_model_checker_with_interactive/)
[19] [https://users.rust-lang.org](https://users.rust-lang.org/t/tla-checker-tla-model-checker-in-rust/138086)
[20] [https://crates.io](https://crates.io/crates/tla-checker/0.3.1)
[21] [https://www.youtube.com](https://www.youtube.com/watch?v=ABoCnrVWCKY)
[22] [https://www.youtube.com](https://www.youtube.com/watch?v=ms8zKpS_dZE&t=17)
