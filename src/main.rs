/*
## author Yancy 
## date 2021-10-26
*/

// import std io for dealing with error
use std::io::{self, Read, Write};
// import std net for network
use std::net::{TcpListener, TcpStream};
// import std thread for thread processing
use std::thread;
// import std time for time
use std::time;

//Create a method to handle the client
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    //Define an array for storage
    let mut buf = [0; 512];
    //Create a loop to read the input information
    loop {
        //Use the read method
        let bytes_read = stream.read(&mut buf)?;
        //End if the inside is empty
        if bytes_read == 0 {
            //Exit
            return Ok(());
        }
        //What is input and what is returned
        stream.write(&buf[..bytes_read])?;
        //Intermittent process
        thread::sleep(time::Duration::from_secs(1));
    }
}

//Program entry function
fn main() -> io::Result<()> {
    //Create a Tcp listener
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //Create a running process
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //Call the incoming() method to receive the client's link information
    for stream in listener.incoming() {
        //Pattern matching
        match stream {
            //If it matches ok
            Ok(stream) => {
                //Start a new thread
                let handle = thread::spawn(move || {
                    //Decouple the client processing information to the handle_client function
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}",error));
                });
                //Handover process
                thread_vec.push(handle);
            }
            //When the Result enumeration matches incorrectly
            Err(e) => {
                //Output error, and terminate the program
                panic!("Err {:?}", e) 
            }
        }
    }
    //Wait for the thread to end
    for handle in thread_vec {
        //Wait end
        handle.join().unwrap();
    }
    //End
    Ok(())
}