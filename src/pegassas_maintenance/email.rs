use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

//local Email server needs to be installed
pub struct Email {
    from: String, 
    reply_to: String, 
    recipent: String, 
    subject: String, 
    body: String
}

let sender : String
impl Email {
    pub fn new(from: String, reply_to: String, recipent: String, subject: String, body: String) -> Email {
        from, 
        reply_to, 
        recipent, 
        subject, 
        body
    }

    pub fun buildEmail{
        let email = Message::builder()
            .from(from.parse().unwrap())
            .reply_to(reply_to.parse().unwrap())
            .to(recipent.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();
    }
    //Cred held locally within maintenance session.
    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    pub fn sendEmail(&Email, creds) {
        // Open a remote connection to email server
        let mailer = SmtpTransport::relay("") //local server Address
            .unwrap()
            .credentials(creds)
            .build();
        match mailer.send(&email) {
            Ok(_) => {println!("Email sent successfully!"); 
            lettre::file::FileTransport new ("")},// local area to write email logs
            Err(e) => panic!("Could not send email: {:?}", e),
        }
    }
}
