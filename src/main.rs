use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use unicode_width::UnicodeWidthStr;
use axum::Router;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(name = "jp")]
#[command(about = "A CLI tool for learning Japanese", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Input text (romaji or Chinese), used when no subcommand is provided
    #[arg(value_name = "TEXT")]
    text: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a web server to host the web application
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
        
        /// Host address to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
    /// Lookup romaji or translate Chinese (default command)
    Lookup {
        /// Input text (romaji or Chinese)
        text: String,
    },
}

struct JapaneseChar {
    romaji: String,
    hiragana: String,
    katakana: String,
    examples: Vec<String>,
}

// 为假名创建 ASCII art 大字效果
fn get_ascii_art(character: &str) -> Vec<String> {
    match character {
        "あ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   -##+                           ".to_string(),
            "                   *@@-    ...''',,.              ".to_string(),
            "            +******%%@######****#**;              ".to_string(),
            "            ',,,,':@@%.     :;:,                  ".to_string(),
            "                  :@%%-==+++@@@@+=-;,.            ".to_string(),
            "              ';+#%%%@=;:,:%%@+:;-=*%@*-'         ".to_string(),
            "            ;#@#-,.+@%-  .+@@-       ;@@%;        ".to_string(),
            "          .*@%:    .#%@;-%@*,         +%%@        ".to_string(),
            "          -@%;      -@@@@#,          ;%@@=        ".to_string(),
            "          .+%%=-=+*##+=#+:  ',,:;-=*%@#=,         ".to_string(),
            "            .:---;:'        '=##*+=;:.            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "い" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           ++=;                  .                ".to_string(),
            "          ,@@@:                ;*@#='             ".to_string(),
            "          ;%%%.                 '-%@@*,           ".to_string(),
            "          -@@#                    .=@@@+.         ".to_string(),
            "          -%@#                      ;%%@#'        ".to_string(),
            "          ;@%%.      '.              ;@@@%.       ".to_string(),
            "          .@@%-     -@@#'             ;;:,.       ".to_string(),
            "           :#@@*--=%@%='                          ".to_string(),
            "             ,-+***=:.                            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "う" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 .***+=--;:,,''..                 ".to_string(),
            "                 ';-==+**###%%%%*                 ".to_string(),
            "                                                  ".to_string(),
            "                 .',,:;-=++**+=-:'                ".to_string(),
            "             ,#%%%%%##*++=---=*@@@*:              ".to_string(),
            "              ':,'.            .+%@@,             ".to_string(),
            "                                =%%@:             ".to_string(),
            "                              .-@@@=              ".to_string(),
            "                           ';*@@%+,               ".to_string(),
            "                    ,;-=*#@@@#=:.                 ".to_string(),
            "                    ,+#*=-:'.                     ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "え" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 :,,'''.....                      ".to_string(),
            "                '**############,                  ".to_string(),
            "                                                  ".to_string(),
            "            .::::;;;---=---==+*#='                ".to_string(),
            "            .+*++++==-;;;=#@@%+;,.                ".to_string(),
            "                      ,-**=:.                     ".to_string(),
            "                  '-#@@@%+=-;'                    ".to_string(),
            "               ,=%@%+;,'',=@@#                    ".to_string(),
            "           .,=%@#-,       '@%*                    ".to_string(),
            "          :*@#-'          .+%@*+++++++++:         ".to_string(),
            "            .               .'::::::::::.         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "お" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                  -+=:                            ".to_string(),
            "                  #@@'          .:;'              ".to_string(),
            "          .,,,,,::#%%+=++*#*    :+%@%*-,.         ".to_string(),
            "          '****+++%@@=;;:,'.       ';+%@*,        ".to_string(),
            "                  *@@.      ...        '          ".to_string(),
            "                .:#%%*************+=:'            ".to_string(),
            "            ';+##*%%@;'..       .';+@@*:          ".to_string(),
            "          '+@%=,. *@%'              .#@@:         ".to_string(),
            "         .%@#     #@@,   '-+-.      ,%@%,         ".to_string(),
            "          +%%-:::+@%+    .;*%%#*++*#%*='          ".to_string(),
            "           ':=+++-:'         .,,::,'              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "か" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                  .,''.                           ".to_string(),
            "                  *@@+'                           ".to_string(),
            "                 =@%+          .,;,               ".to_string(),
            "        .::::;;:=@@@+=+++-;'   '=%@#-'            ".to_string(),
            "        '*++++-#@@#:;;::;#@%;     ;#@%='          ".to_string(),
            "              :%@#.      ,%@%       -@@%;         ".to_string(),
            "             =@@*.       '@%%.       ,#@@*.       ".to_string(),
            "           '*@@=         ;%%#         '=;:.       ".to_string(),
            "          -%@#:  '--'   ,%@@:                     ".to_string(),
            "        :#@@=.   :+%%%#%@%+,                      ".to_string(),
            "        '','        .',,.                         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "き" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      ;+==                        ".to_string(),
            "                      '%@@'  ..'',:.              ".to_string(),
            "            =++++++++++%@@%******++,              ".to_string(),
            "            ',,,,,'''''.:#@%:     .',:;:          ".to_string(),
            "           ;;;;;;-----===#@@@##***##**+=.         ".to_string(),
            "           ;===--;;;::::,,''-#%#-'                ".to_string(),
            "                 ',:;;;------=*@@@#=,             ".to_string(),
            "             '=#%#+-;;:::::::::;;-=:.             ".to_string(),
            "             *@@-                                 ".to_string(),
            "             '+%%*=--;;;;;;;;;;;;:                ".to_string(),
            "               .,;-=+++++++++++++:                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "く" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                            '=-,.                 ".to_string(),
            "                         .;#@@*;'                 ".to_string(),
            "                      ';*@@*;.                    ".to_string(),
            "                   ,=#@%+:.                       ".to_string(),
            "               .:=#@#=,                           ".to_string(),
            "              '%@@#'                              ".to_string(),
            "               '-*%%*-,                           ".to_string(),
            "                  .:=%@%*-'                       ".to_string(),
            "                      ';*@@%+:.                   ".to_string(),
            "                         .:+%@@*:                 ".to_string(),
            "                             :=;'                 ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "け" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                               :--,               ".to_string(),
            "            +%#+.              *@@;               ".to_string(),
            "           ,@@@,               +%%,               ".to_string(),
            "           =%%*     ========+++%@@#*****#*        ".to_string(),
            "           *@@;     ------;;;;:#@@-,,''...        ".to_string(),
            "           #@@:                +@%:               ".to_string(),
            "           #@@;                #%@,               ".to_string(),
            "           +%%+               =%@*                ".to_string(),
            "           ,@@@'            '+@@+.                ".to_string(),
            "            ::,.          ,+@@#;                  ".to_string(),
            "                          ',;:                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "こ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "              .====================.              ".to_string(),
            "              .--------------------.              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            '#**'                                 ".to_string(),
            "            ;@@@'                                 ".to_string(),
            "             ;*%@#**++++++++++++++++++'           ".to_string(),
            "               .',::::::::::::::::::::.           ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "さ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      -++=                        ".to_string(),
            "                      .%@@;                       ".to_string(),
            "            ''''''',,,,;@@@-:;---=++*,            ".to_string(),
            "            ;********+++++@@@*----;;:'            ".to_string(),
            "                          '+%%-'                  ".to_string(),
            "                   .'''''''':*@@#=,.              ".to_string(),
            "              .;+##**+++++++++*#@@@@*;            ".to_string(),
            "             ;%@#,.             .',;:.            ".to_string(),
            "             -@@*.                                ".to_string(),
            "              ,=#@#*+=-;;;;;;;;:;'                ".to_string(),
            "                 .,;-==++++++++++'                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "し" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               .'''.                              ".to_string(),
            "               -@@@.                              ".to_string(),
            "               -%%%.                              ".to_string(),
            "               -@%%.                              ".to_string(),
            "               -@%%.                              ".to_string(),
            "               -@%%.                              ".to_string(),
            "               -%%%.                              ".to_string(),
            "               ;@%%.               ';**:          ".to_string(),
            "               .#@@#:.       .':-+%@@#=,          ".to_string(),
            "                 :=#%%#######%%#*+;,.             ".to_string(),
            "                     .''',,''.                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "す" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                         -##;                     ".to_string(),
            "                         +@@:                     ".to_string(),
            "       .------========+++#%%#+********#*#=        ".to_string(),
            "       .==-----;;:::::::,*@@=''''''......         ".to_string(),
            "                 :=++==++%%@:                     ".to_string(),
            "               =@@+'    '#%@+                     ".to_string(),
            "               -%@+,''',;#%%%.                    ".to_string(),
            "                .:=+++=-=%%@*                     ".to_string(),
            "                       ,*@%=                      ".to_string(),
            "                  ',-*%@#-'                       ".to_string(),
            "                 .;*#=:.                          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "せ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                              ....                ".to_string(),
            "               ';;:           +%%-                ".to_string(),
            "               -@@=           =@@:                ".to_string(),
            "            ...=%%=''',,,,,:::*%%+;;------        ".to_string(),
            "       ;#####**#@@#**++++++++=#@%*--------        ".to_string(),
            "        ...    -@@-           =%@:                ".to_string(),
            "               -@@=    '-----=%@@:                ".to_string(),
            "               -%%-    .======-:'                 ".to_string(),
            "               ;@@*'                              ".to_string(),
            "                ;*#%################-             ".to_string(),
            "                   .................              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "そ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                .....'',,,,::-=:'                 ".to_string(),
            "               '####**+==+%@@@@#='                ".to_string(),
            "                      ';+#%*-,.                   ".to_string(),
            "                  .:=*#+-,         ....'.         ".to_string(),
            "         '',,::-+%@@@#=--=+*##########*#=         ".to_string(),
            "         =#***++=-;;::-*@@#+-:,'...               ".to_string(),
            "                    ,+%#;.                        ".to_string(),
            "                   ,@@@:                          ".to_string(),
            "                   .*@@#:.                        ".to_string(),
            "                     :=#@@%#**++==.               ".to_string(),
            "                        .',;--=++:                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "た" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 ,;::.                            ".to_string(),
            "                 %@@=                             ".to_string(),
            "                :@%%'',,:;'                       ".to_string(),
            "        .*******%%@#**+++=:                       ".to_string(),
            "         ..... ;@@*         .',::;----=:          ".to_string(),
            "              .#%@'      =####**++===--;          ".to_string(),
            "              *@@-       .'.                      ".to_string(),
            "             +@@+                                 ".to_string(),
            "            +@@+       .,.                        ".to_string(),
            "          ,%@@=        -*###***+++++++++-         ".to_string(),
            "          ',::            .',,:;;;;;;;;;:         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ち" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   '-;:.                          ".to_string(),
            "                   +@@=                           ".to_string(),
            "          ,,,:::::-@%@=-==++****.                 ".to_string(),
            "          =+++++=+@%%-;;;:::,,''                  ".to_string(),
            "                 =%@;                             ".to_string(),
            "                -@%= ',:;-==++===-:'              ".to_string(),
            "              .-@@@###*+-;;:::::;=#@%+'           ".to_string(),
            "             .-%%*-:'              :@@@.          ".to_string(),
            "                .                .,+@@*.          ".to_string(),
            "                     :----===++*#%%#+:            ".to_string(),
            "                     '=====--;;:,'.               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "つ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                     ..',,::::::,,.               ".to_string(),
            "         ..',:;=+**####***+++++**#%%#=:           ".to_string(),
            "         -%@%#*+=;:'.             .:+@@%:         ".to_string(),
            "          ''                         -%%@'        ".to_string(),
            "                                     ;%@@'        ".to_string(),
            "                                  .,+@@#:         ".to_string(),
            "                    ....'',,:;-=+#%%#=:           ".to_string(),
            "                    =#######**+=-:'.              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "て" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                ...''',,,,,:;--==++***#*          ".to_string(),
            "         -########***++*%@@@%#+-;::,,'''          ".to_string(),
            "         .,'''..     :+%#+:'                      ".to_string(),
            "                   .*@@-.                         ".to_string(),
            "                   *@%;                           ".to_string(),
            "                   #@%;                           ".to_string(),
            "                   '*@@='                         ".to_string(),
            "                     ,=%@%*+-;:,,'''.             ".to_string(),
            "                        .,;=+**####*.             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "と" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   ,:::                           ".to_string(),
            "                   =@@%                           ".to_string(),
            "                   :%%%                           ".to_string(),
            "                   '@%@'         .',              ".to_string(),
            "                    +%%*,:;=+*#%%%%%+.            ".to_string(),
            "                 .,-*@@%%#*+=;:,'..               ".to_string(),
            "              :=#@%*=:'.                          ".to_string(),
            "            :#@@=,                                ".to_string(),
            "            +@@%'                                 ".to_string(),
            "             -*%@#**++++++++++++++++:             ".to_string(),
            "               .',::;;;;;;;;;;;;;;;;'             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "な" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                  '+=-.                           ".to_string(),
            "                  #@%:                            ".to_string(),
            "         ';;----;*@@%=+**#,   ;#**=-;,'.          ".to_string(),
            "         '====-+@@@-:::,''.   ..,:-+*##%+.        ".to_string(),
            "              .*@%,           :::.     .          ".to_string(),
            "             -@@*'           .@@@'                ".to_string(),
            "           ,*@%-              #%%                 ".to_string(),
            "         '+@@*'     ,;=++++++=%%%;'.              ".to_string(),
            "         ':;:     :#@#:,''''';%@@*####+=:         ".to_string(),
            "                  ;@@#:,''',;#@%;  .':--'         ".to_string(),
            "                   .:-++****+-:.                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "に" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            .                                     ".to_string(),
            "           #%#=.                                  ".to_string(),
            "          ;@@@'      ,==++++***********.          ".to_string(),
            "          *%%+       ';;::::,,,,'''''''           ".to_string(),
            "         .%%@:                                    ".to_string(),
            "         ,@%@'                                    ".to_string(),
            "         ,@%@.                                    ".to_string(),
            "         '@%%'      .=%*-:'.                      ".to_string(),
            "         .%@@:       ':-+###############*.        ".to_string(),
            "          #%%-             ..''',,,,,,,,,         ".to_string(),
            "          ...                                     ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ぬ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                         ,,''                     ".to_string(),
            "            ,::,         #@@+                     ".to_string(),
            "           .@@@=        '%%%,                     ".to_string(),
            "            =%%# .,;-+**#%@%*###**+=:'            ".to_string(),
            "            .%@%###*=;,=@@%'....':-*@@#=.         ".to_string(),
            "          .;*@%@@+    ,%@#'         .+@@%.        ".to_string(),
            "         ;%@*:.-%@#:.-@@+.           '%%@;        ".to_string(),
            "        ;@@-    .-%@%@%;   .:--===-;,;%%%.        ".to_string(),
            "        =@@:    .:%@#%%-  +@@;,',,:=%@@@%:.       ".to_string(),
            "         -#@*+*#%#=, ''   ;#%+----=*%*-;#%=.      ".to_string(),
            "           ',::,.           .,:;;;:'.             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ね" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               -==,                               ".to_string(),
            "               #@@,                               ".to_string(),
            "               *%%:.        ..''''.               ".to_string(),
            "       '::;--==%@%%=..,;=+********#%*=,           ".to_string(),
            "       :*+==-::#%%+=*#*+-:'.       :@@@:          ".to_string(),
            "             .=%%%*=:.              =%%*          ".to_string(),
            "           :*@@%%@,                 =@%*          ".to_string(),
            "       .,=%@*:.*@@,      :=**+++++=-#%@=          ".to_string(),
            "       :*#-'   *%%,     #@%,    .';%@@%%#*;       ".to_string(),
            "               #@@,     ;*#+----=+##+:.';-'       ".to_string(),
            "               -=='       .,:::::,.               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "の" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                ':-+***##**####*+-:'              ".to_string(),
            "            ';*%%#+-:':@%*='.',;=*@@#=,           ".to_string(),
            "          :*@@*:.     =%%@,       .;%@@+'         ".to_string(),
            "         +@@#'       '@%@-          .#%@#.        ".to_string(),
            "        =@%%.        #%@*            +%%@'        ".to_string(),
            "        +@%#       .*@@*            :%@@+         ".to_string(),
            "        .*@@*,   '-%@@=          '-#@@#;          ".to_string(),
            "          :+%@%#%@@#='    .,:;=#%@@#=,            ".to_string(),
            "            .':::,.       '-#%*=;,.               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "は" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                             .:::'                ".to_string(),
            "           *##-              '@@@,                ".to_string(),
            "          -@@#             ...#%%;''''',,'        ".to_string(),
            "          #%%;    .*########**%@@%*******=        ".to_string(),
            "         '@%@'     ''.....    ;@@*                ".to_string(),
            "         :%%%.                :@@#                ".to_string(),
            "         ;@@%            .... ;@@*                ".to_string(),
            "         ;%%%       ,=*#******#%%%-;,.            ".to_string(),
            "         :@%@.     :@@*.     .+@@#=*#%%*=;.       ".to_string(),
            "         .%%%;     .-##*====+#%#=.   .,;+-.       ".to_string(),
            "          ..          .,:;;;:,.                   ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ひ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                    .                             ".to_string(),
            "         ,;--=+++*#%@=.       -+++                ".to_string(),
            "         :*+=-;-#@@+,.        :@@@-               ".to_string(),
            "              :#@*,            %%%@;              ".to_string(),
            "            .*@@-              #@%%@*,            ".to_string(),
            "            *@%;               #@%,-@@*:          ".to_string(),
            "           :@%%               '%%@. '=@@%=.       ".to_string(),
            "           ,@@%:             .*@@=    .;;'        ".to_string(),
            "            :#@@*;'.     .':=@@%-                 ".to_string(),
            "              ,-*#%%#####%%%*=:.                  ".to_string(),
            "                   .'''''.                        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ふ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   ::'.                           ".to_string(),
            "                  ;*%%%#*+=;:,                    ".to_string(),
            "                      ',;=+**,                    ".to_string(),
            "                 '-;:.                            ".to_string(),
            "                .%@@@-'                           ".to_string(),
            "                 .:-+#%%#+-,.    '-*=,            ".to_string(),
            "              ,-=:.   .';+%@%*:  .:+%@%=,         ".to_string(),
            "          '-*@@%+;.        ;@@@-     :*@@#-.      ".to_string(),
            "       '=%@%*;'   '.       :@%@=       '=+;.      ".to_string(),
            "        ':,      -%%%##***%@%#-                   ".to_string(),
            "                   ',:;;;;:,'                     ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "へ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 ,-*#%#*=:.                       ".to_string(),
            "             '-*@@%+-;;+%@@#=:.                   ".to_string(),
            "        ',-*%@@*-'       ';+%@@%+;'               ".to_string(),
            "       '=%@*-,               ':+#@@@*=:'          ".to_string(),
            "          .                      .:=#@@@%+,       ".to_string(),
            "                                     .,;'         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ほ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           ''..                                   ".to_string(),
            "          ,@@@-    ,++++++**********#*#+          ".to_string(),
            "          +%%*     '::::::,,,-@@%,''...           ".to_string(),
            "          %%@:               '%%%,...''',.        ".to_string(),
            "         ,@@%.    .###########%@@%###****;        ".to_string(),
            "         :@@#      ....       +@@-                ".to_string(),
            "         ;@@*          ..''''.+@@=                ".to_string(),
            "         ;%%*      .;+#*++++**%%%%=-:'            ".to_string(),
            "         ,@@@.     +@@;      '%@@=-+#%%#+;        ".to_string(),
            "         .###:     '=##*++++*%#*;    .,;=,        ".to_string(),
            "                      .',,,,'.                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ま" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                        :==;                      ".to_string(),
            "                        =@@-       .....          ".to_string(),
            "         '+++++*********%%%#*########***,         ".to_string(),
            "         .:,,,'''''''''.+@@-                      ".to_string(),
            "             ..'''''''''+@@=,:::::::;,            ".to_string(),
            "            :**********+#@@#+++++++==;            ".to_string(),
            "                        =@@;                      ".to_string(),
            "              :=********#%%#==-;,'.               ".to_string(),
            "            '%@#,.     .#@@=:;=+*####+=,          ".to_string(),
            "            .*%%-:,,,:-*@%=       .,;==.          ".to_string(),
            "              ':-==+===;,.                        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "み" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      ..'                         ".to_string(),
            "             ;==+++**+*%@%='                      ".to_string(),
            "             '-;:,,' '#@@=.      ,:,'             ".to_string(),
            "                    ,#@+'        +@@=             ".to_string(),
            "             ',:;;;=@@%;,:,'.    +%%;             ".to_string(),
            "         '-*#*+=-=@@%-;=+++*****+%%@-'.           ".to_string(),
            "       '*@#;'   :#@+'        ..'*@@**###*+:       ".to_string(),
            "       %@%'  .;#@#:            -%@+   .,;-.       ".to_string(),
            "       ,*%%##%%+:           .;%@%;                ".to_string(),
            "          .''.            .-%@#;.                 ".to_string(),
            "                            .'                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "む" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 :==:                             ".to_string(),
            "                 +@@;                             ".to_string(),
            "         ;---====#%%#+*****-    .-=:.             ".to_string(),
            "         ;=-;;;::*@@=,,''''.    '-*%@%+;'         ".to_string(),
            "           .,::::*%@:               ';*%@*'       ".to_string(),
            "         -#@+-;;=#%%+                   ,.        ".to_string(),
            "        ;@@;     -@@*                             ".to_string(),
            "        ,#@#:,,;+%%;              ###'            ".to_string(),
            "         .:=++=+@@#               #@@'            ".to_string(),
            "                +@@+;:::::::::::;+@@*             ".to_string(),
            "                 ':-===========+==;,              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "め" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                          .:,,.                   ".to_string(),
            "             :--,         =@@+                    ".to_string(),
            "             ;@@+    .',:;%%@;:,'.                ".to_string(),
            "              +%%-=****++@@@-==+*###+:.           ".to_string(),
            "            .;#@@@+,.   +@%:      ':+@%+'         ".to_string(),
            "          '+@@=:#@#'  '#@#'          ,%@%'        ".to_string(),
            "         -@@+.  .=@@-;%%=.            +%@-        ".to_string(),
            "        :@%*      :%@@@=             :%@%'        ".to_string(),
            "        '#@%:. .,=%%+;+%*:       .,-#@%+'         ".to_string(),
            "          :+##*##*-'   .     ;+*#%%*=:.           ".to_string(),
            "              .              .::,.                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "も" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                     :+++,                        ".to_string(),
            "                     *@@#                         ".to_string(),
            "             ''''''',%%@;'''''                    ".to_string(),
            "             +******%@@%******.                   ".to_string(),
            "                    #@%,                          ".to_string(),
            "           :;;;;;;;-@@@-;;;;;;'                   ".to_string(),
            "           ;-------%@@+-------,   '*#*.           ".to_string(),
            "                   #%%'            *@@=           ".to_string(),
            "                  .%@@:            *@@=           ".to_string(),
            "                   ,+%@#=;::,:::-=%@#-            ".to_string(),
            "                     .,-++******+=;'              ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "や" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                           ';;;.                  ".to_string(),
            "               +#%=        ;@@*                   ".to_string(),
            "               :@@#      ',*@@*-====-;'.          ".to_string(),
            "               .#%@=-+*##**=-;;;::;;=*@%+:        ".to_string(),
            "        ':;=+*###%%@*:,'.             .*@@;       ".to_string(),
            "        '+*=-:,. ,%%#'      '          *@@-       ".to_string(),
            "                  ,%@%:    =%%#*+===+*%%*;        ".to_string(),
            "                   ,%@%;    .',:;---;:'.          ".to_string(),
            "                    '#@@=                         ".to_string(),
            "                     .+@@#,                       ".to_string(),
            "                       :,'.                       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ゆ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                        ,**+                      ".to_string(),
            "         ::,'           ,@@%...                   ".to_string(),
            "        ,@@@:     .,-=+**%%%#*****+-,             ".to_string(),
            "        *%%-   ,=##*=;,'.#@@:   .,=%@#;           ".to_string(),
            "       '@@%.'=%%+:.      +@@;      .*@@;          ".to_string(),
            "       :%%%+@#;    '     +@@;       =%@+          ".to_string(),
            "       :@%@@;    '*@#;.  +%%:      :%@%'          ".to_string(),
            "       .%%%=      .:+%%*+%@@-;;-=*%%*-.           ".to_string(),
            "        +@@*          .-@@@=----;:'.              ".to_string(),
            "        .:,'         ,+%@*,                       ".to_string(),
            "                    '=++:                         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "よ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      :::,                        ".to_string(),
            "                      #@@-                        ".to_string(),
            "                      -%%=                        ".to_string(),
            "                      :@%%+++*******#-            ".to_string(),
            "                      '@%@;,,,,,'''..             ".to_string(),
            "                       #@%'                       ".to_string(),
            "              .,,:::::,*%%;                       ".to_string(),
            "           :+##*+======%%@%##*+=-:,.              ".to_string(),
            "          ;@@+        ,%@@,.',;-+*#%%#=.          ".to_string(),
            "          .-#%*+====+#@#=,         .,;,           ".to_string(),
            "             .,:;;;::,.                           ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ら" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 '-;:,'.                          ".to_string(),
            "                 :-=+*####*+=,                    ".to_string(),
            "                        ',:--.                    ".to_string(),
            "               **+;                               ".to_string(),
            "              ,@@@'                               ".to_string(),
            "              =%%+   .,:-==+++++=-:'              ".to_string(),
            "              %%%+-*##*=-;:,,,,:;+%@#-            ".to_string(),
            "             :@@@#+:'              *@@-           ".to_string(),
            "             .'''                '-%@#'           ".to_string(),
            "                    :=======++**#%#+:             ".to_string(),
            "                    .-------;;:,'.                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "り" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               .:::'           ,===.              ".to_string(),
            "               -@@%.           ,@@@.              ".to_string(),
            "               +%%-            .%%%'              ".to_string(),
            "               *@@;            .%@@,              ".to_string(),
            "               *%@;    ',       #@%,              ".to_string(),
            "               +@@= '-#@#;      %%@,              ".to_string(),
            "               ,#%%#%#=,       ;%@#               ".to_string(),
            "                 .'.         .=@@*'               ".to_string(),
            "                          .:+@@*;                 ".to_string(),
            "                      ,-+#@%*-'                   ".to_string(),
            "                      ';=;'                       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "る" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                ',,,,,,::::::;=-,                 ".to_string(),
            "                :#*****+==*@@@%+;.                ".to_string(),
            "                        :+%#=,                    ".to_string(),
            "                    .:+%#=,                       ".to_string(),
            "                 .:+%@@*=--====--:,.              ".to_string(),
            "              .;*@@%+-::::::,::;-+%@*;.           ".to_string(),
            "           ,-#@@*;.                ;@@#'          ".to_string(),
            "          .;*+:.  :+**++*#+:        *%@-          ".to_string(),
            "                 ;@@-    '*@#'    .-@@*.          ".to_string(),
            "                  ;*%*-;:,;@@@--+*%#=:            ".to_string(),
            "                    .:;-=======-;,.               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "れ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                =++'                              ".to_string(),
            "               .@@@                               ".to_string(),
            "               .%%%,.      ',::,'                 ".to_string(),
            "        ,;-=+***%%%#:.';+###**#@@*'               ".to_string(),
            "        ,+=-:,..%@%=+#%*-,.    #%@:               ".to_string(),
            "              '+%%%#=,        .%@%'               ".to_string(),
            "           .;#@%%@%.          ,@@#                ".to_string(),
            "        .:+@@*:.%@%.          -%%+      ;-,       ".to_string(),
            "       -%@#-.  .%%%.          =@@*''',-#@%-       ".to_string(),
            "       .,,     .@@@.           ;+#####*=,         ".to_string(),
            "                =+=                               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ろ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      ..'''',,.                   ".to_string(),
            "              .+######**+#%@@@+'                  ".to_string(),
            "               .'''.   '-%@#-'                    ".to_string(),
            "                     ,+%%='                       ".to_string(),
            "                  ,=%@@+;:;------;,.              ".to_string(),
            "               ,=%@@%*=---;;::;;-=*%#=,           ".to_string(),
            "           .;+%@#=:.               '#@@;          ".to_string(),
            "           .;*='                    +%@=          ".to_string(),
            "                                .,;#@%=           ".to_string(),
            "                    ,=+++++***####*=:.            ".to_string(),
            "                     ,:::::,,''.                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "わ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               .*#*'                              ".to_string(),
            "                @@%                               ".to_string(),
            "             ..,%%%-:'                            ".to_string(),
            "        +**##***%@%#-'.':-=+*********=:.          ".to_string(),
            "        ',''.   %%%-=*##+=;,'.....',-#@%+'        ".to_string(),
            "              :#@%%#=:.               -@@%'       ".to_string(),
            "           ,=%@#%@%                   :%%@,       ".to_string(),
            "       .,=%@%='.%@%.                '=@@%-        ".to_string(),
            "       ,+%+,   .%%%.          .,;=*%@%+;.         ".to_string(),
            "               .@@@.        :#%##+-:'             ".to_string(),
            "               .*#*.         ..                   ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "を" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                       ;*+;                       ".to_string(),
            "                      ;@@%,.''',,,::.             ".to_string(),
            "           :*******+*#@@#*+****+++++,             ".to_string(),
            "            ''..  '-%@*:                          ".to_string(),
            "               .:+@@@%++**+-,      .',:;.         ".to_string(),
            "           ,-+#@%*-:'. ..:%@@-=+**####**;         ".to_string(),
            "           ,-+;'    .:-=+*#%%#-:,'.               ".to_string(),
            "                .;+%%*=;,.=@@-                    ".to_string(),
            "               =%@#,      ,--,                    ".to_string(),
            "               +@@#;,''''............             ".to_string(),
            "                ';=*****************=             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ん" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      .'.                         ".to_string(),
            "                     :%@%=,                       ".to_string(),
            "                   '*@@+,                         ".to_string(),
            "                 .=@@+,                           ".to_string(),
            "                ;#@+'                             ".to_string(),
            "              ,*@@#==+**=;.           ,-:,        ".to_string(),
            "            ,+@@#=;:,,:*@@#          '%@@+        ".to_string(),
            "          '+@@+,        %%@;        ,%@@=         ".to_string(),
            "        ,+@@*,          ;@@%.     ,+@@#:          ".to_string(),
            "        :-=-             -#@%***#%%#=:            ".to_string(),
            "                           ',:::,'                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        // Default: 显示原字符在方框中
        _ => vec![
            "  ╔═══════════════╗   ".to_string(),
            "  ║               ║   ".to_string(),
            "  ║               ║   ".to_string(),
            format!("  ║       {}       ║   ", character),
            "  ║               ║   ".to_string(),
            "  ║               ║   ".to_string(),
            "  ╚═══════════════╝   ".to_string(),
            "                      ".to_string(),
            "                      ".to_string(),
        ],
    }
}

