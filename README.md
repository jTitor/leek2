# Summary
All of the code - engine, game, everything.
(https://leumi1umbc.visualstudio.com/DefaultCollection/Leek2/)
A Trello page might be added later.

# Required Tools
To properly build and test the entire product, you'll need:
  * Rust
  * Gradle
    * Java, since Gradle depends on this

The bootstrapper in `tools` should install all of these by default.

# Building and Testing
To build all artifacts in the project, go to the `src/` directory and call `gradle doBuild`. To test those artifacts, call `gradle doTest` afterward.