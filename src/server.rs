pub mod server {
    use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Error}, collections::HashMap};
    use crate::queue::queue::{Queue, new_queue};

    /// Server handles connections and stores all messages in a Queue
    pub struct Server {
        port: i32,
        queues: HashMap<String, Queue>,
    }

    /// create a new server
    pub fn new_server() -> Server {

        let port = 8080 as i32;
        let queues: HashMap<String, Queue> = HashMap::new();
        let server = Server{port, queues};
        return server;
    }


    impl Server {
        /// start the server
        pub fn start(&mut self) {
            println!("Starting message-queue server...\n");

            // handle tcp connections
            let conn_listener = TcpListener::bind(format!("0.0.0.0:{}", self.port)).unwrap();
            for stream in conn_listener.incoming() {
                let stream = stream.unwrap();
                self.handle_connection(stream);
            }
        }

        /// handle a single connection
        fn handle_connection(&mut self, mut stream: TcpStream) {
            // read lines of request header
            let reader = BufReader::new(&mut stream);
            let (header, body) = parse_request(reader);

            // parse first line of header to get METHOD and PATH
            let first_line = &header[0];
            let first_line_parsed = first_line.split(" ").collect::<Vec<&str>>();
            let method = first_line_parsed[0];
            let path = first_line_parsed[1];
            

            // handle POST /new/:queuename
            if method == "POST" && path.starts_with("/new/") {
                let queue_name = path.replace("/new/", "");
                
                // queue already exists
                if self.queues.contains_key(&queue_name) {
                    send_bad_request(&mut stream).unwrap();
                    send_body(&mut stream, format!("Queue '{}' already exists", queue_name)).unwrap();
                    return;
                }

                println!("creating new queue called: {}", queue_name.clone());

                self.queues.insert(queue_name, new_queue()); 

                send_ok(&mut stream).unwrap();
                return;
            }

            // handle DELETE /delete/:queuename
            else if method == "DELETE" && path.starts_with("/delete/") {
                let queue_name = path.replace("/delete/", "");

                let result = self.queues.remove(&queue_name);
                if result.is_some() { // ok
                    send_ok(&mut stream).unwrap();
                    return;
                }

                // could not remove queue (does not exist)
                send_bad_request(&mut stream).unwrap();
                send_body(&mut stream, format!("Queue '{}' cannot be removed as it does not exist", queue_name)).unwrap();
                return;
            }

            // handle GET /get/:queuename
            else if method == "GET" && path.starts_with("/get/") {
                let queue_name = path.replace("/get/", "");

                // queue does not exist
                if self.queues.contains_key(&queue_name) == false {
                    send_not_found(&mut stream).unwrap();
                    return;
                }
                
                let message = self.queues.get_mut(&queue_name).unwrap().retrieve_message();
                println!("GET request to queue: {}, retrieving message '{}'", queue_name, message);

                send_ok(&mut stream).unwrap();
                send_body(&mut stream, message.to_string()).unwrap();
                return;
            }

            // handle POST /add/:queuename
            else if method == "POST" && path.starts_with("/add/") {
                let queue_name = path.replace("/add/", "");
                
                // queue does not exist
                if self.queues.contains_key(&queue_name) == false {
                    send_not_found(&mut stream).unwrap();
                    return;
                }

                let message = body.join("\n");
                self.queues.get_mut(&queue_name).unwrap().add_message(message);

                println!("POST request to queue: {}, body: {:?}", queue_name, body);
                
                send_ok(&mut stream).unwrap();
                return;
            }

            // invalid method
            println!("{} request to {} (invalid)", method, path);
            send_not_found(&mut stream).unwrap();


        }
    }

    /// read from buffered reader and return (header, body)
    fn parse_request(mut reader: BufReader<&mut TcpStream>) -> (Vec<String>, Vec<String>) {

        let data = reader.fill_buf().unwrap().to_vec();
        reader.consume(data.len());
        let data_string = String::from_utf8(data).unwrap();

        let mut header: Vec<String> = Vec::new();
        let mut body: Vec<String> = Vec::new();

        let mut is_header = true;
        for line in data_string.split("\n") {

            if line == "\r" {
                if is_header {
                    is_header = false;
                    continue;
                }
            }

            if is_header {
                header.push(line.to_string());
            } else {
                body.push(line.to_string());
            }
        }
        return (header, body);
    }

    /// send 200 using stream
    fn send_ok(stream: &mut TcpStream) -> Result<(), Error> {
        let http_response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write_all(http_response.as_bytes())?;
        Ok(())
    }

    /// send 400 using stream
    fn send_bad_request(stream: &mut TcpStream) -> Result<(), Error> {
        let http_response = "HTTP/1.1 400 Bad Request\r\n\r\n";
        stream.write_all(http_response.as_bytes())?;
        Ok(())
    }

    /// send 404 using stream
    fn send_not_found(stream: &mut TcpStream) -> Result<(), Error> {
        let http_response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write_all(http_response.as_bytes())?;
        Ok(())
    }

    /// send body (string) using stream
    fn send_body(stream: &mut TcpStream, body: String) -> Result<(), Error> {
        stream.write_all(body.as_bytes())?;
        Ok(())
    }

}