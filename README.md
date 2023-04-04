# text-reminder
Sends a text using the twilio API at a specified time and interval.

Sign up for a trial account at [twilio.com](https://twilio.com) to get an API key and a phone number to send texts from.

Store the API credentials, the phone number from twilio and the recievers phone number, as well as the message to be sent in a ".env" file at the root of the project.
(see [dotenv](https://crates.io/crates/dotenv) documentation)



```bash
# Build and run: 
$ cargo build --release && ./target/release/text_reminder [OPTIONS]
```

```bash
Usage: text_reminder [OPTIONS]

Options:
      --hour <HOUR>          [default: 8]
      --minute <MINUTE>      [default: 30]
      --interval <INTERVAL>  [default: 12]
  -h, --help                 Print help
  -V, --version              Print version
  ```
