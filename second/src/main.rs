fn main() {
    println!("Hello, decode!");


    //Variables 

    let x = 45 ;  // type i32 
    let y =44;
    let k = x+y;
    //x=67;  //error immutable 
    let mut z   =44; // mutable variable
    let  _pa = z+1;
    z=77;
    println!("value of k {}",k);
    println!("value of z {}",z);


    //Data type 
    // https://doc.rust-lang.org/std/index.html
    let _p:i32 = 45 ;
    let _p1:i64 =123123213;
    let _p11:u32 = 1111;
    let _f:f32 =111.11;   //f64
    let _b:bool =true; //


    // If statments in Rust programming  

      if _p>30 {
          println!("ok")
      }else{
        println!(" not ok")
      }


    //   if _p >= 30 && _p =< 65 {
    //     println!("ok1")
    // }
    
    // loops  keyword
    let mut n = 0;
    loop {
        n += 1;
        println!("{}",n);
        if n==7{
            continue;
        } 

        if n==10{
            break;
        } 
        println!("Hola");
    }


        // while  loop 
      n=0;
     while n<=50{
        println!("Hola {}",n);
        n+=1;
     }
     // for loop use 
     for i in 1..11 {   //non inclusive upper bound 
        println!("Hola {}",i);
     }

     // constatnts

     const USER_LIMIT:i32 = 100;   
     const IX:i32=000;
     //IX=11;

     // Strings 

     let str1:&str ="Hello sarath";
     println!("{}",str1);
     // String literals are static by default
     let company:&'static str = "TutorialsPoint";

     //This is from the standrad library
     let empty_string = String::new();
     println!("length is {}",empty_string.len());

     let content_string = String::from("TutorialsPoint");
     println!("length is {}",content_string.len());


     let name1 = "Hello TutorialsPoint , Hello!".to_string();         //String object
     let name2 = name1.replace("Hello","Howdy");    //find and replace
     println!("{}",name2);


     let example_string = String::from("example_string");
     print_literal(example_string.as_str());
     print_literal(&example_string);


     //functions
     let mut kkk:i32=11;
     add(12233,&mut kkk); // passing number and reference
     println!("{}",kkk);
     println!("{}",addA(1,44));

}

fn print_literal(data:&str){
    println!("xyz {}",data)
}

fn add(a:i32,b:&mut i32){ // receiving address
    *b = a  // updating address value
}

fn addA(a:i32,b:i32) -> i32{
    a+b
}




// Single line comment 
/* This is multi line command 
 ok great */


