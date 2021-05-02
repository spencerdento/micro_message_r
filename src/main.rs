//Spencer Denton

//UNDEFINED BEHAVIOR if LaunchPad Unplugged while UART0 is being listend to

use uart0::{uart0_init_port, uart0_listen};

mod uart0;
fn main() {
    let mut command = String::new();
    let mut com_5 = uart0_init_port().expect("Couldn't Open COM5");

    //MAKE A DEFAULT COMMAND FOR ERRORS, SO IT CAN REPEAT TASK

    loop {
        match uart0_listen(&mut com_5) {
            Ok(message) => {
                command = message;
                println!("{}", command);
            },
            Err(error) => {
                println!("{}", error);
                break;
            },
        };

        //handle command
    }


}
