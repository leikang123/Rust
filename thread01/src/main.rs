use std::{thread,time::Duration};
fn main() {
    thread::spawn(||{
        for i in 1 ..10 {
            println!("hi number {} from the spawed thread ",i);
            //thread::sleep(dur:Duration::from_millis(1));
        }
    });
    for i in 1 ..5{
        println!("hi number {} from the main thread!",i);
        //thread::sleep(dur:Duration::from_millis(1));

    }
   
}
