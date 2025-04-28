use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread, vec,
};

fn network_processing() {
    let mut _sock = TcpListener::bind("127.0.0.1:6789").unwrap();
    let mut handlers = Vec::new();

    match _sock.accept() {
        Ok((mut socket, addr)) => {
            println!("new client: {:?}", addr);

            let handler = thread::spawn(move || {
                // spawns a new thread to handle the client. "move" transfers ownership of socket
                // to the closure (threads require owned data)
                let mut buf = [0; 10]; // Creates a fixed-size array of 10 byte-size buffers to
                // store incoming data.
                let _ = socket.read(&mut buf);

                println!("read: {:?}", buf);
            });
            handlers.push(handler); // Stores the thread handle in 'handlers' vector to track the
            // thread
        }
        Err(e) => println!("couldn't get client: {:?}", e),
    }

    for handler in handlers {
        handler.join().unwrap();
    }
}

fn client_process() {
    let mut stream = TcpStream::connect("127.0.0.1:6789").unwrap();
    stream.write(&[1]).unwrap();
}

fn main() {
    let ps = {
        let p1 = thread::spawn(client_process);
        let p2 = thread::spawn(network_processing);
        vec![p1, p2]
    };

    for process in ps {
        let _ = process.join();
    }
}

// fn main() -> std::io::Result<()> {
// let _dict: Vec<(String, String)> =
//     vec![("someword".to_string(), "the description".to_string())];
//
// // for addr in _sock.incoming() {
// //     let conn_status = match addr {
// //         Ok(addr) => format!("Connected to {:?}", addr.peer_addr()?),
// //         Err(_e) => format!("Connection failed: {:?}", _e),
// //     };
// //
// // }
//
// // .accept() method returns tuple: (TcpStream, SocketAddr)
// // need TcpStream to send and receive data.
// // Below, this accepts a single connection
// // accept connections and process them serially
// for stream in _sock.incoming() {
//     handle_client(stream?);
// }
// // let (mut _stream, _addr) = match _sock.accept() {
// //     Ok((_stream, _addr)) => (_stream, _addr), // Capture both the stream and address
// //     Err(_e) => {
// //         println!("Connection failed: {:?}", _e);
// //         return Err(_e);
// //     }
// // };
// //
// println!("Connected to {:?}", _addr);
//
// _stream.write_all(b"Welcome!\n")?;
// _stream.flush()?;
//
// Ok(())
// }
//
// fn handle_client(_stream: TcpStream) -> _ {
//     loop {
//         let mut buffer = [0u8; 4096]; // A buffer to store incoming data (4096 bytes)
//         let bytes_read = _stream.read(&mut buffer)?;
//         let recv = String::from_utf8_lossy(&buffer[..bytes_read]);
//
//         if !&recv.starts_with("GET ") {
//             println!("Received: {}", recv);
//
//             let new_recv: &[u8] = recv.as_bytes();
//             let binding = String::from("I got your message:");
//             let reply = binding.as_bytes();
//             let rep_o = [reply, new_recv].join(" ".as_bytes());
//             _stream.write_all(&rep_o)?;
//             _stream.flush()?;
//         } else if let Some(description) = process_cmd(&recv, &_dict) {
//             match description.contains("ERROR undefined") {
//                 true => {
//                     let fmt_desc = description.as_bytes();
//                     _stream.write_all(fmt_desc)?;
//                     _stream.flush()?;
//                 }
//                 false => println!("ANSWER {}", description),
//             }
//             // println!("ANSWER {}", description);
//         }
//
//         if recv.starts_with("exit") {
//             break;
//         }
//     }
// }
//
// // Learned Info Notes
// // -------------------------------------------
// // format! creates a String by formatting text and agruments.
// // It returns a Strings (an owned string type) that I can store, manipulate, or use
// // later.
// // It doesn't output anything to the console; only produces a string value.
// // Useful when needed to generate a formatted string for further processing, such as
// // storing in a variable, passing to a function, or logging.
// // println! formats text and agruments; Prints the result to the console (standard
// // output), it writes directly to the console and returns (). It's for immediate
// // output, not for storing the formatted result.
// // --------------------------------------------
// // '?' operator is a shortcut in Rust that says, "If something goes wrong, just tell the
// // function about it and stop". It's used when you're doing something that might fail, like
// // opening a file or binding a socket, and you don't want to write a lot of error-handling
// // code.
// // -----------------------------------------------
//
// fn process_cmd<'a>(us_inp: &'a str, cmd: &'a [(String, String)]) -> Option<&'a str> {
//     let target_word = us_inp.trim_start_matches("GET ").trim();
//
//     let err = "ERROR undefined\n";
//     Some(
//         cmd.iter()
//             .find(|(word, _)| word == target_word)
//             .map(|(_, desc)| desc.as_str())
//             .unwrap_or(err),
//     )
// }
//
// // fun test_get_definition() {
// //     var client = connect("127.0.0.1", 5678);
// //     client.send("GET word\n");
// //     var line = client.recv(4096);
// //     assert(line == "ANSWER something interesting here\n");
// // }
//
// #[cfg(test)]
// mod test {
//     use super::process_cmd;
//
//     #[test]
//     fn test_process_cmd_valid_input() {
//         let commands = vec![("someword".to_string(), "the description".to_string())];
//         let input = "GET someword";
//         let result = process_cmd(input, &commands);
//         assert_eq!(result, Some("the description"));
//     }
//
//     #[test]
//     fn test_process_cmd_invalid_input() {
//         let commands = vec![("someword".to_string(), "the description".to_string())];
//         let input = "GET invalid";
//         let result = process_cmd(input, &commands);
//         assert_eq!(result, Some("ERROR undefined\n"));
//     }
//
//     #[test]
//     fn test_process_cmd_non_get_input() {
//         let commands = vec![("someword".to_string(), "the description".to_string())];
//         let input = "SET someword";
//         let result = process_cmd(input, &commands);
//         assert_eq!(result, Some("ERROR undefined\n"));
//     }
// }