// 获取片假名 ASCII art
fn get_katakana_ascii_art(character: &str) -> Vec<String> {
    match character {
        "ア" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                      .           ".to_string(),
            "         =***####################*##%%@+,         ".to_string(),
            "         '::,,,,,,,,'',,,,''.. .,=%@@*-'          ".to_string(),
            "                      ,###,  '-*@@*;'             ".to_string(),
            "                      :@@@;=%@%+:.                ".to_string(),
            "                      ;%%@. ''                    ".to_string(),
            "                     ,%@@;                        ".to_string(),
            "                   ,+@@#:                         ".to_string(),
            "              .':+%@@+:                           ".to_string(),
            "              ;#%#=,.                             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "イ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                 :,.              ".to_string(),
            "                              '-%@@+:             ".to_string(),
            "                           ,-#@@*;.               ".to_string(),
            "                       ';+#@%+:.                  ".to_string(),
            "                  ':-*%@@@@%                      ".to_string(),
            "            ',;+#%@%#=;'=%%#                      ".to_string(),
            "         ,+%@%#+-:'     =@@#                      ".to_string(),
            "           ''           =@@#                      ".to_string(),
            "                        =%%#                      ".to_string(),
            "                        +@@%                      ".to_string(),
            "                        ,;;;                      ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ウ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      .+++;                       ".to_string(),
            "                      .@@@:                       ".to_string(),
            "          ':::;;;;;;;;-@@@+;;;;;;;;;::,           ".to_string(),
            "          ;@@@*++++++++===++++++++++@@@#          ".to_string(),
            "          ,%%%'                    '%%@+          ".to_string(),
            "          '@@@;                   '#%@#.          ".to_string(),
            "           :::'                  ;%@@+.           ".to_string(),
            "                              '-#@@*:             ".to_string(),
            "                          ':=#@@#-'               ".to_string(),
            "                     ,-=*%@@#+:.                  ".to_string(),
            "                     ';+=;,.                      ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "エ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            ,::::::::::,,,:::::::::::'            ".to_string(),
            "            =**********@@@%**********:            ".to_string(),
            "                       *%%+                       ".to_string(),
            "                       #@@+                       ".to_string(),
            "                       #@@+                       ".to_string(),
            "                       *%%+                       ".to_string(),
            "         :+++++++++++++@@@%+++++++++++++;         ".to_string(),
            "         ';;;;;;;;;;;;;::::;;;;;;;;;;;;;,         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "オ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                        .===:                     ".to_string(),
            "                        '@@@;                     ".to_string(),
            "         ...............,%%%;............         ".to_string(),
            "        =############*#%%@%@%############;        ".to_string(),
            "               ..   .;#@@%%@; .                   ".to_string(),
            "                  '-%@%-:%%@;                     ".to_string(),
            "              .,=%@@+:  .@%@;                     ".to_string(),
            "          .,-*@@%+:.    '@%@;                     ".to_string(),
            "       .;*@@%*-'        .%%%;                     ".to_string(),
            "         ':'        ;;--*@@@:                     ".to_string(),
            "                    ;+++==-,                      ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "カ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                     ';;;'                        ".to_string(),
            "                     ;@@@'                        ".to_string(),
            "                     ;%%%.                        ".to_string(),
            "          =*+********#@%@#********###;            ".to_string(),
            "          ',,,,''''''*%%#''''....'@@@-            ".to_string(),
            "                    ,%@@,        .%%%;            ".to_string(),
            "                   -@@%:         .%%%;            ".to_string(),
            "                .-%@%=.          ,%%@,            ".to_string(),
            "             ';*@@#-'     .    .,#@@+             ".to_string(),
            "          '=%@%*-'        *%###%%#*;              ".to_string(),
            "           ':,            .''''..                 ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "キ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                    ;-=;                          ".to_string(),
            "                    ;@@%.                         ".to_string(),
            "            ...''',,,%@%*;-==+++***#;             ".to_string(),
            "           -####*****+#@@@--;;::,,,'.             ".to_string(),
            "            .          #%%:                       ".to_string(),
            "                   ..'.-@%@::;;--===+++***,       ".to_string(),
            "        :++****#######***@%@*+==---;;:::,,.       ".to_string(),
            "        .::,,'''...      +@%+                     ".to_string(),
            "                         .#%@-                    ".to_string(),
            "                          ,@@@;                   ".to_string(),
            "                           :=-;                   ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ク" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   ,=;;.                          ".to_string(),
            "                  :%@@-                           ".to_string(),
            "                .=@@@%+**********##-:             ".to_string(),
            "              '=@@#-,,,,,,,,,:,,#@@@;             ".to_string(),
            "           ';*@@*:             -%%@;              ".to_string(),
            "         .-#@#-'             ,*@@%:               ".to_string(),
            "            .              ,+@@%=.                ".to_string(),
            "                        ,-#@@#-.                  ".to_string(),
            "                   .,-*%@@*-'                     ".to_string(),
            "              ';=*%@@%*-,.                        ".to_string(),
            "               ,-=;,.                             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ケ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                 ;;:,.                            ".to_string(),
            "                -@@@-                             ".to_string(),
            "               -@%#:                              ".to_string(),
            "             ,*@@@%*########****#######*#;        ".to_string(),
            "           ,+@@*:'','''''','+@@@;',''''''.        ".to_string(),
            "         ;#@%+,            '#%@=                  ".to_string(),
            "         .',.             ;%@@-                   ".to_string(),
            "                        :*@@*,                    ".to_string(),
            "                     ,=%@%+,                      ".to_string(),
            "                 .;+%@@+:.                        ".to_string(),
            "                  ':;,.                           ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "コ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            ........................              ".to_string(),
            "           =#####################%%%-             ".to_string(),
            "                                .%@@-             ".to_string(),
            "                                 %%%-             ".to_string(),
            "                                .%%@-             ".to_string(),
            "                                .%%%-             ".to_string(),
            "           =#####################@@@=             ".to_string(),
            "           .'''''''''''''''''''''===:             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "サ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   .         .:::,                ".to_string(),
            "               .#%%-         :@@@:                ".to_string(),
            "                #@@:         ,%%%'                ".to_string(),
            "       :========%%%*=========+@%@+========,       ".to_string(),
            "       :--------%%%*---------=@%%*--------,       ".to_string(),
            "                *@@-         .@%@:                ".to_string(),
            "                =##-         -%@%.                ".to_string(),
            "                           '+@@*'                 ".to_string(),
            "                       .:=#@%+:                   ".to_string(),
            "                   :+#%%%*-,                      ".to_string(),
            "                    ,:,.                          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "シ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                ,;,.                              ".to_string(),
            "               '+%@@%*-:'                         ".to_string(),
            "                  .,-+%*,                         ".to_string(),
            "           ,=+;,.                      ,.         ".to_string(),
            "           ;=*%@@%*-'               .;#@@=        ".to_string(),
            "               ':=+:             .:+@@%=,         ".to_string(),
            "                             .,=#@@%+:.           ".to_string(),
            "                        ':-+%@@%+;'               ".to_string(),
            "              .,,:;=+*%%@%#+-,.                   ".to_string(),
            "              '+@@#*+-;,.                         ".to_string(),
            "                ..                                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ス" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            ::;;;;;------====-==-,.               ".to_string(),
            "            +****+++++====--=@@@@='               ".to_string(),
            "                           :#@%-.                 ".to_string(),
            "                        ,=%@*;.                   ".to_string(),
            "                     ,-#@@@%;.                    ".to_string(),
            "                 .:+%@%+:,-*@@%+;'                ".to_string(),
            "             ';+%@@#-,      .,=#@@%+:.            ".to_string(),
            "        .:=*%@%*-,.             .;+%@@#-.         ".to_string(),
            "        .:=+;'                      ,=-,          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "セ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   ...                            ".to_string(),
            "                  :%%%,                           ".to_string(),
            "                  :@%@.                           ".to_string(),
            "                  ;%%%:',::;--==+++**%*;.         ".to_string(),
            "        :-=++**###%@%@##**++==-;:;*@@%=,          ".to_string(),
            "        '+==-;::,'-@%@'        .-%@*;.            ".to_string(),
            "                  :@%@'      :#@@+,               ".to_string(),
            "                  :%%%.      .':.                 ".to_string(),
            "                  :@@@:                           ".to_string(),
            "                   -*%%###############;           ".to_string(),
            "                      ................            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ソ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            ,=**:                 .**+=.          ".to_string(),
            "            .:*@@#;               =@@@;           ".to_string(),
            "               :#@@#,            :@%@+            ".to_string(),
            "                 :'.            :%%@+             ".to_string(),
            "                              '=@@%;              ".to_string(),
            "                           .:*@@#-.               ".to_string(),
            "                       .:=#@@%=,                  ".to_string(),
            "                 .,;=*%@%#=:'                     ".to_string(),
            "                 ,=#*=;'                          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "タ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                   .=-;:.                         ".to_string(),
            "                  '#@@*'                          ".to_string(),
            "                .-@@@@***********#*-              ".to_string(),
            "              .-%@%-'',,,,,,,,,*@@%,              ".to_string(),
            "           .:+@@#;. ,,.       :%@%,               ".to_string(),
            "          :*@%=,   :+#%%%*=:,=@%*.                ".to_string(),
            "            .         .':=%@@@@%;'.               ".to_string(),
            "                        ,+%@#-=*@%-               ".to_string(),
            "                   .:-#@@%=,    ..                ".to_string(),
            "             .,;=*%@@#+;'                         ".to_string(),
            "              ,;+=;,.                             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "チ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                 .'               ".to_string(),
            "            :::;;;;;---====+**#####+'             ".to_string(),
            "            ,+++++====--*@@@;,,'..  .             ".to_string(),
            "                        :%%%                      ".to_string(),
            "        .*************###@%%#***********+         ".to_string(),
            "         ,,,,,,,,,,,,,,,=@%@:,,,,,,,,,,,,         ".to_string(),
            "                        *%@+                      ".to_string(),
            "                      ;%@@=                       ".to_string(),
            "                  .:+%@#='                        ".to_string(),
            "                :+%%*-,                           ".to_string(),
            "                  .                               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ツ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            .'      :+#*;           :=-;,         ".to_string(),
            "          '*@@*,    .;#@@%;        .%@@@'         ".to_string(),
            "           .-%@@*'     -##*:       +%%@:          ".to_string(),
            "             '**+;                =@@@;           ".to_string(),
            "                                ,*@@#,            ".to_string(),
            "                             .:*@@#-              ".to_string(),
            "                         ':=*@@%=,                ".to_string(),
            "                 ',:;=*#%@%#+;'                   ".to_string(),
            "                 :+%#*=;:'                        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "テ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "              .,',,,,,,,,,,,,,,,,',               ".to_string(),
            "              :####################,              ".to_string(),
            "                                                  ".to_string(),
            "         ::::::::::::::::::::::::::::::::         ".to_string(),
            "        .+*+************#@@@**********+*+         ".to_string(),
            "                        ;%%%                      ".to_string(),
            "                        *%@*                      ".to_string(),
            "                      .+@@*.                      ".to_string(),
            "                    ,=@@#-                        ".to_string(),
            "                  ,+%@+:.                         ".to_string(),
            "                    ..                            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ト" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                    ''''                          ".to_string(),
            "                    #@@*                          ".to_string(),
            "                    *%%+                          ".to_string(),
            "                    *@@+                          ".to_string(),
            "                    *@%%#*+-;:'.                  ".to_string(),
            "                    *@%*:-=+#%%%%#*=;:'           ".to_string(),
            "                    *@@+       ',;=*##:           ".to_string(),
            "                    *@@+                          ".to_string(),
            "                    *%%+                          ".to_string(),
            "                    #@@*                          ".to_string(),
            "                    .''.                          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ナ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                       :;;:                       ".to_string(),
            "                       #@@*                       ".to_string(),
            "                       *%%+                       ".to_string(),
            "       .+++++++++++++++%@@#++++++++++++++'        ".to_string(),
            "       .;;;;;;;;;;;;;;;*@@%;;;;;;;;;;;;;;.        ".to_string(),
            "                       +@%*                       ".to_string(),
            "                      ,%%@;                       ".to_string(),
            "                     ;%@@-                        ".to_string(),
            "                 .,=%@%+'                         ".to_string(),
            "              ,=#@@%+:.                           ".to_string(),
            "              .,;:'                               ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ニ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "             ',,,,,,,,,,,,,,,,,,,,,,              ".to_string(),
            "             +**********************.             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "         .'''''''''''''''''''''''''''''''.        ".to_string(),
            "         +###############################;        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヌ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            :---------=============:.             ".to_string(),
            "            ;====---------;;;;;#@@@='             ".to_string(),
            "                 ..          .=%@*'               ".to_string(),
            "                :*%%#+-,.  '=@@#:                 ".to_string(),
            "                  .';=*#%**@@*,                   ".to_string(),
            "                     .;#@@##@%*=:.                ".to_string(),
            "                ':=*%@#=:.  ';+%@@*-,.            ".to_string(),
            "         .:-=*#%%#+-,.          ';*@#;            ".to_string(),
            "          ,=+-:'.                                 ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ネ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                      '---'                       ".to_string(),
            "                      ,@@@'                       ".to_string(),
            "           .;;;;;;;;--+@@@+-----=+;.              ".to_string(),
            "           .++++++=====-;;;-*@@@@*-'              ".to_string(),
            "                        .:=##*-'                  ".to_string(),
            "                   ':=*%@@+:. '-=;,.              ".to_string(),
            "            .,:-+#%%#++@%%'   :=*%@@#+;'.         ".to_string(),
            "         ;*%%%#+-:'   ,%%@,       ':+#@@=.        ".to_string(),
            "          ',.         :%%%,           ..          ".to_string(),
            "                      :@@@,                       ".to_string(),
            "                      '---.                       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ノ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                             -+-;,                ".to_string(),
            "                            =@@@#,                ".to_string(),
            "                          '*@@%-                  ".to_string(),
            "                        .-%@@*,                   ".to_string(),
            "                      .-%@@*:                     ".to_string(),
            "                   .;*@@%=,                       ".to_string(),
            "               .:=#@@%+:.                         ".to_string(),
            "          .:-+#@@@*-,                             ".to_string(),
            "          .:-*+;'                                 ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ハ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                               .                  ".to_string(),
            "                -;,'         ;#@#-.               ".to_string(),
            "               +@@@-.        .:+@@%='             ".to_string(),
            "             .*@@%;             ,+@@%+'           ".to_string(),
            "            ;%@@*'                :#@@%-          ".to_string(),
            "          :*@@#;                    =@%@#'        ".to_string(),
            "       '-%@@#;.                      ;%@@@:       ".to_string(),
            "       .:-=:                          ,;,''       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヒ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "              +**=                                ".to_string(),
            "              %@@+                .'              ".to_string(),
            "              #%%+..'',,:;--=+*##%%%#'            ".to_string(),
            "              #@%%%%%###**++=-;:,,'...            ".to_string(),
            "              #@@+..                              ".to_string(),
            "              #@@=                                ".to_string(),
            "              #%%=                                ".to_string(),
            "              #@@#:,,,,,,,,,,,,,,,,,,'            ".to_string(),
            "              '-=********************:            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "フ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           '=========================+;.          ".to_string(),
            "           .-----------------------@@@@'          ".to_string(),
            "                                  '%%@+           ".to_string(),
            "                                 :%@@+            ".to_string(),
            "                               ,+@@%;             ".to_string(),
            "                            ,-#@@*;.              ".to_string(),
            "                      .,;=#@@@*-'                 ".to_string(),
            "               ,-=+*#%@@%#+;'                     ".to_string(),
            "               .;++=;:'.                          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヘ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                    .''.                          ".to_string(),
            "                 '-#@@@@#=:.                      ".to_string(),
            "             .:=%@@*:',;*@@@*-,                   ".to_string(),
            "          ';*@@@*:.       ,=#@@%*;'               ".to_string(),
            "        '+@@@*;.             .:+%@@%+;'           ".to_string(),
            "          ,:.                    '-*@@@%+:        ".to_string(),
            "                                    .,=+;'        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ホ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                       -==;                       ".to_string(),
            "                       @@@=                       ".to_string(),
            "                      .%%%-                       ".to_string(),
            "        .##############%@@%#############*         ".to_string(),
            "         .......  ....'%%@=..... .. .....         ".to_string(),
            "             .=*+;.    %%@-    .-**;.             ".to_string(),
            "            ;%@@+'    .%%@-    .:+%@@*;.          ".to_string(),
            "         .;#@@*,      .%%@-        :*@@%=,        ".to_string(),
            "        .+%@*,        .%%%-          ,+#*-.       ".to_string(),
            "           .          .@@@=                       ".to_string(),
            "                       -==,                       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "マ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "         '''''''''''',,,,,,,,,,::,,,,:,           ".to_string(),
            "         +%%%%%%%%%##########***+#@@@@%-.         ".to_string(),
            "          ..                   '-%@%+:.           ".to_string(),
            "                  '         .;#@@#;'              ".to_string(),
            "                ,*@@#=:. .;*@@#-'                 ".to_string(),
            "                  ':+%@@#%@%-.                    ".to_string(),
            "                      ';+%@@+;'                   ".to_string(),
            "                          ,=%@@+'                 ".to_string(),
            "                             ,'                   ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ミ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               ;%%%###**+==-;:,'..                ".to_string(),
            "               ',,::;;-=++*##%%@@*                ".to_string(),
            "                               .'.                ".to_string(),
            "                +###**+=-;:,'.                    ".to_string(),
            "                ,::;-==+*##%%@%%#*,               ".to_string(),
            "                             .':--                ".to_string(),
            "              ;%###*++=-;:,'.                     ".to_string(),
            "              ,::;-=+**#%%@@@@%#*+-;.             ".to_string(),
            "                          .',:;=+*%#.             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ム" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                       .                          ".to_string(),
            "                      -%%#:                       ".to_string(),
            "                     ;@%@;                        ".to_string(),
            "                    ;@%@;                         ".to_string(),
            "                   -@@%:                          ".to_string(),
            "                  +@@#'       ,;-,                ".to_string(),
            "                '#@@=         ;*@@#-'             ".to_string(),
            "               ;%@#:            ,=%@@*:           ".to_string(),
            "        -++***#@@@%*#############*###@@#-'        ".to_string(),
            "        '*=---;;::::::,,,''''.....   ,*%#=.       ".to_string(),
            "                                       .          ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "メ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                             -%#*:                ".to_string(),
            "                            :@%@=                 ".to_string(),
            "                 ,=;,'     ;@@@-                  ".to_string(),
            "                .-*#%@%#+;=@%#,                   ".to_string(),
            "                     .,+@@@@@#-:.                 ".to_string(),
            "                     ,+%@#-,;+%@@%-               ".to_string(),
            "                 ,-*@@%=,      ';;.               ".to_string(),
            "           ',;=#@@%*-'                            ".to_string(),
            "          .;*%#+;'                                ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "モ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           '=========----===========:             ".to_string(),
            "           '---------%@@%-----------,             ".to_string(),
            "                     +%%*                         ".to_string(),
            "        .;;;;;;;;;;;;#@@#;;;;;;;;;;;;;;;'         ".to_string(),
            "        '++++++++++++%@@%+++++++++++++++:         ".to_string(),
            "                     +@@*                         ".to_string(),
            "                     *@%*                         ".to_string(),
            "                     -@@@*++++++++++++-           ".to_string(),
            "                      '::;;;;;;;;;;;;;:           ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヤ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                ,,::                              ".to_string(),
            "                ;@@@=                             ".to_string(),
            "                 '#%@+     .'',:;-=++*##+;.       ".to_string(),
            "              .',,=@@@*+*####**++-;;*@@@#:.       ".to_string(),
            "        -**######*+=*@@@;..       :*@@+,          ".to_string(),
            "        ';:,'..      +@%#'     .-%@%='            ".to_string(),
            "                      +@@%,     ',:.              ".to_string(),
            "                       =@%@:                      ".to_string(),
            "                        -@%@-                     ".to_string(),
            "                         ;@@@=                    ".to_string(),
            "                          ,:''                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ユ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "             ...................                  ".to_string(),
            "             =##############%%%%,                 ".to_string(),
            "                            ;@%@'                 ".to_string(),
            "                            -%%%.                 ".to_string(),
            "                            +@@%.                 ".to_string(),
            "                            *%%#                  ".to_string(),
            "        ++++++++++++++++++++@@@@+++++++++-        ".to_string(),
            "        :;;;;;;;;;;;;;;;;;;;::::;;;;;;;;;,        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヨ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "             +*******************+++,             ".to_string(),
            "             ,::::::::::::::::::-@@@;             ".to_string(),
            "                                 %%%:             ".to_string(),
            "              ,-;--------------;=@%@:             ".to_string(),
            "              ;++++++++++++++++++@%@:             ".to_string(),
            "                                .@%@:             ".to_string(),
            "                                ,%%%:             ".to_string(),
            "             +###################@@@;             ".to_string(),
            "             ...................,+++,             ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ラ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "             '::::::::::::::::::::,               ".to_string(),
            "             ;*********************               ".to_string(),
            "                                                  ".to_string(),
            "          ,+++++++++++++++++++++++=+=;,           ".to_string(),
            "          ';;;;;;;;;;;;;;;;;;;;-;;%@@@:           ".to_string(),
            "                                 ;%@%:            ".to_string(),
            "                               ,*@@*,             ".to_string(),
            "                           .,=#@@+:               ".to_string(),
            "                     .':-+#@@%+:.                 ".to_string(),
            "                 .;*%%%%#+-:.                     ".to_string(),
            "                   .,'.                           ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "リ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                               ::::               ".to_string(),
            "               *##=            %@@*               ".to_string(),
            "               %@@=            #%%+               ".to_string(),
            "               %%%=            #@@+               ".to_string(),
            "               %%%=            #@@+               ".to_string(),
            "               %@@=            #%%+               ".to_string(),
            "               :::'           -%%@:               ".to_string(),
            "                            '+@@%;                ".to_string(),
            "                        .,-#@@#-.                 ".to_string(),
            "                     ,=#@@%*-'                    ".to_string(),
            "                      ':,.                        ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ル" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               ,:::     .###;                     ".to_string(),
            "               +@@%     '@@@;                     ".to_string(),
            "               =%%#     '%%%;                     ".to_string(),
            "               *@%*     '@%@;                     ".to_string(),
            "              :%%@:     '@%@;         '-%%-       ".to_string(),
            "             :%@@=      '%%@;     .,=#@@+;'       ".to_string(),
            "           '=@@%;       '@%%;.,;+#%@#=:.          ".to_string(),
            "        .;*@@#-.        '%@@%%%#+-,.              ".to_string(),
            "        ;+#=,            .,:'.                    ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "レ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "               =**=                               ".to_string(),
            "               #@@*                               ".to_string(),
            "               *%%+                               ".to_string(),
            "               *@@+                   '+=:        ".to_string(),
            "               *@@+                .:*@@#;        ".to_string(),
            "               *@@+             ,-*@@#-'          ".to_string(),
            "               *%%+      .':;+#%@%+;'             ".to_string(),
            "               #@@#-=+*#%%%%#+-:.                 ".to_string(),
            "               :=***+=-;,'.                       ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ロ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "             ;;;;-----------------;;;'            ".to_string(),
            "            .@@@#================+@@@-            ".to_string(),
            "            .%%%-                 %%%;            ".to_string(),
            "            .%%@-                .%%@;            ".to_string(),
            "            .%%@-                .%%@;            ".to_string(),
            "            .%%%-                .%%%;            ".to_string(),
            "            .%@@%#################@@@-            ".to_string(),
            "             *##-'''''''''''''''''===,            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ワ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           =##########################=           ".to_string(),
            "           *@@#''''''''''''''''''';@@@=           ".to_string(),
            "           +%%*                   -%%@,           ".to_string(),
            "           *@@#                  .%%@*            ".to_string(),
            "           ''''                 '#@@#.            ".to_string(),
            "                              '=@@%=.             ".to_string(),
            "                          .,=#@@#-'               ".to_string(),
            "                    ':;=*%@@%+;'                  ".to_string(),
            "                    :+#*=;,.                      ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ヲ" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "           -##########################=.          ".to_string(),
            "           .'.....................:@@@=           ".to_string(),
            "             '''''''''''''''''''''+%%%.           ".to_string(),
            "             *#################*#@@@%,            ".to_string(),
            "                               '+@@*'             ".to_string(),
            "                            .:*@@*;               ".to_string(),
            "                        ':=#@@#=,                 ".to_string(),
            "               .'':;=+#%@@#+;'                    ".to_string(),
            "               '-#%#*=-:'                         ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        "ン" => vec![
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "            .-##=:'                               ".to_string(),
            "            .:-*%@@%+;'                           ".to_string(),
            "                 ';*#='               ;=;,        ".to_string(),
            "                                    :#@@#:        ".to_string(),
            "                                 ,=%@@+:          ".to_string(),
            "                            .,;*%@@*;'            ".to_string(),
            "                    .',:-=*%@@%*=:.               ".to_string(),
            "           .+*###%%%%%%%#*=;:'                    ".to_string(),
            "            ,+;::,,''.                            ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
            "                                                  ".to_string(),
        ],
        // Default
        _ => vec![
            "  ╔═══════════════╗   ".to_string(),
            "  ║               ║   ".to_string(),
            "  ║               ║   ".to_string(),
            format!("  ║       {}       ║   ", character),
            "  ║               ║   ".to_string(),
            "  ║               ║   ".to_string(),
            "  ╚═══════════════╝   ".to_string(),
            "                      ".to_string(),
            "                      ".to_string(),
        ],
    }
}

