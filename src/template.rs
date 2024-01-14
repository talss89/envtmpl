use gtmpl_derive::Gtmpl;
use gtmpl::{FuncError, Context, Value, gtmpl_fn};
use gtmpl_value::Number;
use std::{env, path::Path, collections::HashMap};
use semver::{Version, VersionReq};
use path_dedot::*;
use std::os::unix::fs::MetadataExt;
use url::*;

#[derive(Gtmpl)]
struct TheContext {
    Env: Value,
    Os: Value
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

fn list(args: &[Value]) -> Result<Value, FuncError> {
    Ok(Value::Array(args.to_vec()))
}

fn has(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::ExactlyXArgs("has".into(), 2));
    }

    if let Value::Array(arr) = &args[1] {
        return Ok(Value::from(arr.into_iter().fold(false,|acc, e| { e.eq(&args[0]) || acc })));
    } else {
        return Err(FuncError::UnableToConvertFromValue);
    }
}

fn lower(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("lower".into(), 1));
    }

    if let Value::String(s) = &args[0] {
        return Ok(Value::from(s.to_uppercase()));
    } else {
        return Err(FuncError::UnableToConvertFromValue);
    }
}

fn upper(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("upper".into(), 1));
    }

    if let Value::String(s) = &args[0] {
        return Ok(Value::from(s.to_uppercase()));
    } else {
        return Err(FuncError::UnableToConvertFromValue);
    }
}

fn is_true(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("isTrue".into(), 1));
    }

    if let Value::String(s) = &args[0] {
        return Ok(Value::from(!(s.to_uppercase() == "FALSE" || s == "0" || s == "")));
    } else {
        return Ok(Value::Bool(false));
    }
}

fn quote(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::AtLeastXArgs("quote".into(), 1));
    }    
    if let Value::String(s) = &args[0] {
        return Ok(Value::from(format!("\"{}\"", snailquote::escape(s))));
    } else {
        return Err(FuncError::UnableToConvertFromValue);
    }
}

fn max(args: &[Value]) -> Result<Value, FuncError> {

    let nums: Vec<Number> = args.iter().filter_map(|v| match v {
        Value::Number(v) => Some(v.clone()),
        _ => None,
    }).collect::<Vec<Number>>();

    if nums.len() == 0 {
        return Err(FuncError::UnableToConvertFromValue);
    }

    match nums.into_iter().max_by(|x,y| {
        x.partial_cmp(y).unwrap()
    }) {
        Some(max) => return Ok(Value::Number(max)),
        None => return Err(FuncError::Generic("Unable to compare numbers".to_string())),
    }
}

fn min(args: &[Value]) -> Result<Value, FuncError> {

    let nums: Vec<Number> = args.iter().filter_map(|v| match v {
        Value::Number(v) => Some(v.clone()),
        _ => None,
    }).collect::<Vec<Number>>();

    if nums.len() == 0 {
        return Err(FuncError::UnableToConvertFromValue);
    }

    match nums.into_iter().min_by(|x,y| {
        x.partial_cmp(y).unwrap()
    }) {
        Some(max) => return Ok(Value::Number(max)),
        None => return Err(FuncError::Generic("Unable to compare numbers".to_string())),
    }
}

gtmpl_fn!(
    fn trim_all(delim: String, subject: String) -> Result<String, FuncError> {
        Ok(subject.strip_prefix(&delim).or(Some(&subject)).unwrap().to_string().strip_suffix(&delim).or(Some(&subject)).unwrap().to_string())
    }
);

gtmpl_fn!(
    fn trim_prefix(delim: String, subject: String) -> Result<String, FuncError> {
        Ok(subject.strip_prefix(&delim).or(Some(&subject)).unwrap().to_string())
    }
);

gtmpl_fn!(
    fn trim_suffix(delim: String, subject: String) -> Result<String, FuncError> {
        Ok(subject.strip_suffix(&delim).or(Some(&subject)).unwrap().to_string())
    }
);

gtmpl_fn!(
    fn clean(path: String) -> Result<String, FuncError> {
        let path = Path::new(&path);
        Ok(path.parse_dot().unwrap().to_str().unwrap().to_string())
    }
);

gtmpl_fn!(
    fn add(a: i64, b: i64) -> Result<i64, FuncError> {
        Ok(a + b)
    }
);

gtmpl_fn!(
    fn add1(a: i64) -> Result<i64, FuncError> {
        Ok(a + 1)
    }
);

gtmpl_fn!(
    fn div(a: i64, b: i64) -> Result<i64, FuncError> {
        Ok(a / b)
    }
);

gtmpl_fn!(
    fn sub(a: i64, b: i64) -> Result<i64, FuncError> {
        Ok(a - b)
    }
);

fn compact(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("compact".into(), 1));
    }

    if let Value::Array(arr) = &args[0] {

        let strings: Vec<String> = arr.iter().filter_map(|v| match v {
            Value::String(v) => {
                if v != "" {
                    Some(v.clone())
                } else {
                    None
                }
            },
            _ => None,
        }).collect::<Vec<String>>();
        
        return Ok(Value::from(strings))
    } else {
        return Err(FuncError::Generic("Cannot compact non-list".to_string()));
    }
}

