use gtmpl_derive::Gtmpl;
use gtmpl::{FuncError, Context, Value};
use std::env;
use semver::{Version, VersionReq};

#[derive(Gtmpl)]
struct TheContext {
    Env: Value,
}

fn default(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::ExactlyXArgs("default".into(), 2));
    }

    if let Value::NoValue = args[1] {
        return Ok(args[0].clone());
    }

    Ok(args[1].clone())
}

fn atoi(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::AtLeastXArgs("atoi".into(), 1));
    }

    println!("{:?}", &args);
    
    if let Value::String(s) = &args[0] {
        let string = s.parse::<i64>().or(Err(FuncError::UnableToConvertFromValue))?;
        return Ok(Value::from(string));
    } else {
        return Err(FuncError::UnableToConvertFromValue);
    }
}

fn semver_compare(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::ExactlyXArgs("semverCompare".into(), 2));
    }

    if let Value::NoValue = args[1] {
        return Ok(Value::Bool(false));
    }

    let req = VersionReq::parse(&args[0].to_string());

    if let Ok(req) = req {
        let version = Version::parse(&args[1].to_string());

        if let Ok(version) = version {
            return Ok(Value::Bool(req.matches(&version)));
        }
    } 

    Ok(Value::Bool(false))
    
}

pub fn render(input: String) -> anyhow::Result<String> {

    let mut tmpl = gtmpl::Template::default();
    tmpl.add_func("semverCompare", semver_compare);
    tmpl.add_func("default", default);
    tmpl.add_func("atoi", atoi);

    let ctx = TheContext {
        Env: Value::Map(env::vars().map(|(key, val)| { (key, Value::String(val)) }).collect())
    };

    tmpl.parse(input).unwrap();

    let output = tmpl.render(&Context::from(ctx));

    Ok(output?)
}