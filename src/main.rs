use device_query::{DeviceQuery, DeviceState, Keycode};
use ansi_term::{Style, Colour};
use std::fs::File;
use serde_json::Value;
use std::process::Command;
use execute::Execute;
use std::time::Duration;
use std::thread::sleep;

type KeyMap = [bool; 4];

struct Applist
{
    headers: Vec<String>,
    titles: Vec<Vec<String>>,
    addresses: Vec<Vec<String>>,
}

#[derive(Copy, Clone)]
struct Sys
{
    cursor_index: i32,
    max_index: i32,
    key_map: KeyMap,
    sub_screen: usize,
}

//
// ----- Main -----
//
fn main()
{
    // Init decvice_query for keyboard inputs
    let device_state = DeviceState::new();

    // Parse JSON file
    let json: Value = get_json(&String::from("src/apps.json"));
    let mut applist = Applist
    {
        headers: get_headers(&json),
        titles: get_titles(&json),
        addresses: get_addresses(&json),
    };

    let mut sys = Sys
    {
        cursor_index: 0,
        max_index: applist.headers.len() as i32,
        key_map: [false; 4],
        sub_screen: 0,
    };

    // Add extra menu options
    applist.headers.push(String::from("Quit"));
    for i in 0..applist.titles.len()
    {
        applist.titles[i].push(String::from("Back"));
    }

    //
    // Main program loop
    //

    render_menu(&applist, &sys);

    loop
    {
        let key_code: Vec<Keycode> = device_state.get_keys();   // Get a vector of all held keys
        sys.key_map = get_input(key_code);
        
        // Is key pressed
        if sys.key_map != [false; 4]
        {
            sys = parse_input(&mut sys, &applist);
            render_menu(&applist, &sys);
            sleep(Duration::from_millis(500))
        }
    }
}   // main

fn parse_input(sys: &mut Sys, applist: &Applist) -> Sys
{
    // Up arrow pressed
    if sys.key_map[0] == true && sys.cursor_index != 0
    {
        sys.cursor_index -= 1;
        sys.key_map[0] = false
    }

    // Loop to bottom of screen
    if sys.key_map[0] == true && sys.cursor_index == 0
    {
        sys.cursor_index = sys.max_index;
        sys.key_map[0] = false
    }

    // Down arrow pressed
    if sys.key_map[1] == true && sys.cursor_index != sys.max_index
    {
        sys.cursor_index += 1;
        sys.key_map[1] = false
    }

    // Loop to top of screen
    if sys.key_map[1] == true && sys.cursor_index == sys.max_index
    {
        sys.cursor_index = 0;
        sys.key_map[0] = false
    }

    // Quit option selected
    if sys.key_map[2] == true && sys.sub_screen == 0 && sys.cursor_index == sys.max_index
    {
        std::process::exit(0);
    }

    // Back option selected
    if sys.key_map[2] == true && sys.sub_screen != 0 && sys.cursor_index == sys.max_index
    {
        sys.sub_screen = 0;
        sys.cursor_index = 0;
        sys.max_index = applist.headers.len() as i32 - 1;
        sys.key_map[2] = false
    }

    // Enter key pressed on category screen
    if sys.key_map[2] == true && sys.sub_screen == 0
    {
        sys.sub_screen += sys.cursor_index as usize + 1;
        sys.cursor_index = 0;
        sys.max_index = applist.titles[sys.sub_screen-1].len() as i32 - 1;
        sys.key_map[2] = false
    }

    // Enter key pressed on an app screen
    if sys.key_map[2] == true && sys.sub_screen != 0 && sys.cursor_index != sys.max_index
    {
        Command::new(&applist.addresses[sys.sub_screen-1][sys.cursor_index as usize])
            .execute()
            .expect("Error launching program");

        std::process::exit(0)
    }

    // Escape key pressed on category screen
    if sys.key_map[3] == true && sys.sub_screen == 0
    {
        std::process::exit(0)
    }

    // Escape key pressed on an app screen
    if sys.key_map[3] == true && sys.sub_screen != 0
    {
        sys.sub_screen = 0;
        sys.key_map[3] = false
    }

    *sys
}   // parse_input

fn render_menu(applist: &Applist, sys: &Sys)
{
    let keyword: String;
    if sys.sub_screen == 0
    { 
        keyword = String::from("Category list")
    }
    else
    {
        keyword = String::from("App list") 
    }

    print!("{}[2J", 27 as char);

    println!("]--------------->");
    println!(" App Centre");
    println!("]--------------->");
    println!("");
    println!("? {}", keyword);

    // Print current title list with character formatting
    if keyword == String::from("Category list")
    {
        // Output each header with formatting
        for i in 0..applist.headers.len()
        {
            if sys.cursor_index == i as i32
            {
                let header_string: String = Colour::Blue.paint(&applist.headers[i]).to_string();
                println!(">   {}", Style::new().bold().underline().paint(header_string))
            }
            else
            { 
                println!("    {}", applist.headers[i])
            }
        }
    }
    else if keyword == String::from("App list")
    {
        // Output each title with formatting
        for y in 0..applist.titles[sys.sub_screen-1].len()
        {
            if sys.cursor_index == y as i32
            {
                let title_string: String = Colour::Blue.paint(&applist.titles[sys.sub_screen-1][y]).to_string();
                let add_string: String;
                if sys.cursor_index != sys.max_index
                {
                    add_string = Colour::RGB(107, 107, 107)
                                             .paint(&applist.addresses[sys.sub_screen-1][y]).to_string();
                }
                else
                {
                    add_string = String::from("");
                }
                println!(">   {} {}", 
                          Style::new().bold().underline().paint(title_string),
                          Style::new().bold().paint(add_string)
                        )
            }
            else
            {
                println!("    {}", applist.titles[sys.sub_screen-1][y]) 
            }
        }
    }
    
}   // render_menu

