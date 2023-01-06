use std::any::{Any,TypeId,type_name};
//write checks for every type in standard library
pub struct TypeChecker<T>(T);
impl <T:'static>TypeChecker<T>{
#[inline(always)]
///check if variable is a box
pub fn is_boxed(_:T)->bool{
type_name::<T>().contains("::Box")
}
#[inline(always)]
///checks if variable is a string
pub fn is_string(_:T)->bool{
TypeId::of::<T>()==TypeId::of::<String>()
}
#[inline(always)]
///checks if variable is a type of int ranging from u8-usize
pub fn is_unsigned(_:T)->bool{
let vec:Vec<TypeId>=vec![1u8.type_id(),1u16.type_id(),1u32.type_id(),
1u64.type_id(),1u128.type_id(),1usize.type_id()];
let mut iterated=vec.iter();
let mut count:i32=0;
for _ in 0..=vec.len(){
let wrong_id=TypeId::of::<String>();
let unsigned_type_id=iterated.next().unwrap_or(&wrong_id);
if *unsigned_type_id==TypeId::of::<T>(){
count+=1;
}
else{
count+=0;
}
}
match count{
1=>return true,
_=>return false,
}
}
#[inline(always)]
///checks if variable is a type of int ranging from i8-isize
pub fn is_integer(_:T)->bool{
    let vec:Vec<TypeId>=vec![1i8.type_id(),1i16.type_id(),1i32.type_id(),
    1i64.type_id(),1i128.type_id(),1isize.type_id()];
    let mut iterated=vec.iter();
    let mut count:i32=0;
    for _ in 0..=vec.len(){
    let wrong_id=TypeId::of::<String>();
    let unsigned_type_id=iterated.next().unwrap_or(&wrong_id);
    if *unsigned_type_id==TypeId::of::<T>(){
    count+=1;
    }
    else{
    count+=0;
    }
    }
    match count{
    1=>return true,
    _=>return false,
    }
}
#[inline (always)]
///checks if variable is a Cell
pub fn is_cell(_:T)->bool{
type_name::<T>().contains("::Cell")
}
///checks if variable is a RefCell
#[inline (always)]
pub fn is_ref_cell(_:T)->bool{
type_name::<T>().contains("::RefCell")
}
///checks if variable is a Vec
pub fn is_vec(_:T)->bool{
type_name::<T>().contains("::Vec")
}
///checks if variable is a HashMap
pub fn is_hash_map(_:T)->bool{
type_name::<T>().contains("::HashMap")
}
///checks if variable is a BinaryHeap
pub fn is_binary_heap(_:T)->bool{
type_name::<T>().contains("::BinaryHeap")
}
}
#[cfg (test)]
mod tests{
use std::cell::{RefCell,Cell};
use std::collections::{HashMap,BinaryHeap};
use crate::TypeChecker;
struct Test;
#[inline(always)]
#[test]
fn testing(){
assert_eq!(TypeChecker::is_boxed(Box::new(30)),true);
assert_eq!(TypeChecker::is_string("Hello".to_string()),true);
assert_eq!(TypeChecker::is_unsigned(1u8),true);
assert_eq!(TypeChecker::is_unsigned(3u16),true);
assert_eq!(TypeChecker::is_unsigned(2u32),true);
assert_eq!(TypeChecker::is_unsigned(10u64),true);
assert_eq!(TypeChecker::is_unsigned(5u128),true);
assert_eq!(TypeChecker::is_unsigned(9usize),true);
assert_eq!(TypeChecker::is_integer(25i8),true);
assert_eq!(TypeChecker::is_integer(80i16),true);
assert_eq!(TypeChecker::is_integer(40i32),true);
assert_eq!(TypeChecker::is_integer(12i64),true);
assert_eq!(TypeChecker::is_integer(22i128),true);
assert_eq!(TypeChecker::is_integer(55isize),true);
assert_eq!(TypeChecker::is_cell(Cell::new(30)),true);
assert_eq!(TypeChecker::is_ref_cell(RefCell::new(30)),true);
assert_eq!(TypeChecker::is_cell(RefCell::new(40)),false);
assert_eq!(TypeChecker::is_vec(Vec::<i32>::new()),true);
assert_eq!(TypeChecker::is_vec(Vec::<Test>::new()),true);
assert_eq!(TypeChecker::is_hash_map(HashMap::<String,i32>::new()),true);
assert_eq!(TypeChecker::is_binary_heap(BinaryHeap::<i32>::new()),true);
}
}