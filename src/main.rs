use chrono::{Duration, Local};
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)] // Added Clone trait
struct Reminder {
    medication: String,
    interval: Duration,
    next_alert: chrono::DateTime<chrono::Local>,
}

fn add_reminder(reminders: &mut HashMap<u32, Reminder>, next_id: &mut u32) {
    print!("Enter the medication and reminder interval (e.g., 'aspirin_medication every 30 minutes'): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    // Split the input into words and parse
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 4 || parts[1].to_lowercase() != "every" || parts[3].to_lowercase() != "minutes" {
        println!("Invalid input format. Please follow 'medication every N minutes'.");
        return;
    }

    let medication = parts[0].to_string();
    let interval_minutes: i64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number of minutes. Please enter a valid number.");
            return;
        }
    };

    let interval = Duration::minutes(interval_minutes);
    let next_alert = Local::now() + interval;

    let reminder = Reminder {
        medication,
        interval,
        next_alert,
    };

    reminders.insert(*next_id, reminder);
    println!("Reminder added with ID: {}", next_id);
    *next_id += 1;
}

fn list_reminders(reminders: &HashMap<u32, Reminder>) {
    if reminders.is_empty() {
        println!("No reminders set.");
        return;
    }

    println!("\nCurrent Reminders:");
    for (id, reminder) in reminders {
        println!(
            "ID: {} | Medication: {} | Interval: {} mins | Next Alert: {}",
            id,
            reminder.medication,
            reminder.interval.num_minutes(),
            reminder.next_alert.format("%Y-%m-%d %H:%M:%S")
        );
    }
}

fn remove_reminder(reminders: &mut HashMap<u32, Reminder>) {
    print!("Enter the ID of the reminder to remove: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    match input.parse::<u32>() {
        Ok(id) => {
            if reminders.remove(&id).is_some() {
                println!("Reminder with ID {} has been removed.", id);
            } else {
                println!("No reminder found with ID {}", id);
            }
        }
        Err(_) => println!("Invalid ID input. Please enter a valid number."),
    }
}

fn main() {
    let mut reminders: HashMap<u32, Reminder> = HashMap::new();
    let mut next_id = 1;

    loop {
        println!("\nChoose an option:");
        println!("1. Add Reminder");
        println!("2. List Reminders");
        println!("3. Remove Reminder");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => add_reminder(&mut reminders, &mut next_id),
            "2" => list_reminders(&reminders),
            "3" => remove_reminder(&mut reminders),
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}