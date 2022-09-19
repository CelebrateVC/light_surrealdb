use ws::{connect, Handler, Sender, Handshake, Result as wsResult, Message, CloseCode};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    resp: std::sync::mpsc::Sender<wsResult<String>>,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {

    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> wsResult<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        println!("ws opened");
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> wsResult<()> {
        // Close the connection when we get a response from the server
        self.resp.send(msg.into_text()).unwrap();
        Ok(())
    }
}

impl Client{
    fn new( resp: std::sync::mpsc::Sender<wsResult<String>> )-> Self{
        Client { resp }
    }
}

pub struct ClientInterface{
    sender: Sender,
    reciever: std::sync::mpsc::Receiver<wsResult<String>>,
    terminus: std::thread::JoinHandle<()>
}

impl ClientInterface{
    pub fn new(url: String) -> Self{
        let (send_ws_resp,reciever) = std::sync::mpsc::channel();
        let (x,y) = std::sync::mpsc::channel();

        
        
        let terminus = std::thread::spawn(move ||{
            connect(url, |sender: Sender|
                { 
                    x.send(sender.clone()).unwrap();
                    Client::new(send_ws_resp.clone())
                }).unwrap()
            });

        let sender = y.recv().unwrap();
        

        ClientInterface { sender, reciever, terminus }

        }
    
    pub fn close(self) -> wsResult<()>{
        let x= self.sender.close(CloseCode::Normal);
        self.terminus.join().unwrap();
        x
    }

    pub fn send(&self, query:String) -> wsResult<()>{
        self.sender.send(query)
    }

    pub fn recv(&self) -> wsResult<String>{
        self.reciever.recv().unwrap()
    }


}