use std::{sync::{Arc, Mutex}};

use once_cell::sync::Lazy;

use crate::{GAME_STARTED, print::{GColor, TermPrint}};



#[macro_export]
/// Debug Warn Err Debug
macro_rules! LOG {
    ($level:ident, $($msg:tt)*) => {{
        let line = line!{};
        // Get the function path using type_name_of_val
        let func_name = {
            // Use a dummy fn pointer to get the current function
            fn f() {}
            let full = std::any::type_name_of_val(&f);
            // teşekkürler chatgbt
            // Try to trim trailing "::f" to make output cleaner
            full.trim_end_matches("::f")
        };
        let level = $crate::game::logger::LogLevel::from_string(stringify!($level));
        $crate::game::logger::log(level,func_name,format!($($msg)*).as_str(),line);
    }};
}



pub enum LogLevel{
    Info,
    Warn,
    Error,
    Debug,
}
impl LogLevel {
    fn as_string(&self) -> &str{
        match self {
            LogLevel::Info =>   "Info",
            LogLevel::Warn =>   "Warn",
            LogLevel::Error =>  "Error",
            LogLevel::Debug =>  "Debug",
        }
    }
    pub fn from_string(str:&'_ str) -> LogType<'_>{
        match str {
            "Info"  => LogType::Level(Self::Info),
            "Warn"  => LogType::Level(Self::Warn),
            "Error"   => LogType::Level(Self::Error),
            "Debug" => LogType::Level(Self::Debug),
            _ => LogType::Str(str)
        }
    }
}
pub enum LogType<'a>{
    Level(LogLevel),
    Str(&'a str)
}


pub struct Logger{
    pub logs:Vec<String>,
    pub log_path:Option<String>,
}
impl Logger {
    pub fn new(path:Option<String>) -> Self {
        let m = Vec::new();
        Logger { logs: m, log_path: path }
    }
    pub fn log<S:Into<String>>(&mut self,level:LogType,path:S,msg:S,line:u32){
        let msg = msg.into();
        let path = path.into();
        let timestamp = GAME_STARTED.elapsed().as_millis();
        let logmsg = {
            match level {
                LogType::Level(l) => 
                    match l {
                        LogLevel::Info => {
                            if !cfg!(feature = "info"){
                                return;
                            }
                            // light blue
                            // old from((100,180,255))
                            TermPrint::from((l.as_string(),(),GColor::Cyan))
                        },
                        LogLevel::Warn => {
                            if !cfg!(feature = "warn"){
                                return;
                            }
                            // yellow
                            // old from((255,210,90))
                            TermPrint::from((l.as_string(),(),GColor::Yellow))
                        },
                        LogLevel::Error => {
                            if !cfg!(feature = "error"){
                                return;
                            }

                            // red
                            // old from((255,85,85))
                            TermPrint::from((l.as_string(),(),GColor::Red))
                        },
                        LogLevel::Debug => {
                            //if feature debug is not active do not print debug
                            
                            if !cfg!(feature = "debug"){
                                return;
                            }
                            // green
                            // from((80,200,120))
                            TermPrint::from((l.as_string(),(),GColor::Green))
                        },
                    }
                ,
                LogType::Str(s) => {
                    TermPrint::from((s,(),GColor::DarkGrey))
                },
            }
        };
        let msg = format!("{}:{}\x1b[0m:{}:{line}:{}",timestamp,logmsg,path,msg);
        self.logs.push(msg);
    }

    pub fn set_path(&mut self,path:Option<String>){
        self.log_path = path;
    }

}
pub static GLOBAL_LOGGER: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| {
    Arc::new(Mutex::new(Logger::new(None)))
});

pub fn log<S:Into<String>>(level:LogType,path:S,msg:S,line:u32){
    
    let l = get_logger();
    let mut l = l.lock().unwrap();
    l.log(level, path, msg, line);
}

pub fn get_logger() -> Arc<Mutex<Logger>>{
    Arc::clone(&GLOBAL_LOGGER)
}




#[cfg(test)]
mod log_test{
    use crate::game::logger::{LogLevel, get_logger};

    /*
    macro_rules! log {
        ($level:ident, $($msg:tt)*) => {{
            // Get the function path using type_name_of_val
            let func_name = {
                // Use a dummy fn pointer to get the current function
                fn f() {}
                let full = std::any::type_name_of_val(&f);
                // Try to trim trailing "::f" to make output cleaner
                full.trim_end_matches("::f")
            };

            eprintln!("{}:{}:{}",
                stringify!($level),
                func_name,
                format!($($msg)*)
            );
            }};
    }
    */

    
    
    #[test]
    fn test_path(){
        let _l = LogLevel::Debug;
        let str = stringify!(l);
        println!("{}",str);
        LOG!(warn,"asafas");
    }
    #[test]
    fn macro_test(){
        LOG!(Warn,"WARNED");
        let binding = get_logger();
        let g = binding.lock().unwrap();
        let g = &g.logs;
        for x in g.iter(){
           println!("{x}")
        }
    }
}