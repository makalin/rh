#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpStream;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_server_response() {
        // Start the server in a separate thread
        thread::spawn(|| {
            super::main();
        });

        // Give the server time to start
        thread::sleep(Duration::from_secs(1));

        // Connect to the server
        let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
        
        // Send a simple GET request
        let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
        stream.write(request.as_bytes()).unwrap();
        
        // Read the response
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).unwrap();
        let response = String::from_utf8_lossy(&buffer[..n]);
        
        // Verify the response
        assert!(response.contains("HTTP/1.1 200 OK"));
        assert!(response.contains("Hello from RH!"));
    }
} 