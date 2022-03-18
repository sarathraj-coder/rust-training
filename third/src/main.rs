enum Direction{
    Up,
    Down,
    Left,
    Right
}
const MA:i32 =97687;


fn main() {
   
    let player_direction:Direction = Direction::Up;

    match player_direction {
        Direction::Up => println!("Heading Up"),
        Direction::Down => println!("Heading Down"),
        Direction::Left => println!("Heading Left"),
        Direction::Right => println!("Heading Right"),
    }

println!("Heading Right{}",MA);

let m =(20,33,88,55,"sarath",67.98,true,(56,78));
println!("tuple {}",m.2);
println!("tuple {}",m.4);
println!("tuple {}",(m.7).1);

let (a,b) =m.7;

println!("tuple assigned {}",a);


// Code block 
let mut x =1;
{
    //isolated 
   let y =2;
   let x = 10; // shadowing
   println!(" x: {} , y: {}",x,y);
}
// yhas no scope 
//println!(" x: {} , y: {}",x,y);
println!(" x: {} ",x);

let x ="x is a string ";
println!(" x: {} ",x);

let x=true;
println!(" x: {} ",x);



//refrences in rust 

let mut x =10 ;
let xref = &x;   //refernece
println!(" x: {} ",xref);  //10
println!(" x: {} ",x);    //10

// let dom = &mut x;  // mutable reference
// println!(" x: {} ",dom); 
// *dom =*dom+1;

{
    let dom = &mut x;  //limiting  mutable reference inside the code
    *dom =*dom+1;
    println!(" x: {} ",dom); 
    println!(" x: {} ",*dom); 
}

//mutable refence https://doc.rust-lang.org/1.12.0/book/references-and-borrowing.html







}