// OpenAI API 请求和响应结构
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

// 调用 OpenAI compatible API 进行翻译
async fn translate_with_llm(chinese_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 从环境变量获取 API 配置
    let api_key = env::var("OPENAI_API_KEY")
        .map_err(|_| "OPENAI_API_KEY environment variable not set")?;

    // Validate API key is not empty
    if api_key.trim().is_empty() {
        return Err("OPENAI_API_KEY is set but empty".into());
    }

    let api_base = env::var("OPENAI_API_BASE")
        .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

    let model = env::var("OPENAI_MODEL")
        .unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

    let url = format!("{}/chat/completions", api_base);

    let prompt = format!(
        "请将以下中文翻译成日语，并提供以下信息：\n\
        1. 日文汉字（如果有）\n\
        2. 平假名读音\n\
        3. 罗马音\n\
        \n\
        中文：{}\n\
        \n\
        请按照这个格式回复：日文汉字 (平假名/罗马音)\n\
        如果没有汉字，直接用平假名表示。",
        chinese_text
    );

    let request = ChatRequest {
        model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: "你是一个专业的中日翻译助手。".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ],
        temperature: 0.3,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API request failed with status {}: {}", status, error_text).into());
    }

    let chat_response: ChatResponse = response.json().await?;

    if let Some(choice) = chat_response.choices.first() {
        Ok(choice.message.content.trim().to_string())
    } else {
        Err("No response from API".into())
    }
}

