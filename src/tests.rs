use super::*;

struct AddTo { 
    addend: usize,
}

impl Composable<usize,usize> for AddTo {
    fn apply(&self, input: usize) -> Result<usize> {
        Ok(input + self.addend)
    }
}

struct MultiplyBy { 
    factor: usize,
}

impl Composable<usize,usize> for MultiplyBy {
    fn apply(&self, input: usize) -> Result<usize> {
        Ok(input * self.factor)
    }
}

struct DivideBy {
    divisor: f64,
}

impl Composable<usize, f64> for DivideBy {
    fn apply(&self, input: usize) -> Result<f64> {
        if self.divisor == 0.0 {
            Err("division by zero".into())
        }
        else {
            Ok(input as f64 / self.divisor)
        }
    }
}


#[test]
fn test_compose() -> Result<()> {
    let a = AddTo { addend: 4 };
    let b = MultiplyBy { factor: 2 };
    let c = compose(a, b);
    let result = c.apply(1)?;
    assert_eq!(result, 10);
    Ok(())
}


#[test]
fn test_compose_method() -> Result<()> {
    let a = AddTo { addend: 4 };
    let b = MultiplyBy { factor: 2 };
    let c = a.compose(b);
    let result = c.apply(1)?;
    assert_eq!(result, 10);
    Ok(())
}


#[test]
fn test_composed() -> Result<()> {
    let a = AddTo { addend: 4 };
    let b = MultiplyBy { factor: 2 };
    let c = DivideBy { divisor: 2.0 };
    let d = composed![ a, b, c ];
    let result = d.apply(1)?;
    assert_eq!(result, 5.0);
    Ok(())
}



#[test]
fn test_error() -> Result<()> {
    let a = AddTo { addend: 4 };
    let b = DivideBy { divisor: 0.0 };
    let c = composed![ a, b ];
    let result = c.apply(1);
    assert!(result.is_err());
    Ok(())
}


#[test]
fn test_closure() -> Result<()> {
    let a = AddTo { addend: 4 };
    let b = |x: usize| Ok(x * x);
    let c = composed![ a, b ];
    let result = c.apply(1)?;
    assert_eq!(result, 25);
    Ok(())
}


#[test]
fn test_function() -> Result<()> {
    fn squared(x: usize) -> Result<usize> { Ok(x * x) }
    let a = AddTo { addend: 4 };    
    let b = MultiplyBy { factor: 2 };
    let c = composed![ a, squared, b ];
    let result = c.apply(1)?;
    assert_eq!(result, 50);
    Ok(())
}


struct AddToMsg { addend: usize }
 
impl Composable<usize, (usize, String)> for AddToMsg {
    fn apply(&self, input: usize) -> Result<(usize, String)> { Ok((input + self.addend, "hello".to_string())) }
}

#[test]
fn test_compose_t() -> Result<()> {
    let increment = AddToMsg { addend: 1 };
    let square = |x: usize| Ok(((x * x), "world".to_string()));
    let composition = compose_t(increment, square);
    let (result, (msg1, msg2)) = composition.apply(2)?;
    assert_eq!(result, 9);
    assert_eq!(msg1, "hello");
    assert_eq!(msg2, "world");
    Ok(())
}