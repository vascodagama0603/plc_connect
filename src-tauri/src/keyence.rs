use std::net::{ToSocketAddrs,TcpStream};
use std::io::{BufRead, Write, BufReader, BufWriter};

  pub fn connect(host:&str, port:&str, address:&str) -> Result<String,String>{
    let host_and_port = format!("{}:{}", host, port);
    let mut addrs = host_and_port.to_socket_addrs().unwrap();
    let mut msg = String::new();
    if let Some(addr) = addrs.find(|x| (*x).is_ipv4()) {
      match TcpStream::connect(addr) {
        Err(_) => {
          println!("Connection NG.");
        }
        Ok(stream) => {
          println!("Connection Ok.");
          let mut _writer = BufWriter::new(&stream);
          _writer.write(address.as_bytes()).expect("SEND FAILURE!!!");
          _writer.flush().unwrap();         // TODO
          let mut _reader = BufReader::new(&stream);

          
          _reader.read_line(&mut msg).expect("RECEIVE FAILURE!!!");
          println!("First line is {msg} bytes long");

        }
      }
      
    } 
    else {
      eprintln!("Invalid Host:Port Number");
    }
    Ok(msg)
  }
  pub fn read(host:&str, port:&str, addresssuffix:&str) -> Result<String,String>{
    let address = format!("RD {}.D\r",addresssuffix);
    let msg = connect(host, port, &address);
    msg
  }
  pub fn write(host:&str, port:&str, addresssuffix:&str, data:&str) -> Result<String,String>{
    let address = format!("WR {}.D {}\r",addresssuffix,data);
    println!("address:{}",&address);
    let msg = connect(host, port, &address);
    msg
  }

  


//fn reads(self, addresssuffix:String, num:String){
//  rcv = self.sendrecive(("RDS " + addresssuffix + " " + str(num) + "\r").encode());
//  return rcv
//  }

//fn write(self, addresssuffix:String){
//  rcv = self.sendrecive(("WR " + addresssuffix + " " + data + "\r").encode());
//  return rcv
//}

//fn writes(self, addresssuffix:String, num:String){
//  rcv = self.sendrecive(("WRS " + addresssuffix + " " + str(num) + " " +  data + "\r").encode());
//  return rcv
//}