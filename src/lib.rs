#![feature(core_intrinsics)]
use core::option::Option;
use core::convert::From;

#[cfg(test)]
mod test;

pub trait Array <T: Value<Self, O, N>, O: Object<T, Self, N>, N: Null<T, Self, O>> where Self: Sized{
    fn push(&mut self, v: T);
    fn new() -> Self;
}

pub trait Object<T: Value<A, Self, N>, A: Array<T, Self, N>, N: Null<T, A, Self>> where Self: Sized{
    fn insert(&mut self, k: String, v: T);
    fn new() -> Self;
}

pub trait Null<T: Value<A, O, Self>, A: Array<T, O, Self>, O: Object<T, A, Self>> where Self: Sized{
    fn new() -> Self;
}

pub trait Value<A: Array<Self, O, N>, O: Object<Self, A, N>, N: Null<Self, A, O>>:
From<String> + From<f64> + From<bool> + From<A> + From<O> + From<N> {
}

fn is_space(c: char) -> bool {
    c.is_whitespace() || c == '\t' || c == '\n' || c == '\r'
}
pub fn parse<T: Value<A, O, N>, A: Array<T, O, N>, O: Object<T, A, N>, N: Null<T, A, O>>
(src: &[char], index: &mut usize) -> Option<T> {
    while src.len() > *index && is_space(src[*index]) {
        *index += 1;
    }
    if src.len() <= *index {
        return Option::None;
    }
    if src[*index] == '{' {
        parse_object::<T, A, O, N>(src, index).map(|v| T::from(v))
    } else if src[*index] == '[' {
        parse_array::<T, A, O, N>(src, index).map(|v| T::from(v))
    } else if src[*index] == 't' {
        parse_true(src, index).map(|v| T::from(v))
    } else if src[*index] == 'f' {
        parse_false(src, index).map(|v| T::from(v))
    } else if src[*index] == '"' {
        parse_string(src, index).map(|v| T::from(v))
    } else if src[*index] == 'n' {
        parse_null::<T, A, O, N>(src, index).map(|v| T::from(v))
    } else if src[*index] == '-' || src[*index].is_ascii_digit() {
        parse_number(src, index).map(|v| T::from(v))
    } else {
        Option::None
    }
}

pub fn parse_object<T: Value<A, O, N>, A: Array<T, O, N>, O: Object<T, A, N>, N: Null<T, A, O>>
(src: &[char], index: &mut usize) -> Option<O> {
    if src.len() <= *index + 1 || src[*index] != '{' {
        return Option::None;
    }
    *index += 1;
    let mut v = O::new();
    while src.len() > *index {
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        if src[*index] == '}' {
            *index += 1;
            return Some(v);
        }
        let k = parse_string(src, index);
        if k.is_none() {
            return Option::None;
        }
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        if src[*index] != ':' {
            return Option::None;
        }
        *index += 1;
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        let c = parse::<T, A, O, N>(src, index);
        if c.is_none() {
            return Option::None;
        }
        v.insert(k.unwrap(), c.unwrap());
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        if src[*index] == ',' {
            *index += 1;
        } else if src[*index] == '}' {
            *index += 1;
            return Some(v);
        } else {
            return Option::None;
        }
    }
    Option::None
}

pub fn parse_array<T: Value<A, O, N>, A: Array<T, O, N>, O: Object<T, A, N>, N: Null<T, A, O>>
(src: &[char], index: &mut usize) -> Option<A> {
    if src.len() <= *index + 1 || src[*index] != '[' {
        return Option::None;
    }
    *index += 1;
    let mut v = A::new();
    while src.len() > *index {
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        if src[*index] == ']' {
            *index += 1;
            return Some(v);
        }
        let i = parse::<T, A, O, N>(src, index);
        if i.is_none() {
            return Option::None;
        }
        v.push(i.unwrap());
        while src.len() > *index && is_space(src[*index]) {
            *index += 1;
        }
        if src.len() <= *index {
            return Option::None;
        }
        if src[*index] == ',' {
            *index += 1;
        } else if src[*index] == ']' {
            *index += 1;
            return Some(v);
        } else {
            return Option::None;
        }
    }
    Option::None
}

