use std::io::{self, Error, ErrorKind};
use std::env;

use mrusty::*;

use crate::lib::term;

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
        end
        "#)?;
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
