use crate::{evaluator::NULL, object::Object};

/// Provides built-in functions for the Monkey language interpreter.
pub struct Builtins;

impl Builtins {
    /// Returns a list of all built-in functions.
    pub fn all_builtins(&self) -> Vec<(String, Object)> {
        vec![
            (String::from("len"), Object::Builtin(b_len)),
            (String::from("first"), Object::Builtin(b_first)),
            (String::from("last"), Object::Builtin(b_last)),
            (String::from("rest"), Object::Builtin(b_rest)),
            (String::from("push"), Object::Builtin(b_push)),
            (String::from("puts"), Object::Builtin(b_puts)),
        ]
    }
}

/// Built-in function: len(obj)
/// Returns the length of a string or array, or an error otherwise.
fn b_len(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }
    return match &args[0] {
        Object::StringObj(string_literal) => Object::Integer(string_literal.len() as i64),
        Object::Array(arr) => Object::Integer(arr.len() as i64),
        other => Object::Error(format!(
            "argument to 'len' not supported, got {}",
            other.object_type()
        )),
    };
}

/// Built-in function: first(array)
/// Returns the first element of an array, or null if empty or not an array.
fn b_first(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            args[0].object_type()
        ));
    }

    if let Object::Array(arr) = &args[0] {
        if arr.len() > 0 {
            return arr[0].clone();
        }
    }
    NULL
}

/// Built-in function: last(array)
/// Returns the last element of an array, or null if empty or not an array.
fn b_last(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            args[0].object_type()
        ));
    }

    if let Object::Array(arr) = &args[0] {
        if arr.len() > 0 {
            return arr[arr.len() - 1].clone();
        }
    }
    NULL
}

/// Built-in function: rest(array)
/// Returns all elements of an array except the first, or null if not an array.
fn b_rest(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=1",
            args.len()
        ));
    }

    if args[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            args[0].object_type()
        ));
    }

    if let Object::Array(arr) = &args[0] {
        if arr.len() > 0 {
            let new_elements = arr[1..].to_vec();
            return Object::Array(new_elements);
        }
    }
    NULL
}

/// Built-in function: push(array, value)
/// Returns a new array with value appended, or an error if not an array.
fn b_push(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error(format!(
            "wrong number of arguments. got={}, want=2",
            args.len()
        ));
    }

    if args[0].object_type() != "ARRAY" {
        return Object::Error(format!(
            "argument to `first` must be ARRAY, got={}",
            args[0].object_type()
        ));
    }

    if let Object::Array(arr) = &args[0] {
        if arr.len() > 0 {
            let mut new_elements = arr.clone();
            new_elements.push(args[1].clone());
            return Object::Array(new_elements);
        }
    }
    NULL
}

/// Built-in function: puts(...args)
/// Prints all arguments to stdout. Returns null.
fn b_puts(args: Vec<Object>) -> Object {
    for arg in args {
        println!("{}", arg);
    }
    NULL
}
