# CS128H Group 4 FinalProject

### Group Name: Group 4
### Group Members: Sid Bhushan (NetID: bhushan7)

### Project Introduction
This project will be a simple chat client that will allow 2 users to connect over a network connection and send messages to each other.

The objective of this project is to create a terminal application that will allow users to do the following tasks:
* Connect to another user
* Send messages to this user
* Receive messages from this user
* Cleanly terminate the connection

Some other goals that I would like to implement if I have time are:
* Have the chat be encrypted
* Be able to send data other than text using the application

I am choosing this project because network communication is interesting to me, and I've enjoyed working on similar projects in the past. I would also like to gain experience with network communication in Rust, as this was not covered in depth in the lectures.

### System Overview

The chat client will use HTTP to send messages over a network communication. The main components I am envisioning for the application are:
* A system to read and parse the messages typed into the terminal by the user
  * This would be able to interpret whether the text typed in is some sort of command (e.g. "connect to <user>" or "end session")
  * It would then call the relevant code to handle whatever type of input this is
* A simple HTTP server
  * This would listen on a port for an incoming HTTP messages, and deliver acknowledgement responses
  * These HTTP messages would contain chat messages from the other user
  * This should then handle the messages appropriately (i.e. by printing them to the user for text messages)
* A simple HTTP client 
    * This would send HTTP messages on a port and wait for an acknowledgement response
    * These HTTP messages would contain chat messages to the other user
    * This code would be called on by the system to handle user input
 
For HTTP interactions, I intend to use some library/crate to assist me, instead of directly implementing an HTTP server/client from scratch. 
  
The other potential features that I would like to implement if I have time would be implemeneted as follows:
* Chat encryption
  * I would ideally like to implement a simple end-to-end encryption algorithm, at least for text-based messages
* Sending non-text data
  * I would ideally like to implement a way to send files other than text using this application
  * This would likely consist of being able to send files that would then be saved by the other user
  
### Possible Challenges

Some possible challenges I anticipate in the creation of this project are:
* I will have to select a library to use for HTTP interactions and learn how to use it, which may be more difficult than I expect.
* This application will likely have several threads running concurrently to complete different tasks, and these threads may need to use the same resources at the same time (e.g. needing to both read and write input from the terminal if two users are conversing at the same time)
* If I have time to implement it, sending large files over HTTP may pose issues; I may have to reevaluate how to send these
* If I have time, implementing end-to-end encyrption would require me to learn more about the mathematical algorithms used
  
### References
No sources other than the list of sample project ideas in the document provided by the course were used in the creation of the idea of this project.
