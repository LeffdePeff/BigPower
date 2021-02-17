extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gmp::mpz::Mpz;

use std::fs;

fn main()
{
    let application = gtk::Application::new(
        Some("com.leffdepeff.big_power"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate( |app| {
        let glade_src = include_str!("../gtk_layout.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let window: gtk::Window = builder.get_object("application_window").unwrap();

        window.set_default_size(350, 200);
        window.set_application(Some(app));
        window.set_title("BigPower");

        let first_number_entry: gtk::Entry = builder.get_object("first_number_entry").unwrap();
        let second_number_entry: gtk::Entry = builder.get_object("second_number_entry").unwrap();
        let calculate_button: gtk::Button = builder.get_object("calculate_button").unwrap();
        let calculate_length: gtk::CheckButton = builder.get_object("calculate_length").unwrap();
        let log_label: gtk::Label = builder.get_object("log_label").unwrap();

        calculate_button.connect_clicked(move |_| {
            let first_number = Mpz::from(first_number_entry.get_text().to_string().parse::<i32>().unwrap());
            let second_number = second_number_entry.get_text().to_string().parse::<u32>().unwrap();
            let calculate_length_bool = calculate_length.get_active();
            let file_name = first_number.to_string() + "^" + second_number.to_string().as_str() + "_answer.txt";
            let file_name_length = first_number.to_string() + "^" + second_number.to_string().as_str() + "_length.txt";

            log_label.set_text("");

            println!("Calculating...");
            let answer = first_number.pow(second_number);

            println!("Converting answer to string...");
            let answer_string = answer.to_string();

            if calculate_length_bool {
                println!("Calculating length...");
                let answer_length = answer_string.len();

                println!("The answer has {} numbers.", answer_length);

                println!("Writing length to {} ...", file_name_length);
                write_to_file(file_name_length, answer_length.to_string());
            }

            println!("Writing answer to {} ...", file_name);
            write_to_file(file_name, answer_string);

            println!("Done!");
            log_label.set_text("Done!");
        });

        window.show_all();
    });

    application.run(&[]);
}

fn write_to_file(file_name: String, content: String)
{
    fs::write(file_name, content).expect("Unable to write file");
}
