use std::{cell::UnsafeCell, ops::Deref, io::Write};

use once_cell::unsync;
use walkdir::WalkDir;

/// For readability. Ternary operator
macro_rules! if2 {
    ($cond:expr, $tru:expr, $fal:expr) => {
        {if $cond { $tru } else {$fal}}
    };
}

pub(crate) use if2;

/// A global variable initialised on its first access.
/// 
/// Able to be modified during runtime.
/// If you wish to guarantee project soundness do not modify in a multithreaded environment.
pub struct SuperLazy<T, F = fn() -> T> {
    lazy: UnsafeCell<unsync::Lazy<T, F>>,
}
impl<T, F: FnOnce() -> T> SuperLazy<T, F>{
    pub const fn new(init: F) -> Self {
        Self { lazy: UnsafeCell::new(unsync::Lazy::new(init)) }
    }
    pub unsafe fn elevate_mut(&self) -> &mut T{
        // self.lazy.get().as_mut().unwrap().force_mut()
        once_cell::unsync::Lazy::<T, F>::force_mut(self.lazy.get().as_mut().unwrap())
    }
}

impl<T, F> Deref for SuperLazy<T, F>{
    type Target = unsync::Lazy<T, F>;
    fn deref(&self) -> &Self::Target {
        unsafe{ self.lazy.get().as_ref().unwrap() }
    }
}

// Guys its safe I promise
unsafe impl<T, F> Sync for SuperLazy<T, F> {}

/// Equivalent to running cmd's 'pause' command
pub fn wait_for_press_enter(){
    let mut stdout = std::io::stdout();
    stdout.write("Press Enter to continue...".as_bytes()).unwrap();
    stdout.flush().unwrap();
    std::io::stdin().read_line(&mut String::new()).unwrap();
}


// I don't know the best way to do this
// pub struct AsyncDirectoryWalker{

// }
// impl AsyncDirectoryWalker{
//     pub fn new(){
//         let iter = WalkDir::new(input).into_iter().skip(1);
//         let aiter = futures::stream::iter(iter);
//     }
// }