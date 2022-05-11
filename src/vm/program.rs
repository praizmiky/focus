use std::fs::File;
use std::io::{self, BufReader};

use super::*;
use crate::error::Error;
use crate::reader::Reader;
use crate::value::{Object, ParseletRef, RefValue};

/** Programs are containers holding statics and a pointer to the main parselet.

A program is the result of a successful compiler run. */
#[derive(Debug)]
pub struct Program {
    pub(crate) statics: Vec<RefValue>, // Static values referenced by this program
    main: Option<ParseletRef>,         // The main parselet
}

impl Program {
    pub fn new(statics: Vec<RefValue>) -> Self {
        let mut main = None;

        // Find main parselet by selecting the last parselet defined.
        // todo: allow to specify main parselet.
        for i in (0..statics.len()).rev() {
            if let Some(parselet) = statics[i].borrow().object::<ParseletRef>() {
                main = Some(parselet.clone());
                break;
            }
        }

        Self { statics, main }
    }

    /// Returns a reference to the program's main parselet.
    pub fn main(&self) -> &ParseletRef {
        self.main.as_ref().unwrap()
    }

    pub fn dump(&self) {
        for i in 0..self.statics.len() {
            println!("{} => {:#?}", i, self.statics[i]);
        }
    }

    pub fn run(&self, runtime: &mut Runtime) -> Result<Option<RefValue>, Error> {
        if let Some(main) = &self.main {
            match main
                .0
                .borrow()
                .run(runtime, runtime.stack.len(), None, true, 0)
            {
                Ok(Accept::Push(Capture::Value(value, ..))) => {
                    if value.is_void() {
                        Ok(None)
                    } else {
                        Ok(Some(value.clone()))
                    }
                }
                Ok(_) => Ok(None),
                Err(Reject::Error(error)) => Err(*error),
                Err(other) => Err(Error::new(None, format!("Runtime error {:?}", other))),
            }
        } else {
            Ok(None)
        }
    }

    pub fn run_from_reader(&self, mut reader: Reader) -> Result<Option<RefValue>, Error> {
        let mut runtime = Runtime::new(&self, &mut reader);
        self.run(&mut runtime)
    }

    pub fn run_from_str(&self, src: &'static str) -> Result<Option<RefValue>, Error> {
        self.run_from_reader(Reader::new(Box::new(BufReader::new(std::io::Cursor::new(
            src,
        )))))
    }

    pub fn run_from_string(&self, src: String) -> Result<Option<RefValue>, Error> {
        self.run_from_reader(Reader::new(Box::new(BufReader::new(std::io::Cursor::new(
            src,
        )))))
    }

    pub fn run_from_file(&self, filename: &str) -> Result<Option<RefValue>, Error> {
        if filename == "-" {
            self.run_from_reader(Reader::new(Box::new(BufReader::new(io::stdin()))))
        } else if let Ok(file) = File::open(filename) {
            self.run_from_reader(Reader::new(Box::new(BufReader::new(file))))
        } else {
            Err(Error::new(
                None,
                format!("Unable to read from filename '{}'", filename),
            ))
        }
    }
}
