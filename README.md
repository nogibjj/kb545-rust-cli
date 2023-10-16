[![Clippy](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/lint.yml)
[![Tests](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml/badge.svg)](https://github.com/nogibjj/rust-data-engineering/actions/workflows/tests.yml)


# Mini-Project 7: Rust Command Line Tool

The following project is some Rust code that was written up, to perform basic tasks, that also was created with the intent of being used as a CLI. The program will take in 2 parameters, and output the results so far. At this point, the user will actually be asked whether they want the program to end or if they want to customize the output themselves. If they elect to do so, the program will offer the user the choice, and then proceed to make the proper changes. All functions have been tested, linted, and formatted. 

### Tasks Performed

* Created library rust file, contains multiple methods that deal with vector creation, alteration, and random number generation. Also contains the testing function, which is the function that is utilized to test the code during the CI/CD process
* Created main rust file, that sets up the CLI interface for reading in what the user provides, and utilizing this information with the files from the library file. Also, sets up an interactive enviroment with the user utilizng the command line
* Sets up linting and formatting within the makefile
