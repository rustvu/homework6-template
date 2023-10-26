# RustVU (CS 3891) - Homework 6

## Optional / warmup exercises for the week

I highly recommend the [Rustlings project](https://github.com/rust-lang/rustlings) for practicing the basic concepts we learn in this class. These completely optional, not graded/submitted exercises can help you to teach Rust programming to your "fingers".

The recommended exercises for this week:

- `iterators`

## Assignment

This assignment contains a single Rust project focusing on iterators. The goal of the exercise is to understand how iterators implement lazy evaluation and how iterators can encapsulate other iterators to implement a complex processing workflow. 

The project has a library crate which implements the necessary iterators to emulate the workflow at a pizza parlor, starting from raw ingredients, and combining these to produce pizzas. The project also has an application (binary) crate, which demonstrates the use of the library. Your _only_ job is to implement the necessary pieces in the library crate (`lib.rs`).
The pizza baking workflow is shown below (the boxes represent the iterators, and the produced values are shown on the arrows).

```
                                                capacity
        ┌────────┐  Water                      ┌─────────┐
infinte │ Faucet ├─────────┐                   │   Can   │
        └────────┘         │                   └────┬────┘
                           │                        │Sauce
        ┌────────┐Flour    │                        │
capacity│  Sack  ├───┐     │                        │       
        └────────┘   └►┌───▼──────────┐Dough ┌──────▼───────┐RawPizza┌───────┐
                       │  BreadBoard  ├──────►  PrepTable   ├────────► Oven  ├────► Pizza
        ┌────────┐   ┌►└───▲──────────┘      └──────▲───────┘        └───────┘
capacity│  Jar   ├───┘     │                        │ n_cheese      baking_time
        └────────┘ Yeast   │                        │
                           │                        │Cheese
        ┌────────┐   Salt  │                   ┌────┴─────┐
capacity│ Shaker ├─────────┘                   │  Grater  │
        └────────┘                             └──────────┘
                                                throughput

```

Please, read the comments in the source code to understand what is expected from the iterator types.

I placed `// TODO` comments in the code where I expect you to add implementation code. The test code is separated to a standalone module and is clearly marked with a `// DO NOT EDIT BELOW THIS LINE` comment. This should be evident: changing the test code is a (not too intelligent) way of cheating. I will handle any such attempts accordingly. However, you are allowed and encouraged to look at the test code to better understand what is expected from you.

## Use

You can always check your work with `cargo test`. You can also run individual tests by running `cargo test <test-name>` (see the names below).

You can use/modify the application (binary) program (`main.rs`) as you wish to experiment with your library. This won't be graded. Just make sure, that it compiles.

## Grading

Make sure you __commit__ and __push__ your assignment repository once you manage to run `cargo test` without any errors or warnings.

The homework is graded by test (no partial credits are given for failed tests):

|Test          |Points|
|--------------|------|
|faucet        |     5|
|sack          |    10|
|jar           |    10|
|shaker        |     5|
|breadboard    |    10|
|can           |     5|
|grate         |    30|
|prep_board    |    15|
|oven          |    10|

Once you __push__ your solution to the repository, GitHub Classroom will run the automated test. I highly recommend to [verify your results of this CI/CD worflow](https://docs.github.com/en/education/manage-coursework-with-github-classroom/learn-with-github-classroom/view-autograding-results) - I use these results for grading your work.
