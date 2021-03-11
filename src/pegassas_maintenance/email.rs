use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

//local Email server needs to be installed

let sender : String
match sender == String {
    
}
let email = Message::builder()
    .from("NoBody <nobody@domain.tld>".parse().unwrap())
    .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
    .to("Hei <hei@domain.tld>".parse().unwrap())
    .subject("Happy new year")
    .body("Be happy!")
    .unwrap();

let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

// Open a remote connection to gmail
let mailer = SmtpTransport::relay("smtp.gmail.com")
    .unwrap()
    .credentials(creds)
    .build();

// Send the email
match mailer.send(&email) {
    Ok(_) => println!("Email sent successfully!"),
    Err(e) => panic!("Could not send email: {:?}", e),
}