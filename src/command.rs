use std::process::{ Command, Stdio};
use std::str;
fn test_command() {
   get_desktop()
}

pub fn get_desktop() {
   let c1 = Command::new( "bspc" )
      .arg( "query" )
      .arg( "-T" )
      .arg( "-d" )
      .arg( "focused" )
      .stdout( Stdio::piped() )
      .spawn()
      .unwrap();

   let c2 = Command::new("jshon")
      .arg( "-e" )
      .arg( "name" )
      .arg( "-u" )
      .stdin( Stdio::from( c1.stdout.unwrap() ) )
      .stdout( Stdio::piped() )
      .spawn()
      .unwrap();

   let output = c2.wait_with_output().unwrap();
   let result = str::from_utf8( &output.stdout ).unwrap();
   println!("{}", result);
}
