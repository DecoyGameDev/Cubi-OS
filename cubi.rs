use std::io;

fn Longterm(){
    let mut username = String::new();
    let mut password = String::new();
}

fn KernalStart() -> io::Result<()> {
    let mut kernalbaseinput = String::new();
    println!("Please enter your username.");
    io:stdin().read_line(&mut kernalbaseinput)?;
    if (&mut username == &mut kernalbaseinput) {
        boot();
    }
    Ok(())
}

fn boot(){
    println!("Staring User Input Stream...");
    let mut MainInputStream = String::new();
    println!("OK");
    println!("Loading NAV Stream");
    let mut NavStream = String::new();
    println!("Booted! As per usual.");
    let Markov = 1;
    println!("Loading memory/independent varibles");
    let Programs = 0;
    let Windows = 1;
    let Ascii = 1;
    let Uni = 0;
    println!("- D O N E -");
    UserInteract();
}

fn UserInteract(){
    // allow the user to interact with the system
    println!("Please enter your command.");
    let mut UserInput = String::new();
    io::stdin().read_line(&mut UserInput)?;
    println!("{}", UserInput);
    println!("Satisified.");
    Shell();
}

fn Shell() -> io::Result<()>{
    let mut shelltKinput = String::new();

    io::stdin().read_line(&mut input)?;

    if (input == "help"){
        println!("List of commands: ");
        println!("makeDir");
        println!("removeDir");
        println!("text");
        println!("program");
    }

    if (input == "makeDir"){
        println!("Input directory name.");
        let mut dirname = String::new();
        io::stdin().read_line(&mut dirname)?;
        fn makeDir(dirname: String){
            println!("Making directory: {}", dirname);
            let mut dircount = 0;
            dircount = dircount + 1;
            println!("Dircount: {}", dircount);
        }
    }

    if (input == "removeDir"){
        println!("Input directory name.");
        let mut dirname = String::new();
        io::stdin().read_line(&mut dirname)?;
        fn removeDir(dirname: String){
            println!("Removing directory: {}", dirname);
            let mut dircount = 0;
            dircount = dircount - 1;
            println!("Dircount: {}", dircount);
        }
    }

    if (input == "text"){
        println!("Welcome to the text editor!");
        let mut textname = String::new();
        io::stdin().read_line(&mut textname)?;
        fn text(textname: String){
            println!("Creating text: {}", textname);
            let mut textcount = 0;
            textcount = textcount + 1;
            println!("Textcount: {}", textcount);
        }
        // give user option to save or not
        println!("Would you like to save this file?");
        let mut save = String::new();
        io::stdin().read_line(&mut save)?;
        if (save == "yes"){
            println!("Saving text...");
        }
        if (save == "no"){
            println!("Not saving text...");
        }
    }

    if (input == "program"){
        println!("Welcome to the program editor!");
        let mut programname = String::new();
        io::stdin().read_line(&mut programname)?;
        fn program(programname: String){
            println!("Creating program: {}", programname);
            let mut programcount = 0;
            programcount = programcount + 1;
            println!("Programcount: {}", programcount);
        }
        // give user option to save or not
        println!("Would you like to save this file?");
        let mut save = String::new();
        io::stdin().read_line(&mut save)?;
        if (save == "yes"){
            println!("Saving program...");
        }
        if (save == "no"){
            println!("Not saving program...");
        }
    }

    if (input == else){
        println!("Sorry. That's not a shell command.");
    }
}