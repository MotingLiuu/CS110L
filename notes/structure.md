# Structure

## main()

The responsibilities that remain in the `main` function should be limited
to the following:

- Calling the command line parsing logic with the argument values

- Setting up any other configuration

- Calling a run function in `lib.rs`

- Handling the error if run returns an error
