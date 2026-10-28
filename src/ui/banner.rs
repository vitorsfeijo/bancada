use std::env;

// Cores inspiradas no Gemini/Google:                                                                                                          
const AZUL: &str = "\x1b[38;2;66;133;244m";                                                                                                    
const ROXO: &str = "\x1b[38;2;161;66;244m";                                                                                                    
const CIANO: &str = "\x1b[38;2;38;198;218m";                                                                                                   
const CINZA: &str = "\x1b[38;2;140;140;140m";                                                                                                  
const RESET: &str = "\x1b[0m";                                                                                                                 
const BOLD: &str = "\x1b[1m";                                                                                                                  

fn start() {
    println!("{AZUL} ====================== {RESET}   {BOLD}Bancada v0.1.0{RESET}");
    println!("{AZUL} [  ________________  ] {RESET}   {CINZA}Harness para LLMs locais{RESET}");
    println!("{ROXO}  | |              | |  {RESET}   Modelo:    {CIANO}llama3.2{RESET}");
    println!("{ROXO}  | |==============| |  {RESET}   Status:    {AZUL}pronto{RESET}");
    println!("{CIANO}  | |              | |  {RESET}");
    println!("{CIANO} [___]            [___] {RESET}");
    println!();
}
