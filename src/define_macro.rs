macro_rules! __fill_type {
    (, $default_ty:ty) => {
        $default_ty
    };
    ($some_ty:ty, $default_ty:ty) => {
        $some_ty
    };
}

macro_rules! define {
    (default-no-args
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident,
        capability: $capability:ty,
        size_hint: $size:expr,
        unsupported_msg: $unsupported_msg:literal $(,)?
        $(--add-command-implementation-errors-docs $($placeholder:tt)? )?
    ) => {
        define!(custom-impl
            $(#[$attrs])*
            definition: $visible struct $typ,
            capability: $capability,
            size_hint: $size,
            unsupported_msg: $unsupported_msg,
            write_to_impl: |self, _database, capability, ctx, target| {

                match capability.expand().with(ctx).to(target) {
                    // writing to `target` was successful
                    Ok(_) => (),
                    // writing to `target` was unsuccessful
                    Err(error) => return match error {
                        ::terminfo::Error::Parse => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::InvalidData,
                            error
                        )),
                        ::terminfo::Error::NotFound => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::NotFound,
                            error
                        )),
                        ::terminfo::Error::Expand(_) => Err(::std::io::Error::new(
                            ::std::io::ErrorKind::Other,
                            error
                        )),
                        ::terminfo::Error::Io(io_error) => Err(io_error)
                    }
                }
            },
            is_supported_impl: |self, database: &::terminfo::Database, capability| {
                true
            }
            $(--add-command-implementation-errors-docs $($placeholder)?)?
        );
    };
    (custom-impl
        $(#[$attrs:meta])*
        definition: $visible:vis struct $typ:ident $(( $( $args:tt  )+ ))?,
        capability: $capability:ty,
        size_hint: $size_hint:expr,
        unsupported_msg: $unsupported_msg:literal,
        write_to_impl: |$write_to_self_var_name:ident, $write_to_database_var_name:ident, $cap_var_name:ident, $ctx_var_name:ident, $target_var_name:ident $(,)?| $write_to_impl:block,
        is_supported_impl: |$is_supported_self_var_name:ident $(: $is_supported_self_var_ty:ty)?, $is_supported_database_var_name:ident $(: $is_supported_database_var_ty:ty)?, $is_supported_capability_var_name:ident $(: $is_supported_capability_var_ty:ty)? $(,)?| $is_supported_impl:block $(,)?
        $(--add-command-implementation-errors-docs $($placeholder:tt)? )?
    ) => {

        $(#[$attrs])*
        #[doc = concat!(
            $($($placeholder)?
                "# `Command` implementation errors\n",
                "Returns:\n",
                "- `Err(io::Error)` with an `ErrorKind` of `Unsupported` when the terminal does not support this command\n",
                "- `Err(io::Error)` with an `ErrorKind` of `InvalidData` when there was an error parsing the terminfo library\n",
                "- `Err(io::Error)` with an `ErrorKind` of `NotFound` when the terminfo entry for this terminal was not found\n",
                "- `Err(io::Error)` with an `ErrorKind` of `Other` when there was an error expanding a parameterised terminfo capability.\n",
                "May also return any other `io::Error`",
            )?
        )]
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        $visible struct $typ $(( $($args)+ ))?;

        impl crate::Command for $typ {
            fn size_hint(&self) -> Option<usize> {
                $size_hint
            }

            fn write_to(
                $write_to_self_var_name: &Self,
                $write_to_database_var_name: &::terminfo::Database,
                #[allow(unused)] $ctx_var_name: &mut ::terminfo::expand::Context,
                #[allow(unused)] $target_var_name: &mut dyn ::std::io::Write
            ) -> ::std::io::Result<()> {
                match $write_to_database_var_name.get::<$capability>() {
                    // this command is supported
                    Some($cap_var_name) => $write_to_impl,
                    // this command is unsupported
                    None => return Err(::std::io::Error::new(
                        ::std::io::ErrorKind::Unsupported,
                        $unsupported_msg
                    ))
                }
                Ok(())
            }

            fn is_supported(
                $is_supported_self_var_name: $crate::__fill_type!($($is_supported_self_var_ty)?, &Self),
                $is_supported_database_var_name: $crate::__fill_type!($($is_supported_database_var_ty)?, &::terminfo::Database),
            ) -> bool {
                match $is_supported_database_var_name.get::<$capability>() {
                    #[allow(unused)] Some($is_supported_capability_var_name) => $is_supported_impl,
                    None => false
                }
            }
        }
    };
}

macro_rules! add_semicolon_if_unit_or_tuple_struct {
    // normal struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)? {$(
            $first_field:ident: $first_field_ty:ty
            $(
                ,$field:ident: $field_ty:ty
            )* $(,)?
        )?}
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)? {$(
            $first_field: $first_field_ty
            $(
                ,$field: $field_ty
            )* $(,)?
        )?}
    };
    // tuple struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)? $(($( $first_field:ty $(,$fields:ty)* $(,)? )?))?
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)? $(($( $first_field $(,$fields)* $(,)? )?))?;
    };
    // unit struct
    (
        $visibility:vis struct $ty:ident $(<$($generic_params:tt)*>)?
    ) => {
        $visibility struct $ty $(<$($generic_params)*>)?;
    }
}

macro_rules! new_define {
    (
        $(#[$attrs:meta])*
        $visibility:vis command $ty:ident $(<$($generic_params:tt)*>)? $($fields:tt)* {
            $(#[$write_to_attrs:meta])*
            fn write_to($self_var_name:ident: &Self) $write_to_impl:block

            $(#[$is_supported_attrs:meta])*
            fn is_supported $is_supported_args:tt $is_supported_impl:block

            $(#[$size_hint_attrs:meta])*
            fn size_hint $size_hint_args:tt $size_hint_impl:block
        }
    ) => {
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        add_semicolon_if_unit_or_tuple_struct! {
            $visibility struct $ty $(<$($generic_params)*>)? $($fields)*
        }

        impl $crate::Command for $ty {

            fn size_hint(&self) -> Option<usize> $size_hint_impl

            fn is_supported(&self, database: &$crate::terminfo::Database) -> bool $is_supported_impl

            fn write_to(&self, database: &Database, ctx: &mut Context, target: &mut dyn std::io::Write) -> std::io::error::Result<()> {

            }
        }
    }
}

pub(crate) use {
    define,
    new_define,
    __fill_type,
};