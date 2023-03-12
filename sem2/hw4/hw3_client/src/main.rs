use reqwest::{RequestBuilder, StatusCode};
use rustyline::{error::ReadlineError, DefaultEditor};

static SERVER_ADDRESS: &str = "http://127.0.0.1:8080";
static HELP: &str = "Usage:

Registration:
>> register <username> <preferred currency>

Adding new item:
>> add <name> <price>

Viewing catalogue:
>> view <username>

Showing this help message:
>> help
";

fn print_help() {
    println!("{}", HELP);
}

async fn send_and_get_text_response(req: RequestBuilder) -> Result<String, String> {
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let text = resp.text().await.map_err(|e| e.to_string())?;
    if status == StatusCode::OK {
        Ok(text)
    } else {
        Err(format!("Error, status code: {}\n{}", status, text))
    }
}

async fn handle_line_safe(client: &reqwest::Client, line: &str) -> Result<(), String> {
    if line == "help" {
        print_help();
        return Ok(());
    }
    let args: Vec<&str> = line.split_whitespace().collect();
    match *args.get(0).ok_or("expected arguments")? {
        "register" => {
            let (username, currency) = if args.len() == 3 {
                Ok((args[1], args[2]))
            } else {
                Err("Expected three arguments")
            }?;
            let resp = send_and_get_text_response(client.post(format!(
                "{}/register/{}/{}",
                SERVER_ADDRESS, username, currency
            )))
            .await?;
            println!("{}", resp);
        }
        "add" => {
            let (name, price) = if args.len() == 3 {
                Ok((
                    args[1],
                    args[2]
                        .parse::<f64>()
                        .map_err(|_| "price must be a number")?,
                ))
            } else {
                Err("Expected three arguments")
            }?;

            let resp = send_and_get_text_response(
                client.post(format!("{}/add_item/{}/{}", SERVER_ADDRESS, name, price)),
            )
            .await?;
            println!("{}", resp);
        }
        "view" => {
            let username = args.get(1).ok_or("Expected two arguments")?;
            let resp = send_and_get_text_response(
                client.get(format!("{}/view_catalogue/{}", SERVER_ADDRESS, username)),
            )
            .await?;
            println!("{}", resp);
        }
        _ => return Err("Unrecognized command".to_string()),
    }

    Ok(())
}

async fn handle_line(client: &reqwest::Client, line: String) {
    match handle_line_safe(client, line.as_str()).await {
        Result::Ok(()) => {}
        Result::Err(e) => {
            println!("Error: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> rustyline::Result<()> {
    println!("Hello!");
    print_help();
    let mut rl = DefaultEditor::new()?;
    let client = reqwest::Client::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                handle_line(&client, line).await;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Good bye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
