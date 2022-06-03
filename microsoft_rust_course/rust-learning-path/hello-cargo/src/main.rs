// Define a tuple struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct KeyPress(String, char);

// Define a classic struct
// Set the Debug flag so we can check the data in the output
#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

// Define an enum with three variants
// Set the Debug flag so we can check the enum data in the output
#[derive(Debug)]
enum WebEvent {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}

fn main() {
    // Instantiate a MouseClick struct and bind the coordinate values
    let click = MouseClick { x: 100, y: 250 };

    // Print the MouseClick coordinate values
    println!("Mouse click location: {}, {}", click.x, click.y);

    // Instantiate a KeyPress tuple and bind the key values
    let keys = KeyPress(String::from("Ctrl+"), 'N');

    // Print the KeyPress values
    println!("\nKeys pressed: {}{}", keys.0, keys.1);

    // Instantiate the WebEvent enum variants
    // Set the boolean WELoad value to true
    let we_load = WebEvent::WELoad(true);
    // Set the WEClick variant to use the data in the click struct
    let we_click = WebEvent::WEClick(click);
    // Set the WEKeys variant to use the data in the keys tuple
    let we_key = WebEvent::WEKeys(keys);

    // Print the values in the WebEvent enum variants
    // Use the {:#?} syntax to display the enum structure and data in a readable form
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}
