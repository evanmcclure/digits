// test.rs

// Test the NN
fn test(config: Box<dyn Config>) -> Result<(), Error> {
    let config = match config.as_any().downcast_ref::<TestConfig>() {
        Some(config) => config,
        None => panic!("&config isn't a TestConfig!"),
    };

    println!("Test config is {:#?}", config);

    // Load the model

    // Create the NN

    // Load the test data CSV

    // Calculate the performance score

    Ok(())
}
