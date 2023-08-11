use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;
use std::error::Error;

fn main()  {
    let filename = String::from("hello.txt");
    // {
    //     let f = File::open(&filename);

    //     let f = match f {
    //         Ok(file) => file,
    //         Err(error) => {
    //             panic!("Problem opening the file: {:?}", error)
    //         },
    //     };
    // }

    // {
    //     let f = File::open(&filename);

    //     let f = match f {
    //         Ok(file) => file,
    //         Err(error) => match error.kind() {
    //             ErrorKind::NotFound => match File::create(&filename) {
    //                 Ok(fc) => fc,
    //                 Err(e) => panic!("Problem creating the file: {:?}", e),
    //             },
    //             other_error => panic!("Problem opening the file: {:?}", other_error),
    //         }
    //     };
    // }

    // {
    //     let f = File::open(&filename).unwrap_or_else(|error| {
    //         if error.kind() == ErrorKind::NotFound {
    //             File::create(&filename).unwrap_or_else(|error| {
    //                 panic!("Problem creating the file: {:?}", error);
    //             })
    //         } else {
    //             panic!("Problem opening the file: {:?}", error);
    //         }
    //     });
    // }

    // {
    //     let f = File::open("hello.txt").unwrap();
    // }

    // {
    //     let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // }

        // {
        //     let f = read_username_from_file();

        //     let f = match f {
        //         Ok(file) => file,
        //         Err(error) => {
        //             panic!("Problem opening the file: {:?}", error)
        //         },
        //     };
        // }

        {
           let f=  read_username_from_file_1();

           let f = match f {
                Ok(file) => file,
                Err(error) => {
                    panic!("Problem opening the file: {:?}", error)
                },
            };
        }

        //  fn main() -> Result<(), Box<dyn Error>>
        // {
        //     let f = File::open("hello.txt")?;

        //     Ok(())
        // }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}
