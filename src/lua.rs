use hematita::{
    ast::{
        lexer::Lexer,
        parser::{parse_block, Error as ParserError, TokenIterator},
        Error as ASTError,
    },
    compiler::compile_block,
    vm::value::Function,
};
use hematita::{
    lua_tuple,
    vm::{
        value::{Table, Value},
        VirtualMachine,
    },
};
use std::sync::Arc;
use std::{fs::File, io::Read};

pub fn compile<'n>(code: &str) -> Result<Function<'n>, hematita::ast::Error> {
    let tokens = Lexer {
        source: code.chars().peekable(),
    };
    let mut tokens = TokenIterator(tokens.peekable());
    let block = parse_block(&mut tokens)?;
    if let Some(token) = tokens.next().transpose()? {
        return Err(ASTError::Parser(ParserError(Some(token))));
    };
    let chunk = compile_block(&block);
    Ok(chunk.into())
}

pub fn require<'n>(
    arguments: Arc<Table<'n>>,
    vm: &VirtualMachine,
) -> Result<Arc<Table<'n>>, String> {
    let modname = arguments.index(&Value::Integer(1)).option();

    if let Some(modname) = modname {
        let file = File::open(modname.to_string());

        if let Ok(mut file) = file {
            let mut code = String::new();
            let res = file.read_to_string(&mut code);

            if res.is_err() {
                return Err(res.unwrap_err().to_string());
            }

            match compile(code.as_str()) {
                Ok(function) => match vm.execute(&function, Table::default().arc()) {
                    Ok(_res) => Ok(lua_tuple![].arc()),
                    Err(error) => Err(format!("runtime error: {}", error)),
                },
                Err(error) => Err(format!("syntax error: {}", error)),
            }
        } else {
            Err("runtime error: module not found".to_string())
        }
    } else {
        Err("runtime error: module not found".to_string())
    }
}
