fn main() {
    println!("Hello, world!");
}
//Usamos varios tipos de compiladores por terminal
//El primero para crear esta carpeta principal "CARGO"
//Se escribe el sigiente comoando -> cargo new hello_cargo
//Despues ingresar cargo build para ejecutar el main.rs 
//que se encuentra en la subcarpeta src-ojo esto no ejecuta el programa
//solo construye el ejecutable :D
//Luego vamos a ejecutar ese ejecutable que encuentra en la nueva subcarpeta target
//Ahora ejecutamos el hello_cargo.exe creado
//Con este comando -> .\target\debug\hello_cargo.exe
//AHORA, si ya tenemos el todo hasta el target, no es necesario
//realizar esos dos pasos anteriores, puedes ejecutar el programa
//defrente sin la necesidad de estar compilandolo otra vez
//esto es posible con el comando -> cargo run, ahora ya no veras 
//el mensaje de compiling, sino que solo el de finished y running
//Si se realiza algún cambio al programa, y decides poner Run defrente
//Esta vez si va a aparecer compiling para así tener el progrmaa con sus ultimas modificaciones
//entonces lo ejecuta :D -> BE HAPPY 
//Por ultimo, con el comando -> cargo check ,puedes comprobar
//rapidamente si el codigo puede compilar, este comando no produce 
//ningun ejecutable 

