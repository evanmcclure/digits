use std::io::Error;

use clap::{Args, Parser, Subcommand};

// Setup constants that control the neural network (NN)
const DEFAULT_INPUT_NODES: u16 = 784;
const DEFAULT_HIDDEN_NODES: u16 = 200;
const DEFAULT_OUTPUT_NODES: u16 = 10;
const DEFAULT_LEARNING_RATE: f32 = 0.1;
const DEFAULT_MNIST_TRAINING_DATA_CSV_FILENAME: &str = "mnist_train.csv";
const DEFAULT_NUM_TRAINING_EPOCHS: u16 = 5;
const DEFAULT_MODEL_OUTPUT_FILENAME: &str = "model.bin";
const DEFAULT_MNIST_TEST_DATA_CSV_FILENAME: &str = "mnist_test.csv";
const DEFAULT_MODEL_INPUT_FILENAME: &str = "model.bin";
const DEFAULT_PNG_INPUT_FILENAME: &str = "image.png";

#[derive(Debug, Parser)]
#[clap(about, version)]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Train the neural network
    Train {
        #[arg(long, default_value_t = DEFAULT_INPUT_NODES, global = true)]
        input_nodes: u16,

        #[arg(long, default_value_t = DEFAULT_HIDDEN_NODES, global = true)]
        hidden_nodes: u16,

        #[arg(long, default_value_t = DEFAULT_OUTPUT_NODES, global = true)]
        output_nodes: u16,

        #[arg(long, default_value_t = DEFAULT_LEARNING_RATE)]
        learning_rate: f32,

        #[arg(long, default_value = DEFAULT_MNIST_TRAINING_DATA_CSV_FILENAME)]
        mnist_training_data_csv_filename: String,

        #[arg(long, default_value_t = DEFAULT_NUM_TRAINING_EPOCHS)]
        num_training_epochs: u16,

        #[arg(long, default_value = DEFAULT_MODEL_OUTPUT_FILENAME)]
        model_output_filename: String,
    },

    /// Test the neural network
    Test {
        #[arg(long, default_value = DEFAULT_MNIST_TEST_DATA_CSV_FILENAME)]
        mnist_test_data_csv_filename: String,

        #[arg(long, default_value = DEFAULT_MODEL_INPUT_FILENAME)]
        model_input_filename: String,
    },

    /// Infer a hand-written digit
    Infer {
        #[arg(long, default_value = DEFAULT_MODEL_INPUT_FILENAME)]
        model_input_filename: String,

        #[arg(long, default_value = DEFAULT_PNG_INPUT_FILENAME)]
        png_input_filename: String,
    },
}

// Train the NN
fn train(
    input_nodes: u16,
    hidden_nodes: u16,
    output_nodes: u16,
    learning_rate: f32,
    mnist_training_data_csv_filename: &str,
    num_training_epochs: u16,
    model_output_filename: &str,
) -> Result<(), Error> {
    // Create the NN

    // Load the training data CSV

    // Save the model

    println!("train");
    Ok(())
}

// Test the NN
fn test(mnist_test_data_csv_filename: &str, model_input_filename: &str) -> Result<(), Error> {
    // Load the model

    // Create the NN

    // Load the test data CSV

    // Calculate the performance score

    println!("test");
    Ok(())
}

// Infer a new handwritten digit
fn infer(model_input_filename: &str, png_input_filename: &str) -> Result<(), Error> {
    // Load the model

    // Create the NN

    // Read an image and infer the digit

    println!("infer");
    Ok(())
}

fn main() -> Result<(), Error> {
    let app = App::parse();

    println!("app args are {:#?}", &app);

    match &app.command {
        Commands::Train {
            input_nodes,
            hidden_nodes,
            output_nodes,
            learning_rate,
            mnist_training_data_csv_filename,
            num_training_epochs,
            model_output_filename,
        } => train(
            *input_nodes,
            *hidden_nodes,
            *output_nodes,
            *learning_rate,
            mnist_training_data_csv_filename,
            *num_training_epochs,
            model_output_filename,
        ),
        Commands::Test {
            mnist_test_data_csv_filename,
            model_input_filename,
        } => test(mnist_test_data_csv_filename, model_input_filename),
        Commands::Infer {
            model_input_filename,
            png_input_filename,
        } => infer(model_input_filename, png_input_filename),
    }
}
