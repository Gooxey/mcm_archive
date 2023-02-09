//! This module provides the [`class_lock! macro`](crate::class_lock!) which can be used to assign a `class_lock` variable if it does not matter that the class gets reset on a failed
//! attempt to acquire the lock.


/// This macro can be used to assign a `class_lock` variable if it does not matter that the class gets reset on a failed attempt to acquire the lock. Since this is no real
/// error handling, the function will return a [`fatal error`](crate::mcmanage_error::MCManageError::FatalError).
/// 
/// # Example
/// ```ignore
/// fn anyFunc<T, C>(class: &Mutex<T>, log_messages: bool) -> Result<(), MCManageError>
/// where
///     T: ConcurrentClass<T, C> + std::marker::Send + 'static,
///     C: ConfigTrait
/// {
///     let class_lock = class_lock!();
///     let mut class_lock_mut = class_lock!();
/// 
///     /* Code using the variable `class_lock` */
///     /* Code using the variable `class_lock_mut` */
///     # Ok(())
/// }
/// ```
#[macro_export]
macro_rules! class_lock {
    ($class: ident, $log_messages: ident) => {
        match $class.lock() {
            Ok(lock) => {
                lock
            }
            Err(erro) => {
                let class_lock = erro.into_inner();
                if $log_messages { log!("erro", Self::get_name_poison_error(&class_lock), "This struct got corrupted."); }
                Self::reset($class);
                return Err(crate::mcmanage_error::MCManageError::FatalError);
            }
        }
    };
}