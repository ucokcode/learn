extern crate async_std;
use async_std::task;

#[async_std::main]
async fn main()  {
   for i in 0..10 {
   task::spawn(async move {
         println!("task {}", i);
    });
   }
         println!("task ");
}

