use std::any::Any;
use std::io::Error;

use clap::{Parser, Subcommand};

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

// App commands
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

enum ConfigKind {
    TRAIN,
    TEST,
    INFER,
}

trait Config {
    fn kind(&self) -> ConfigKind;

    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct TrainConfig {
    input_nodes: u16,
    hidden_nodes: u16,
    output_nodes: u16,
    learning_rate: f32,
    mnist_training_data_csv_filename: String,
    num_training_epochs: u16,
    model_output_filename: String,
}

impl Config for TrainConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::TRAIN
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TrainConfig {
    pub fn new(
        input_nodes: u16,
        hidden_nodes: u16,
        output_nodes: u16,
        learning_rate: f32,
        mnist_training_data_csv_filename: &str,
        num_training_epochs: u16,
        model_output_filename: &str,
    ) -> Self {
        TrainConfig {
            input_nodes: input_nodes,
            hidden_nodes: hidden_nodes,
            output_nodes: output_nodes,
            learning_rate: learning_rate,
            mnist_training_data_csv_filename: String::from(mnist_training_data_csv_filename),
            num_training_epochs: num_training_epochs,
            model_output_filename: String::from(model_output_filename),
        }
    }
}

impl Default for TrainConfig {
    fn default() -> Self {
        Self {
            input_nodes: DEFAULT_INPUT_NODES,
            hidden_nodes: DEFAULT_HIDDEN_NODES,
            output_nodes: DEFAULT_OUTPUT_NODES,
            learning_rate: DEFAULT_LEARNING_RATE,
            mnist_training_data_csv_filename: String::from(
                DEFAULT_MNIST_TRAINING_DATA_CSV_FILENAME,
            ),
            num_training_epochs: DEFAULT_NUM_TRAINING_EPOCHS,
            model_output_filename: String::from(DEFAULT_MODEL_OUTPUT_FILENAME),
        }
    }
}

// Train the NN
fn train(config: Box<dyn Config>) -> Result<(), Error> {
    let config = match config.as_any().downcast_ref::<TrainConfig>() {
        Some(config) => config,
        None => panic!("&config isn't a TrainConfig!"),
    };

    println!("Train config is {:#?}", config);

    // Create the NN

    // Load the training data CSV

    // Save the model

    Ok(())
}

#[derive(Debug)]
struct TestConfig {
    mnist_test_data_csv_filename: String,
    model_input_filename: String,
}

impl Config for TestConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::TEST
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TestConfig {
    pub fn new(mnist_test_data_csv_filename: &str, model_input_filename: &str) -> Self {
        TestConfig {
            mnist_test_data_csv_filename: String::from(mnist_test_data_csv_filename),
            model_input_filename: String::from(model_input_filename),
        }
    }
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            mnist_test_data_csv_filename: String::from(DEFAULT_MNIST_TEST_DATA_CSV_FILENAME),
            model_input_filename: String::from(DEFAULT_MODEL_INPUT_FILENAME),
        }
    }
}

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

#[derive(Debug)]
struct InferConfig {
    model_input_filename: String,
    png_input_filename: String,
}

impl Config for InferConfig {
    fn kind(&self) -> ConfigKind {
        ConfigKind::INFER
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl InferConfig {
    pub fn new(model_input_filename: &str, png_input_filename: &str) -> Self {
        InferConfig {
            model_input_filename: String::from(model_input_filename),
            png_input_filename: String::from(png_input_filename),
        }
    }
}

impl Default for InferConfig {
    fn default() -> Self {
        Self {
            model_input_filename: String::from(DEFAULT_MODEL_INPUT_FILENAME),
            png_input_filename: String::from(DEFAULT_PNG_INPUT_FILENAME),
        }
    }
}

// Infer a new handwritten digit
fn infer(config: Box<dyn Config>) -> Result<(), Error> {
    let config = match config.as_any().downcast_ref::<InferConfig>() {
        Some(config) => config,
        None => panic!("&config isn't a InferConfig!"),
    };

    println!("Inter config is {:#?}", config);

    // Load the model

    // Create the NN

    // Read an image and infer the digit

    Ok(())
}

fn main() -> Result<(), Error> {
    let app = App::parse();

    let config: Box<dyn Config> = match &app.command {
        Commands::Train {
            input_nodes,
            hidden_nodes,
            output_nodes,
            learning_rate,
            mnist_training_data_csv_filename,
            num_training_epochs,
            model_output_filename,
        } => {
            let config = TrainConfig::new(
                *input_nodes,
                *hidden_nodes,
                *output_nodes,
                *learning_rate,
                mnist_training_data_csv_filename,
                *num_training_epochs,
                model_output_filename,
            );
            Box::new(config)
            // train(config)
        }
        Commands::Test {
            mnist_test_data_csv_filename,
            model_input_filename,
        } => {
            let config = TestConfig::new(mnist_test_data_csv_filename, model_input_filename);
            Box::new(config)
            // test(config)
        }
        Commands::Infer {
            model_input_filename,
            png_input_filename,
        } => {
            let config = InferConfig::new(model_input_filename, png_input_filename);
            Box::new(config)
            // infer(config)
        }
    };

    match config.kind() {
        ConfigKind::TRAIN => train(config),
        ConfigKind::TEST => test(config),
        ConfigKind::INFER => infer(config),
    }
}
