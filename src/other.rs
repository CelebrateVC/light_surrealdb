use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};

const SURREAL_EP: &str = "ws://localhost:8000/rpc";
const SURREAL_PING: &str = "{\"method\":\"ping\",\"params\":[],\"id\":\"0000000000000000\"}";
// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
    resp: std::sync::mpsc::Sender<Result<String>>,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {

    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        println!("opened");
        self.out.send(SURREAL_PING).unwrap();
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        self.resp.send(msg.into_text());
        Ok(())
    }
}

struct ClientInterface{
    sender: Sender,
    reciever: std::sync::mpsc::Receiver<Result<String>>,
    terminus: std::thread::JoinHandle<()>
}

impl ClientInterface{
    fn new() -> Self{
        let (send_ws_resp,rcv_ws_resp) = std::sync::mpsc::channel();
        let (x,y) = std::sync::mpsc::channel();
        
        
        let x = std::thread::spawn(move ||{
            connect(SURREAL_EP, |out|
                { 
                    x.send(out.clone()).unwrap();
                    Client { out: out.clone(), resp: send_ws_resp.clone()} }
                ).unwrap()
            });

        

        ClientInterface { sender: y.recv().unwrap(), reciever: rcv_ws_resp, terminus: x }

        }


}


pub fn main(){
    let interf = ClientInterface::new();

    let z = interf.reciever.recv().unwrap().unwrap();

    println!("main response {}",z);
    
    let send = interf.sender.send(SURREAL_PING).unwrap();

    println!("main response {}",interf.reciever.recv().unwrap().unwrap());


    interf.sender.close(CloseCode::Normal).unwrap();

    interf.terminus.join().unwrap();
}