fn init_romaji_map() -> HashMap<String, JapaneseChar> {
    let mut map = HashMap::new();

    // 基本五十音
    // あ行
    map.insert("a".to_string(), JapaneseChar {
        romaji: "a".to_string(),
        hiragana: "あ".to_string(),
        katakana: "ア".to_string(),
        examples: vec!["愛(あい/ai) - love".to_string(), "赤(あか/aka) - red".to_string(), "朝(あさ/asa) - morning".to_string()],
    });
    map.insert("i".to_string(), JapaneseChar {
        romaji: "i".to_string(),
        hiragana: "い".to_string(),
        katakana: "イ".to_string(),
        examples: vec!["犬(いぬ/inu) - dog".to_string(), "家(いえ/ie) - house".to_string(), "石(いし/ishi) - stone".to_string()],
    });
    map.insert("u".to_string(), JapaneseChar {
        romaji: "u".to_string(),
        hiragana: "う".to_string(),
        katakana: "ウ".to_string(),
        examples: vec!["馬(うま/uma) - horse".to_string(), "海(うみ/umi) - sea".to_string(), "歌(うた/uta) - song".to_string()],
    });
    map.insert("e".to_string(), JapaneseChar {
        romaji: "e".to_string(),
        hiragana: "え".to_string(),
        katakana: "エ".to_string(),
        examples: vec!["絵(え/e) - picture".to_string(), "駅(えき/eki) - station".to_string(), "円(えん/en) - yen".to_string()],
    });
    map.insert("o".to_string(), JapaneseChar {
        romaji: "o".to_string(),
        hiragana: "お".to_string(),
        katakana: "オ".to_string(),
        examples: vec!["音(おと/oto) - sound".to_string(), "男(おとこ/otoko) - man".to_string(), "女(おんな/onna) - woman".to_string()],
    });

    // か行
    map.insert("ka".to_string(), JapaneseChar {
        romaji: "ka".to_string(),
        hiragana: "か".to_string(),
        katakana: "カ".to_string(),
        examples: vec!["川(かわ/kawa) - river".to_string(), "家族(かぞく/kazoku) - family".to_string(), "鏡(かがみ/kagami) - mirror".to_string()],
    });
    map.insert("ki".to_string(), JapaneseChar {
        romaji: "ki".to_string(),
        hiragana: "き".to_string(),
        katakana: "キ".to_string(),
        examples: vec!["木(き/ki) - tree".to_string(), "黄色(きいろ/kiiro) - yellow".to_string(), "昨日(きのう/kinou) - yesterday".to_string()],
    });
    map.insert("ku".to_string(), JapaneseChar {
        romaji: "ku".to_string(),
        hiragana: "く".to_string(),
        katakana: "ク".to_string(),
        examples: vec!["靴(くつ/kutsu) - shoes".to_string(), "雲(くも/kumo) - cloud".to_string(), "口(くち/kuchi) - mouth".to_string()],
    });
    map.insert("ke".to_string(), JapaneseChar {
        romaji: "ke".to_string(),
        hiragana: "け".to_string(),
        katakana: "ケ".to_string(),
        examples: vec!["毛(け/ke) - hair".to_string(), "消しゴム(けしゴム/keshi gomu) - eraser".to_string(), "景色(けしき/keshiki) - scenery".to_string()],
    });
    map.insert("ko".to_string(), JapaneseChar {
        romaji: "ko".to_string(),
        hiragana: "こ".to_string(),
        katakana: "コ".to_string(),
        examples: vec!["子供(こども/kodomo) - child".to_string(), "心(こころ/kokoro) - heart".to_string(), "声(こえ/koe) - voice".to_string()],
    });

    // さ行
    map.insert("sa".to_string(), JapaneseChar {
        romaji: "sa".to_string(),
        hiragana: "さ".to_string(),
        katakana: "サ".to_string(),
        examples: vec!["桜(さくら/sakura) - cherry blossom".to_string(), "魚(さかな/sakana) - fish".to_string(), "寒い(さむい/samui) - cold".to_string()],
    });
    map.insert("shi".to_string(), JapaneseChar {
        romaji: "shi".to_string(),
        hiragana: "し".to_string(),
        katakana: "シ".to_string(),
        examples: vec!["白(しろ/shiro) - white".to_string(), "塩(しお/shio) - salt".to_string(), "静か(しずか/shizuka) - quiet".to_string()],
    });
    map.insert("su".to_string(), JapaneseChar {
        romaji: "su".to_string(),
        hiragana: "す".to_string(),
        katakana: "ス".to_string(),
        examples: vec!["寿司(すし/sushi) - sushi".to_string(), "好き(すき/suki) - like".to_string(), "砂(すな/suna) - sand".to_string()],
    });
    map.insert("se".to_string(), JapaneseChar {
        romaji: "se".to_string(),
        hiragana: "せ".to_string(),
        katakana: "セ".to_string(),
        examples: vec!["世界(せかい/sekai) - world".to_string(), "先生(せんせい/sensei) - teacher".to_string(), "背(せ/se) - back".to_string()],
    });
    map.insert("so".to_string(), JapaneseChar {
        romaji: "so".to_string(),
        hiragana: "そ".to_string(),
        katakana: "ソ".to_string(),
        examples: vec!["空(そら/sora) - sky".to_string(), "外(そと/soto) - outside".to_string(), "祖父(そふ/sofu) - grandfather".to_string()],
    });

    // た行
    map.insert("ta".to_string(), JapaneseChar {
        romaji: "ta".to_string(),
        hiragana: "た".to_string(),
        katakana: "タ".to_string(),
        examples: vec!["食べる(たべる/taberu) - to eat".to_string(), "誕生日(たんじょうび/tanjoubi) - birthday".to_string(), "高い(たかい/takai) - high/expensive".to_string()],
    });
    map.insert("chi".to_string(), JapaneseChar {
        romaji: "chi".to_string(),
        hiragana: "ち".to_string(),
        katakana: "チ".to_string(),
        examples: vec!["父(ちち/chichi) - father".to_string(), "小さい(ちいさい/chiisai) - small".to_string(), "血(ち/chi) - blood".to_string()],
    });
    map.insert("tsu".to_string(), JapaneseChar {
        romaji: "tsu".to_string(),
        hiragana: "つ".to_string(),
        katakana: "ツ".to_string(),
        examples: vec!["月(つき/tsuki) - moon".to_string(), "机(つくえ/tsukue) - desk".to_string(), "強い(つよい/tsuyoi) - strong".to_string()],
    });
    map.insert("te".to_string(), JapaneseChar {
        romaji: "te".to_string(),
        hiragana: "て".to_string(),
        katakana: "テ".to_string(),
        examples: vec!["手(て/te) - hand".to_string(), "天気(てんき/tenki) - weather".to_string(), "手紙(てがみ/tegami) - letter".to_string()],
    });
    map.insert("to".to_string(), JapaneseChar {
        romaji: "to".to_string(),
        hiragana: "と".to_string(),
        katakana: "ト".to_string(),
        examples: vec!["友達(ともだち/tomodachi) - friend".to_string(), "鳥(とり/tori) - bird".to_string(), "時計(とけい/tokei) - clock".to_string()],
    });

    // な行
    map.insert("na".to_string(), JapaneseChar {
        romaji: "na".to_string(),
        hiragana: "な".to_string(),
        katakana: "ナ".to_string(),
        examples: vec!["名前(なまえ/namae) - name".to_string(), "夏(なつ/natsu) - summer".to_string(), "長い(ながい/nagai) - long".to_string()],
    });
    map.insert("ni".to_string(), JapaneseChar {
        romaji: "ni".to_string(),
        hiragana: "に".to_string(),
        katakana: "ニ".to_string(),
        examples: vec!["日本(にほん/nihon) - Japan".to_string(), "肉(にく/niku) - meat".to_string(), "虹(にじ/niji) - rainbow".to_string()],
    });
    map.insert("nu".to_string(), JapaneseChar {
        romaji: "nu".to_string(),
        hiragana: "ぬ".to_string(),
        katakana: "ヌ".to_string(),
        examples: vec!["布(ぬの/nuno) - cloth".to_string(), "塗る(ぬる/nuru) - to paint".to_string(), "温い(ぬるい/nurui) - lukewarm".to_string()],
    });
    map.insert("ne".to_string(), JapaneseChar {
        romaji: "ne".to_string(),
        hiragana: "ね".to_string(),
        katakana: "ネ".to_string(),
        examples: vec!["猫(ねこ/neko) - cat".to_string(), "眠い(ねむい/nemui) - sleepy".to_string(), "値段(ねだん/nedan) - price".to_string()],
    });
    map.insert("no".to_string(), JapaneseChar {
        romaji: "no".to_string(),
        hiragana: "の".to_string(),
        katakana: "ノ".to_string(),
        examples: vec!["飲む(のむ/nomu) - to drink".to_string(), "野菜(やさい/yasai) - vegetable".to_string(), "喉(のど/nodo) - throat".to_string()],
    });

    // は行
    map.insert("ha".to_string(), JapaneseChar {
        romaji: "ha".to_string(),
        hiragana: "は".to_string(),
        katakana: "ハ".to_string(),
        examples: vec!["花(はな/hana) - flower".to_string(), "春(はる/haru) - spring".to_string(), "母(はは/haha) - mother".to_string()],
    });
    map.insert("hi".to_string(), JapaneseChar {
        romaji: "hi".to_string(),
        hiragana: "ひ".to_string(),
        katakana: "ヒ".to_string(),
        examples: vec!["火(ひ/hi) - fire".to_string(), "人(ひと/hito) - person".to_string(), "低い(ひくい/hikui) - low".to_string()],
    });
    map.insert("fu".to_string(), JapaneseChar {
        romaji: "fu".to_string(),
        hiragana: "ふ".to_string(),
        katakana: "フ".to_string(),
        examples: vec!["冬(ふゆ/fuyu) - winter".to_string(), "船(ふね/fune) - ship".to_string(), "古い(ふるい/furui) - old".to_string()],
    });
    map.insert("he".to_string(), JapaneseChar {
        romaji: "he".to_string(),
        hiragana: "へ".to_string(),
        katakana: "ヘ".to_string(),
        examples: vec!["部屋(へや/heya) - room".to_string(), "蛇(へび/hebi) - snake".to_string(), "減る(へる/heru) - to decrease".to_string()],
    });
    map.insert("ho".to_string(), JapaneseChar {
        romaji: "ho".to_string(),
        hiragana: "ほ".to_string(),
        katakana: "ホ".to_string(),
        examples: vec!["本(ほん/hon) - book".to_string(), "星(ほし/hoshi) - star".to_string(), "欲しい(ほしい/hoshii) - want".to_string()],
    });

    // ま行
    map.insert("ma".to_string(), JapaneseChar {
        romaji: "ma".to_string(),
        hiragana: "ま".to_string(),
        katakana: "マ".to_string(),
        examples: vec!["街(まち/machi) - town".to_string(), "窓(まど/mado) - window".to_string(), "毎日(まいにち/mainichi) - everyday".to_string()],
    });
    map.insert("mi".to_string(), JapaneseChar {
        romaji: "mi".to_string(),
        hiragana: "み".to_string(),
        katakana: "ミ".to_string(),
        examples: vec!["水(みず/mizu) - water".to_string(), "耳(みみ/mimi) - ear".to_string(), "道(みち/michi) - road".to_string()],
    });
    map.insert("mu".to_string(), JapaneseChar {
        romaji: "mu".to_string(),
        hiragana: "む".to_string(),
        katakana: "ム".to_string(),
        examples: vec!["村(むら/mura) - village".to_string(), "紫(むらさき/murasaki) - purple".to_string(), "難しい(むずかしい/muzukashii) - difficult".to_string()],
    });
    map.insert("me".to_string(), JapaneseChar {
        romaji: "me".to_string(),
        hiragana: "め".to_string(),
        katakana: "メ".to_string(),
        examples: vec!["目(め/me) - eye".to_string(), "飯(めし/meshi) - meal".to_string(), "姪(めい/mei) - niece".to_string()],
    });
    map.insert("mo".to_string(), JapaneseChar {
        romaji: "mo".to_string(),
        hiragana: "も".to_string(),
        katakana: "モ".to_string(),
        examples: vec!["森(もり/mori) - forest".to_string(), "文字(もじ/moji) - character".to_string(), "桃(もも/momo) - peach".to_string()],
    });

    // や行
    map.insert("ya".to_string(), JapaneseChar {
        romaji: "ya".to_string(),
        hiragana: "や".to_string(),
        katakana: "ヤ".to_string(),
        examples: vec!["山(やま/yama) - mountain".to_string(), "野菜(やさい/yasai) - vegetable".to_string(), "安い(やすい/yasui) - cheap".to_string()],
    });
    map.insert("yu".to_string(), JapaneseChar {
        romaji: "yu".to_string(),
        hiragana: "ゆ".to_string(),
        katakana: "ユ".to_string(),
        examples: vec!["雪(ゆき/yuki) - snow".to_string(), "夢(ゆめ/yume) - dream".to_string(), "指(ゆび/yubi) - finger".to_string()],
    });
    map.insert("yo".to_string(), JapaneseChar {
        romaji: "yo".to_string(),
        hiragana: "よ".to_string(),
        katakana: "ヨ".to_string(),
        examples: vec!["夜(よる/yoru) - night".to_string(), "四(よん/yon) - four".to_string(), "良い(よい/yoi) - good".to_string()],
    });

    // ら行
    map.insert("ra".to_string(), JapaneseChar {
        romaji: "ra".to_string(),
        hiragana: "ら".to_string(),
        katakana: "ラ".to_string(),
        examples: vec!["来月(らいげつ/raigetsu) - next month".to_string(), "楽(らく/raku) - easy".to_string(), "ラーメン(らーめん/raamen) - ramen".to_string()],
    });
    map.insert("ri".to_string(), JapaneseChar {
        romaji: "ri".to_string(),
        hiragana: "り".to_string(),
        katakana: "リ".to_string(),
        examples: vec!["林(りん/rin) - forest".to_string(), "理由(りゆう/riyuu) - reason".to_string(), "料理(りょうり/ryouri) - cooking".to_string()],
    });
    map.insert("ru".to_string(), JapaneseChar {
        romaji: "ru".to_string(),
        hiragana: "る".to_string(),
        katakana: "ル".to_string(),
        examples: vec!["留守(るす/rusu) - absence".to_string(), "ルール(るーる/ruuru) - rule".to_string(), "昼(ひる/hiru) - noon".to_string()],
    });
    map.insert("re".to_string(), JapaneseChar {
        romaji: "re".to_string(),
        hiragana: "れ".to_string(),
        katakana: "レ".to_string(),
        examples: vec!["例(れい/rei) - example".to_string(), "歴史(れきし/rekishi) - history".to_string(), "冷蔵庫(れいぞうこ/reizouko) - refrigerator".to_string()],
    });
    map.insert("ro".to_string(), JapaneseChar {
        romaji: "ro".to_string(),
        hiragana: "ろ".to_string(),
        katakana: "ロ".to_string(),
        examples: vec!["六(ろく/roku) - six".to_string(), "廊下(ろうか/rouka) - corridor".to_string(), "ロボット(ろぼっと/robotto) - robot".to_string()],
    });

    // わ行
    map.insert("wa".to_string(), JapaneseChar {
        romaji: "wa".to_string(),
        hiragana: "わ".to_string(),
        katakana: "ワ".to_string(),
        examples: vec!["私(わたし/watashi) - I".to_string(), "若い(わかい/wakai) - young".to_string(), "悪い(わるい/warui) - bad".to_string()],
    });
    map.insert("wo".to_string(), JapaneseChar {
        romaji: "wo".to_string(),
        hiragana: "を".to_string(),
        katakana: "ヲ".to_string(),
        examples: vec!["(particle)".to_string(), "本を読む(ほんをよむ) - read a book".to_string(), "水を飲む(みずをのむ) - drink water".to_string()],
    });
    map.insert("n".to_string(), JapaneseChar {
        romaji: "n".to_string(),
        hiragana: "ん".to_string(),
        katakana: "ン".to_string(),
        examples: vec!["本(ほん/hon) - book".to_string(), "天気(てんき/tenki) - weather".to_string(), "簡単(かんたん/kantan) - simple".to_string()],
    });

    map
}

