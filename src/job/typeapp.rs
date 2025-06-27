use std::io::{self, Write,Read};
use std::str::FromStr;

#[derive(Debug,Clone)]
pub enum TypeApp {
    System,
    Web,
    Network,
    DataProcess,
    Service,
}
impl TypeApp {
    fn variants() -> &'static [TypeApp] {
        &[
            TypeApp::System,
            TypeApp::Web,
            TypeApp::Network,
            TypeApp::DataProcess,
            TypeApp::Service,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            TypeApp::System      => "System",
            TypeApp::Web         => "Web",
            TypeApp::Network     => "Network",
            TypeApp::DataProcess => "Data Process",
            TypeApp::Service     => "Service",
        }
    }
}
impl FromStr for TypeApp {
    type Err = ();
    fn from_str(s:&str) -> Result<Self,Self::Err> {
        let index :usize = s.trim().parse().map_err(|_| ())?;
        let var = TypeApp::variants();
        if index == 0 || index > var.len() {
            Err(())
        }else{
            Ok(var[index-1].clone())
        }
    } 
}

pub fn choose_type() -> TypeApp {
    println!("Choose the type of the application:");
    for (i,v) in TypeApp::variants().iter().enumerate() {
        println!(" {})->{}",i+1,v.name());
    }

    loop {
        print!("Enter Your choice:");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading your choice , try again.");
            continue;
        }

        match input.parse::<TypeApp>() {
            Ok(choice) => return choice,
            Err(_) => eprintln!("Invaid choice , only between 1 and {}.",TypeApp::variants().len()),
        }
    }
}

pub fn choose_root() -> bool {
    loop {
        print!("Does this application need root permission ? (y/n):");
        io::stdout().flush().unwrap();
        
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        ans.trim_end_matches(&['\r','\n'][..]).to_string();
        
        match ans.chars().next() {
            Some('y') => return true,
            Some('n') => return false,
            _ => {
                eprintln!("Invalid choice, only y or n.");
                continue;
            }
        }
    }
}
