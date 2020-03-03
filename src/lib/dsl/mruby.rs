use std::io::{self, Error, ErrorKind};
use std::{env, process};

use mrusty::*;

use crate::lib::term;
use crate::lib::cmd;

pub fn mruby_new() -> Result<mrusty::MrubyType, mrusty::MrubyError> {
    let mruby = mrusty::Mruby::new();
    {
        mruby.def_class("MrubyInitialize")
            .def_const("ARGV", {
                let mut argv: Vec<mrusty::Value> = Vec::new();
                for x in env::args().skip(1) {
                    argv.push(mruby.string(&x));
                }
                mruby.array(argv)
            });

        // global const ARGV
        mruby.run("ARGV = MrubyInitialize::ARGV")?;
    }
    {
        setup_array_cmd_legacy(&mruby);

        // array extension
        mruby.run(r#"
        require 'array_cmd'
        require 'array_cmd_deep'

        class Array
          def to_cmd
            MrubyArrayCmd.new self
          end

          def to_cmd_deep
            MrubyArrayCmdDeep.new self
          end

          def self.from_cmd(cmd_args_string)
            MrubyArrayFromCmd.from_cmd cmd_args_string
          end

          def from_cmd!(cmd_args_string)
            self[0..-1] = MrubyArrayFromCmd.from_cmd cmd_args_string
            self
          end

          def from_cmd(cmd_args_string)
            MrubyArrayFromCmd.from_cmd cmd_args_string
          end
        end
        "#)?;
    }
    {
        setup_stdin_legacy(&mruby);

        // global STDIN object
        mruby.run("STDIN = MrubyStdin.new")?;
    }
    {
        setup_stdout_legacy(&mruby);

        // global STDOUT object
        mruby.run("STDOUT = MrubyStdout.new")?;
    }
    {
        setup_stderr_legacy(&mruby);

        // global STDERR object
        mruby.run("STDERR = MrubyStderr.new")?;
    }

    Ok(mruby)
}

struct ArrayCmd {
    value: mrusty::Value
}
struct ArrayCmdDeep {
    value: mrusty::Value
}

// NOTE: warning: use of deprecated item 'std::mem::uninitialized': use `mem::MaybeUninit` instead
#[allow(deprecated)]
fn setup_array_cmd_legacy(mruby: &mrusty::MrubyType) {
    mrusty_class!(ArrayCmd, "MrubyArrayCmd", {
        def!("initialize", |v: Value| {
            ArrayCmd { value: v }
        });
        def!("value", |_mruby, slf: (&ArrayCmd)| {
            slf.value.clone()
        });
    });
    mruby.def_file::<ArrayCmd>("array_cmd");

    mrusty_class!(ArrayCmdDeep, "MrubyArrayCmdDeep", {
        def!("initialize", |v: Value| {
            ArrayCmdDeep { value: v }
        });
        def!("value", |_mruby, slf: (&ArrayCmdDeep)| {
            slf.value.clone()
        });
    });
    mruby.def_file::<ArrayCmdDeep>("array_cmd_deep");

    mruby_class!(mruby.clone(), "MrubyArrayFromCmd", {
        def_self!("from_cmd", |mruby, _slf: Value, cmd_args_string: Value| {
            let cmd_args_string = cmd_args_string.to_str();
            if let Err(err) = cmd_args_string {
                eprintln!("{}: in Array.from_cmd: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }

            let a = cmd::split_args(cmd_args_string.unwrap()).iter()
                .map(|x| cmdstr2value(&mruby, x))
                .collect();
            mruby.array(a)
        });
    });
}
fn cmdstr2value(mruby: &mrusty::MrubyType, s: &str) -> Value {
    if let Ok(n) = s.parse::<i32>() {
        return mruby.fixnum(n);
    }
    if let Ok(f) = s.parse::<f64>() {
        return mruby.float(f);
    }
    if let Ok(b) = s.parse::<bool>() {
        return mruby.bool(b);
    }

    mruby.string(s)
}

#[allow(deprecated)]
fn setup_stdin_legacy(mruby: &mrusty::MrubyType) {
    mruby_class!(mruby.clone(), "MrubyStdin", {
        def!("gets", |mruby, _slf: Value| {
            let mut s = String::new();
            let size = io::stdin().read_line(&mut s);
            if let Err(err) = size {
                eprintln!("{}: in STDIN.gets: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            if size.unwrap() == 0 {
                return mruby.nil();
            }

            mruby.string(&s)
        });

        def!("readline", |mruby, _slf: Value| {
            let mut s = String::new();
            let size = io::stdin().read_line(&mut s);
            if let Err(err) = size {
                eprintln!("{}: in STDIN.readline: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            if size.unwrap() == 0 {
                eprintln!("{}: in STDIN.readline: end of file reached", term::ewrite("mruby failed").unwrap());
                process::exit(1);
            }

            mruby.string(&s)
        });

        def!("readlines", |mruby, _slf: Value| {
            let mut a = Vec::new();

            let mut s = String::new();
            loop {
                let size = io::stdin().read_line(&mut s);
                if let Err(err) = size {
                    eprintln!("{}: in STDIN.readlines: {}", term::ewrite("mruby failed").unwrap(), err);
                    process::exit(1);
                }
                if size.unwrap() == 0 {
                    break;
                }

                a.push(mruby.string(&s));
                s.clear();
            }

            mruby.array(a)
        });
    });
}

#[allow(deprecated)]
fn setup_stdout_legacy(mruby: &mrusty::MrubyType) {
    mruby_class!(mruby.clone(), "MrubyStdout", {
        def!("puts", |mruby, _slf: Value, value: Value| {
            let s = value2str(&mruby, value);
            if let Err(err) = s {
                eprintln!("{}: in STDOUT.puts: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            println!("{}", s.unwrap());

            mruby.nil()
        });

        def!("print", |mruby, _slf: Value, value: Value| {
            let s = value2str(&mruby, value);
            if let Err(err) = s {
                eprintln!("{}: in STDOUT.print: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            print!("{}", s.unwrap());

            mruby.nil()
        });
    });
}

#[allow(deprecated)]
fn setup_stderr_legacy(mruby: &mrusty::MrubyType) {
    mruby_class!(mruby.clone(), "MrubyStderr", {
        def!("puts", |mruby, _slf: Value, value: Value| {
            let s = value2str(&mruby, value);
            if let Err(err) = s {
                eprintln!("{}: in STDERR.puts: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            eprintln!("{}", s.unwrap());

            mruby.nil()
        });

        def!("print", |mruby, _slf: Value, value: Value| {
            let s = value2str(&mruby, value);
            if let Err(err) = s {
                eprintln!("{}: in STDERR.print: {}", term::ewrite("mruby failed").unwrap(), err);
                process::exit(1);
            }
            eprint!("{}", s.unwrap());

            mruby.nil()
        });
    });
}

enum MrubyValue {
    Str(String),
    I32(i32),
    F64(f64),
    Bool(bool),
    Array(Vec<mrusty::Value>),
    ArrayCmd(Vec<mrusty::Value>),
    ArrayCmdDeep(Vec<mrusty::Value>),
}

pub fn value2str(mruby: &mrusty::MrubyType, value: mrusty::Value) -> io::Result<String> {
    match parse_mruby_value(value)? {
        MrubyValue::Str(s) => Ok(format!(r#""{}""#, s)),
        MrubyValue::I32(i) => Ok(i.to_string()),
        MrubyValue::F64(f) => Ok(f.to_string()),
        MrubyValue::Bool(b) => Ok(b.to_string()),
        MrubyValue::Array(a) => {
            let mut s = String::new();
            s.push('[');
            for (i, x) in a.into_iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&value2str(mruby, x)?);
            }
            s.push(']');
            Ok(s)
        },
        MrubyValue::ArrayCmd(a) => {
            let mut s = String::new();
            for (i, x) in a.into_iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                s.push_str(&value2str(mruby, x)?);
            }
            Ok(s)
        },
        MrubyValue::ArrayCmdDeep(a) => {
            let mut s = String::new();
            for (i, x) in a.into_iter().enumerate() {
                if i > 0 {
                    s.push(' ');
                }
                if let Ok(_) = x.to_vec() {
                    // wrap ArrayCmdDeep
                    let inst = mruby.obj(ArrayCmdDeep { value: x});
                    s.push_str(&value2str(mruby, inst)?);
                } else {
                    s.push_str(&value2str(mruby, x)?);
                }
            }
            Ok(s)
        },
    }
}

fn parse_mruby_value(value: mrusty::Value) -> io::Result<MrubyValue> {
    if let Ok(s) = value.to_str() {
        return Ok(MrubyValue::Str(s.to_owned()));
    }
    if let Ok(f) = value.to_f64() {
        return Ok(MrubyValue::F64(f));
    }
    if let Ok(i) = value.to_i32() {
        return Ok(MrubyValue::I32(i));
    }
    if let Ok(b) = value.to_bool() {
        return Ok(MrubyValue::Bool(b));
    }
    if let Ok(a) = value.to_vec() {
        return Ok(MrubyValue::Array(a));
    }
    if let Ok(inst) = value.to_obj::<ArrayCmd>() {
        let inst = inst.borrow();
        return Ok(MrubyValue::ArrayCmd(inst.value.to_vec().unwrap()));
    }
    if let Ok(inst) = value.to_obj::<ArrayCmdDeep>() {
        let inst = inst.borrow();
        return Ok(MrubyValue::ArrayCmdDeep(inst.value.to_vec().unwrap()));
    }

    Err(Error::new(ErrorKind::InvalidData, format!("{}: unknown value type", term::ewrite("mruby failed")?)))
}