fn get_input(key_code: Vec<Keycode>) -> KeyMap
{
    let mut key_map: KeyMap = [false; 4];

    for i in 0..key_code.len()
    {
        match key_code[i]
        {
            Keycode::Up => key_map[0] = true,
            Keycode::Down => key_map[1] = true,
            Keycode::Enter => key_map[2] = true,
            Keycode::Escape => key_map[3] = true,
            _ => key_map = [false; 4],
        }
    }

    key_map
}

// Returns apps.json as a JSON value
fn get_json(json_dir: &String) -> serde_json::Value
{
    let file = File::open(json_dir)
        .expect("Error opening JSON file");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("Error reading opened JSON file");

    json
}   //get_json

// Returns the headings of the categories
fn get_headers(json: &serde_json::Value) -> Vec<String>
{
    let raw_string: String = json.to_string();
    let mut headers: Vec<String> = vec![];
    
    // Iterates through raw_string using
    // criteria to search for headings
    let mut in_bracket: bool = false;
    let mut char_store: Vec<char> = vec![];
    for i in 1..raw_string.chars().count()-1
    {
        if in_bracket   // Need to check if brackets have ended
        {
            if raw_string.chars().nth(i).unwrap() == '}'
            { 
                in_bracket = false 
            }
        }
        else    // Need to check if brackets have started
        {
            if raw_string.chars().nth(i).unwrap() == '{'
            { 
                // The current header has ended
                // Time to add it to the headers vector
                in_bracket = true;
                headers.push(char_store.iter().collect());
                char_store.clear()
            }
            else    // Now we can add the char to our vector
            {
                char_store.push(raw_string.chars().nth(i).unwrap())
            }
        }   // else in_bracket

    }   // for

    // Clear up the headers vector for loose characters
    for i in 0..headers.len()
    { 
        headers[i] = headers[i].replace(['"', '\"', ':', ','], "") 
    }

    headers
}   // get_headers

fn get_titles(json: &Value) -> Vec<Vec<String>>
{
    let headers: Vec<String> = get_headers(&json);
    let mut titles: Vec<Vec<String>> = vec![vec![]; headers.len()];

    for i in 0..headers.len()   // Loop for each category
    {
        let content: String = json.get(&headers[i])
            .and_then(|value| Some(value.to_string())).unwrap();    // Get content as string
        
        let mut in_title: bool = true;
        let mut end_reached: bool = false;
        let mut index: usize = 2;
        let mut char_store: Vec<char> = vec![];
        while end_reached == false
        {
            if in_title     // Push char or end pushing
            {
                if content.chars().nth(index).unwrap() == '"'
                { 
                    in_title = false;
                    titles[i].push(char_store.iter().collect());
                    char_store.clear()
                }
                else
                { 
                    char_store.push(content.chars().nth(index).unwrap()) 
                }
            }
            else    // Start of new title?
            {
                if content.chars().nth(index).unwrap() == ','
                {
                    index += 1;
                    in_title = true
                }
            }

            if content.chars().nth(index).unwrap() == '}'   // End while loop
            { 
                end_reached = true 
            }

            index += 1
        }   // while not end_reached
    }   // for i

    titles
}   // get_titles

fn get_addresses(json: &Value) -> Vec<Vec<String>>
{
    let headers: Vec<String> = get_headers(&json);
    let mut addresses: Vec<Vec<String>> = vec![vec![]; headers.len()];

    for i in 0..headers.len()   // Loop for each category
    {
        let content: String = json.get(&headers[i])
            .and_then(|value| Some(value.to_string())).unwrap();    // Get content as string

        let mut in_add: bool = false;
        let mut end_reached: bool = false;
        let mut index: usize = 2;
        let mut char_store: Vec<char> = vec![];
        while end_reached == false
        {
            if in_add   // Push char or end pushing
            {
                if content.chars().nth(index).unwrap() == '"'
                {
                    in_add = false;
                    addresses[i].push(char_store.iter().collect());
                    char_store.clear()
                }
                else
                { 
                    char_store.push(content.chars().nth(index).unwrap()) 
                }
            }
            else    // Start pushing chars?
            {
                if content.chars().nth(index).unwrap() == ':'
                {
                    index += 1;
                    in_add = true
                }
                else
                {
                    if content.chars().nth(index).unwrap() == '}'
                    { 
                        end_reached = true 
                    }    
                }
            }

            
            if content.chars().nth(index).unwrap() == '}'   // End while loop
            { 
                end_reached = true 
            }

            index += 1
        }   // while not end_reached
    }   // for i

    addresses
}   // fn get_addresses