fn split_list(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 2 {
        return Err(FuncError::ExactlyXArgs("splitList".into(), 2));
    }

    let delim = match &args[0] {
        Value::String(s) => s,
        _ => { return Err(FuncError::UnableToConvertFromValue); }
    };

    let subject = match &args[1] {
        Value::String(s) => s,
        _ => { return Err(FuncError::UnableToConvertFromValue); }
    };

    if subject == "" {
        return Ok(Value::Array([].to_vec()))
    }

    Ok(Value::Array(subject.split(delim).map(|s| Value::String(s.to_string())).collect::<Vec<Value>>()))
}

gtmpl_fn!(
    fn nospace(subject: String) -> Result<String, FuncError> {
        Ok(str::replace(&subject, " ", ""))
    }
);

gtmpl_fn!(
    fn replace(subject: String, find: String, replace: String) -> Result<String, FuncError> {
        println!("{} {} {}", subject, find, replace);
        Ok(str::replace(&subject, &find, &replace))
    }
);

fn to_string(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("toString".into(), 1));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::from(n.to_string())),
        _ => Err(FuncError::UnableToConvertFromValue)
    }
}

gtmpl_fn!(
    fn has_prefix(prefix: String, subject: String) -> Result<bool, FuncError> {
        Ok(subject.starts_with(&prefix))
    }
);

gtmpl_fn!(
    fn has_suffix(suffix: String, subject: String) -> Result<bool, FuncError> {
        Ok(subject.ends_with(&suffix))
    }
);

gtmpl_fn!(
    fn empty(subject: String) -> Result<bool, FuncError> {
        Ok(subject == "")
    }
);

fn ternary(args: &[Value]) -> Result<Value, FuncError> {
    if args.len() != 3 {
        return Err(FuncError::ExactlyXArgs("ternary".into(), 3));
    }

    match args[2] {
        Value::Bool(state) => {
            if state {
                return Ok(args[0].clone());
            } else {
                return Ok(args[1].clone());
            }           
        },
        _ => {
            return Err(FuncError::UnableToConvertFromValue);
        }
    }

}

fn dict(args: &[Value]) -> Result<Value, FuncError> {
    let argvec = args.to_vec();
    let mut out: HashMap<String, Value> = HashMap::new();

    if &args.len() % 2 != 0 {
        return Err(FuncError::Generic("dict must have even number of args".to_string()));
    }

    for argpair in argvec.chunks(2).into_iter() {
        out.insert(argpair[0].to_string().clone(), argpair[1].clone());
    }

    Ok(Value::Object(out))
}

fn url_parse(args: &[Value]) -> Result<Value, FuncError> {
    
    if args.len() != 1 {
        return Err(FuncError::ExactlyXArgs("urlParse".into(), 1));
    }

    match &args[0] {
        Value::String(url) => {
            let url = Url::parse(&url).unwrap();
            let mut out: HashMap<String, Value> = HashMap::new();

            out.insert("scheme".to_string(), Value::String(url.scheme().to_string()));
            out.insert("host".to_string(), Value::String(url.host_str().or(Some("")).unwrap().to_string()));
            out.insert("path".to_string(), Value::String(url.path().to_string()));
            out.insert("query".to_string(), Value::String(url.query().or(Some("")).unwrap().to_string()));
            out.insert("opaque".to_string(), Value::Nil);
            out.insert("fragment".to_string(), Value::String(url.fragment().or(Some("")).unwrap().to_string()));
            out.insert("userinfo".to_string(), Value::String(format!("{}{}", url.username(), match url.password() {
                Some(pass) => format!(":{}", pass),
                None => "".to_string()
            })));


            Ok(Value::Object(out))

        }
        _ => Err(FuncError::UnableToConvertFromValue)
    }
}

pub fn render(input: String) -> anyhow::Result<String> {

    let mut tmpl = gtmpl::Template::default();

    tmpl.add_func("semverCompare", semver_compare);
    tmpl.add_func("default", default);
    tmpl.add_func("atoi", atoi);
    tmpl.add_func("list", list);
    tmpl.add_func("has", has);
    tmpl.add_func("lower", lower);
    tmpl.add_func("upper", upper);
    tmpl.add_func("isTrue", is_true);
    tmpl.add_func("quote", quote);
    tmpl.add_func("trimAll", trim_all);
    tmpl.add_func("trimPrefix", trim_prefix);
    tmpl.add_func("hasPrefix", has_prefix);
    tmpl.add_func("trimSuffix", trim_suffix);
    tmpl.add_func("hasSuffix", has_suffix);
    tmpl.add_func("clean", clean);
    tmpl.add_func("max", max);
    tmpl.add_func("min", min);
    tmpl.add_func("add", add);
    tmpl.add_func("add1", add1);
    tmpl.add_func("div", div);
    tmpl.add_func("sub", sub);
    tmpl.add_func("compact", compact);
    tmpl.add_func("splitList", split_list);
    tmpl.add_func("nospace", nospace);
    tmpl.add_func("replace", replace);
    tmpl.add_func("toString", to_string);
    tmpl.add_func("empty", empty);
    tmpl.add_func("ternary", ternary);
    tmpl.add_func("dict", dict);
    tmpl.add_func("urlParse", url_parse);

    let mut os_vars: HashMap<String, Value> = HashMap::new();

    os_vars.insert("UID".to_string(), Value::from(std::fs::metadata("/proc/self").map(|m| m.uid()).or::<u32>(Ok(0)).unwrap().to_string()));

    let ctx = TheContext {
        Env: Value::Map(env::vars().map(|(key, val)| { (key, Value::String(val)) }).collect()),
        Os: Value::Object(os_vars)
    };

    tmpl.parse(input).unwrap();

    let output = tmpl.render(&Context::from(ctx));

    Ok(output?)
}