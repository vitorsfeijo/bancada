use std::io;

fn calc_gradient(init_color, color1, color2, color3, color4, end_color){

    n: 4;
    
    
}

fn show_gradient(init_color, color1, color2, color3, color4, end_color){
    calc_gradient(init_color, color1, color2, color3, color4, end_color);
    format!("{init_color}█{init_color}{color1}█{color1}{color2}█{color2}{color3}█{color3}{color4}█{color4}{end_color}█{end_color}");
}

pub fn gradient(){
    init_color: &str = "\x1b[38;2;66;133;244m";
    end_color: &str = "\x1b[38;2;66;133;244m";
    
    color1: &str = "\x1b[38;2;66;133;244m";
    color2: &str = "\x1b[38;2;66;133;244m";
    color3: &str = "\x1b[38;2;66;133;244m";
    color4: &str = "\x1b[38;2;66;133;244m";

    while(true){
        calc_gradient(init_color, color1, color2, color3, color4, end_color);
    }

    pritnln!("")

}