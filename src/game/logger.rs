use std::{sync::{Arc, Mutex}};

use once_cell::sync::Lazy;

use crate::{GAME_STARTED, print::{RGB, TermPrint}};



#[macro_export]
/// Debug Warn Err Debug
macro_rules! LOG {
    ($level:ident, $($msg:tt)*) => {{
        // Get the function path using type_name_of_val
        let func_name = {
            // Use a dummy fn pointer to get the current function
            fn f() {}
            let full = std::any::type_name_of_val(&f);
            // teşekkürler chatgbt
            // Try to trim trailing "::f" to make output cleaner
            full.trim_end_matches("::f")
        };
        let level = LogLevel::from_string(stringify!($level));
        log(level,func_name,format!($($msg)*).as_str());
        /*
        eprintln!("{}:{}:{}",
            stringify!($level),
            func_name,
            format!($($msg)*)
        );
        */
    }};
}



pub enum LogLevel{
    Info,
    Warn,
    Err,
    Debug,
}
impl LogLevel {
    fn as_string(&self) -> &str{
        match self {
            LogLevel::Info =>   "Info",
            LogLevel::Warn =>   "Warn",
            LogLevel::Err =>    "Err",
            LogLevel::Debug =>  "Debug",
        }
    }
    pub fn from_string(str:&'_ str) -> LogType<'_>{
        match str {
            "Info"  => LogType::Level(Self::Info),
            "Warn"  => LogType::Level(Self::Warn),
            "Err"   => LogType::Level(Self::Err),
            "Debug" => LogType::Level(Self::Debug),
            _ => LogType::Str(str)
        }
    }
}
pub enum LogType<'a>{
    Level(LogLevel),
    Str(&'a str)
}



pub type Logger = Arc<Mutex<Vec<String>>>;
pub static GLOBAL_LOGGER: Lazy<Logger> = Lazy::new(|| {
    let m = Vec::new();
    Arc::new(Mutex::new(m))
});

pub fn log<S:Into<String>>(level:LogType,path:S,msg:S){
    let msg = msg.into();
    let path = path.into();
    let l = get_logger();
    let timestamp = GAME_STARTED.elapsed().as_millis();
    let logmsg = {
        match level {
            LogType::Level(l) => 
                match l {
                    LogLevel::Info => {
                        // light blue
                        TermPrint::from((l.as_string(),(),RGB::from((100,180,255))))
                    },
                    LogLevel::Warn => {
                        // yellow
                        TermPrint::from((l.as_string(),(),RGB::from((255,210,90))))
                    },
                    LogLevel::Err => {
                        // red
                        TermPrint::from((l.as_string(),(),RGB::from((255,85,85))))
                    },
                    LogLevel::Debug => {
                        // green
                        TermPrint::from((l.as_string(),(),RGB::from((80,200,120))))
                    },
                }
            ,
            LogType::Str(s) => TermPrint::from((s,(),RGB::from((120, 120, 120)))),
        }
    };
    let mut log = l.lock().unwrap();
    let msg = format!("{}:{}\x1b[0m:{}:{}",timestamp,logmsg,path,msg);
    log.push(msg);

}

pub fn get_logger() -> Logger{
    Arc::clone(&GLOBAL_LOGGER)
}




#[cfg(test)]
mod log_test{
    use crate::game::logger::{LogLevel, get_logger, log};

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
        LOG!(warn,"asafas" );
    }
    #[test]
    fn macro_test(){
        LOG!(Warn,"WARNED");
        let g = get_logger();
        let g = g.lock().unwrap();
        for x in g.iter(){
           println!("{x}")
        }
    }
}