fn init_chinese_map() -> HashMap<String, String> {
    let mut map = HashMap::new();

    // 基本问候语
    map.insert("你好".to_string(), "こんにちは (konnichiwa)".to_string());
    map.insert("早上好".to_string(), "おはよう (ohayou)".to_string());
    map.insert("晚上好".to_string(), "こんばんは (konbanwa)".to_string());
    map.insert("谢谢".to_string(), "ありがとう (arigatou)".to_string());
    map.insert("对不起".to_string(), "すみません (sumimasen)".to_string());
    map.insert("再见".to_string(), "さようなら (sayounara)".to_string());
    map.insert("是".to_string(), "はい (hai)".to_string());
    map.insert("不是".to_string(), "いいえ (iie)".to_string());
    map.insert("请".to_string(), "お願いします (onegaishimasu)".to_string());

    // 基本词汇
    map.insert("爱".to_string(), "愛 (あい/ai)".to_string());
    map.insert("水".to_string(), "水 (みず/mizu)".to_string());
    map.insert("火".to_string(), "火 (ひ/hi)".to_string());
    map.insert("山".to_string(), "山 (やま/yama)".to_string());
    map.insert("天".to_string(), "天 (てん/ten)".to_string());
    map.insert("地".to_string(), "地 (ち/chi)".to_string());
    map.insert("人".to_string(), "人 (ひと/hito)".to_string());
    map.insert("日".to_string(), "日 (ひ/hi)".to_string());
    map.insert("月".to_string(), "月 (つき/tsuki)".to_string());
    map.insert("花".to_string(), "花 (はな/hana)".to_string());
    map.insert("树".to_string(), "木 (き/ki)".to_string());
    map.insert("书".to_string(), "本 (ほん/hon)".to_string());
    map.insert("学习".to_string(), "勉強 (べんきょう/benkyou)".to_string());
    map.insert("朋友".to_string(), "友達 (ともだち/tomodachi)".to_string());
    map.insert("家".to_string(), "家 (いえ/ie)".to_string());
    map.insert("学校".to_string(), "学校 (がっこう/gakkou)".to_string());
    map.insert("老师".to_string(), "先生 (せんせい/sensei)".to_string());
    map.insert("学生".to_string(), "学生 (がくせい/gakusei)".to_string());

    // 数字
    map.insert("一".to_string(), "一 (いち/ichi)".to_string());
    map.insert("二".to_string(), "二 (に/ni)".to_string());
    map.insert("三".to_string(), "三 (さん/san)".to_string());
    map.insert("四".to_string(), "四 (し・よん/shi/yon)".to_string());
    map.insert("五".to_string(), "五 (ご/go)".to_string());
    map.insert("六".to_string(), "六 (ろく/roku)".to_string());
    map.insert("七".to_string(), "七 (しち・なな/shichi/nana)".to_string());
    map.insert("八".to_string(), "八 (はち/hachi)".to_string());
    map.insert("九".to_string(), "九 (きゅう/kyuu)".to_string());
    map.insert("十".to_string(), "十 (じゅう/juu)".to_string());

    // 家庭成员
    map.insert("父亲".to_string(), "父 (ちち/chichi)".to_string());
    map.insert("母亲".to_string(), "母 (はは/haha)".to_string());
    map.insert("哥哥".to_string(), "兄 (あに/ani)".to_string());
    map.insert("姐姐".to_string(), "姉 (あね/ane)".to_string());
    map.insert("弟弟".to_string(), "弟 (おとうと/otouto)".to_string());
    map.insert("妹妹".to_string(), "妹 (いもうと/imouto)".to_string());

    // 颜色
    map.insert("红色".to_string(), "赤 (あか/aka)".to_string());
    map.insert("蓝色".to_string(), "青 (あお/ao)".to_string());
    map.insert("白色".to_string(), "白 (しろ/shiro)".to_string());
    map.insert("黑色".to_string(), "黒 (くろ/kuro)".to_string());
    map.insert("黄色".to_string(), "黄色 (きいろ/kiiro)".to_string());
    map.insert("绿色".to_string(), "緑 (みどり/midori)".to_string());

    // 季节
    map.insert("春天".to_string(), "春 (はる/haru)".to_string());
    map.insert("夏天".to_string(), "夏 (なつ/natsu)".to_string());
    map.insert("秋天".to_string(), "秋 (あき/aki)".to_string());
    map.insert("冬天".to_string(), "冬 (ふゆ/fuyu)".to_string());

    // 星期
    map.insert("星期一".to_string(), "月曜日 (げつようび/getsuyoubi)".to_string());
    map.insert("星期二".to_string(), "火曜日 (かようび/kayoubi)".to_string());
    map.insert("星期三".to_string(), "水曜日 (すいようび/suiyoubi)".to_string());
    map.insert("星期四".to_string(), "木曜日 (もくようび/mokuyoubi)".to_string());
    map.insert("星期五".to_string(), "金曜日 (きんようび/kinyoubi)".to_string());
    map.insert("星期六".to_string(), "土曜日 (どようび/doyoubi)".to_string());
    map.insert("星期日".to_string(), "日曜日 (にちようび/nichiyoubi)".to_string());

    map
}

