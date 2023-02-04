use twilio::{OutboundMessage, Client};
pub async fn msg_handler() {
    
    // get credentials
    let ssid = std::env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID must be set");
    let auth = std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN must be set");//.clone();
    let from = std::env::var("SENDER").expect("SENDER must be set");//.clone();
    let to = std::env::var("RECIEVER").expect("RECIEVER must be set");//.clone();
                                                                      
    // set message to be sent
    // Could implement a link a website to log exact time for confirmation
    let msg = format!("{}", std::env::var("MESSAGE").expect("MESSAGE must be set"));

    // client should perhaps be initialized
    let client = twilio::Client::new(&ssid, &auth);
    match send_text(&client, &from, &to, &msg).await {
        Ok(msg) => println!("{:#?}", &msg),
        Err(err) => eprintln!("{:?}", err),
    }
}


async fn send_text(client: &Client, from: &str, to: &str,  msg: &str) -> Result<twilio::Message, twilio::TwilioError>{
    client.send_message(OutboundMessage::new(from, to, msg)).await
}
