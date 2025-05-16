use borsh::{BorshDeserialize, BorshSerialize};
use crosstown_bus::{CrosstownBus, HandleError, MessageHandler};
use dotenv::dotenv;
use std::{env, thread, time};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

pub struct UserCreatedHandler;

impl MessageHandler<UserCreatedEventMessage> for UserCreatedHandler {
    fn handle(&self, message: Box<UserCreatedEventMessage>) -> Result<(), HandleError> {
        println!("Message received on handler 1: {:?}", message);
        Ok(())
    }

    fn get_handler_action(&self) -> String {
        return "UserCreatedHandler".to_owned();
    }
}

fn main() {
    println!("Starting one-time publisher application...");
    // Load environment variables
    dotenv().ok();

    // Get and print RabbitMQ URL
    let rabbitmq_url = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");

    // Create publisher
    println!("Creating RabbitMQ publisher...");
    let mut publisher = match CrosstownBus::new_queue_publisher(rabbitmq_url.to_owned()) {
        Ok(p) => {
            println!("Successfully created RabbitMQ publisher");
            p
        }
        Err(e) => {
            println!("Failed to create publisher: {:?}", e);
            std::process::exit(1);
        }
    };

    // List of messages to publish - expanded to 30 messages
    let mut messages = Vec::with_capacity(30);

    // Generate 30 user messages
    for i in 1..=30 {
        messages.push(UserCreatedEventMessage {
            user_id: i.to_string(),
            user_name: format!("129500004y-User{}", i),
        });
    }

    println!("Publishing 30 messages...");
    // Publish all messages
    let mut success_count = 0;
    let mut failure_count = 0;
    for (i, message) in messages.iter().enumerate() {
        println!(
            "Publishing message {}/{}: {:?}",
            i + 1,
            messages.len(),
            message
        );
        match publisher.publish_event("user_created".to_owned(), message.clone()) {
            Ok(_) => {
                println!(
                    "✓ Successfully published message for user {}",
                    message.user_name
                );
                success_count += 1;
            }
            Err(e) => {
                println!(
                    "✗ Failed to publish message for user {}: {:?}",
                    message.user_name, e
                );
                failure_count += 1;
            }
        }
        // Add a small delay between messages to avoid overwhelming the broker
        thread::sleep(time::Duration::from_millis(100));
    }

    // Print summary
    println!("\nPublishing completed:");
    println!("- Messages published successfully: {}", success_count);
    println!("- Messages failed: {}", failure_count);

    // Add a small delay to ensure messages are delivered before exiting
    println!("Waiting 1 second for messages to be fully delivered...");
    thread::sleep(time::Duration::from_secs(1));
    println!("Publisher application completed");
}