fn parse_true(src: &[char], index: &mut usize) -> Option<bool> {
    let mut test_true = "true".chars();
    while src.len() > *index {
        let c = test_true.next();
        if c.is_none() {
            return Some(true);
        }
        if src[*index] == c.unwrap() {
            *index += 1;
        } else {
            return Option::None;
        }
    }
    Option::None
}
fn parse_false(src: &[char], index: &mut usize) -> Option<bool> {
    let mut test_false = "false".chars();
    while src.len() > *index {
        let c = test_false.next();
        if c.is_none() {
            return Some(false);
        }
        if src[*index] == c.unwrap() {
            *index += 1;
        } else {
            return Option::None;
        }
    }
    Option::None
}

fn parse_null<T: Value<A, O, N>, A: Array<T, O, N>, O: Object<T, A, N>, N: Null<T, A, O>>
(src: &[char], index: &mut usize) -> Option<N> {
    let mut test_null = "null".chars();
    while src.len() > *index {
        let c = test_null.next();
        if c.is_none() {
            return Some(N::new());
        }
        if src[*index] == c.unwrap() {
            *index += 1;
        } else {
            return Option::None;
        }
    }
    Option::None
}

fn parse_string_unicode(src: &[char], index: &mut usize) -> Option<char> {
    if src.len() <= *index + 4 {
        return Option::None;
    }
    let mut v: u32 = 0;
    for i in 1..5 {
        let d = src[*index + i].to_digit(16).unwrap_or(16);
        if d == 16 {
            return Option::None;
        }
        v = v * 16 + d;
    }
    *index += 4; // because there is another `*index += 1` in `parse_string`
    use core::char;
    unsafe { Some(char::from_u32_unchecked(v)) }
}

fn parse_string(src: &[char], index: &mut usize) -> Option<String> {
    if src.len() <= *index + 1 || src[*index] != '"'  {
        return Option::None;
    }
    *index += 1;
    let mut v = String::new();
    let mut escaped = false;
    while src.len() > *index {
        if escaped {
            let c = match src[*index] {
                'b' => '\u{0008}',
                'f' => '\u{000c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                '\n' => '\0',
                '\r' => '\0',
                'u' => parse_string_unicode(src, index).unwrap_or('\u{fffd}'),
                _ => src[*index]
            };
            if c == '\u{fffd}' {
                return Option::None;
            } else {
                if c!= '\0' {
                    v.push(c);
                }
            }
            escaped = false;
        } else if src[*index] == '\\' {
            escaped = true;
        } else if src[*index] == '"' {
            *index += 1;
            return Some(v);
        } else {
            v.push(src[*index]);
        }
        *index += 1;
    }
    Option::None 
}

fn parse_number_integer(src: &[char], index: &mut usize) -> f64 {
    let mut v: f64 = 0 as f64;
    while src.len() > *index && src[*index].is_ascii_digit() {
        v = v * 10.0 + src[*index].to_digit(10).unwrap() as f64;
        *index += 1;
    }
    v
}

fn parse_number_decimal(src: &[char], index: &mut usize) -> f64 {
    let head = *index;
    let v = parse_number_integer(src, index);
    v * unsafe { core::intrinsics::powif64(0.1, (*index - head) as i32) }
}

fn parse_number(src: &[char], index: &mut usize) -> Option<f64> {
    let mut v: f64 = 0 as f64;
    let mut sign = 1;
    if src.len() <= *index {
        return Option::None;
    }
    if src[*index] == '-' {
        sign = -1;
        *index += 1;
        if src.len() <= *index {
            return Option::None;
        }
    }
    if src[*index] != '0' {
        v += parse_number_integer(src, index);
    } else {
        *index += 1;
    }
    if src.len() <= *index {
        return Some(v * sign as f64);
    }
    if src[*index] == '.' {
        *index += 1;
        v += parse_number_decimal(src, index);
        if src.len() <= *index {
            return Some(v * sign as f64);
        }
    }
    if src[*index] == 'e' || src[*index] == 'E' {
        *index += 1;
        if src.len() <= *index {
            return Option::None;
        }
        let mut e_sign = 1;
        if src[*index] == '-' || src[*index] == '+' {
            e_sign = if src[*index] == '-' { -1 } else { 1 };
            *index += 1;
            if src.len() <= *index {
                return Option::None;
            }
        }
        let e = parse_number_integer(src, index);
        v *= unsafe { core::intrinsics::powif64(10.0, e as i32 * e_sign) };
    }
    Some(v * sign as f64)
}
