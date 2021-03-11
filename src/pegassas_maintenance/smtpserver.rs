use mailin_embedded::{Server, SslConfig, Handler};



impl ServerStart{
    #[derive(Clone)]
    struct MyHandler {}
    impl Handler for MyHandler{}

    let handler = MyHandler {};
    let mut server = Server::new(handler);

    server.with_name("pegassas_email.com")
    .with_ssl(SslConfig::None)?
    .with_addr("127.0.0.1:25")?;
    let serverState = match server.serve(){
        Some(Ok(r)) => println!("Server has started correctly"),
        Err (_) => 
    }  
}

impl Handler for MyHandler {
        match Client::connect("host=localhost", postgres::NoTls){
            Ok(client) => {
                let email_tuple = (email_check : String, not_rule : String );
                let email_details: Vec<email_tuple> = Vec::new(email_tuple)
                };
                match email_details.db_client.query("SELECT  email, destination FROM user_account, notify_user WHERE user_account.userID = notify_user.userID", &[]){
                    Ok(rows) =>{
                        for row in rows {
                            let emailaddress : String = row.get(0);
                            let destination : String = row.get(1);
                            email_details.push((emailaddress, destination));
                        }
                    },
                    Err(e) => return Err(e)
                }

                Ok(cell_id_map)
            },
            Err(e) => Err(e)
        }
    }
    fn helo(&mut self, ip: IpAddr, domain: &str) -> Response {
       if domain == "pegassass_email.com" {
           OK
       } else {
           BAD_HELLO
       }
    }
    fn rcpt(&mut self, to: &str) -> Response {
       for i to email_details
        {
            if  to == email_details[1].email_tuple.0
                {
               OK
                } 
            else 
                {
                NO_MAILBOX
                }
        }
    }
}