// Web server function
async fn start_web_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    // Get the executable directory
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent()
        .ok_or("Failed to get executable directory")?;
    
    // Try multiple possible locations for the web directory
    let mut possible_paths = vec![
        exe_dir.join("web"),                          // Same directory as exe
    ];
    
    // Add project root path if available
    if let Some(parent) = exe_dir.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
        possible_paths.push(parent.join("web"));
    }
    
    // Add current working directory
    if let Ok(cwd) = std::env::current_dir() {
        possible_paths.push(cwd.join("web"));
    }
    
    let web_dir = possible_paths.into_iter()
        .find(|p| p.exists())
        .ok_or("Web directory not found. Please run from the project root or ensure 'web/' directory exists.")?;
    
    println!("📂 Serving files from: {}", web_dir.display());
    
    // Build the router
    let app = Router::new()
        .nest_service("/", ServeDir::new(&web_dir));
    
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    
    println!("🚀 Web server started!");
    println!("🌐 Open your browser and visit: http://{}:{}", host, port);
    println!("📝 Press Ctrl+C to stop the server\n");
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

// Handle the lookup functionality (original main logic)
async fn handle_lookup(text: String) {
    let romaji_map = init_romaji_map();
    let chinese_map = init_chinese_map();

    // Helpers for width-aware padding and printing
    const INNER_WIDTH: usize = 85; // width between the vertical borders
    fn pad_right(text: &str, width: usize) -> String {
        let w = UnicodeWidthStr::width(text);
        if w >= width { text.to_string() } else { format!("{}{}", text, " ".repeat(width - w)) }
    }
    fn center_text(text: &str, width: usize) -> String {
        let w = UnicodeWidthStr::width(text);
        if w >= width { text.to_string() } else {
            let left = (width - w) / 2;
            let right = width - w - left;
            format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
        }
    }
    fn emit(text: &str) {
        println!("║{}║", pad_right(text, INNER_WIDTH));
    }
    fn emit_center(text: &str) {
        println!("║{}║", center_text(text, INNER_WIDTH));
    }

    let input = text.to_lowercase();

    // 首先尝试作为罗马音查询
    if let Some(jp_char) = romaji_map.get(&input) {
        let hiragana_art = get_ascii_art(&jp_char.hiragana);
        let katakana_art = get_katakana_ascii_art(&jp_char.katakana);

        println!("\n╔═════════════════════════════════════════════════════════════════════════════════════╗");
        emit_center("JAPANESE KANA FLASHCARD");
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        emit("");
        emit(&format!("   Romaji: {}", jp_char.romaji.to_uppercase()));
        emit("");
        emit(&format!("   平假名: {}        片假名: {}", jp_char.hiragana, jp_char.katakana));
        emit("");
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        emit_center("平假名 (Hiragana) ASCII Art");
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        for line in hiragana_art {
            emit(&format!(" {}", line));
        }
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        emit_center("片假名 (Katakana) ASCII Art");
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        for line in katakana_art {
            emit(&format!(" {}", line));
        }
        println!("╠═════════════════════════════════════════════════════════════════════════════════════╣");
        emit("  Example Words (例词):");
        for (i, example) in jp_char.examples.iter().enumerate() {
            emit(&format!("  {}. {}", i + 1, example));
        }
        println!("╚═════════════════════════════════════════════════════════════════════════════════════╝\n");
    }
    // 否则尝试作为中文查询
    else if let Some(japanese) = chinese_map.get(&text) {
        println!("╔═══════════════════════════════════════════════");
        println!("║ Chinese (中文): {}", text);
        println!("║ Japanese (日文): {}", japanese);
        println!("╚═══════════════════════════════════════════════");
    }
    // 如果本地字典中找不到，尝试使用 LLM 翻译
    else {
        // 检查是否包含中文字符
        let has_chinese = text.chars().any(|c| {
            ('\u{4E00}'..='\u{9FFF}').contains(&c) || // CJK统一汉字
            ('\u{3400}'..='\u{4DBF}').contains(&c)    // CJK扩展A
        });

        if has_chinese {
            println!("╔═══════════════════════════════════════════════");
            println!("║ 🔍 本地字典未找到，正在使用 LLM 翻译...");
            println!("╠═══════════════════════════════════════════════");

            match translate_with_llm(&text).await {
                Ok(translation) => {
                    println!("║ Chinese (中文): {}", text);
                    println!("║ Japanese (日文): {}", translation);
                    println!("║");
                    println!("║ 💡 提示：这是由 AI 生成的翻译");
                    println!("╚═══════════════════════════════════════════════");
                }
                Err(e) => {
                    println!("║ ❌ LLM 翻译失败: {}", e);
                    println!("║");
                    println!("║ 💡 请确保已设置以下环境变量：");
                    println!("║    - OPENAI_API_KEY: 你的 API key");
                    println!("║    - OPENAI_API_BASE: API 地址 (可选)");
                    println!("║    - OPENAI_MODEL: 模型名称 (可选)");
                    println!("╚═══════════════════════════════════════════════");
                }
            }
        } else {
            println!("❌ Sorry, '{}' not found in the database.", text);
            println!("💡 Try:");
            println!("   - Romaji like: a, ka, chi, tsu, etc.");
            println!("   - Chinese words like: 你好, 谢谢, 爱, 水, etc.");
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match (cli.command, cli.text) {
        (Some(Commands::Serve { port, host }), _) => {
            if let Err(e) = start_web_server(host, port).await {
                eprintln!("❌ Error starting web server: {}", e);
                std::process::exit(1);
            }
        }
        (Some(Commands::Lookup { text }), _) => {
            handle_lookup(text).await;
        }
        (None, Some(text)) => {
            handle_lookup(text).await;
        }
        (None, None) => {
            // Default behavior: show help if no command or text provided
            eprintln!("❌ No input provided.");
            eprintln!("\nUsage:");
            eprintln!("  jp <TEXT>                Lookup romaji or translate Chinese");
            eprintln!("  jp serve [OPTIONS]       Start web server");
            eprintln!("  jp lookup <TEXT>         Lookup romaji or translate Chinese");
            eprintln!("\nRun 'jp --help' for more information.");
            std::process::exit(1);
        }
    }
}
