use std::io;
use std::io::{Write};
pub mod typeapp;
use crate::typeapp::*;
use serde_json::json;

// ~/programming/shiva && ./shiva => ./home/devpsiarch/shiva/shiva
// ~/programming/shiva && ./check ; ./build run => ./home/devpsiarch/shiva/shiva

const LEDGER_FILE: &str = "~/.shiva_ledger.json";
const SERVICE_DIR: &str = "/etc/systemd/system/";
 
enum StatusApp {
    Running,
    Sleep,
    Crashed,
    Stopped,
    Dead,
}

impl StatusApp {
    pub fn name(&self) -> &'static str {
        match self {
            StatusApp::Running => return "Running",
            StatusApp::Sleep => return "Sleep",
            StatusApp::Crashed => return "Crashed",
            StatusApp::Stopped => return "Stopped",
            StatusApp::Dead => return "Dead",
        }
    }
}

pub struct Job {
    name:String,
    istype:TypeApp,
    mainDir:String,
    needRoot:bool,
    command:String,
    state:StatusApp,
}

impl Job {
    /*
    * @brief This creates hust object that cannot do shit for now ,
    * */
    pub fn new(n:&String,t:TypeApp,m:&String,r:bool,c:&String) -> Self {
        Job {
            name:n.to_string(),
            istype:t,
            mainDir:m.to_string(),
            needRoot:r,
            command:c.to_string(),
            state:StatusApp::Dead,
        }
    }

    pub fn read(filepath:&String) -> Result<Self,String> {
        Err(String::from("Unimplimented"))
    }

    fn execute(&self) -> Result<(),String> {
        Ok(())
    }
    
    /*
    * @brief Walks You though setting up a job
    * */

    fn new_wizard() -> Result<Self,()> {
        println!("You have entered the wizard , we will walk you though setting up Your app.");
        print!("Application name:");
        
        io::stdout().flush().expect("Error flushing stdout");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Error reading line");
        if name.ends_with('\n') { name.pop(); }          // remove '\n'
        if name.ends_with('\r') { name.pop(); }          // remove '\r' if on Windows 

        let mut type_app = choose_type();
        
        print!("Application Main directory:");
        io::stdout().flush().expect("Error flushing stdout");
        let mut dir = String::new();
        io::stdin().read_line(&mut dir).expect("Error reading line");
        if dir.ends_with('\n') { dir.pop(); }          // remove '\n'
        if dir.ends_with('\r') { dir.pop(); }          // remove '\r' if on Windows        

        let mut root = choose_root();

        print!("Application running command (becarefull):");
        io::stdout().flush().expect("Error flushing stdout");
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd).expect("Error reading line");
        if cmd.ends_with('\n') { cmd.pop(); }          // remove '\n'
        if cmd.ends_with('\r') { cmd.pop(); }          // remove '\r' if on Windows        

        Ok(Self::new(&name,type_app,&dir,root,&cmd))
    }

    fn make_service_template(desc:&str,exe_path:&str,dir:&str) -> String {
        format!(r#"
            [Unit]
            Description={}
            After=network.target

            [Service]
            Type=simple
            WorkingDirectory={}
            ExecStart={}
            Restart=on-failure

            [Install]
            WantedBy=multi-user.target
            "#,
            desc,dir,exe_path)
    }

    fn make_serice_file_name(name:&str) -> String {
        format!("shiva-<{}>.service",name)
    }

    fn create_entry() -> Result<(),String> {
        let entry = match Self::new_wizard() {
            Ok(s) => s,
            Err(e) => return Err("Error creating an entry from \"wizard\"".to_string()),
        };
        let file_name = Self::make_serice_file_name(&entry.name);
        let mut service_file = match std::fs::File::create(file_name.clone()) {
            Ok(f) => f,
            Err(e) => return Err("could not create service file.".to_string()),
        };

        let write_result = match service_file.write(Self::make_service_template(
                format!("{} app managed by shiva ",&entry.name).as_str(),
                &entry.command,
                &shellexpand::tilde(&entry.mainDir)
            ).as_bytes()) {
            Ok(s) => s,
            Err(e) => {
                std::fs::remove_file(file_name).unwrap_or_else(|error|{
                    panic!("problem cleaning up after failing to create service file. {error:?}");
                }); 
                return Err(format!("Could not write bytes to service file. {}",e));
            }
        };
    
        let json_entry = json!({
            "name":entry.name,
            "istype":entry.istype.name(),
            "mainDir":entry.mainDir,
            "needRoot":entry.needRoot,
            "command":entry.command,
            "state":entry.state.name() 
        }); 

        let expanded: std::borrow::Cow<'_, str> = shellexpand::tilde(LEDGER_FILE);

        let ledger_file = match std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .write(true)
            .open(expanded.into_owned()) {
            Ok(s) => s,
            Err(e) => {
                let clean_up = std::fs::remove_file(file_name.clone()).unwrap_or_else(|error|{
                    panic!("problem cleaning up after failing to create write entry in ledger. {error:?}");
                });
                return Err(format!("Could not open the ledger file. {}",e));
            }
        };
        
        serde_json::to_writer_pretty(ledger_file,&json_entry).unwrap_or_else(|error| {
            // clean up the service file created before 
            let clean_up = std::fs::remove_file(file_name).unwrap_or_else(|error|{
                panic!("problem cleaning up after failing to create write entry in ledger. {error:?}");
            }); 
        });

        Ok(())
    } 

    pub fn create_service() {
        let result_entry = Self::create_entry().unwrap_or_else(|error| {
            panic!("Error creating a service, {error:?}");
        });
    }

}
