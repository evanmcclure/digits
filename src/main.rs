use std::io::Error;

use clap::*;

include!("infer.rs");
include!("neural_network.rs");
include!("test.rs");
include!("train.rs");

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
const DEFAULT_PROCESSOR: Processor = Processor::Cpu;

// App commands
#[derive(clap::Parser, Debug)]
#[clap(about, version)]
struct App {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Processor {
    AppleSilicon,
    Cpu,
}

#[derive(clap::Subcommand, Debug)]
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

        #[arg(value_enum, long, default_value_t = DEFAULT_PROCESSOR)]
        processor: Processor,
    },

    /// Test the neural network
    Test {
        #[arg(long, default_value = DEFAULT_MNIST_TEST_DATA_CSV_FILENAME)]
        mnist_test_data_csv_filename: String,

        #[arg(long, default_value = DEFAULT_MODEL_INPUT_FILENAME)]
        model_input_filename: String,

        #[arg(value_enum, long, default_value_t = DEFAULT_PROCESSOR)]
        processor: Processor,
    },

    /// Infer a hand-written digit
    Infer {
        #[arg(long, default_value = DEFAULT_MODEL_INPUT_FILENAME)]
        model_input_filename: String,

        #[arg(long, default_value = DEFAULT_PNG_INPUT_FILENAME)]
        png_input_filename: String,

        #[arg(value_enum, long, default_value_t = DEFAULT_PROCESSOR)]
        processor: Processor,
    },
}

fn main() -> Result<(), Error> {
    let app = App::parse();

    match &app.command {
        Commands::Train {
            input_nodes,
            hidden_nodes,
            output_nodes,
            learning_rate,
            mnist_training_data_csv_filename,
            num_training_epochs,
            model_output_filename,
            processor,
        } => train(
            *input_nodes,
            *hidden_nodes,
            *output_nodes,
            *learning_rate,
            mnist_training_data_csv_filename,
            *num_training_epochs,
            model_output_filename,
            processor.clone(),
        ),
        Commands::Test {
            mnist_test_data_csv_filename,
            model_input_filename,
            processor,
        } => test(
            mnist_test_data_csv_filename,
            model_input_filename,
            processor.clone(),
        ),
        Commands::Infer {
            model_input_filename,
            png_input_filename,
            processor,
        } => infer(model_input_filename, png_input_filename, processor.clone()),
    }
}
