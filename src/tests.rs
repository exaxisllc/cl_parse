use crate::cl_def::CommandLineDef;

#[test]
fn should_return_default_boolean_false() {
  let args=vec![String::from("test")];
  let cl = CommandLineDef::new()
      .add_option(vec!["-b"], None, Some("true"), "A boolean value always defaults to false")
      .add_option(vec!["-c"], None, Some("false"), "Another boolean value")
      .add_option(vec!["-d"], None, None, "Another boolean value")
      .add_flag(vec!["-f"], "A flag")
      .parse(args.into_iter());

  let b:bool = cl.option("-b");
  assert_eq!(b, false);

  let c:bool = cl.option("-c");
  assert_eq!(c, false);

  let d:bool = cl.option("-d");
  assert_eq!(d, false);

  let f:bool = cl.option("-f");
  assert_eq!(f, false);
}

#[test]
fn should_return_boolean() {
  let args=vec![String::from("test"), String::from("-c")];
  let cl = CommandLineDef::new()
      .add_option(vec!["-b"], None, Some("false"), "A boolean value")
      .add_option(vec!["-c"], None, Some("false"), "Another boolean value")
      .parse(args.into_iter());

  let b:bool = cl.option("-b");
  assert_eq!(b, false);

  let c:bool = cl.option("-c");
  assert_eq!(c, true);
}

#[test]
fn should_return_i16() {
  let args=vec![
    "test".to_string(),
    "--negative".to_string(), "-1".to_string(),
    "--positive".to_string(), "1".to_string(),
  ];
  let cl = CommandLineDef::new()
      .add_option(vec!["--negative"], Some("neg"), None, "A negative value")
      .add_option(vec!["--positive"], Some("pos"), None, "A positive value")
      .parse(args.into_iter());

  assert_eq!(cl.program_name(), "test");

  let neg:i16 = cl.option("--negative");
  assert_eq!(neg, -1);

  let pos:i16 = cl.option("--positive");
  assert_eq!(pos, 1);
}

#[test]
fn aliases_should_have_value() {
  let args=vec![
    "test".to_string(),
    "-n".to_string(), "-1".to_string(),
    "-p".to_string(), "1".to_string(),
  ];
  let cl = CommandLineDef::new()
      .add_option(vec!["-n","--negative"], Some("neg"), None, "A negative value")
      .add_option(vec!["-p","--positive"], Some("pos"), None, "A positive value")
      .parse(args.into_iter());

  assert_eq!(cl.program_name(), "test");

  let n:i16 = cl.option("-n");
  assert_eq!(n, -1);

  let neg:i16 = cl.option("--negative");
  assert_eq!(neg, n);

  let p:i16 = cl.option("-p");
  assert_eq!(p, 1);

  let pos:i16 = cl.option("--positive");
  assert_eq!(pos, p);
}

#[test]
fn should_capture_interleaved_args() {
  let args=vec![
    "test".to_string(),
    "arg1".to_string(),
    "--bool".to_string(),
    "arg2".to_string(),
    "-n".to_string(), "-1".to_string(),
    "arg3".to_string(),
  ];
  let cl = CommandLineDef::new()
      .add_option(vec!["-b","--bool"], None, None, "A boolean value")
      .add_option(vec!["-n","--num"], Some("num"), None, "A numeric value")
      .add_argument("arg-0")
      .add_argument("arg-1")
      .add_argument("arg-2")
      .parse(args.into_iter());

  assert_eq!(cl.program_name(), "test");

  let b:bool = cl.option("-b");
  assert_eq!(b, true);

  let n:i16 = cl.option("-n");
  assert_eq!(n, -1);

  assert_eq!(cl.arguments(), 3);

  let arg0:String = cl.argument(0);
  assert_eq!(arg0, "arg1");

  let arg1:String = cl.argument(1);
  assert_eq!(arg1, "arg2");

  let arg2:String = cl.argument(2);
  assert_eq!(arg2, "arg3");
}

#[test]
#[should_panic(expected = "Option --increment is required")]
fn should_panic_for_missing_required_option() {
  let args=vec![String::from("test"), String::from("-c")];
  let cl = CommandLineDef::new()
      .add_option(vec!["--increment"], Some("numeric value"), None, "A number to increment by")
      .add_flag(vec!["-c"], "Another boolean value")
      .parse(args.into_iter());

  let inc:i16 = cl.option("--increment");
  assert_eq!(inc, -1);
}

#[test]
#[should_panic(expected = "Option -c not defined")]
fn should_panic_for_option_undefined() {
  let args=vec![String::from("test"), String::from("-c")];
  let cl = CommandLineDef::new()
      .parse(args.into_iter());

  let c:bool = cl.option("-c");
  assert_eq!(c, true);
}

#[test]
#[should_panic(expected = "A value is required for option --increment")]
fn should_panic_for_missing_value() {
  let args=vec![String::from("test"), String::from("--increment")];
  let cl = CommandLineDef::new()
      .add_option(vec!["--increment"], Some("numeric value"), None, "A number to increment by")
      .parse(args.into_iter());

  let inc:i16 = cl.option("--increment");
  assert_eq!(inc, -1);
}

#[test]
#[should_panic(expected = "1 arguments defined, 2 arguments found")]
fn should_panic_for_too_many_args() {
  let args=vec![String::from("test"), String::from("arg1"), String::from("arg2")];
  let cl = CommandLineDef::new()
      .add_argument("arg-1")
      .parse(args.into_iter());

  let arg1:String = cl.argument(0);
  assert_eq!(arg1, "arg-1");
}

#[test]
#[should_panic(expected = "3 arguments defined, 2 arguments found")]
fn should_panic_for_too_few_args() {
  let args=vec![String::from("test"), String::from("arg1"), String::from("arg2")];
  let cl = CommandLineDef::new()
      .add_argument("arg-1")
      .add_argument("arg-2")
      .add_argument("arg-3")
      .parse(args.into_iter());

  let arg1:String = cl.argument(0);
  assert_eq!(arg1, "arg-1");
}
