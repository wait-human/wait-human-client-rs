use wait_human::WaitHuman;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wait_human =
        WaitHuman::new_from_key("sk_3a3b8e8e4bdfd121288cb995d41361042337ef6cada0e48d")?;

    // Example: Multiple Choice Question
    let answer = wait_human
        .ask_multiple_choice(
            "Send invoice?",
            ["yes, send", "no"],
            Some("Customer asked for a 3-page website. is 500$ ok?"),
            None,
        )
        .await?;

    if answer == "yes, send" {
        println!("Send!");
    } else {
        println!("wait...");
    }

    // Example: Free Text
    let feedback = wait_human
        .ask_free_text(
            "User Feedback",
            Some("Please explain why you rejected the invoice."),
            None,
        )
        .await?;

    println!("{}", feedback);

    Ok(())
}
