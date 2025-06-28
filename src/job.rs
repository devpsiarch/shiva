use std::io;
use std::io::{Write,Read};
pub mod typeapp;
use crate::typeapp::*;
use serde_json::json;
use serde_json::Value;
use std::path::Path;
use std::fs::File;
use std::fs;
use prettytable::{Table, Row, Cell};
use std::process::Command;

// ~/programming/shiva && ./shiva => ./home/devpsiarch/shiva/shiva
// ~/programming/shiva && ./check ; ./build run => ./home/devpsiarch/shiva/shiva

const LEDGER_FILE: &str = "~/.shiva_ledger.json";
const SERVICE_DIR: &str = "/etc/systemd/system";
 
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

    fn read_ledger() -> serde_json::Value {
        let expanded: std::borrow::Cow<'_, str> = shellexpand::tilde(LEDGER_FILE);
        let mut content = fs::read_to_string(expanded.into_owned()).unwrap_or_else(|_| "[]".to_string());
        serde_json::from_str(&content).expect("Could not read Value from ledger")
    }

    pub fn list_services() {
        let v = Self::read_ledger();
        let arr = match v {
            Value::Array(s) => s,
            _ => {
                println!("Expected a JSON object");
                return;
            }
        };

        let cols: Vec<String> = if let Some(Value::Object(first)) = arr.get(0) {
            let mut keys: Vec<_> = first.keys().cloned().collect();
            keys.sort();
            keys
        }else{
            println!("No services or first entry is faulty.");
            return;
        };

        let mut table = Table::new();
        // header row 
        table.add_row(Row::new(
            cols.iter().map(|h| Cell::new(h)).collect()
        ));
        for obj in arr {
            if let Value::Object(map) = obj {
                let row = cols.iter().map(|key| {
                    let cell = map
                    .get(key)
                    .map_or("".to_string(), |v| v.to_string());
                    Cell::new(&cell)
                });
                table.add_row(Row::new(row.collect()));
            }
        }
        table.printstd();
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

    fn make_service_file_name(name:&str) -> String {
        format!("{}/shiva-{}.service",SERVICE_DIR,name)
    }

    fn create_entry() -> Result<(),String> {
        let entry = match Self::new_wizard() {
            Ok(s) => s,
            Err(e) => return Err("Error creating an entry from \"wizard\"".to_string()),
        };
        let file_name = Self::make_service_file_name(&entry.name);
        let mut service_file = match std::fs::File::create(file_name.clone()) {
            Ok(f) => f,
            Err(e) => return Err((format!("could not create service file.{} ",e))),
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

        let file_path = expanded.clone().into_owned();

        let content = fs::read_to_string(expanded.into_owned()).unwrap_or_else(|_| "[]".to_string()); 

        let mut v = serde_json::from_str(&content).expect("Ledger did not have corret format.");
        if let Value::Array(ref mut arr) = v {
            arr.push(json_entry);
        }else{
            return Err("Expected json root to be array.".to_string());
        }
        
        let pretty = serde_json::to_string_pretty(&v)
        .expect("Failed to serialize JSON");

        fs::write(file_path.clone(), pretty).unwrap_or_else(|_|{
            panic!("Could not write to file");
        });

        println!("✅ Successfully added new entry to `{}`", file_path);
        Ok(())
    } 

    pub fn create_service() {
        let result_entry = Self::create_entry().unwrap_or_else(|error| {
            panic!("Error creating a service, {error:?}");
        });
    }

    fn handle_display_cmd_result(result:&std::process::Output) {
        if result.status.success() {
            print!("✅ Command succeeded.");
            if !result.stdout.is_empty() {
                print!("stdout:\n{}", String::from_utf8_lossy(&result.stdout));
            }
        } else {
            println!("❌ Command failed with exit code: {}", result.status);
            if !result.stderr.is_empty() {
                print!("stderr:\n{}", String::from_utf8_lossy(&result.stderr));
            }
            if !result.stdout.is_empty() {
                print!("stdout:\n{}", String::from_utf8_lossy(&result.stdout));
            }
        }
    }

    pub fn alter_service(state:&str,name:&str) {
        let result = Command::new("systemctl")
            .arg(state)
            .arg(Self::make_service_file_name(&name))
            .output()
            .unwrap_or_else(|error| {
                panic!("Could not enable service due to : {}",error);
            });
            Self::handle_display_cmd_result(&result); 
    }

    pub fn start_service() {

    }
    
    pub fn stop_service() {

    }

}
