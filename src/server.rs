pub mod server {
    use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write, Error}};
    use crate::queue::queue::{Queue, new_queue};

    /// Server handles connections and stores all messages in a Queue
    pub struct Server {
        port: i32,
        queue: Queue
    }

    /// create a new server
    pub fn new_server() -> Server {

        let port = 8080 as i32;
        let queue = new_queue();
        let server = Server{port, queue};
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


            // handle GET /get/message
            if method == "GET" && path == "/get/message" {

                let message = self.queue.retrieve_message();
                println!("GET request to {}, retrieving message '{}'", path, message);

                send_ok(&mut stream).unwrap();
                send_body(&mut stream, message).unwrap();
                return;
            }

            // handle POST /add/message
            else if method == "POST" && path == "/add/message" {

                let message = body.join("\n");
                self.queue.add_message(message);

                println!("POST request to {}, body: {:?}", path, body);
                
                send_ok(&mut stream).unwrap();
                return;
            }

            // invalid method
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