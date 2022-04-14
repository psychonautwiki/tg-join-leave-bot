use std::env;

use futures::StreamExt;
use pw_telegram_bot_fork::*;

fn print_member(
    op: &str,
    member: &User,
) {
    println!(
        "{} id {} lang {:?} is_bot {:?} user {:?} name {} {:?}",
        op,
        member.id,
        member.language_code,
        member.is_bot,
        member.username,
        member.first_name,
        member.last_name,
    );
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::NewChatMembers { ref data, .. } = message.kind {
                for member in data {
                    print_member("JOIN", &member);
                }

                api.send(
                    message.delete(),
                ).await;
            }

            if let MessageKind::LeftChatMember { ref data, .. } = message.kind {
                print_member("LEAVE", &data);

                api.send(
                    message.delete(),
                ).await;
            }
        }
    }
    Ok(())
}
