use std::fs::File;
use std::{io, fs};
use std::io::{ErrorKind, Read};

fn main() {
    {
        // These code comment snippets, when uncommented, cause panics.

        // panic!("crash and burn");

        // let v = vec![1, 2, 3];
        //
        // v[99];
    }

    {
        let f: io::Result<File> = File::open("hello.txt"); // io::Result<_> is a specialization of std::result::Result<_, io::Error>
        let _f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    match File::create("hello.txt") {
                        Ok(created_file) => created_file,
                        Err(error) => panic!("Problem creating the file: {:?}", error)
                    }
                },
                other_error_kind => panic!("Problem opening the file: {:?}", other_error_kind)
            }
        };

        // Use of closures below is equivalent to use of matches above.
        let _f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });

        let _f = File::open("hello.txt").unwrap(); // Throws a generic panic if Err
        let _f = File::open("hello.txt").expect("Failed to open hello.txt"); // Throws a panic with given error message if Err
    }

    {
        fn _read_username_from_file() -> Result<String, io::Error> {
            let file = File::open("hello.txt");

            let mut file = match file {
                Ok(file) => file,
                Err(error) => return Err(error)
            };

            let mut contents = String::new();

            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(error) => Err(error)
            }
        }

        fn _read_username_from_file_2() -> Result<String, io::Error> {
            let file = File::open("hello.txt");

            let mut file = match file {
                Ok(file) => file,
                Err(error) => return Err(error)
            };

            let mut contents = String::new();

            // The ? operator returns the result if its an Err, otherwise everything continues as
            // normal. In other words, it propagates errors.
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }

        fn _read_username_from_file_3() -> Result<String, io::Error> {
            let mut contents = String::new();
            File::open("hello.txt")?.read_to_string(&mut contents)?;
            Ok(contents)
        }

        fn _read_username_from_file_4() -> io::Result<String> {
            fs::read_to_string("hello.txt")
        }
    }

    {
        let _f = File::open("hello.txt").unwrap(); // Shortcut for panicking on Result.Err
    }

    {
        // A "Result.Err into panic" shortcut that allows the panic message to be specified
        let _f = File::open("hello.txt").expect("Failed to open hello.txt");
    }
}
