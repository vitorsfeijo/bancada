// Cores inspiradas no Gemini/Google:                                                                                                          
const BLUE: &str = "\x1b[38;2;66;133;244m";                                                                                                    
const PURP: &str = "\x1b[38;2;161;66;244m";                                                                                                    
const CYAN: &str = "\x1b[38;2;38;198;218m";                                                                                                   
const GRAY: &str = "\x1b[38;2;140;140;140m";                                                                                                  
const RESET: &str = "\x1b[0m";                                                                                                                 
const BOLD: &str = "\x1b[1m";                                                                                                                  

pub fn start() {
    println!("{BLUE} ====================== {RESET}   {BOLD}Bancada v0.1.0{RESET}");
    println!("{AZUL} [  ________________  ] {RESET}   {CINZA}Harness para LLMs locais{RESET}");
    println!("{PURP}  | |              | |  {RESET}   Modelo:    {CIANO}llama3.2{RESET}");
    println!("{ROXO}  | |==============| |  {RESET}   Status:    {AZUL}pronto{RESET}");
    println!("{CIANO}  | |              | |  {RESET}");
    println!("{CYAN} [___]            [___] {RESET}");
    println!();
}
