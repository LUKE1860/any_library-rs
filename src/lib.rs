use std::any::{Any,TypeId,type_name};
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
///checks if variable is a type of unsigned ranging from u8-usize
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
#[inline (always)]
pub fn is_vec(_:T)->bool{
type_name::<T>().contains("::Vec")
}
///checks if variable is a HashMap
#[inline (always)]
pub fn is_hash_map(_:T)->bool{
type_name::<T>().contains("::HashMap")
}
///checks if variable is a BinaryHeap
#[inline (always)]
pub fn is_binary_heap(_:T)->bool{
type_name::<T>().contains("::BinaryHeap")
}
///checks if variable is a ()
#[inline (always)]
pub fn is_unit(_:T)->bool{
type_name::<T>().contains("()")
}
///checks if variable is a &dyn Any or dyn Any
#[inline (always)]
pub fn is_any(_:T)->bool{
type_name::<T>().contains("::Any")
}
///checks if variable is a char
#[inline (always)]
pub fn is_char(_:T)->bool{
type_name::<T>().contains("char")
}
///checks if variable is a &str
#[inline (always)]
pub fn is_str(_:T)->bool{
type_name::<T>().contains("&str")
}
///checks if variable is a float
#[inline (always)]
pub fn is_float(_:T)->bool{
type_name::<T>().contains("f32")
}
///checks if variable is a double
#[inline (always)]
pub fn is_double(_:T)->bool{
type_name::<T>().contains("f64")
}
///checks if variable is a *const pointer or *mut pointer 
#[inline (always)]
pub fn is_ptr(_:T)->bool{
type_name::<T>().contains("*const") || type_name::<T>().contains("*mut")
}
///checks if variable is a Mutex
#[inline (always)]
pub fn is_mutex(_:T)->bool{
type_name::<T>().contains("::Mutex")
}
///checks if variable is a Condvar
#[inline (always)]
pub fn is_condvar(_:T)->bool{
type_name::<T>().contains("Condvar")
}
///checks if variable is a Arc
#[inline (always)]
pub fn is_arc(_:T)->bool{
type_name::<T>().contains("Arc")
}
///checks if variable is a Barrier
#[inline (always)]
pub fn is_barrier(_:T)->bool{
type_name::<T>().contains("Barrier")
}
}
#[cfg (test)]
mod tests{
use std::sync::{Mutex,Arc,Condvar,Barrier};
use std::cell::{RefCell,Cell};
use std::collections::{HashMap,BinaryHeap};
use crate::TypeChecker;
use std::any::Any;
struct Test;
#[inline(always)]
#[test]
fn testing(){
let float:f32=30.2;
let mut num=30;
let any:&dyn Any=&40;
let mut_ptr:*mut i32=&mut num;
let const_ptr:*const f32=&float;
let any_box:Box<dyn Any>=Box::new(40);
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
assert_eq!(TypeChecker::is_unit(()),true);
assert_eq!(TypeChecker::is_any(Box::<&dyn Any>::new(&30)),true);
assert_eq!(TypeChecker::is_any(any_box),true);
assert_eq!(TypeChecker::is_any(any),true);
assert_eq!(TypeChecker::is_char('s'),true);
assert_eq!(TypeChecker::is_str("hello"),true);
assert_eq!(TypeChecker::is_float(float),true);
assert_eq!(TypeChecker::is_double(30.2),true);
assert_eq!(TypeChecker::is_ptr(const_ptr),true);
assert_eq!(TypeChecker::is_ptr(mut_ptr),true);
assert_eq!(TypeChecker::is_mutex(Arc::new(Mutex::new(0))),true);
assert_eq!(TypeChecker::is_arc(Arc::new(Mutex::new(2))),true);
assert_eq!(TypeChecker::is_condvar(Arc::new((Mutex::new(false), Condvar::new()))),true);
assert_eq!(TypeChecker::is_barrier(Arc::new(Barrier::new(10))),true);
}
}