use std::any::{Any,TypeId,type_name};
use std::env;
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
1=>true,
_=>false,
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
    1=>true,
    _=>false,
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
///checks if variable is a VecDeque
#[inline (always)]
pub fn is_vec_deque(_:T)->bool{
type_name::<T>().contains("VecDeque")
}
///checks if variable is a LinkedList
#[inline (always)]
pub fn is_linked_list(_:T)->bool{
type_name::<T>().contains("LinkedList")
}
///checks if variable is a BTreeMap
#[inline (always)]
pub fn is_btree_map(_:T)->bool{
type_name::<T>().contains("BTreeMap")
}
///checks if variable is a HashSet
pub fn is_hash_set(_:T)->bool{
type_name::<T>().contains("HashSet")
}
///checks if variable is a BTreeSet
#[inline (always)]
pub fn is_btree_set(_:T)->bool{
type_name::<T>().contains("BTreeSet")
}
///checks if variable is a Cow
#[inline (always)]
pub fn is_cow(_:T)->bool{
type_name::<T>().contains("Cow")
}
///checks if variable is Args
#[inline (always)]
pub fn is_args(_:T)->bool{
type_name::<T>().contains("Args")
}
///checks if variable is a PathBuf
#[inline (always)]
pub fn is_path_buf(_:T)->bool{
type_name::<T>().contains("PathBuf")
}
///checks if variable is a fn pointer
#[inline (always)]
pub fn is_fn_pointer(_:T)->bool{
type_name::<T>().contains("fn")
}
///checks if variable is a function
#[inline (always)]
pub fn is_fn(_:T)->bool{
    let name=env!("CARGO_PKG_NAME");
    if name.contains('-'){
    let string=name.replace("-","_");
    return type_name::<T>().contains(&string);
    }
    return type_name::<T>().contains(name);
}
///checks if variable is a path
#[inline (always)]
pub fn is_path(_:T)->bool{
type_name::<T>().contains("::Path")
}
///checks if variable is a tuple
#[inline (always)]
pub fn is_tuple(_:T)->bool{
type_name::<T>().contains('(') && type_name::<T>().contains(',') && type_name::<T>().contains(')')
}
///checks if variable is a command
#[inline (always)]
pub fn is_command(_:T)->bool{
type_name::<T>().contains("::Command")
}
///checks if variable is a boolean
#[inline (always)]
pub fn is_bool(_:T)->bool{
type_name::<T>().contains("bool")
}
///checks if variable is a Rc
#[inline (always)]
pub fn is_rc(_:T)->bool{
type_name::<T>().contains("Rc")
}
///checks if variable is a u8
#[inline (always)]
pub fn is_u8(_:T)->bool{
type_name::<T>().contains("u8")
}
///checks if variable is a u16
#[inline (always)]
pub fn is_u16(_:T)->bool{
type_name::<T>().contains("u16")
}
///checks if variable is a u32
#[inline (always)]
pub fn is_u32(_:T)->bool{
type_name::<T>().contains("u32")
}
///checks if variable is u64
#[inline (always)]
pub fn is_u64(_:T)->bool{
type_name::<T>().contains("u64")
}
///checks if variable is a u128
#[inline (always)]
pub fn is_u128(_:T)->bool{
type_name::<T>().contains("u128")
}
///checks if variable is a usize
pub fn is_usize(_:T)->bool{
type_name::<T>().contains("usize")
}
///checks if variable is a i8
#[inline (always)]
pub fn is_i8(_:T)->bool{
type_name::<T>().contains("i8")
}
///checks if variable is a i16
#[inline (always)]
pub fn is_i16(_:T)->bool{
type_name::<T>().contains("i16")
}
///checks if variable is a i32
#[inline (always)]
pub fn is_i32(_:T)->bool{
type_name::<T>().contains("i32")
}
///checks if variable is a i64
#[inline (always)]
pub fn is_i64(_:T)->bool{
type_name::<T>().contains("i64")
}
///check if variable is a i128
#[inline (always)]
pub fn is_i128(_:T)->bool{
type_name::<T>().contains("i128")
}
///checks if variable is a isize
#[inline (always)]
pub fn is_isize(_:T)->bool{
type_name::<T>().contains("isize")
}
///checks if variable is a TypeId
#[inline (always)]
pub fn is_type_id(_:T)->bool{
type_name::<T>().contains("TypeId")
}
///checks if variable is a NonZero
#[inline (always)]
fn is_non_zero(_:T)->bool{
type_name::<T>().contains("NonZero")
}
///checks if variable is pinned
#[inline (always)]
fn is_pin(_:T)->bool{
type_name::<T>().contains("Pin")
}
///checks if variable is a closure
#[inline (always)]
fn is_closure(_:T)->bool{
type_name::<T>().contains("closure")
}
}
#[cfg (test)]
mod tests{
use std::pin::Pin;
use std::path::{PathBuf,Path};
use std::borrow::Cow;
use std::env;
use std::rc::Rc;
use std::sync::{Mutex,Arc,Condvar,Barrier};
use std::cell::{RefCell,Cell};
use std::collections::{HashMap,BinaryHeap,VecDeque,LinkedList,BTreeMap,HashSet,BTreeSet};
use crate::TypeChecker;
use std::any::Any;
use std::process::Command;
use std::num::NonZeroI8;  
struct Test;
const POINTY:i32=4;
fn hello()->i32{
0
}
#[inline(always)]
#[test]
fn testing(){
let float:f32=30.2;
let fn_pointer:fn()->i32=hello;
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
assert_eq!(TypeChecker::is_vec_deque(VecDeque::<i32>::new()),true);
assert_eq!(TypeChecker::is_linked_list(LinkedList::<i32>::new()),true);
assert_eq!(TypeChecker::is_btree_map(BTreeMap::<f64,String>::new()),true);
assert_eq!(TypeChecker::is_hash_set(HashSet::<String>::new()),true);
assert_eq!(TypeChecker::is_btree_set(BTreeSet::<f32>::new()),true);
assert_eq!(TypeChecker::is_cow(Cow::from("End")),true);
assert_eq!(TypeChecker::is_args(env::args()),true);
assert_eq!(TypeChecker::is_fn_pointer(fn_pointer),true);
assert_eq!(TypeChecker::is_fn(hello),true);
assert_eq!(TypeChecker::is_path_buf(PathBuf::new()),true);
assert_eq!(TypeChecker::is_path(Path::new("hello.txt")),true);
assert_eq!(TypeChecker::is_tuple((30,20)),true);
assert_eq!(TypeChecker::is_tuple(()),false);
assert_eq!(TypeChecker::is_command(Command::new("sh")),true);
assert_eq!(TypeChecker::is_bool(true),true);
assert_eq!(TypeChecker::is_rc(Rc::new(30)),true);
assert_eq!(TypeChecker::is_u8(1u8),true);
assert_eq!(TypeChecker::is_u16(1u16),true);
assert_eq!(TypeChecker::is_u32(1u32),true);
assert_eq!(TypeChecker::is_u64(1u64),true);
assert_eq!(TypeChecker::is_u128(1u128),true);
assert_eq!(TypeChecker::is_usize(1usize),true);
assert_eq!(TypeChecker::is_i8(1i8),true);
assert_eq!(TypeChecker::is_i16(1i16),true);
assert_eq!(TypeChecker::is_i32(1i32),true);
assert_eq!(TypeChecker::is_i64(1i64),true);
assert_eq!(TypeChecker::is_i128(1i128),true);
assert_eq!(TypeChecker::is_isize(1isize),true);
assert_eq!(TypeChecker::is_type_id(1i8.type_id()),true);
assert_eq!(TypeChecker::is_non_zero(NonZeroI8::new(1).unwrap()),true);
assert_eq!(TypeChecker::is_closure(||println!("Hello")),true);
assert_eq!(TypeChecker::is_pin(Pin::new(&POINTY)),true);
}
}