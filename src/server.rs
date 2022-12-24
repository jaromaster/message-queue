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
        let mut queue = new_queue();

        // TESTING test-data
        queue.add_message("this is a test message".to_string());
        queue.add_message("{\"message\": \"this is a json message\"}".to_string());
        queue.add_message("some other message".to_string());


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
            let header = parse_header(reader);

            // TODO read lines of request body

            // parse first line to get METHOD and PATH
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

            // TODO handle POST /add/message
            else if method == "POST" && path == "/add/message" {

                println!("POST request to {}", path);
                send_ok(&mut stream).unwrap();
                return;
            }

            // invalid method
            send_not_found(&mut stream).unwrap();


        }
    }

    /// read from buffered reader and return header as Vec
    fn parse_header(reader: BufReader<&mut TcpStream>) -> Vec<String> {
        let request_header: Vec<_> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty()) // break at empty line (omits request body)
            .collect();

        return request_header;
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