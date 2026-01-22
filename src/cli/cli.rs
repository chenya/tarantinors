mod http_client;
mod services;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use services::{interviews::InterviewsService, movies::MoviesService, quotes::QuotesService};
use tarantino_rs::interviews::api::models::CreateInterviewRequest;
use tarantino_rs::movies::api::models::CreateMovieRequest;
use tarantino_rs::quotes::api::models::CreateQuoteRequest;
#[derive(Parser)]
#[command(name = "tarantinors-cli")]
#[command(author, version, about = "Manage Tarantinors app data through api", long_about = None)]
struct Cli {
    #[arg(short, long, global = true, default_value = "http://localhost:3000")]
    pub url: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Movies Operations
    Movies {
        #[command(subcommand)]
        command: MoviesCommands,
    },
    /// Interviews Operations
    Interviews {
        #[command(subcommand)]
        command: InterviewsCommands,
    },
    /// Quotes Operations
    Quotes {
        #[command(subcommand)]
        command: QuotesCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum MoviesCommands {
    /// List all movies
    List {},

    /// Create a new movie
    Create {
        /// The Movie body
        #[arg(short, long, value_parser = parse_json::<CreateMovieRequest>)]
        body: CreateMovieRequest,
    },

    /// Read a movie
    Read {
        #[arg(short, long, default_value_t = 1)]
        movie_id: i32,
    },

    /// Delete a movie
    Delete {
        #[arg(short, long, default_value_t = 1)]
        movie_id: i32,
    },
}

impl MoviesCommands {
    pub async fn execute(&self, movies_service: &MoviesService) -> String {
        match self {
            MoviesCommands::List {} => {
                // Implement listing movies
                let command_results = match movies_service.list().await {
                    Ok(movies) => serde_json::to_string_pretty(&movies)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            MoviesCommands::Create { body } => {
                // Implement creating a movie

                let command_results = match movies_service.create_movie(body).await {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            MoviesCommands::Read { movie_id } => {
                // Implement reading a movie

                let command_results = match movies_service.get_movie(movie_id.clone()).await {
                    Ok(movies) => serde_json::to_string_pretty(&movies)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            MoviesCommands::Delete { movie_id } => {
                // Implement deleting a movie

                let command_results = match movies_service.delete_movie(movie_id.clone()).await {
                    Ok(movies) => serde_json::to_string_pretty(&movies)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum InterviewsCommands {
    /// List all interviews
    List {},

    /// Create a new interview
    Create {
        /// The Movie body
        #[arg(short, long, value_parser = parse_json::<CreateInterviewRequest>)]
        body: CreateInterviewRequest,
    },

    /// Read an interview
    Read {
        #[arg(short, long, default_value_t = 1)]
        interview_id: i32,
    },

    /// Delete an interview
    Delete {
        #[arg(short, long, default_value_t = 1)]
        interview_id: i32,
    },
}

impl InterviewsCommands {
    pub async fn execute(&self, interviews_service: &InterviewsService) -> String {
        match self {
            InterviewsCommands::List {} => {
                // Implement listing interviews

                let command_results = match interviews_service.list().await {
                    Ok(interviews) => serde_json::to_string_pretty(&interviews)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            InterviewsCommands::Create { body } => {
                // Implement creating an interview

                let command_results = match interviews_service.create_interview(body).await {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            InterviewsCommands::Read { interview_id } => {
                // Implement reading an interview

                let command_results =
                    match interviews_service.get_interview(interview_id.clone()).await {
                        Ok(quote) => serde_json::to_string_pretty(&quote)
                            .map_err(|e| format!("Error: {}", e.to_string()))
                            .unwrap(),
                        Err(e) => format!("Error: {}", e.to_string()),
                    };

                command_results
            }
            InterviewsCommands::Delete { interview_id } => {
                // Implement deleting an interview

                let command_results = match interviews_service
                    .delete_interview(interview_id.clone())
                    .await
                {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum QuotesCommands {
    /// List all quotes
    List {},

    /// Create a new quote
    Create {
        /// The Quote body
        #[arg(short, long, value_parser = parse_json::<CreateQuoteRequest>)]
        body: CreateQuoteRequest,
    },

    /// Read a quote
    Read {
        /// The quote id
        #[arg(short, long, default_value_t = 1)]
        quote_id: i32,
    },

    /// Delete a quote
    Delete {
        /// The quote id
        #[arg(short, long, default_value_t = 1)]
        quote_id: i32,
    },
}

impl QuotesCommands {
    pub async fn execute(&self, quotes_service: &QuotesService) -> String {
        match self {
            QuotesCommands::List {} => {
                let command_results = match quotes_service.list_quotes().await {
                    Ok(quotes) => serde_json::to_string_pretty(&quotes)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            QuotesCommands::Create { body } => {
                // Implement creating a quote
                let command_results = match quotes_service.create_quote(body).await {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            QuotesCommands::Read { quote_id } => {
                // Implement reading a quote

                let command_results = match quotes_service.get_quote(quote_id.clone()).await {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
            QuotesCommands::Delete { quote_id } => {
                // Implement deleting a quote
                let command_results = match quotes_service.delete_quote(quote_id.clone()).await {
                    Ok(quote) => serde_json::to_string_pretty(&quote)
                        .map_err(|e| format!("Error: {}", e.to_string()))
                        .unwrap(),
                    Err(e) => format!("Error: {}", e.to_string()),
                };

                command_results
            }
        }
    }
}

fn parse_json<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let url = cli.url;

    let command = cli.command;

    match command {
        Commands::Movies { command } => {
            // Execute movie commands
            let movies_service = MoviesService::new(url.clone())?;
            let output = command.execute(&movies_service).await;
            println!("{}", output);
        }
        Commands::Interviews { command } => {
            // Execute interview commands
            // let interviews_url = format!("{}/api/v1/interviews", url);
            // command.execute(interviews_url);

            let interviews_service = InterviewsService::new(url.clone())?;
            let output = command.execute(&interviews_service).await;
            println!("{}", output);
        }
        Commands::Quotes { command } => {
            // Execute quote commands
            // let quotes_url = format!("{}/api/v1/quotes", url);
            let quotes_service = QuotesService::new(url.clone())?;
            let output = command.execute(&quotes_service).await;
            println!("{}", output);
        }
    }

    // Execute commands using shared operations

    Ok(())
}
